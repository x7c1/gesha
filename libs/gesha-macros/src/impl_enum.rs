#[macro_export]
macro_rules! impl_enum {
    (
        $enum_name:ident {
            $(
                $type:ident: [
                    $(($variant:ident, $value:literal)),*
                ],
            )*
        }
    ) => {
        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match *self {
                    $($(
                        $enum_name::$variant => $crate::impl_serialize_variant!{ $type [serializer, $value] },
                    )*)*
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct VisitorImpl;
                impl<'de> serde::de::Visitor<'de> for VisitorImpl {
                    type Value = $enum_name;
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        let variants = vec![
                            $($(stringify!($value),)*)*
                        ];
                        write!(formatter, "{}", variants.join(" or "))
                    }
                    $(
                        $crate::impl_deserialize_variant! {
                            $enum_name,
                            $type [$($value => $variant),*]
                        }
                    )*
                }
                deserializer.deserialize_any(VisitorImpl)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deserialize_variant {
    ($enum_name:ident, u64 [ $($value:literal => $variant:ident),* ]) => {
        $crate::impl_deserialize_variant! { $enum_name, visit_u64, u64 [$($value => $variant),*] }
    };
    ($enum_name:ident, i64 [ $($value:literal => $variant:ident),* ]) => {
        $crate::impl_deserialize_variant! { $enum_name, visit_i64, i64 [$($value => $variant),*] }
    };
    ($enum_name:ident, str [ $($value:literal => $variant:ident),* ]) => {
        $crate::impl_deserialize_variant! { $enum_name, visit_str, &str [$($value => $variant),*] }
    };

    (
        $enum_name:ident,
        $fn_name:ident,
        $type:ty [ $($value:literal => $variant:ident),* ]
    ) => {
        fn $fn_name<E>(self, value: $type) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                $(
                    $value => Ok($enum_name::$variant),
                )*
                _ => Err(E::unknown_variant(&value.to_string(), &[
                    $(stringify!($value)),*
                ])),
            }
        }
    };
}

#[macro_export]
macro_rules! impl_serialize_variant {
    (u64 [$serializer:ident, $value:literal]) => {
        $serializer.serialize_u64($value)
    };
    (i64 [$serializer:ident, $value:literal]) => {
        $serializer.serialize_i64($value)
    };
    (str [$serializer:ident, $value:literal]) => {
        $serializer.serialize_str($value)
    };
}
