# 🚀 Markdown to HTML Converter in Rust

A lightweight, blazing-fast CLI tool built in Rust that converts GitHub Flavored Markdown (GFM) into beautiful, responsive HTML files. It generates a completely standalone HTML page featuring automated light/dark mode layouts matching modern aesthetics, dynamic code syntax highlighting, and an integrated build timestamp.

---

## 🛠️ Features
* **Full GFM Support:** Handles tables, task lists, strikethroughs, and footnotes via `pulldown-cmark`.
* **Zero-Asset Standalone Generation:** Injects styling rules directly inside an inline `<style>` block, allowing you to move or share your HTML assets anywhere as a single file.
* **Adaptive Design:** Responsive layout inspired by GitHub's documentation style with automatic light/dark theme swapping via OS preferences.
* **On-the-Fly Syntax Highlighting:** Integrated with Prism.js to dynamically fetch and style code snippets for almost any standard coding language without binary bloat.

---

## 🚀 Quick Start

### 1. Prerequisites
Ensure you have the Rust toolchain installed. If not, get it via [rustup.rs](https://rustup.rs/).

### 2. Setup
Clone or create your project directory, then add the core dependencies to your `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
pulldown-cmark = "0.12"
clap = { version = "4.0", features = ["derive"] }
```

### 3. Compilation & Usage
Compile and execute the program through Cargo. The tool accepts positional targets for your input files alongside parameter flags for the title tag.

#### Example 1: Default Behavior (Same Directory & Name)
If you provide only the input file, the program will generate an HTML file with the exact same name and in the exact same folder, merely changing the extension to .html and utilizing the default page title.

```bash
cargo run -- README.md
```
- Reads from: README.md
- Outputs to: README.html (in the current directory)
- HTML Page Title: Rendered Markdown

#### Example 2: Explicit Custom Title & Target Outputs
To supply a custom string to the generated HTML <title> header tag, pass the -t or --title option flag:

```bash
cargo run -- --title "My Project Documentation" README.md index.html
```
- Reads from: README.md
- Outputs to: index.html
- HTML Page Title: My Project Documentation

#### Example 3: Routing to a Different Output Path using the short flag
You can use the short flag -t for the custom title and route the file output into a completely separate directory (such as a build or distribution folder).

`Note: The target output directory must already exist on your system before running.`

```bash
cargo run -- -t "API Guide" instructions.md dist/index.html
```
- Reads from: instructions.md
- Outputs to: dist/index.html
- HTML Page Title: API Guide


#### Example 4: Building a Standalone Binary for Production
When you are ready to use this tool globally without relying on cargo run, compile a highly optimized, standalone production binary:

```bash
cargo build --release
```
- This generates a compiled executable file with zero dependencies. You can find it at:

`Linux/macOS: ./target/release/md_to_html`
`Windows: .\target\release\md_to_html.exe`

Move this binary into your system's PATH (e.g., /usr/local/bin) to use it anywhere like a native CLI utility:

```bash
md_to_html -t "Production Readme" README.md build.html
```
----------

### Project Evolution & Iterations
This utility was developed across multiple iterative phases, scaling from a minimal hardcoded parser to a production-ready system tool.

#### Iteration 1: The Inline Proof-of-Concept
Goal: Verify parsing integrity and map a cohesive CSS baseline.

Mechanism: The system used a hardcoded, multiline Rust string as input inside main.rs and compiled a static HTML document named output.html.

-----------

#### Iteration 2: Dynamic File Input & Flexible I/O
Goal: Transition the tool from a hardcoded experiment into a functional CLI tool.

Mechanism: Introduced basic argument mapping to safely monitor inputs. The core architecture was decoupled from raw strings to dynamically intercept local target files (fs::read_to_string) and handle automatic naming path expansions.

-----------

#### Iteration 3: Code Highlighting & Layout Enhancements
Goal: Provide semantic syntax coloring across code blocks and add tracking elements without expanding binary footprint size.

Mechanism: Integrated a decentralized PrismJS layer combined with the prism-autoloader plugin to lazy-load required runtime code tokens on the fly. Added a dynamic conversion timestamp using the chrono crate to automatically sign documents upon construction.

-----------

#### Iteration 4: Command Line Option Handling
Goal: Standardize argument structures to allow flexible header title values.

Mechanism: Integrated the clap crate using its structural derive macro features. This decoupled positional arguments cleanly, provided explicit parameter fallback defaults, and automatically implemented an accessible --help text layer.

-----------

### Design Elements & Styles Used
Font Selection: System font-stacks match native OS typography (-apple-system, Segoe UI, Helvetica) preventing layout shift.

Theming Rules:

- Light Theme: Deep gray text, soft white canvas background, light-bordered syntax tags.
- Dark Theme: Deep obsidian profile background, crisp slate borders, contrasting text colors, and custom dark tokens matching the iconic Tomorrow-Night scheme.
