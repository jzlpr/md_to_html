use chrono::Local;
use pulldown_cmark::{Options, Parser, html};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 1. Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file.md> [output_file.html]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];

    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        let path = Path::new(input_path);
        path.with_extension("html").to_string_lossy().into_owned()
    };

    // 2. Capture and format the current date and time of the conversion
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 3. Read the Markdown file
    println!("📖 Reading Markdown from: {}", input_path);
    let markdown_input = fs::read_to_string(input_path)?;

    // 4. Enable GitHub Flavored Markdown options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&markdown_input, options);

    // 5. Parse Markdown into HTML fragments
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 6. Modern GitHub CSS & Prism Theme Adjustments
    let css_styles = r#"
        :root {
            --bg-color: #ffffff;
            --text-color: #24292f;
            --link-color: #0969da;
            --border-color: #d0d7de;
            --code-bg: #f6f8fa;
            --quote-color: #57606a;
        }
        @media (prefers-color-scheme: dark) {
            :root {
                --bg-color: #0d1117;
                --text-color: #c9d1d9;
                --link-color: #58a6ff;
                --border-color: #30363d;
                --code-bg: #161b22;
                --quote-color: #8b949e;
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
        h1, h2, h3 {
            font-weight: 600;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.3em;
            margin-top: 24px;
            margin-bottom: 16px;
        }
        a { color: var(--link-color); text-decoration: none; }
        a:hover { text-decoration: underline; }

        /* Inline code styling */
        :not(pre) > code {
            padding: 0.2em 0.4em;
            background-color: var(--code-bg);
            border-radius: 6px;
            font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, monospace;
            font-size: 85%;
        }

        /* Code block container */
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

    // 7. Wrap everything with dynamic PrismJS CDN assets
    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rendered Markdown</title>
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
        css_styles, timestamp, html_output
    );

    // 8. Save to the target HTML file
    let mut file = File::create(&output_path)?;
    file.write_all(full_html.as_bytes())?;

    println!("🎉 Success! Generated: {}", output_path);
    Ok(())
}
