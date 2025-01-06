#[macro_export]
macro_rules! render {
    ($write:ident, "{}" > $func:expr => $y:expr) => {
        writeln!($write, "{{")?;
        $func($write, $y)?;
        writeln!($write, "}}")?;
    };
    ($write:ident, "()" > $func:expr => $y:expr) => {
        write!($write, "(")?;
        $func($write, $y)?;
        write!($write, ")")?;
    };
    ($write:ident, "<>" > $func:expr => $y:expr) => {
        write!($write, "<")?;
        $func($write, $y)?;
        write!($write, ">")?;
    };
    ($write:ident, "[]" > $func:expr => $y:expr) => {
        write!($write, "[")?;
        $func($write, $y)?;
        write!($write, "]")?;
    };
    ($write:ident, call > $func:expr => $y:expr) => {
        $func($write, $y)?;
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
