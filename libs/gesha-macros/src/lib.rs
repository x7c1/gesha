mod impl_enum_from;
mod impl_enum_serde;

#[macro_export]
macro_rules! impl_enum {
    (
        impl Serialize,
        impl Deserialize,
        $enum_name:ident $tree:tt,
    ) => {
        $crate::impl_enum_serde!($enum_name $tree);
    };
    (
        impl From<$enum_name:ident>,
        impl TryFrom<$( $type:ty ),*>,
        $error_type:ty,
        $pairs:tt,
    ) => {
        $crate::impl_enum_from!(
            $enum_name,
            $error_type,
            [ $( $type ),* ],
            $pairs,
        );
    };
}
