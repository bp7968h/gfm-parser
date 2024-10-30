# gfm-parser

GFM-Parser is a Rust library for parsing GitHub Flavored Markdown (GFM) with support for custom extensions, including admonitions, custom callouts, interactive checklists, and more. This parser is designed to be flexible and extendable, making it easy to convert Markdown syntax into a structured format (like HTML) for web rendering.

The goal of this library is to be 100% complaint with the [GitHub Flavored Markdown Spec](https://github.github.com/gfm/) along with the extension specs and some additional features such as `Alerts (Admonitions)`, `Emoji Shortcodes`, `Syntax Highlighting in Code Blocks` and `HTML tags`. This is just the initial plan but might expand as we go on.
