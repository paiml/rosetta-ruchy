//! Code translation service for converting various languages to Ruchy

use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;

pub struct CodeTranslator {
    translators: HashMap<String, Box<dyn LanguageTranslator>>,
}

trait LanguageTranslator: Send + Sync {
    fn translate(&self, source: &str) -> Result<String>;
}

impl CodeTranslator {
    pub fn new() -> Self {
        let mut translators: HashMap<String, Box<dyn LanguageTranslator>> = HashMap::new();
        
        translators.insert("rust".to_string(), Box::new(RustToRuchyTranslator::new()));
        translators.insert("python".to_string(), Box::new(PythonToRuchyTranslator::new()));
        translators.insert("javascript".to_string(), Box::new(JavaScriptToRuchyTranslator::new()));
        translators.insert("go".to_string(), Box::new(GoToRuchyTranslator::new()));
        translators.insert("c".to_string(), Box::new(CToRuchyTranslator::new()));

        Self { translators }
    }

    pub fn translate_to_ruchy(&self, source: &str, source_language: &str) -> Result<String> {
        if source_language == "ruchy" {
            return Ok(source.to_string());
        }

        match self.translators.get(source_language) {
            Some(translator) => translator.translate(source),
            None => Err(anyhow!("Unsupported source language: {}", source_language)),
        }
    }

    pub fn supported_languages(&self) -> Vec<String> {
        self.translators.keys().cloned().collect()
    }
}

impl Default for CodeTranslator {
    fn default() -> Self {
        Self::new()
    }
}

// Rust to Ruchy translator
struct RustToRuchyTranslator {
    patterns: Vec<(Regex, String)>,
}

impl RustToRuchyTranslator {
    fn new() -> Self {
        let patterns = vec![
            // Function definitions: fn -> fun
            (Regex::new(r"\bfn\b").unwrap(), "fun".to_string()),
            
            // Main function stays the same but add explicit call
            (Regex::new(r"fn main\(\) \{([^}]*)\}").unwrap(), "fun main() {$1}\n\nmain()".to_string()),
            
            // String literals with explicit printing
            (Regex::new(r#"println!\("([^"]+)"\);"#).unwrap(), r#"println("$1");"#.to_string()),
            (Regex::new(r#"print!\("([^"]+)"\);"#).unwrap(), r#"print("$1");"#.to_string()),
            
            // Remove explicit type annotations in simple cases
            (Regex::new(r"let (\w+): (\w+) =").unwrap(), "let $1 =".to_string()),
        ];

        Self {
            patterns: patterns.into_iter().map(|(r, s)| (r, s)).collect(),
        }
    }
}

impl LanguageTranslator for RustToRuchyTranslator {
    fn translate(&self, source: &str) -> Result<String> {
        let mut result = source.to_string();

        // Apply transformation patterns
        for (pattern, replacement) in &self.patterns {
            result = pattern.replace_all(&result, replacement.as_str()).to_string();
        }

        // Add Ruchy-specific enhancements
        result = format!(
            "// Translated from Rust to Ruchy\n// Enhanced with Ruchy's advanced tooling capabilities\n\n{}\n",
            result
        );

        Ok(result)
    }
}

// Python to Ruchy translator
struct PythonToRuchyTranslator;

impl PythonToRuchyTranslator {
    fn new() -> Self {
        Self
    }
}

impl LanguageTranslator for PythonToRuchyTranslator {
    fn translate(&self, source: &str) -> Result<String> {
        let mut result = String::new();
        
        result.push_str("// Translated from Python to Ruchy\n");
        result.push_str("// Enhanced with static typing and formal verification\n\n");

        // Simple Python to Ruchy translation
        let lines: Vec<&str> = source.lines().collect();
        let mut in_function = false;
        let mut indent_level = 0;

        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                result.push('\n');
                continue;
            }

            // Function definitions
            if trimmed.starts_with("def ") {
                let func_def = trimmed.replace("def ", "fun ").replace(":", " {");
                result.push_str(&format!("{}\n", func_def));
                in_function = true;
                indent_level += 1;
                continue;
            }

            // Print statements
            if trimmed.starts_with("print(") {
                let print_stmt = trimmed.replace("print(", "println(");
                result.push_str(&format!("    {}\n", print_stmt));
                continue;
            }

            // Variable assignments
            if trimmed.contains(" = ") && !trimmed.starts_with("if") {
                let assignment = format!("let {};", trimmed);
                result.push_str(&format!("    {}\n", assignment));
                continue;
            }

            // Main guard
            if trimmed.contains("if __name__") {
                result.push_str("\nmain()\n");
                break;
            }

            // Default: add with proper indentation
            let indentation = "    ".repeat(indent_level);
            result.push_str(&format!("{}{}\n", indentation, trimmed));
        }

        // Close any open functions
        if in_function {
            result.push_str("}\n");
        }

        Ok(result)
    }
}

// JavaScript to Ruchy translator
struct JavaScriptToRuchyTranslator;

impl JavaScriptToRuchyTranslator {
    fn new() -> Self {
        Self
    }
}

impl LanguageTranslator for JavaScriptToRuchyTranslator {
    fn translate(&self, source: &str) -> Result<String> {
        let mut result = String::new();
        
        result.push_str("// Translated from JavaScript to Ruchy\n");
        result.push_str("// Enhanced with compile-time safety and verification\n\n");

        let patterns = vec![
            // Function declarations
            (Regex::new(r"function\s+(\w+)\s*\(([^)]*)\)\s*\{").unwrap(), "fun $1($2) {"),
            
            // Arrow functions (simple case)
            (Regex::new(r"const\s+(\w+)\s*=\s*\([^)]*\)\s*=>\s*\{").unwrap(), "fun $1() {"),
            
            // Variable declarations
            (Regex::new(r"\b(const|let|var)\s+(\w+)\s*=").unwrap(), "let $2 ="),
            
            // Console.log
            (Regex::new(r"console\.log\(").unwrap(), "println("),
        ];

        let mut translated = source.to_string();
        for (pattern, replacement) in patterns {
            translated = pattern.replace_all(&translated, replacement).to_string();
        }

        result.push_str(&translated);
        result.push_str("\n\nmain()\n");

        Ok(result)
    }
}

// Go to Ruchy translator
struct GoToRuchyTranslator;

impl GoToRuchyTranslator {
    fn new() -> Self {
        Self
    }
}

impl LanguageTranslator for GoToRuchyTranslator {
    fn translate(&self, source: &str) -> Result<String> {
        let mut result = String::new();
        
        result.push_str("// Translated from Go to Ruchy\n");
        result.push_str("// Enhanced with formal verification and zero-cost abstractions\n\n");

        let patterns = vec![
            // Remove package declaration
            (Regex::new(r"package\s+\w+\n?").unwrap(), ""),
            
            // Remove import statements (for now)
            (Regex::new(r"import\s+[^\n]+\n?").unwrap(), ""),
            
            // Function declarations
            (Regex::new(r"\bfunc\s+(\w+)\s*\(([^)]*)\)").unwrap(), "fun $1($2)"),
            
            // Variable declarations
            (Regex::new(r"(\w+)\s*:=\s*").unwrap(), "let $1 = "),
            
            // fmt.Println
            (Regex::new(r"fmt\.Println\(").unwrap(), "println("),
        ];

        let mut translated = source.to_string();
        for (pattern, replacement) in patterns {
            translated = pattern.replace_all(&translated, replacement).to_string();
        }

        result.push_str(&translated);
        result.push_str("\n\nmain()\n");

        Ok(result)
    }
}

// C to Ruchy translator
struct CToRuchyTranslator;

impl CToRuchyTranslator {
    fn new() -> Self {
        Self
    }
}

impl LanguageTranslator for CToRuchyTranslator {
    fn translate(&self, source: &str) -> Result<String> {
        let mut result = String::new();
        
        result.push_str("// Translated from C to Ruchy\n");
        result.push_str("// Enhanced with memory safety and automatic memory management\n\n");

        let patterns = vec![
            // Remove includes
            (Regex::new(r"#include\s*[^\n]+\n?").unwrap(), ""),
            
            // Main function
            (Regex::new(r"\bint\s+main\s*\([^)]*\)\s*\{").unwrap(), "fun main() {"),
            
            // Return statements in main
            (Regex::new(r"\s*return\s+0;\s*").unwrap(), ""),
            
            // Printf statements
            (Regex::new(r#"printf\s*\(\s*"([^"]+)\\n"\s*\)"#).unwrap(), r#"println("$1")"#),
            (Regex::new(r#"printf\s*\(\s*"([^"]+)"\s*\)"#).unwrap(), r#"print("$1")"#),
            
            // Variable declarations (simple cases)
            (Regex::new(r"\b(int|char|float|double)\s+(\w+)\s*=").unwrap(), "let $2 ="),
        ];

        let mut translated = source.to_string();
        for (pattern, replacement) in patterns {
            translated = pattern.replace_all(&translated, replacement).to_string();
        }

        result.push_str(&translated);
        result.push_str("\n\nmain()\n");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_to_ruchy_translation() {
        let translator = CodeTranslator::new();
        let rust_code = r#"
fn main() {
    let x: i32 = 42;
    println!("Hello, world!");
}
"#;

        let result = translator.translate_to_ruchy(rust_code, "rust").unwrap();
        assert!(result.contains("fun main()"));
        assert!(result.contains("let x ="));
        assert!(result.contains("println("));
        assert!(result.contains("main()"));
    }

    #[test]
    fn test_python_to_ruchy_translation() {
        let translator = CodeTranslator::new();
        let python_code = r#"
def main():
    x = 42
    print("Hello, world!")

if __name__ == "__main__":
    main()
"#;

        let result = translator.translate_to_ruchy(python_code, "python").unwrap();
        assert!(result.contains("fun main()"));
        assert!(result.contains("let x = 42"));
        assert!(result.contains("println("));
    }

    #[test]
    fn test_javascript_to_ruchy_translation() {
        let translator = CodeTranslator::new();
        let js_code = r#"
function main() {
    const x = 42;
    console.log("Hello, world!");
}
"#;

        let result = translator.translate_to_ruchy(js_code, "javascript").unwrap();
        assert!(result.contains("fun main()"));
        assert!(result.contains("let x ="));
        assert!(result.contains("println("));
    }

    #[test]
    fn test_unsupported_language() {
        let translator = CodeTranslator::new();
        let result = translator.translate_to_ruchy("some code", "unsupported");
        assert!(result.is_err());
    }

    #[test]
    fn test_ruchy_passthrough() {
        let translator = CodeTranslator::new();
        let ruchy_code = "fun main() { println(\"Hello\"); }";
        let result = translator.translate_to_ruchy(ruchy_code, "ruchy").unwrap();
        assert_eq!(result, ruchy_code);
    }
}