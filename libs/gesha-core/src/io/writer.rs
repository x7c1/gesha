use crate::renderer::Renderer;
use crate::Error::CannotWriteFile;
use std::fs;
use std::path::PathBuf;

pub fn write<P, A>(path: P, content: A) -> crate::Result<()>
where
    P: Into<PathBuf>,
    A: Renderer,
{
    let code = content.render()?;
    let path = path.into();
    fs::write(&path, code).map_err(|cause| CannotWriteFile {
        path: path.to_string_lossy().to_string(),
        detail: format!("{:?}", cause),
    })?;
    Ok(())
}
