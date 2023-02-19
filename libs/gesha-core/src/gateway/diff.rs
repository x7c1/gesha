use std::fmt::Debug;
use super::Result;
use crate::gateway::file_to_string;
use crate::gateway::Error::DiffDetected;
use console::Style;
use similar::{Change, ChangeTag, TextDiff};
use std::path::Path;
use tracing::instrument;

/// return DiffDetected error if the contents of given files are not same.
#[instrument]
pub fn detect_diff<A, B>(src: A, dst: B) -> Result<()>
where
    A: AsRef<Path> + Debug,
    B: AsRef<Path> + Debug,
{
    let diff = Diff::load(&src, &dst)?;
    if diff.has_change {
        Err(DiffDetected {
            output: diff.output,
            actual: src.as_ref().into(),
            expected: dst.as_ref().into(),
        })
    } else {
        Ok(())
    }
}

pub struct Diff {
    has_change: bool,
    output: String,
}

impl Diff {
    pub fn load<A, B>(src: A, dst: B) -> Result<Diff>
    where
        A: AsRef<Path>,
        B: AsRef<Path>,
    {
        let src_lines = file_to_string(src)?;
        let dst_lines = file_to_string(dst)?;
        let raw = TextDiff::from_lines(src_lines.as_str(), dst_lines.as_str());
        let diff = Diff {
            has_change: raw.iter_all_changes().any(is_changed),
            output: format_text_diff(&raw),
        };
        Ok(diff)
    }
}

fn is_changed(change: Change<&str>) -> bool {
    match change.tag() {
        ChangeTag::Equal => false,
        ChangeTag::Delete => true,
        ChangeTag::Insert => true,
    }
}

fn format_text_diff<'a>(diff: &TextDiff<'a, 'a, 'a, str>) -> String {
    diff.iter_all_changes().into_iter().map(to_line).collect()
}

fn to_line(change: Change<&str>) -> String {
    let (sign, style) = match change.tag() {
        ChangeTag::Delete => ("-", Style::new().red()),
        ChangeTag::Insert => ("+", Style::new().green()),
        ChangeTag::Equal => (" ", Style::new()),
    };
    format!("{}{}", style.apply_to(sign), style.apply_to(change))
}
