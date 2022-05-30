macro_rules! render {
    ($write:ident, block = $x:expr) => {
        writeln!($write, "{{")?;
        crate::renderer::Renderer::render($x, &mut $write)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, text = $x:expr) => {
        writeln!($write, $x)?;
    };
    ($write:ident, $($mode:ident = $x:expr),* $(,)?) => {
        $(render!($write, $mode = $x);)+
    };
}

pub(super) use render;
