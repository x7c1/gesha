macro_rules! render {
    ($write:ident, block: $x:tt) => {
        writeln!($write, "{{")?;
        crate::renderer::Renderer::render($x, &mut $write)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, text: $x:tt) => {
        writeln!($write, $x)?;
    };
    ($write:ident, $($mode:ident: $x:expr),* $(,)?) => {
        $(render!($write, $mode: $x);)+
    };
}

pub(super) use render;
