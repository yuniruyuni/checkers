# Rust dependency migration (2026-10-25)

This change starts from merged PR #2 (`169d04f`). It moves egui/eframe
0.19 to 0.29.1 and Criterion 0.3 to 0.5.1. The lockfile contains
webbrowser 1.2.5 and memmap2 0.9.12; atty is absent. Criterion now uses
maintained terminal detection through clap/is-terminal. Pretty assertions
also moves to 1.4.1, removing its unmaintained ansi_term dependency.

The native viewport API replaces the old window options; the fixed 480 x 300
size, light visuals, move selection and right-click cancellation are retained.
The egui input API now uses a closure. Eframe explicitly selects fonts, Glow,
X11, Wayland and the web screen reader, preserving the old renderer choice.
AccessKit, newly enabled by newer eframe defaults, is not introduced: its
older Linux zbus_xml dependency would retain quick-xml 0.30 vulnerabilities.

The public RustSec database was downloaded and matched locally, without
sending dependency inventories to a service. Snapshot:
`21d909e4c8bd4e14c7aea58bf980c881a0053ab1`.
Relevant advisories: RUSTSEC-2026-0257 (webbrowser), RUSTSEC-2026-0186
(memmap2), RUSTSEC-2021-0145 / 2024-0375 (atty).
No direct calls to the reported vulnerable browser/mapping APIs were found.
The browser issue is Unix-specific and the atty issue requires a custom
allocator on Windows; dependency matches alone do not demonstrate exploitation.

Validation on COCOA uses Rust 1.96.1, x86_64-pc-windows-msvc, with two build
jobs: `cargo build --workspace --all-targets --locked`,
`cargo clippy --workspace --all-targets --locked`,
`cargo test --workspace --locked`, and `cargo doc --workspace --no-deps --locked`.
New tests drive egui frames and real pointer events without opening a window:
selecting a piece, applying a legal move, rendering without changing the board,
and cancelling selection. Existing engine and doctests remain in place.

This is a modern Rust/dependency baseline, not a promise of compatibility with
the previous toolchain. Native window/GPU behavior and Linux/macOS execution
have not been tested. No live deployment or release publishing is configured.
Local RustSec audit still reports maintenance-only warnings for paste and
ttf-parser; there are no vulnerability or unsoundness matches in the lockfile.
