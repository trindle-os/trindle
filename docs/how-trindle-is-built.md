# How Trindle is built

The rules a change has to follow to be taken. They are short on purpose. A change that goes another way is declined with a reason, and forking Trindle for it is welcome.

## One program, all Rust

Trindle is written in Rust. It uses neither GTK nor Qt. Its windows are drawn by the Trindle Toolkit, a library of its own. Do not add a second language or a second toolkit; if something is missing, say so in your pull request.

## The look lives in the style sheet

Colours, sizes, spacing and shapes are not written into the code. They live in the style sheet (`share/style.css`) and the theme files in `share/themes`, with safe limits. A number that looks like a design choice belongs there, in one place, used by every page that needs it.

## Every setting is on a Settings page

A setting is chosen in Settings, not in a file, an environment variable or a command line. No setting is removed or hidden without a clear decision by the maintainers.

## Words

- The words people read are in `share/lang/en/`, never in the code. Translations come from the translations repository; do not edit another language by hand.
- British English, sentence case for headings, buttons and labels: only the first word has a capital.
- Say why. A refusal, a closing or an automatic action says why, in a label where the person is working, in as few words as will do.
- Sample data is made up: John Smith, `@example.com`.

## One look, nothing that moves

Buttons are small icons with the words as tooltips, except Save, which is the word. No pop-up boxes: a problem shows as words under the thing that caused it, and the page does not move to show it. Aim for WCAG 2.2 AA.

## Test everything

Each change comes with a test that fails without it. Before you send a pull request run:

```bash
cargo test --release
cargo clippy --release -- -D warnings
```

Fix every fault the same way: find all the places it can happen, fix them with one shared piece of code, and test them together.

## Keep it small

Lean and fast, nothing it does not need. No dead code, no unused imports. A comment explains a hidden hazard or a reason the code cannot show; it is a line or two, with no history.

## Untrusted plugins

Plugins are strangers' code. Design every plugin feature against a hostile plugin. A plugin reaches only what it was given permission for, and plugins are added only through the catalogue.

## Sending a change

See [CONTRIBUTING.md](../CONTRIBUTING.md). Sign off each commit with `git commit -s`.
