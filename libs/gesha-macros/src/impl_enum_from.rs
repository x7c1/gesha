#[macro_export]
macro_rules! impl_enum_from {
    (
        $enum_name:ident,
        $error_type:ty,
        [ $( $type:ty  ),* ],
        $pairs:tt,
    ) => {
        $(
            $crate::private_impl_enum_from!(
                $enum_name,
                $type,
                $pairs,
            );
            $crate::private_impl_enum_try_from!(
                $enum_name,
                $error_type,
                $type,
                $pairs,
            );
        )*
    };
}

#[macro_export]
macro_rules! private_impl_enum_from {
    (
        $enum_name:ident,
        $type:ty,
        [ $(($variant:ident, $value:expr)),* ],
    ) => {
        impl From<$enum_name> for $type {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$variant => $value,
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! private_impl_enum_try_from {
    (
        $enum_name:ident,
        $error_type:ty,
        $type:ty,
        [ $(($variant:ident, $value:expr)),* ],
    ) => {
        impl TryFrom<$type> for $enum_name {
            type Error = $error_type;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(Self::Error::UnknownEnumValue {
                        enum_name: stringify!($enum_name),
                        value: value.to_string(),
                    }),
                }
            }
        }
    };
}
