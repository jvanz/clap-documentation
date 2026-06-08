# Development

## Quick Command Reference

**Test that the examples are up-to-date:**

The [`tests/test_examples.rs`](../tests/test_examples.rs) program contains
tests that the Markdown and AsciiDoc generated for each sample program in
[`./docs/examples`](../docs/examples/) matches the corresponding `.md` and
`.adoc` files.

Test that the generated example documentation is up to date:

```shell
$ cargo test
```
