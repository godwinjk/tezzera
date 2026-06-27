# TEZZERA

```
 ████████╗███████╗███████╗███████╗███████╗██████╗  █████╗
    ██╔══╝██╔════╝╚══███╔╝╚══███╔╝██╔════╝██╔══██╗██╔══██╗
    ██║   █████╗    ███╔╝   ███╔╝ █████╗  ██████╔╝███████║
    ██║   ██╔══╝   ███╔╝   ███╔╝  ██╔══╝  ██╔══██╗██╔══██║
    ██║   ███████╗███████╗███████╗███████╗██║  ██║██║  ██║
    ╚═╝   ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝
```

**Fast by nature. Beautiful by design.**

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue)
![Phase](https://img.shields.io/badge/phase-1%20complete-success)
![Language](https://img.shields.io/badge/language-Rust-orange)

---

## What is TEZZERA?

TEZZERA is a declarative UI framework written in pure Rust. The name comes from *tessera* — the individual tiles that form a mosaic. Every component in TEZZERA is a tessera tile: self-contained, composable, and precise. Assembled together they form the full picture of your application.

Write your UI once in Rust. Run it on desktop, web (WASM), iOS, and Android at up to 120fps.

TEZZERA draws inspiration from Flutter, Jetpack Compose, SwiftUI, and React, then tightens everything up with Rust's type system: no null pointer surprises, no runtime layout panics, no undefined behaviour.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  tezzera-examples                   │  Example apps
├─────────────────────────────────────────────────────┤
│   tezzera-widgets   │   tezzera-cli (tzr)           │  Widgets + CLI
├─────────────────────────────────────────────────────┤
│   tezzera-platform  │   tezzera-layout              │  Windowing + Flexure
├─────────────────────────────────────────────────────┤
│   tezzera-render    │   tezzera-state               │  Pipeline + Atoms
├─────────────────────────────────────────────────────┤
│   tezzera-core      │   tezzera-trace               │  Components + Bus
├─────────────────────────────────────────────────────┤
│                  tezzera-macros                     │  Proc-macros
└─────────────────────────────────────────────────────┘
        tiny-skia · fontdue · winit · softbuffer
```

Data flows downward (props); state changes propagate upward through reactive atoms.

---

## Getting Started

### Prerequisites

- Rust 1.78+ (stable)
- `cargo` in your PATH

### Run the examples

```bash
# Static PNG renders — produce output images in the working directory
cargo run -p tezzera-examples --bin dashboard
cargo run -p tezzera-examples --bin photo_gallery
cargo run -p tezzera-examples --bin profile_card

# Interactive windowed counter app (opens a native window)
cargo run -p tezzera-examples --bin counter_window
```

### Build the workspace

```bash
cargo build            # debug
cargo build --release  # release — zero warnings, zero unsafe
cargo test             # full test suite
```

### tzr CLI

```bash
tzr dev                      # start dev server with hot reload
tzr build --target desktop   # produce a desktop binary
```

Install the CLI with:

```bash
cargo install --path tezzera-cli
```

---

## Crate Overview

| Crate | Description |
|---|---|
| `tezzera-macros` | Proc-macros: `#[component]`, `view!{}` |
| `tezzera-trace` | `TezzeraTrace` event bus, ring buffer, subscribers |
| `tezzera-core` | Component model, element tree, lifecycle hooks |
| `tezzera-state` | `Atom<T>`, `use_atom()`, `GlobalAtom`, batched updates |
| `tezzera-layout` | Flexure engine: Column, Row, Stack, Flex, Grid, Wrap, SizedBox, AspectRatio |
| `tezzera-render` | tiny-skia render pipeline, dirty regions, layer compositor, `FontCache` |
| `tezzera-widgets` | Built-in widget library |
| `tezzera-platform` | Windowing abstraction (winit + softbuffer) |
| `tezzera-examples` | Example applications |
| `tezzera-cli` | `tzr` command-line tool |

---

## Phase 1 Status

All Phase 1 exit criteria are met.

- [x] Counter app renders at 60fps on desktop
- [x] Reactive state — `Atom<T>` with `use_atom()`, `atom!()`, `GlobalAtom`
- [x] Layout engine — Column, Row, Stack, Flex, Grid, Wrap, SizedBox, AspectRatio
- [x] Real font rendering via fontdue (`FontCache` + `draw_text()`)
- [x] Lifecycle hooks — `on_mount`, `on_update`, `on_unmount`
- [x] `ErrorBoundary` with panic catching
- [x] `TezzeraTrace` event bus with ring buffer
- [x] `tzr dev` and `tzr build --target desktop` CLI commands
- [x] Zero warnings in release builds
- [x] Zero unsafe code

---

## Contributing

Architectural decisions that govern the project are recorded in `.steering/DECISIONS.md`. Read that file before opening a pull request — decisions labelled LOCKED are not open for debate unless a new decision supersedes them.

Other steering documents in `.steering/` cover the phase roadmap, crate responsibilities, and coding conventions.

---

## License

Licensed under either of

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.
