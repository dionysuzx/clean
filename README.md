# clean

Strip formatting junk from LLM terminal output.

## Install

```bash
cargo install --path .
```

## Usage

```bash
cat dirty.txt | clean
```

Useful alias for clipboard:
```bash
alias clip='clean | pbcopy'
```

## What it does

- Removes box-drawing characters (`│┃╏╎▌`)
- Strips leading `>` and border pipes
- Merges broken lines while preserving lists and structure
- Normalizes whitespace
