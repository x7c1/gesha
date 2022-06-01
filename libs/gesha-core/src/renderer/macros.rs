macro_rules! render {
    ($write:ident, "{}" > $func:expr => $y:expr) => {
        writeln!($write, "{{")?;
        $func(&mut $write, $y)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, "<>" > $func:expr => $y:expr) => {
        write!($write, "<")?;
        $func(&mut $write, $y)?;
        write!($write, ">")?;
    };
    ($write:ident, call > $func:expr => $y:expr) => {
        $func(&mut $write, $y)?;
    };
    ($write:ident, echo > $x:expr $(, $i:ident = $z:expr)*) => {
        write!($write, $x $(, $i = $z)*)?;
    };
    (
        $write:ident =>
        $(
            $mode:tt > $x:expr
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
