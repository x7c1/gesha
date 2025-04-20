#[macro_export]
macro_rules! impl_enum_from {
    (
        $enum_name:ident,
        $error_type:ty,
        [ $( $type:tt  ),* ],
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

/**
   For `enum Foo` with inner type is `A`, generate:
   - `impl From<Foo> for A`
   - `impl AsRef<A> for Foo`

   For `enum Foo` with inner type is `&str`, generate:
   - `impl From<Foo> for &str`
   - `impl From<Foo> for String`
   - `impl AsRef<str> for Foo`
*/
#[macro_export]
macro_rules! private_impl_enum_from {
    (
        $enum_name:ident,
        String,
        [ $(($variant:ident, $value:expr)),* ],
    ) => {
        impl From<$enum_name> for &str {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$variant => $value,
                    )*
                }
            }
        }

        impl From<$enum_name> for String {
            fn from(value: $enum_name) -> Self {
                AsRef::<str>::as_ref(&value).to_string()
            }
        }

        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                match self {
                    $(
                        $enum_name::$variant => $value,
                    )*
                }
            }
        }
    };

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

        impl AsRef<$type> for $enum_name {
            fn as_ref(&self) -> &$type {
                match self {
                    $(
                        $enum_name::$variant => &$value,
                    )*
                }
            }
        }
    };
}

/**
    For `enum Foo` with inner type is `A`, generate:
    - `impl TryFrom<A> for Foo`

    For `enum Foo` with inner type is `&str`, generate:
    - `impl TryFrom<&str> for Foo`
    - `impl TryFrom<String> for Foo`
*/
#[macro_export]
macro_rules! private_impl_enum_try_from {
    (
        $enum_name:ident,
        $error_type:ty,
        String,
        [ $(($variant:ident, $value:expr)),* ],
    ) => {

        $crate::private_impl_enum_try_from!(
            $enum_name,
            $error_type,
            &str,
            [ $(($variant, $value)),* ],
        );

        impl TryFrom<String> for $enum_name {
            type Error = $error_type;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }
    };

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
                        given: value.to_string(),
                    }),
                }
            }
        }
    };
}
