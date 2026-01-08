To reproduce: `git clone https://github.com/TheJoWo/Clean-Clode` and feed this prompt to an LLM in that directory.

---

Rewrite `Clean-Clode` as a Rust CLI called `clean` that strips formatting junk from LLM terminal output.

Behavior:
- Clipboard-first: when run without piped input, read from clipboard, clean, print to stdout, and copy result back to clipboard
- When piped: read from stdin, clean, print to stdout, copy to clipboard
- Use `pbpaste`/`pbcopy` on macOS, `xclip` on Linux

Cleaning logic:
- Strip box-drawing chars: │ ┃ ╏ ╎ ▌
- Strip leading/trailing `|` borders per line
- Strip leading `>` and `›` prefixes (markdown quotes, prompt arrows)
- Merge broken lines into paragraphs, preserving structure for:
  - List markers: - * • ● ⏺ ▶ ▪ ◦
  - Numbered lists: 1. 2. etc
  - Lines starting with emoji markers
  - Headings (capital followed by lowercase)
- Collapse multiple blank lines to single blank line
- Normalize multiple spaces to single space

Keep it minimal and idiomatic.
