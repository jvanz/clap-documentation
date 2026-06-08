use clap_documentation::MarkdownOptions;
use pretty_assertions::assert_eq;

#[test]
fn test_example_complex_app_markdown() {
    mod complex_app {
        include!("../docs/examples/complex_app.rs");
    }

    assert_eq!(
        clap_documentation::help_markdown::<complex_app::Cli>(),
        include_str!("../docs/examples/complex-app.md")
    );

    assert_eq!(
        clap_documentation::help_markdown_custom::<complex_app::Cli>(
            &MarkdownOptions::new()
                .title("Some Custom Title for Complex App".to_string())
                .show_table_of_contents(false)
                .show_aliases(false)
        ),
        include_str!("../docs/examples/complex-app-custom.md"),
        "Mismatch testing CUSTOM Markdown output"
    );
}

#[test]
fn test_example_complex_app_asciidoc() {
    mod complex_app {
        include!("../docs/examples/complex_app.rs");
    }

    assert_eq!(
        clap_documentation::help_asciidoc::<complex_app::Cli>(),
        include_str!("../docs/examples/complex-app.adoc")
    );
}
