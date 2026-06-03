
pub(crate) fn pluralize<T>(len: usize, singular: T, plural: T) -> T {
    match len {
        1 => singular,
        _ => plural,
    }
}
/// Utility function to get the canonical name of a command.
///
/// It's logic is to get the display name if it exists, otherwise get the bin
/// name if it exists, otherwise get the package name.
///
/// Note that the default `Command.name` field of a clap command is typically
/// meant for programmatic usage as well as for display (if no `display_name`
/// override is set).
pub(crate) fn get_canonical_name(command: &clap::Command) -> String {
    command
        .get_display_name()
        .or_else(|| command.get_bin_name())
        .map(|name| name.to_owned())
        .unwrap_or_else(|| command.get_name().to_owned())
}

/// Indents non-empty lines. The output always ends with a newline.
pub(crate) fn indent(s: &str, first: &str, rest: &str) -> String {
    if s.is_empty() {
        // For consistency. It's easiest to always add a newline at the end, and
        // there's little reason not to.
        return "\n".to_string();
    }
    let mut result = String::new();
    let mut first_line = true;

    for line in s.lines() {
        if !line.is_empty() {
            result.push_str(if first_line { first } else { rest });
            result.push_str(line);
            first_line = false;
        }
        result.push('\n');
    }
    result
}

pub(crate) fn get_alias_string(aliases: &[&str]) -> Option<String> {
    if aliases.is_empty() {
        return None;
    }

    Some(format!(
        "{}",
        aliases
            .iter()
            .map(|alias| format!("`{alias}`"))
            .collect::<Vec<_>>()
            .join(", ")
    ))
}
