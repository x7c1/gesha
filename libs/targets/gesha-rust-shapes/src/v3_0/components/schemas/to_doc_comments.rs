use gesha_rust_types::DocComments;

pub(crate) fn to_doc_comments(
    title: Option<&str>,
    description: Option<&str>,
) -> Option<DocComments> {
    let trim = |x: &str| x.trim().to_string();
    let maybe = match (title.map(trim), description.map(trim)) {
        (t, None) => t,
        (None, d) => d,
        (t, d) if t == d => t,
        (Some(t), Some(d)) => Some(format!("{t}\n\n{d}")),
    };
    DocComments::wrap(maybe)
}
