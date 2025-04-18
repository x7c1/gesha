use crate::v3_0::components::schemas::TypeShape::Proper;
use crate::v3_0::components::schemas::{
    InlineSchema, InlineShape, Optionality, RefShape, TypePath,
};
use gesha_core::broken;
use gesha_core::conversions::{Error, Result};
use gesha_rust_types::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{FormatModifier, OpenApiDataType, SchemaObject};
use tracing::{error, warn};

#[derive(Clone, Debug)]
pub enum TypeShape {
    Proper {
        data_type: DataType,
        optionality: Optionality,
    },
    Array {
        type_shape: Box<TypeShape>,
        optionality: Optionality,
    },
    Ref(RefShape),
    Expanded {
        type_path: TypePath,
        optionality: Optionality,
    },
    Inline(Box<InlineShape>),

    /// required:true, nullable:true
    Option(Box<TypeShape>),

    /// required:false, nullable:false
    Maybe(Box<TypeShape>),

    /// required:false, nullable:true
    Patch(Box<TypeShape>),
}

impl TypeShape {
    pub fn from_case(schema_case: SchemaCase, is_required: bool) -> Result<TypeShape> {
        let shape = match schema_case {
            Schema(object) => Self::from_object(*object, is_required)?,
            Reference(target) => TypeShape::Ref(RefShape::new(target, is_required)?),
        };
        Ok(shape)
    }

    pub fn from_object(object: SchemaObject, is_required: bool) -> Result<TypeShape> {
        let data_type = object
            .data_type
            .clone()
            .or_else(|| object.all_of.is_some().then_some(OpenApiDataType::Object))
            .or_else(|| object.one_of.is_some().then_some(OpenApiDataType::Object))
            .or_else(|| {
                object
                    .enum_values
                    .is_some()
                    .then_some(OpenApiDataType::Object)
            })
            .ok_or_else(|| {
                error!(
                    "type unspecified:\n{object:#?}\n  at {file}:{line}",
                    file = file!(),
                    line = line!()
                );
                Error::Unimplemented {
                    message: "type unspecified".to_string(),
                }
            })?;

        let to_type = TypeFactory {
            object,
            is_required,
        };
        to_type.apply(data_type)
    }

    pub fn resolve_optionality(self) -> Result<Self> {
        let optionality = match &self {
            Self::Proper { optionality, .. }
            | Self::Array { optionality, .. }
            | Self::Expanded { optionality, .. } => optionality,
            Self::Option(_) | Self::Maybe(_) | Self::Patch(_) => {
                // already resolved
                return Ok(self);
            }
            Self::Inline { .. } => Err(broken!(self))?,
            Self::Ref { .. } => Err(broken!(self))?,
        };
        let resolved = match (optionality.is_required, optionality.is_nullable) {
            (true, false) => self,
            (false, false) => TypeShape::Maybe(Box::new(self)),
            (true, true) => TypeShape::Option(Box::new(self)),
            (false, true) => TypeShape::Patch(Box::new(self)),
        };
        Ok(resolved)
    }

    pub fn require(mut self) -> Self {
        match self {
            Self::Proper {
                ref mut optionality,
                ..
            }
            | Self::Array {
                ref mut optionality,
                ..
            }
            | Self::Expanded {
                ref mut optionality,
                ..
            } => optionality.is_required = true,

            Self::Ref(RefShape {
                ref mut is_required,
                ..
            }) => *is_required = true,

            Self::Inline(ref mut shape) => {
                shape.set_required(true);
            }

            Self::Option(_) | Self::Maybe(_) | Self::Patch(_) => { /* nop */ }
        }
        self
    }

    pub fn define(self) -> Result<DataType> {
        let data_type = match self {
            Self::Proper { data_type, .. } => data_type,
            Self::Array { type_shape, .. } => DataType::Vec(Box::new((*type_shape).define()?)),
            Self::Expanded { type_path, .. } => type_path.into(),
            Self::Option(type_shape) | Self::Maybe(type_shape) => {
                DataType::Option(Box::new((*type_shape).define()?))
            }
            Self::Patch(type_shape) => DataType::Patch(Box::new((*type_shape).define()?)),
            Self::Ref { .. } => Err(broken!(self))?,
            Self::Inline { .. } => Err(broken!(self))?,
        };
        Ok(data_type)
    }

    pub fn get_optionality(&self) -> Option<Optionality> {
        match self {
            Self::Proper { optionality, .. }
            | Self::Array { optionality, .. }
            | Self::Expanded { optionality, .. } => Some(optionality.clone()),
            Self::Inline(shape) => Some(shape.get_optionality().clone()),
            Self::Ref { .. } | Self::Option(_) | Self::Maybe(_) | Self::Patch(_) => None,
        }
    }

    pub fn set_optionality(mut self, target: Optionality) -> TypeShape {
        match self {
            Self::Proper {
                ref mut optionality,
                ..
            }
            | Self::Array {
                ref mut optionality,
                ..
            }
            | Self::Expanded {
                ref mut optionality,
                ..
            } => *optionality = target,

            Self::Inline(ref mut shape) => shape.set_optionality(target),

            Self::Ref { .. } | Self::Option(_) | Self::Maybe(_) | Self::Patch(_) => { /* nop */ }
        }
        self
    }
}

/// OpenApiDataType -> TypeShape
struct TypeFactory {
    object: SchemaObject,
    is_required: bool,
}

impl TypeFactory {
    fn apply(self, data_type: OpenApiDataType) -> Result<TypeShape> {
        use crate::v3_0::components::schemas::TypeShape::Proper;
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        let optionality = Optionality {
            is_required: self.is_required,
            is_nullable: self.object.nullable.unwrap_or(false),
        };
        if self.object.enum_values.is_some() {
            let inline = InlineShape::Enum(InlineSchema::new(self.object, optionality)?);
            return Ok(inline.into());
        }
        match (&data_type, &self.object.format) {
            (ot::Array, _) => self.items_to_shape(),
            (ot::Boolean, _) => Ok(Proper {
                data_type: tp::Bool,
                optionality,
            }),
            (ot::Integer, Some(fm::Int32)) => Ok(Proper {
                data_type: tp::Int32,
                optionality,
            }),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Proper {
                data_type: tp::Int64,
                optionality,
            }),
            (ot::Number, Some(fm::Float)) => Ok(Proper {
                data_type: tp::Float32,
                optionality,
            }),
            (ot::Number, Some(fm::Double) | None) => Ok(Proper {
                data_type: tp::Float64,
                optionality,
            }),
            (ot::String, _) => Ok(Proper {
                data_type: tp::String,
                optionality,
            }),
            (ot::Object, _) => {
                let inline = InlineShape::new(self.object, optionality)?;
                Ok(inline.into())
            }
            // fall back to the default type if the format is unsupported.
            (ot::Number, Some(x)) => Ok(to_default(x, ot::Number, tp::Float64, optionality)),
            (ot::Integer, Some(x)) => Ok(to_default(x, ot::Integer, tp::Int64, optionality)),
        }
    }

    fn items_to_shape(self) -> Result<TypeShape> {
        let items = self
            .object
            .items
            .unwrap_or_else(|| unimplemented!("array must have items"));

        let items_shape = TypeShape::from_case(items.into(), /* is_required */ true)?;
        let type_shape = TypeShape::Array {
            type_shape: Box::new(items_shape),
            optionality: Optionality {
                is_required: self.is_required,
                is_nullable: self.object.nullable.unwrap_or(false),
            },
        };
        Ok(type_shape)
    }
}

fn to_default(
    format: &FormatModifier,
    original: OpenApiDataType,
    data_type: DataType,
    optionality: Optionality,
) -> TypeShape {
    warn!("Unsupported format: {format} (type: {original}).");
    Proper {
        data_type,
        optionality,
    }
}
