# envview-slint

[![Made with Slint](https://github.com/slint-ui/slint/raw/master/logo/MadeWithSlint-logo-whitebg.png)](https://slint.dev)

A small desktop app that displays the current environment variables in a
scrollable, sortable two-column table. Built in Rust with the
[Slint](https://slint.dev) GUI toolkit.

## Features

- Lists all environment variables, sorted alphabetically by name.
- Two-column table (Name / Value) with a fixed header and vertical scrolling.
- Drag the column border to resize the Name / Value split.
- Right-click a row for a context menu: **Copy name**, **Copy value**,
  **Copy name=value** (copied to the system clipboard).

## Running

```sh
cargo run
```

On machines whose display has no working OpenGL context you can use
the software rendering backend instead:

```sh
SLINT_BACKEND=winit-software cargo run
```

## License

This application's own source code is licensed under the [MIT License](LICENSE)
(© 2026 Oliver Anhuth).

The UI is built with [Slint](https://slint.dev), used here under the
**Slint Royalty-free License 2.0**. Slint is available under multiple licenses;
see the [Slint licensing page](https://slint.dev/terms-and-conditions) for
details.
