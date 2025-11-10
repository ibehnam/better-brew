# Better Brew

A faster Homebrew experience with parallel package downloads and upgrades.

## Why Better Brew?

Homebrew's `brew upgrade` fetches packages sequentially, which can be slow when updating multiple packages. Better Brew (`bbrew`) speeds this up by downloading all packages in parallel before installing them.

## Installation

```bash
cargo install better_brew
```

## Usage

### Update Homebrew

```bash
bbrew update
```

### Upgrade packages (parallel downloads)

```bash
bbrew upgrade
```

This will:
1. Update package definitions
2. Check for outdated packages
3. Fetch all packages **in parallel** (the fast part!)
4. Install the upgrades

## Requirements

- macOS or Linux
- [Homebrew](https://brew.sh) installed and in PATH
- Rust 1.70+ (for building from source)

## How it works

Instead of:
```
brew fetch package1 → brew fetch package2 → brew fetch package3 → brew upgrade
```

Better Brew does:
```
brew fetch package1 ┐
brew fetch package2 ├→ (all in parallel) → brew upgrade
brew fetch package3 ┘
```

This significantly reduces wait time when upgrading multiple packages.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

[Behnam Mohammadi](https://github.com/ibehnam)
