macro_rules! render {
    ($write:ident, {$expr:tt}) => {
        writeln!($write, "{{")?;
        crate::renderer::Renderer::render($expr, &mut $write)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, $expr:tt) => {
        writeln!($write, $expr)?;
    };
    ($write:ident, $($expr:tt),+) => {
        $(render!($write, $expr);)+
    };
}

pub(super) use render;
