#[macro_export]
macro_rules! impl_enum_serde {
    (
        $enum_name:ident {
            $(
                $type:ident: [
                    $(($variant:ident, $value:expr)),*
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
                        $enum_name::$variant => $crate::private_impl_serialize_variant!{
                            $type [serializer, $value]
                        },
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
                        $crate::private_impl_deserialize_variant! {
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
macro_rules! private_impl_deserialize_variant {
    ($enum_name:ident, u64 [ $($value:expr => $variant:ident),* ]) => {
        $crate::private_impl_deserialize_variant! { $enum_name, visit_u64, u64 [$($value => $variant),*] }
    };
    ($enum_name:ident, i64 [ $($value:expr => $variant:ident),* ]) => {
        $crate::private_impl_deserialize_variant! { $enum_name, visit_i64, i64 [$($value => $variant),*] }
    };
    ($enum_name:ident, str [ $($value:expr => $variant:ident),* ]) => {
        $crate::private_impl_deserialize_variant! { $enum_name, visit_str, &str [$($value => $variant),*] }
    };
    ($enum_name:ident, bool [ $($value:expr => $variant:ident),* ]) => {
        $crate::private_impl_deserialize_variant! { $enum_name, visit_bool, bool [$($value => $variant),*] }
    };
    ($enum_name:ident, null [ $($value:expr => $variant:ident),* ]) => {
        $crate::private_impl_deserialize_variant! { $enum_name, visit_unit [$($value => $variant),*] }
    };
    (
        $enum_name:ident,
        $fn_name:ident,
        $type:ty [ $($value:expr => $variant:ident),* ]
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
    (
        $enum_name:ident,
        visit_unit [ $($value:expr => $variant:ident),* ]
    ) => {
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            $(
                Ok($enum_name::$variant)
            ),*
        }
    };
}

#[macro_export]
macro_rules! private_impl_serialize_variant {
    (u64 [$serializer:ident, $value:expr]) => {
        $serializer.serialize_u64($value)
    };
    (i64 [$serializer:ident, $value:expr]) => {
        $serializer.serialize_i64($value)
    };
    (str [$serializer:ident, $value:expr]) => {
        $serializer.serialize_str($value)
    };
    (bool [$serializer:ident, $value:expr]) => {
        $serializer.serialize_bool($value)
    };
    (null [$serializer:ident, $value:expr]) => {
        $serializer.serialize_none()
    };
}
