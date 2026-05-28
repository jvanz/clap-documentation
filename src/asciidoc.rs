
use crate::{get_canonical_name, get_alias_string, pluralize, indent};

use std::fmt::{self, Write};

use clap::builder::PossibleValue;


#[non_exhaustive]
pub struct AsciiDocOptions {
    title: Option<String>,
    show_table_of_contents: bool,
    show_aliases: bool,
}

impl AsciiDocOptions {
    pub fn new() -> Self {
        return Self {
            title: None,
            show_table_of_contents: true,
            show_aliases: true,
        };
    }

    /// Set a custom title to use in the generated document.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);

        return self;
    }

    /// Whether to show the default table of contents.
    pub fn show_table_of_contents(mut self, show: bool) -> Self {
        self.show_table_of_contents = show;

        return self;
    }

    /// Whether to show aliases for arguments and commands.
    pub fn show_aliases(mut self, show: bool) -> Self {
        self.show_aliases = show;

        return self;
    }
}

impl Default for AsciiDocOptions {
    fn default() -> Self {
        return Self::new();
    }
}

//======================================
// Public API functions
//======================================

/// Format the help information for `command` as asciidoc.
pub fn help_asciidoc<C: clap::CommandFactory>() -> String {
    let command = C::command();

    help_asciidoc_command(&command)
}

/// Format the help information for `command` as asciidoc, with custom options.
pub fn help_asciidoc_custom<C: clap::CommandFactory>(
    options: &AsciiDocOptions,
) -> String {
    let command = C::command();

    return help_asciidoc_command_custom(&command, options);
}

/// Format the help information for `command` as asciidoc.
pub fn help_asciidoc_command(command: &clap::Command) -> String {
    return help_asciidoc_command_custom(command, &Default::default());
}

/// Format the help information for `command` as asciidoc, with custom options.
pub fn help_asciidoc_command_custom(
    command: &clap::Command,
    options: &AsciiDocOptions,
) -> String {
    let mut buffer = String::with_capacity(100);

    write_help_asciidoc(&mut buffer, &command, options);

    buffer
}

//======================================
// asciidoc
//======================================

/// Format the help information for `command` as asciidoc and print it.
///
/// Output is printed to the standard output, using [`println!`].
pub fn print_help_asciidoc<C: clap::CommandFactory>() {
    let command = C::command();

    let mut buffer = String::with_capacity(100);

    write_help_asciidoc(&mut buffer, &command, &Default::default());

    println!("{}", buffer);
}

fn write_help_asciidoc(
    buffer: &mut String,
    command: &clap::Command,
    options: &AsciiDocOptions,
) {
    //----------------------------------
    // Write the document title
    //----------------------------------

    let title_name = get_canonical_name(command);

    let title = match options.title {
        Some(ref title) => title.to_owned(),
        None => format!("Command-Line Help for `{title_name}`"),
    };
    writeln!(buffer, "= {title}\n",).unwrap();

    writeln!(
        buffer,
        "This document contains the help content for the `{}` command-line program.\n", title_name
    ).unwrap();

    //----------------------------------
    // Write the table of contents
    //----------------------------------

    // writeln!(buffer, r#"<div style="background: light-gray"><ul>"#).unwrap();
    // build_table_of_contents_html(buffer, Vec::new(), command, 0).unwrap();
    // writeln!(buffer, "</ul></div>").unwrap();

    if options.show_table_of_contents {
        writeln!(buffer, "*Command Overview:*\n").unwrap();

        build_table_of_contents_asciidoc(buffer, Vec::new(), command, 0)
            .unwrap();

        write!(buffer, "\n").unwrap();
    }

    //----------------------------------------
    // Write the commands/subcommands sections
    //----------------------------------------

    build_command_asciidoc(buffer, Vec::new(), command, 0, options).unwrap();
}

fn build_command_asciidoc(
    buffer: &mut String,
    // Parent commands of `command`.
    parent_command_path: Vec<String>,
    command: &clap::Command,
    depth: usize,
    options: &AsciiDocOptions,
) -> std::fmt::Result {
    // Don't document commands marked with `clap(hide = true)` (which includes
    // `print-all-help`).
    if command.is_hide_set() {
        return Ok(());
    }

    let title_name = get_canonical_name(command);

    // Append the name of `command` to `command_path`.
    let command_path = {
        let mut command_path = parent_command_path.clone();
        command_path.push(title_name);
        command_path
    };

    //----------------------------------
    // Write the asciidoc heading
    //----------------------------------

    // TODO: `depth` is now unused. Remove if no other use for it appears.
    /*
    if depth >= 6 {
        panic!(
            "command path nesting depth is deeper than maximum asciidoc header depth: `{}`",
            command_path.join(" ")
        )
    }
    */
    writeln!(buffer, "[[{}]]", command_path.join("-"))?;
    writeln!(buffer, "== `{}`\n", command_path.join(" "))?;

    if let Some(long_about) = command.get_long_about() {
        writeln!(buffer, "{}\n", long_about)?;
    } else if let Some(about) = command.get_about() {
        writeln!(buffer, "{}\n", about)?;
    }

    if let Some(help) = command.get_before_long_help() {
        writeln!(buffer, "{}\n", help)?;
    } else if let Some(help) = command.get_before_help() {
        writeln!(buffer, "{}\n", help)?;
    }

    writeln!(
        buffer,
        "*Usage:* `{}{}`\n",
        if parent_command_path.is_empty() {
            String::new()
        } else {
            let mut s = parent_command_path.join(" ");
            s.push_str(" ");
            s
        },
        command
            .clone()
            .render_usage()
            .to_string()
            .replace("Usage: ", "")
    )?;

    if options.show_aliases {
        let aliases = command.get_visible_aliases().collect::<Vec<&str>>();
        if let Some(aliases_str) = get_alias_string(&aliases) {
            writeln!(
                buffer,
                "*{}:* {aliases_str}\n",
                pluralize(aliases.len(), "Command Alias", "Command Aliases")
            )?;
        }
    }

    if let Some(help) = command.get_after_long_help() {
        writeln!(buffer, "{}\n", help)?;
    } else if let Some(help) = command.get_after_help() {
        writeln!(buffer, "{}\n", help)?;
    }

    //----------------------------------
    // Subcommands
    //----------------------------------

    if command.get_subcommands().next().is_some() {
        writeln!(buffer, "=== *Subcommands:*\n")?;

        for subcommand in command.get_subcommands() {
            if subcommand.is_hide_set() {
                continue;
            }

            let title_name = get_canonical_name(subcommand);

            let about = match subcommand.get_about() {
                Some(about) => about.to_string(),
                None => String::new(),
            };

            writeln!(buffer, "* `{title_name}` — {about}",)?;
        }

        write!(buffer, "\n")?;
    }

    //----------------------------------
    // Arguments
    //----------------------------------

    if command.get_positionals().next().is_some() {
        writeln!(buffer, "=== *Arguments:*\n")?;

        for pos_arg in command.get_positionals() {
            write_arg_asciidoc(buffer, pos_arg)?;
        }

        write!(buffer, "\n")?;
    }

    //----------------------------------
    // Options
    //----------------------------------

    let non_pos: Vec<_> = command
        .get_arguments()
        .filter(|arg| !arg.is_positional() && !arg.is_hide_set())
        .collect();

    if !non_pos.is_empty() {
        writeln!(buffer, "=== *Options:*\n")?;

        for arg in non_pos {
            write_arg_asciidoc(buffer, arg)?;
        }

        write!(buffer, "\n")?;
    }

    //----------------------------------
    // Recurse to write subcommands
    //----------------------------------

    // Include extra space between commands. This is purely for the benefit of
    // anyone reading the source .md file.
    write!(buffer, "\n\n")?;

    for subcommand in command.get_subcommands() {
        build_command_asciidoc(
            buffer,
            command_path.clone(),
            subcommand,
            depth + 1,
            options,
        )?;
    }

    Ok(())
}

fn write_arg_asciidoc(buffer: &mut String, arg: &clap::Arg) -> fmt::Result {
    // asciidoc list item
    write!(buffer, "* ")?;

    let value_name: String = match arg.get_value_names() {
        // TODO: What if multiple names are provided?
        Some([name, ..]) => name.as_str().to_owned(),
        Some([]) => unreachable!(
            "clap Arg::get_value_names() returned Some(..) of empty list"
        ),
        None => arg.get_id().to_string().to_ascii_uppercase(),
    };

    match (arg.get_short(), arg.get_long()) {
        (Some(short), Some(long)) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`-{short}`, `--{long} <{value_name}>`")?
            } else {
                write!(buffer, "`-{short}`, `--{long}`")?
            }
        },
        (Some(short), None) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`-{short} <{value_name}>`")?
            } else {
                write!(buffer, "`-{short}`")?
            }
        },
        (None, Some(long)) => {
            if arg.get_action().takes_values() {
                write!(buffer, "`--{} <{value_name}>`", long)?
            } else {
                write!(buffer, "`--{}`", long)?
            }
        },
        (None, None) => {
            debug_assert!(arg.is_positional(), "unexpected non-positional Arg with neither short nor long name: {arg:?}");

            write!(buffer, "`<{value_name}>`",)?;
        },
    }

    if let Some(aliases) = arg.get_visible_aliases().as_deref() {
        if let Some(aliases_str) = get_alias_string(aliases) {
            write!(
                buffer,
                " [{}: {aliases_str}]",
                pluralize(aliases.len(), "alias", "aliases")
            )?;
        }
    }

    if let Some(help) = arg.get_long_help() {
        // TODO: Parse formatting in the string
        buffer.push_str(&indent(&help.to_string(), " — ", "   "))
    } else if let Some(short_help) = arg.get_help() {
        writeln!(buffer, " — {short_help}")?;
    } else {
        writeln!(buffer)?;
    }

    //--------------------
    // Arg default values
    //--------------------

    if !arg.get_default_values().is_empty() {
        let default_values: String = arg
            .get_default_values()
            .iter()
            .map(|value| format!("`{}`", value.to_string_lossy()))
            .collect::<Vec<String>>()
            .join(", ");

        if arg.get_default_values().len() > 1 {
            // Plural
            writeln!(buffer, "+\nDefault values: {default_values}")?;
        } else {
            // Singular
            writeln!(buffer, "+\nDefault value: {default_values}")?;
        }
    }

    //--------------------
    // Arg possible values
    //--------------------

    let possible_values: Vec<PossibleValue> = arg
        .get_possible_values()
        .into_iter()
        .filter(|pv| !pv.is_hide_set())
        .collect();

    // Print possible values for options that take a value, but not for flags
    // that can only be either present or absent and do not take a value.
    if !possible_values.is_empty()
        && !matches!(arg.get_action(), clap::ArgAction::SetTrue)
    {
        let any_have_help: bool =
            possible_values.iter().any(|pv| pv.get_help().is_some());

        if any_have_help {
            // If any of the possible values have help text, print them
            // as a separate item in a bulleted list, and include the
            // help text for those that have it. E.g.:
            //
            //     Possible values:
            //     - `value1`:
            //       The help text
            //     - `value2`
            //     - `value3`:
            //       The help text

            let text: String = possible_values
                .iter()
                .map(|pv| match pv.get_help() {
                    Some(help) => {
                        format!("  - `{}`:\n    {}\n", pv.get_name(), help)
                    },
                    None => format!("  - `{}`\n", pv.get_name()),
                })
                .collect::<Vec<String>>()
                .join("");

            writeln!(buffer, "+\nPossible values:\n+\n{text}")?;
        } else {
            // If none of the possible values have any documentation, print
            // them all inline on a single line.
            let text: String = possible_values
                .iter()
                // TODO: Show PossibleValue::get_help(), and PossibleValue::get_name_and_aliases().
                .map(|pv| format!("`{}`", pv.get_name()))
                .collect::<Vec<String>>()
                .join(", ");

            writeln!(buffer, "+\nPossible values: {text}\n")?;
        }
    }

    Ok(())
}

fn build_table_of_contents_asciidoc(
    buffer: &mut String,
    // Parent commands of `command`.
    parent_command_path: Vec<String>,
    command: &clap::Command,
    depth: usize,
) -> std::fmt::Result {
    // Don't document commands marked with `clap(hide = true)` (which includes
    // `print-all-help`).
    if command.is_hide_set() {
        return Ok(());
    }

    let title_name = get_canonical_name(command);

    // Append the name of `command` to `command_path`.
    let command_path = {
        let mut command_path = parent_command_path;
        command_path.push(title_name);
        command_path
    };

    writeln!(
        buffer,
        "* <<{},`{}`>>",
        command_path.join("-"),
        command_path.join(" "),
    )?;

    //----------------------------------
    // Recurse to write subcommands
    //----------------------------------

    for subcommand in command.get_subcommands() {
        build_table_of_contents_asciidoc(
            buffer,
            command_path.clone(),
            subcommand,
            depth + 1,
        )?;
    }

    Ok(())
}
