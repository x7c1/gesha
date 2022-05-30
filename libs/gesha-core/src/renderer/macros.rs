macro_rules! render {
    ($write:ident, block = $func:expr => $y:expr) => {
        writeln!($write, "{{")?;
        $func(&mut $write, $y)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, block = $x:expr) => {
        writeln!($write, "{{")?;
        crate::renderer::Renderer::render($x, &mut $write)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, text = $x:expr) => {
        writeln!($write, $x)?;
    };
    ($write:ident, $($mode:ident = $x:expr $(=> $y:expr)?),* $(,)?) => {
        $(render!($write, $mode = $x $(=> $y)? );)+
    };
}

pub(super) use render;
