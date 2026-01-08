use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {
    let input = if atty::is(atty::Stream::Stdin) {
        // Interactive: read from clipboard
        match read_clipboard() {
            Some(text) => text,
            None => {
                eprintln!("Failed to read clipboard");
                std::process::exit(1);
            }
        }
    } else {
        // Piped input
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("failed to read stdin");
        input
    };

    let cleaned = clean(&input);

    // Output to stdout
    println!("{}", cleaned);

    // Copy to clipboard
    if copy_to_clipboard(&cleaned) {
        eprintln!("✓ Clipboard cleaned");
    }
}

fn read_clipboard() -> Option<String> {
    // macOS
    if let Ok(output) = Command::new("pbpaste").output() {
        if output.status.success() {
            return String::from_utf8(output.stdout).ok();
        }
    }

    // Linux (xclip)
    if let Ok(output) = Command::new("xclip")
        .args(["-selection", "clipboard", "-o"])
        .output()
    {
        if output.status.success() {
            return String::from_utf8(output.stdout).ok();
        }
    }

    None
}

fn copy_to_clipboard(text: &str) -> bool {
    // macOS
    if let Ok(mut child) = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    // Linux (xclip)
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    false
}

fn clean(input: &str) -> String {
    if input.trim().is_empty() {
        return String::new();
    }

    // Remove box-drawing characters
    let text: String = input
        .chars()
        .filter(|c| !matches!(c, '│' | '┃' | '╏' | '╎' | '▌'))
        .collect();

    // Normalize pipe chars used as box borders (but not in code context)
    let text = remove_border_pipes(&text);

    // Process lines
    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::new();
    let mut current = String::new();

    for line in lines {
        let trimmed = line.trim();

        // Empty line = paragraph break
        if trimmed.is_empty() {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
            result.push(String::new());
            continue;
        }

        // Should this line start a new block?
        if should_break_before(trimmed) {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
            current = trimmed.to_string();
        } else if current.is_empty() {
            current = trimmed.to_string();
        } else {
            // Merge with previous line
            current.push(' ');
            current.push_str(trimmed);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    // Collapse multiple blank lines and normalize spaces within lines
    let mut output = Vec::new();
    let mut prev_blank = false;

    for line in result {
        if line.is_empty() {
            if !prev_blank && !output.is_empty() {
                output.push(String::new());
                prev_blank = true;
            }
        } else {
            output.push(normalize_spaces(&line));
            prev_blank = false;
        }
    }

    // Trim trailing blank lines
    while output.last().map(|s| s.is_empty()).unwrap_or(false) {
        output.pop();
    }

    output.join("\n")
}

fn should_break_before(line: &str) -> bool {
    let line = line.trim_start();

    // List markers
    if line.starts_with('-')
        || line.starts_with('*')
        || line.starts_with('•')
        || line.starts_with('●')
        || line.starts_with('⏺')
        || line.starts_with('▶')
        || line.starts_with('▪')
        || line.starts_with('◦')
    {
        return true;
    }

    // Numbered lists
    if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        && line.contains('.')
    {
        let dot_pos = line.find('.').unwrap();
        if line[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }

    // Git diff line numbers (e.g., "123 + ...")
    if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        let rest: String = line.chars().skip_while(|c| c.is_ascii_digit()).collect();
        let rest = rest.trim_start();
        if rest.starts_with('+') || rest.starts_with('-') {
            return true;
        }
    }

    // Headings (line starting with capital followed by lowercase)
    if let Some(first) = line.chars().next() {
        if first.is_uppercase() {
            if let Some(second) = line.chars().nth(1) {
                if second.is_lowercase() {
                    // Could be a heading/sentence start - check if previous content exists
                    return true;
                }
            }
        }
    }

    // Common emojis used as markers
    let emoji_markers = ['📌', '🎯', '📋', '📖', '✨', '✅', '❌', '⭐', '🔥', '👉', '➡'];
    for emoji in emoji_markers {
        if line.starts_with(emoji) {
            return true;
        }
    }

    false
}

fn remove_border_pipes(text: &str) -> String {
    text.lines()
        .map(|line| {
            let trimmed = line.trim();
            // Remove leading/trailing pipes that look like borders
            let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
            let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
            // Remove leading > (markdown quote / prompt prefix)
            let trimmed = trimmed.trim();
            let trimmed = trimmed.strip_prefix('>').unwrap_or(trimmed);
            trimmed.trim()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_spaces(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_space = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(c);
            prev_space = false;
        }
    }

    result.trim().to_string()
}
