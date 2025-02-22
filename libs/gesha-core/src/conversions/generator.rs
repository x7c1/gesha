use crate::conversions::Converter;
use crate::io::{Reader, Writer};
use crate::Result;
use std::path::PathBuf;
use tracing::debug;

pub struct Generator<'a, A> {
    converter: &'a A,
    output: PathBuf,
}

impl<'a, A: Converter> Generator<'a, A> {
    pub fn new(converter: &'a A, output: impl Into<PathBuf>) -> Self {
        Self {
            converter,
            output: output.into(),
        }
    }

    pub fn generate_from_file(&self, schema: impl Into<PathBuf>) -> Result<()> {
        let reader = Reader::new(schema);
        let target = reader.open_target_type(self.converter)?;

        let writer = Writer::new(&self.output);
        writer.write_code(target)?;

        let output = self.converter.format_code(&self.output)?;
        debug!("format>\n{}", output);
        Ok(())
    }

    pub fn generate_from_type(&self, target: A::TargetType) -> Result<()> {
        let writer = Writer::new(&self.output);
        writer.write_code(target)?;

        let output = self.converter.format_code(&self.output)?;
        debug!("format>\n{}", output);
        Ok(())
    }
}
