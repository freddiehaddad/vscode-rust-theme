// Example Rust code to showcase the Rust Inspired Theme

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RustTheme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub version: semver::Version,
}

impl RustTheme {
    /// Creates a new Rust-inspired theme with warm colors
    pub fn new() -> Result<Self> {
        let mut colors = HashMap::new();

        // Rust orange for keywords
        colors.insert("keyword".to_string(), "#ce422b".to_string());

        // Golden brown for functions
        colors.insert("function".to_string(), "#b58900".to_string());

        // Steel blue for types
        colors.insert("type".to_string(), "#268bd2".to_string());

        // Olive green for strings
        colors.insert("string".to_string(), "#859900".to_string());

        Ok(RustTheme {
            name: "Rust Inspired".to_string(),
            colors,
            version: semver::Version::new(1, 0, 0),
        })
    }

    pub fn apply(&self) -> Result<()> {
        println!("🦀 Applying {} theme v{}", self.name, self.version);

        for (element, color) in &self.colors {
            println!("  {} -> {}", element, color);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = RustTheme::new().unwrap();
        assert_eq!(theme.name, "Rust Inspired");
        assert!(!theme.colors.is_empty());
    }
}

fn main() -> Result<()> {
    let theme = RustTheme::new()?;
    theme.apply()?;

    println!("Theme applied successfully! 🎨");
    Ok(())
}
