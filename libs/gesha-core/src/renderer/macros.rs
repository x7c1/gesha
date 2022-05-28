macro_rules! render {
    ($write:ident, {$expr:tt}) => {
        writeln!($write, "{{").map_err(crate::renderer::Error::CannotWrite)?;
        $expr.render(&mut $write)?;
        writeln!($write, "}}").map_err(crate::renderer::Error::CannotWrite)?;
    };
    ($write:ident, $expr:tt) => {
        writeln!($write, $expr).map_err(crate::renderer::Error::CannotWrite)?;
    };
    ($write:ident, $($expr:tt),+) => {
        $(render!($write, $expr);)+
    };
}

pub(super) use render;
