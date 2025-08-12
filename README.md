# Rust Inspired VS Code Theme 🦀

A warm, earthy VS Code theme inspired by Rust's branding with rich oranges and earth tones. Perfect for Rust developers who want their editor to match the spirit of their favorite systems programming language!

## Features

🎨 **Rust-Inspired Color Palette**

- Warm background with deep brown (`#1f1611`)
- Iconic Rust orange for keywords and status bar (`#ce422b`)
- Golden brown for functions (`#b58900`)
- Olive green for strings (`#859900`)
- Steel blue for types (`#268bd2`)
- Burlywood for variables and text (`#deb887`)

🔥 **Optimized for Rust Development**

- Special highlighting for Rust-specific syntax
- Enhanced macro and attribute colors
- Lifetime syntax highlighting
- Derive macro support

✨ **Complete Theme Coverage**

- Editor interface (background, selection, cursor)
- Activity bar and sidebar
- Status bar and title bar
- Terminal colors
- Syntax highlighting for all languages
- Git integration colors

## Installation

### Method 1: Manual Installation

1. Download or clone this repository
2. Copy the `vscode-rust-theme` folder to your VS Code extensions directory:
   - **Windows**: `%USERPROFILE%\\.vscode\\extensions\\`
   - **macOS**: `~/.vscode/extensions/`
   - **Linux**: `~/.vscode/extensions/`
3. Restart VS Code
4. Go to `File > Preferences > Color Theme` (or `Ctrl+K Ctrl+T`)
5. Select "Rust Inspired"

### Method 2: Package and Install

1. Install `vsce` (VS Code Extension Manager):

   ```bash
   npm install -g vsce
   ```

2. Package the extension:

   ```bash
   cd vscode-rust-theme
   vsce package
   ```

3. Install the `.vsix` file:

   ```bash
   code --install-extension rust-inspired-theme-1.0.0.vsix
   ```

## Color Reference

| Element | Color | Usage |
|---------|-------|--------|
| Background | `#1f1611` | Editor background |
| Foreground | `#deb887` | Default text |
| Keywords | `#ce422b` | `fn`, `let`, `pub`, etc. |
| Functions | `#b58900` | Function names |
| Types | `#268bd2` | Struct, enum, type names |
| Strings | `#859900` | String literals |
| Numbers | `#d33682` | Numeric values |
| Comments | `#93a1a1` | Code comments |
| Operators | `#cb4b16` | `+`, `-`, `->`, etc. |

## Contributing

Feel free to submit issues and enhancement requests! This theme is designed to be a warm, comfortable coding environment that celebrates Rust's aesthetic.

## License

MIT License - see LICENSE file for details.

---

Made with ❤️ for the Rust community 🦀
