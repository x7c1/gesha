use crate::gateway::Error::{CannotWriteFile, FormatFailed};
use crate::gateway::Result;
use crate::renderer::Renderer;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct Writer {
    pub path: PathBuf,
    // e.g. "Generated file, do not edit by hand.";
    pub preamble: Option<String>,
}

impl Writer {
    pub fn print<A: Renderer>(self, a: A) -> Result<()> {
        let mut file = File::create(&self.path).unwrap();

        if let Some(preamble) = self.preamble {
            let bytes = preamble.as_bytes();
            file.write(bytes).map_err(|cause| CannotWriteFile {
                path: self.path.clone(),
                detail: format!("{:?}", cause),
            })?;
        }

        a.render(&file).map_err(|cause| CannotWriteFile {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;

        let output = format(&self.path).map_err(|e| FormatFailed {
            path: self.path.clone(),
            detail: format!("{:?}", e),
        })?;
        println!("[done] print: {}", output);

        Ok(())
    }
}

fn format(path: &PathBuf) -> io::Result<String> {
    let output = Command::new("rustfmt")
        .arg("--verbose")
        .arg(path)
        .output()
        .unwrap();

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
