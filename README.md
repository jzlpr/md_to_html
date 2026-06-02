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
