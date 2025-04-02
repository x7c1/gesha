use crate::{Error, Output};

pub fn format_errors<A>(output: Output<A>) -> Option<String> {
    let (_, errors) = output.into_tuple();
    if errors.is_empty() {
        return None;
    }
    let errors = errors
        .into_iter()
        .flat_map(format_core_error)
        .collect::<Vec<_>>();

    let lines = errors.join("\n").to_string();
    Some(lines)
}

fn format_core_error(err: Error) -> Vec<String> {
    let mut lines = vec![];
    match err {
        Error::OpenApiTypes { path, cause } => {
            lines.push(format!("path: {}", path.to_string_lossy()));
            lines.append(&mut oas::format_error(cause, vec![]));
        }
        Error::Conversion { path, cause } => {
            lines.push(format!("path: {}", path.to_string_lossy()));
            lines.append(&mut conv::format_error(cause, vec![]));
        }
        Error::Multiple(errors) => {
            let mut formatted = errors.into_iter().flat_map(format_core_error).collect();
            lines.append(&mut formatted);
        }
        e => lines.push(format!("{:#?}", e)),
    }
    lines.push("".to_string());
    lines
}

macro_rules! generate {
    () => {
        pub fn format_error(err: Error, mut keys: Vec<String>) -> Vec<String> {
            let mut lines = vec![];
            match err {
                Error::Multiple(causes) => {
                    let mut next_lines = causes
                        .into_iter()
                        .flat_map(|e| format_error(e, keys.clone()))
                        .collect();

                    lines.append(&mut next_lines)
                }
                Error::Enclosed { key, causes } => {
                    keys.push(key);
                    let mut next_lines = format_enclosed_error(causes, keys);
                    lines.append(&mut next_lines)
                }
                _ => {
                    lines.push(format!("\n    @ {}", keys.join(" > ")));

                    let mut next_lines = format!("{:#?}\n", err)
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect::<Vec<_>>();

                    lines.append(&mut next_lines);
                }
            }
            lines
        }
        fn format_enclosed_error(causes: Vec<Error>, keys: Vec<String>) -> Vec<String> {
            causes
                .into_iter()
                .flat_map(|cause| format_error(cause, keys.clone()))
                .collect()
        }
    };
}

mod oas {
    use openapi_types::Error;
    generate!();
}

mod conv {
    use crate::conversions::Error;
    generate!();
}
