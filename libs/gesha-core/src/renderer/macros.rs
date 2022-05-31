macro_rules! render {
    ($write:ident, block > $func:expr => $y:expr) => {
        writeln!($write, "{{")?;
        $func(&mut $write, $y)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, block > $x:expr) => {
        writeln!($write, "{{")?;
        crate::renderer::Renderer::render($x, &mut $write)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, text > $func:expr => $y:expr) => {
        $func(&mut $write, $y)?;
    };
    ($write:ident, text > $x:expr $(, $i:ident = $z:expr)*) => {
        write!($write, $x $(, $i = $z)*)?;
    };
    (
        $write:ident =>
        $(
            $mode:ident > $x:expr
            $(=> $y:expr)?
            $(, $i:ident = $z:expr )*
        );+
        $(;)?
    ) => {
        $(
            render!(
                $write,
                $mode > $x $(=> $y)? $(, $i = $z)*
            );
        )+
    };
}

pub(super) use render;
