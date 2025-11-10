# 🍻 Better Brew

A faster, smarter Homebrew experience with parallel package operations, intelligent concurrency control, and real-time progress tracking.

## Why Better Brew?

Homebrew's default commands process packages sequentially, which can be slow when managing multiple packages. Better Brew (`bbrew`) speeds this up by:
- **Parallel Operations**: Downloads and installs packages concurrently
- **Smart Concurrency**: Limits concurrent operations to prevent CPU overload
- **Visual Feedback**: Real-time progress bars with ETA for all operations

## Features

✨ **Parallel Package Operations** - Install, upgrade, and reinstall multiple packages simultaneously
⚡ **Concurrency Control** - Intelligent limiting (max 4 concurrent ops) prevents system slowdown
📊 **Progress Tracking** - Beautiful progress bars with elapsed time and ETA
🎯 **Drop-in Replacement** - Use `bbrew` instead of `brew` for faster operations
🔒 **Safe & Reliable** - Individual package failures don't stop the entire operation

## Installation

```bash
cargo install better_brew
```

## Usage

### Update Homebrew

```bash
bbrew update
```

Updates Homebrew and package definitions.

### Upgrade All Outdated Packages

```bash
bbrew upgrade
```

This will:
1. Update package definitions
2. Check for outdated packages
3. Fetch all packages **in parallel** with progress tracking
4. Install the upgrades

### Install Packages in Parallel

```bash
bbrew install wget curl jq ripgrep fd bat
```

Installs multiple packages concurrently with real-time progress.

### Reinstall Packages

```bash
# Reinstall specific packages
bbrew reinstall node python rust

# Reinstall ALL installed packages (useful for troubleshooting)
bbrew reinstall --all
```

## Example Output

```
=== Better Brew Install ===

Installing 4 package(s): wget, curl, jq, ripgrep

Installing packages with 4 concurrent operations...
✓ Installed: wget
✓ Installed: curl
🔄 [00:00:15] [####>-----] 3/4 (00:00:05) Installing ripgrep
✓ Installed: jq

✓ Successfully installed 4 package(s)

✓ Install complete!
```

## Requirements

- macOS or Linux
- [Homebrew](https://brew.sh) installed and in PATH
- Rust 1.70+ (for building from source)

## How it works

### Traditional Sequential Approach
```
brew install pkg1 → brew install pkg2 → brew install pkg3 → brew install pkg4
Total time: Sum of all operations
```

### Better Brew Parallel Approach (v0.3.0)
```
brew install pkg1 ┐
brew install pkg2 ├─→ (max 4 concurrent)
brew install pkg3 │   + progress tracking
brew install pkg4 ┘

Total time: ~Longest operation × (total_packages ÷ 4)
```

**Key Features**:
- **Concurrency Limiting**: Maximum 4 concurrent operations prevents CPU overload
- **Progress Tracking**: Real-time progress bars show what's happening
- **Smart Resource Management**: Balances speed with system stability

This significantly reduces wait time when managing multiple packages while keeping your system responsive.

## Version History

### v0.3.0 (Latest)
- ✨ Added real-time progress bars with ETA
- ⚡ Implemented concurrency limiting (max 4 concurrent operations)
- 🎯 Prevents CPU overload from too many simultaneous brew processes
- 📊 Better visual feedback during operations

### v0.2.0
- Added `install` command for parallel package installation
- Added `reinstall` command with `--all` flag support
- Parallel execution for all package operations

### v0.1.0
- Initial release with parallel `upgrade` command

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Author

[Behnam Mohammadi](https://github.com/ibehnam)
