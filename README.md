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

## What it does

- Removes box-drawing characters (`│┃╏╎▌`)
- Strips leading `>` and border pipes
- Merges broken lines while preserving lists and structure
- Normalizes whitespace
- Copies result to clipboard
