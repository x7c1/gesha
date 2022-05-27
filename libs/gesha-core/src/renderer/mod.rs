mod rust_type;

pub trait Renderer {
    fn render(self) -> crate::Result<String>;
}
