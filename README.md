# 🚀 Markdown to HTML Converter in Rust

A lightweight, blazing-fast CLI tool built in Rust that converts GitHub Flavored Markdown (GFM) into beautiful, responsive HTML files. It features automated light/dark mode styling matching modern aesthetics and fully dynamic code syntax highlighting.

---

## 🛠️ Features
* **Full GFM Support:** Handles tables, task lists, strikethroughs, and footnotes via `pulldown-cmark`.
* **Adaptive Styling:** Responsive layout inspired by GitHub's documentation style with automatic light/dark theme swapping via OS preferences.
* **On-the-Fly Syntax Highlighting:** Integrated with Prism.js to dynamically fetch and style code snippets for almost any standard coding language without binary bloat.

---

## 🚀 Quick Start

### 1. Prerequisites
Ensure you have the Rust toolchain installed. If not, get it via [rustup.rs](https://rustup.rs/).

### 2. Setup
Clone or create your project directory, then add the core dependencies to your `Cargo.toml`:

```toml
[dependencies]
pulldown-cmark = "0.12"  # GFM-compliant markdown parsing engine
```

### 3. Compilation & Usage
Compile and execute the program through Cargo. The tool is flexible and accepts either a single input file or an explicit output destination path.

#### Example 1: Default Behavior (Same Directory & Name)
If you provide only the input file, the program will generate an HTML file with the exact same name and in the exact same folder, merely changing the extension to `.html`.

```bash
cargo run -- README.md
```
- Reads from: README.md
- Outputs to: README.html (in the current directory)

#### Example 2: Explicit Custom File Name
To give your generated web page a completely different name, provide it as the second argument:

```bash
cargo run -- README.md index.html
```
- Reads from: README.md
- Outputs to: index.html

#### Example 3: Routing to a Different Output Path
You can direct the program to output the file into a completely separate directory (such as a build or distribution folder).

`Note: The target output directory must already exist on your system before running.`

```bash
# Save to a dedicated build folder
cargo run -- instructions.md dist/index.html

# Save to an entirely different path location
cargo run -- docs/api_v1.md /var/www/html/api-docs.html
```
- Reads from: docs/api_v1.md
- Outputs to: /var/www/html/api-docs.html
