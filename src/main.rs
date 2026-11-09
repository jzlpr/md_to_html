use chrono::Local;
use clap::Parser as ClapParser;
use pulldown_cmark::{Event, Options, Parser as MdParser, Tag, TagEnd, html};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    let input_path = PathBuf::from(&args.input_file);

    let output_path = match args.output_file {
        Some(path) => PathBuf::from(path),
        None => input_path.with_extension("html"),
    };

    // 2. Read the Markdown file
    println!("📖 Reading Markdown from: {}", input_path.display());
    let markdown_input = fs::read_to_string(&input_path)?;

    // 3. Track current conversion time
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 4. Set up GFM Markdown parser
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // 5. Prepare image handling
    let input_dir = input_path.parent().unwrap_or(Path::new(""));
    let output_dir = output_path.parent().unwrap_or(Path::new(""));
    let images_dir = output_dir.join("images");

    // Map from original image path to new relative path in images/
    let mut image_map: HashMap<String, String> = HashMap::new();
    // Track used filenames in images/ for conflict resolution
    let mut used_filenames: HashSet<String> = HashSet::new();

    // Scan existing images in output images/ directory to avoid conflicts
    if images_dir.exists() {
        for entry in WalkDir::new(&images_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    used_filenames.insert(name.to_string());
                }
            }
        }
    }

    // 6. Parse markdown into events
    let parser = MdParser::new_ext(&markdown_input, options);
    let events: Vec<Event> = parser.collect();

    // 7. Process events to handle images
    let mut processed_events = Vec::new();
    let mut i = 0;
    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                let dest = dest_url.to_string();
                let alt = title.to_string();

                // Process image if it's a relative path (not HTTP/HTTPS)
                let new_dest = if !dest.starts_with("http://") && !dest.starts_with("https://") {
                    let src_path = input_dir.join(&dest);
                    if src_path.exists() {
                        // Determine target filename (flatten: just the basename)
                        let filename = src_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("image.png");

                        // Resolve conflicts by appending _1, _2, etc.
                        let mut final_name = filename.to_string();
                        let mut counter = 1;
                        while used_filenames.contains(&final_name) {
                            let stem = Path::new(filename).file_stem().and_then(|s| s.to_str()).unwrap_or("image");
                            let ext = Path::new(filename).extension().and_then(|e| e.to_str()).unwrap_or("png");
                            final_name = format!("{}_{}.{}", stem, counter, ext);
                            counter += 1;
                        }
                        used_filenames.insert(final_name.clone());

                        // Create images directory
                        fs::create_dir_all(&images_dir)?;

                        // Copy the image
                        let dst_path = images_dir.join(&final_name);
                        fs::copy(&src_path, &dst_path)?;
                        println!("🖼️  Copied image: {} -> images/{}", dest, final_name);

                        // Map to new relative path from output HTML
                        let new_rel_path = Path::new("images").join(&final_name);
                        new_rel_path.to_string_lossy().to_string()
                    } else {
                        println!("⚠️  Image not found, keeping original reference: {}", dest);
                        dest
                    }
                } else {
                    // Keep HTTP/HTTPS URLs as-is
                    dest
                };

                // Create new image tag with updated destination
                processed_events.push(Event::Start(Tag::Image {
                    link_type: pulldown_cmark::LinkType::Inline,
                    dest_url: new_dest.into(),
                    title: alt.into(),
                    id: "".into(),
                }));
                i += 1;
                // Skip the corresponding End event since we'll add our own
                while i < events.len() && !matches!(events[i], Event::End(TagEnd::Image)) {
                    processed_events.push(events[i].clone());
                    i += 1;
                }
                if i < events.len() && matches!(events[i], Event::End(TagEnd::Image)) {
                    processed_events.push(Event::End(TagEnd::Image));
                    i += 1;
                }
            }
            _ => {
                processed_events.push(events[i].clone());
                i += 1;
            }
        }
    }

    // 8. Render HTML from processed events
    let mut html_output = String::new();
    html::push_html(&mut html_output, processed_events.into_iter());

    // 9. Native Responsive Theme CSS
    let css_styles = r#":root {
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
/* Image styling */
.markdown-body img {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1.5rem auto;
    border-radius: 6px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}
"#;

    // 10. Assemble HTML template
    let full_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<style>{}</style>
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css" />
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" media="(prefers-color-scheme: dark)" />
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

    // 11. Write everything to a single autonomous HTML target file
    let mut html_file = File::create(&output_path)?;
    html_file.write_all(full_html.as_bytes())?;
    println!("🎉 Success! Standalone file generated at: {}", output_path.display());

    Ok(())
}