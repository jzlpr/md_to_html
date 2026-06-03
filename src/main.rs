use chrono::Local;
use clap::Parser as ClapParser;
use pulldown_cmark::{Options, Parser as MdParser, html};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// A lightweight utility to convert GitHub Flavored Markdown into beautiful, responsive HTML.
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input Markdown (.md) file path
    #[arg(required = true)]
    input_file: String,

    /// The output HTML (.html) file path (Defaults to replacing .md with .html)
    #[arg(required = false)]
    output_file: Option<String>,

    /// Custom string to inject into the HTML <title> tag
    #[arg(short, long, default_value = "Rendered Markdown")]
    title: String,
}

fn main() -> std::io::Result<()> {
    // 1. Parse CLI arguments using clap
    let args = Args::parse();

    let input_path = &args.input_file;

    // Fallback logic for output file path if not provided
    let output_path = match args.output_file {
        Some(path) => path,
        None => {
            let path = Path::new(input_path);
            path.with_extension("html").to_string_lossy().into_owned()
        }
    };

    // 2. Read the Markdown file
    println!("📖 Reading Markdown from: {}", input_path);
    let markdown_input = fs::read_to_string(input_path)?;

    // 3. Track current conversion time
    let timestamp = Local::now().format("%Y-%m-%d").to_string();

    // 4. Set up GFM Markdown parser
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = MdParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 5. Embedded Responsive CSS & Prism Themes
    let css_styles = r#"
        :root {
            --bg-color: #ffffff;
            --text-color: #24292f;
            --link-color: #0969da;
            --border-color: #d0d7de;
            --code-bg: #f6f8fa;
            --quote-color: #57606a;
            --meta-color: #57606a;
        }
        @media (prefers-color-scheme: dark) {
            :root {
                --bg-color: #0d1117;
                --text-color: #c9d1d9;
                --link-color: #58a6ff;
                --border-color: #30363d;
                --code-bg: #161b22;
                --quote-color: #8b949e;
                --meta-color: #8b949e;
            }
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            font-size: 16px;
            line-height: 1.6;
            background-color: var(--bg-color);
            color: var(--text-color);
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }
        .meta-timestamp {
            font-size: 0.85rem;
            color: var(--meta-color);
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.5rem;
            margin-bottom: 2rem;
        }
        h1, h2, h3 {
            font-weight: 600;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.3em;
            margin-top: 24px;
            margin-bottom: 16px;
        }
        a { color: var(--link-color); text-decoration: none; }
        a:hover { text-decoration: underline; }
        :not(pre) > code {
            padding: 0.2em 0.4em;
            background-color: var(--code-bg);
            border-radius: 6px;
            font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, monospace;
            font-size: 85%;
        }
        pre {
            padding: 16px;
            background-color: var(--code-bg) !important;
            border: 1px solid var(--border-color);
            border-radius: 6px;
            overflow: auto;
            margin: 1em 0;
        }
        pre code {
            font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, monospace !important;
            font-size: 85% !important;
            text-shadow: none !important;
            background: none !important;
            padding: 0 !important;
        }
        blockquote {
            padding: 0 1em;
            color: var(--quote-color);
            border-left: 0.25em solid var(--border-color);
            margin: 0;
        }
        ul, ol { padding-left: 2em; }
        li + li { margin-top: 0.25em; }
        input[type="checkbox"] { margin-right: 0.5em; vertical-align: middle; }
        hr { height: 0.25em; background-color: var(--border-color); border: 0; margin: 24px 0; }
    "#;

    // 6. Assemble HTML template using the custom dynamic title
    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css" />
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" media="(prefers-color-scheme: dark)" />
    <style>{}</style>
</head>
<body>
    <article class="markdown-body">
        <div class="meta-timestamp">📄 Generated on: {}</div>
        {}
    </article>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
</body>
</html>"#,
        args.title, css_styles, timestamp, html_output
    );

    // 7. Write out to target location
    let mut file = File::create(&output_path)?;
    file.write_all(full_html.as_bytes())?;

    println!("🎉 Success! Generated: '{}' at {}", output_path, timestamp);
    Ok(())
}
