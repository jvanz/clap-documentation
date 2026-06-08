//! Autogenerate Markdown or AsciiDoc documentation for clap command-line tools

// Ensure that doc tests in the README.md file get run.
#[doc(hidden)]
mod test_readme {
    #![doc = include_str!("../README.md")]
}

mod utils;
mod asciidoc;
mod markdown;
mod common;

pub use asciidoc::{
    AsciiDocOptions,
    help_asciidoc,
    help_asciidoc_custom,
    help_asciidoc_command,
    help_asciidoc_command_custom,
    print_help_asciidoc,
};

pub use markdown::{
    MarkdownOptions,
    help_markdown,
    help_markdown_custom,
    help_markdown_command,
    help_markdown_command_custom,
    print_help_markdown,
};
