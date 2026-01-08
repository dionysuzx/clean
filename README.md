# clean

Strip formatting junk from LLM terminal output.

## Install

```bash
cargo build --release
cp target/release/clean /usr/local/bin/
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
