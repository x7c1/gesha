use crate::renderer::Renderer;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write<P, A>(path: P, content: A) -> crate::Result<()>
where
    P: Into<PathBuf>,
    A: Renderer,
{
    let code = content.render()?;

    // TODO: remove unwrap
    let mut file = File::create(path.into()).unwrap();
    write!(file, "{}", code).unwrap();

    Ok(())
}
