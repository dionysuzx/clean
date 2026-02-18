# clean

Strip formatting junk from LLM terminal output.

## Install

```bash
cargo install --path .
```

## Usage

Copy messy text, then:
```bash
clean
```
Clipboard is cleaned. Paste.

Also works with pipes:
```bash
cat dirty.txt | clean
```

## Raycast

Add `clean-clipboard.sh` as a [Raycast script command](https://github.com/raycast/script-commands):

1. Open Raycast Settings > Extensions > Script Commands
2. Add the script directory (this repo)
3. Bind a keyboard shortcut

Then just copy messy text and hit your shortcut. Clipboard is cleaned.

## What it does

- Removes box-drawing characters (`│┃╏╎▌`)
- Strips leading `>` and border pipes
- Merges broken lines while preserving lists and structure
- Normalizes whitespace
- Copies result to clipboard
