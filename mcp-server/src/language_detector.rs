//! Language detection for source code analysis

use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LanguageDetector {
    patterns: HashMap<String, Vec<&'static str>>,
    extensions: HashMap<String, String>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        let mut extensions = HashMap::new();

        // Rust patterns - using simple string contains for now
        patterns.insert("rust".to_string(), vec![
            "fn main(",
            "let mut",
            "impl ",
            "use std::",
            "match ",
            "Result<",
            "println!",
            ": i32",
        ]);
        extensions.insert(".rs".to_string(), "rust".to_string());

        // Python patterns
        patterns.insert("python".to_string(), vec![
            "def ",
            "import ",
            "from ",
            "if __name__",
            "class ",
            "print(",
        ]);
        extensions.insert(".py".to_string(), "python".to_string());

        // JavaScript patterns
        patterns.insert("javascript".to_string(), vec![
            "function ",
            "const ",
            "let ",
            "var ",
            "=>",
            "require(",
            "console.log",
        ]);
        extensions.insert(".js".to_string(), "javascript".to_string());
        extensions.insert(".ts".to_string(), "typescript".to_string());

        // Go patterns
        patterns.insert("go".to_string(), vec![
            "func ",
            "package ",
            "import \"",
            "type ",
            " struct {",
            "go ",
        ]);
        extensions.insert(".go".to_string(), "go".to_string());

        // C patterns
        patterns.insert("c".to_string(), vec![
            "#include",
            "int main(",
            "printf(",
            "malloc(",
            "void ",
            "#define",
        ]);
        extensions.insert(".c".to_string(), "c".to_string());
        extensions.insert(".h".to_string(), "c".to_string());

        // Ruchy patterns (similar to Rust but with 'fun' instead of 'fn')
        patterns.insert("ruchy".to_string(), vec![
            "fun ",
            "let ",
            "use ",
            "impl ",
            "ruchy::",
            "#[verify",
        ]);
        extensions.insert(".ruchy".to_string(), "ruchy".to_string());

        Self { patterns, extensions }
    }

    pub fn detect(&self, code: &str) -> Result<String> {
        let mut scores = HashMap::new();

        // Score each language based on pattern matches
        for (language, patterns) in &self.patterns {
            let mut score = 0;
            for pattern in patterns {
                score += code.matches(pattern).count();
            }
            scores.insert(language.clone(), score);
        }

        // Find the language with the highest score
        let detected = scores
            .iter()
            .max_by_key(|(_, &score)| score)
            .map(|(lang, _)| lang.clone());

        match detected {
            Some(lang) if scores[&lang] > 0 => Ok(lang),
            _ => Err(anyhow!("Could not detect programming language")),
        }
    }

    pub fn detect_by_extension(&self, filename: &str) -> Option<String> {
        for (ext, lang) in &self.extensions {
            if filename.ends_with(ext) {
                return Some(lang.clone());
            }
        }
        None
    }

    pub fn supported_languages(&self) -> Vec<String> {
        self.patterns.keys().cloned().collect()
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_detection() {
        let detector = LanguageDetector::new();
        let rust_code = "fn main() { let x: i32 = 42; println!(\"Hello\"); }";
        assert_eq!(detector.detect(rust_code).unwrap(), "rust");
    }

    #[test]
    fn test_python_detection() {
        let detector = LanguageDetector::new();
        let python_code = "def main():\n    print(\"Hello\")\n\nif __name__ == \"__main__\":\n    main()";
        assert_eq!(detector.detect(python_code).unwrap(), "python");
    }

    #[test]
    fn test_javascript_detection() {
        let detector = LanguageDetector::new();
        let js_code = "function main() { const x = 42; console.log(\"Hello\"); }";
        assert_eq!(detector.detect(js_code).unwrap(), "javascript");
    }

    #[test]
    fn test_go_detection() {
        let detector = LanguageDetector::new();
        let go_code = "package main\nfunc main() { fmt.Println(\"Hello\") }";
        assert_eq!(detector.detect(go_code).unwrap(), "go");
    }

    #[test]
    fn test_c_detection() {
        let detector = LanguageDetector::new();
        let c_code = "#include <stdio.h>\nint main() { printf(\"Hello\"); }";
        assert_eq!(detector.detect(c_code).unwrap(), "c");
    }

    #[test]
    fn test_ruchy_detection() {
        let detector = LanguageDetector::new();
        let ruchy_code = "fun main() { let x = 42; println(\"Hello\"); }";
        assert_eq!(detector.detect(ruchy_code).unwrap(), "ruchy");
    }

    #[test]
    fn test_extension_detection() {
        let detector = LanguageDetector::new();
        
        assert_eq!(detector.detect_by_extension("main.rs"), Some("rust".to_string()));
        assert_eq!(detector.detect_by_extension("script.py"), Some("python".to_string()));
        assert_eq!(detector.detect_by_extension("app.js"), Some("javascript".to_string()));
        assert_eq!(detector.detect_by_extension("main.go"), Some("go".to_string()));
        assert_eq!(detector.detect_by_extension("program.c"), Some("c".to_string()));
        assert_eq!(detector.detect_by_extension("example.ruchy"), Some("ruchy".to_string()));
    }
}