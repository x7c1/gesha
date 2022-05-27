use crate::renderer::Renderer;
use crate::Error::{CannotWriteFile, FormatFailed};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fs, io};

pub struct Writer {
    pub path: PathBuf,
    // e.g. "Generated file, do not edit by hand.";
    pub preamble: Option<String>,
}

impl Writer {
    pub fn print<A: Renderer>(self, a: A) -> crate::Result<()> {
        let rendered = a.render()?;
        let formatted = format(rendered).map_err(|e| FormatFailed {
            path: self.path.clone(),
            detail: format!("{:?}", e),
        })?;

        let code = if let Some(preamble) = self.preamble {
            format!("// {}\n\n{}", preamble, formatted)
        } else {
            formatted
        };

        fs::write(&self.path, code).map_err(|cause| CannotWriteFile {
            path: self.path.clone(),
            detail: format!("{:?}", cause),
        })?;
        Ok(())
    }
}

fn format(text: String) -> io::Result<String> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    write!(rustfmt.stdin.take().unwrap(), "{}", text)?;

    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}
