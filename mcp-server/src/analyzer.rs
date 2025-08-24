//! Code analysis service for complexity and performance analysis

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::mcp_server::PerformancePrediction;

#[derive(Debug)]
pub struct CodeAnalyzer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    pub cyclomatic: u32,
    pub cognitive: u32,
    pub loc: u32,
    pub big_o_estimate: String,
    pub hotspots: Vec<String>,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_complexity(&self, code: &str, language: &str) -> Result<ComplexityAnalysis> {
        let loc = self.count_lines_of_code(code);
        let cyclomatic = self.calculate_cyclomatic_complexity(code, language)?;
        let cognitive = self.calculate_cognitive_complexity(code, language)?;
        let big_o_estimate = self.estimate_big_o_complexity(code, language)?;
        let hotspots = self.identify_complexity_hotspots(code, language)?;

        Ok(ComplexityAnalysis {
            cyclomatic,
            cognitive,
            loc,
            big_o_estimate,
            hotspots,
        })
    }

    pub fn predict_performance(
        &self,
        original_code: &str,
        ruchy_code: &str,
        source_language: &str,
    ) -> Result<PerformancePrediction> {
        // Performance prediction based on language characteristics and code patterns
        let speedup_multiplier = self.get_language_speedup_factor(source_language)?;
        let memory_efficiency = self.estimate_memory_efficiency(original_code, ruchy_code)?;
        let binary_size = self.estimate_binary_size(ruchy_code)?;
        let compilation_time = self.estimate_compilation_time(ruchy_code)?;

        Ok(PerformancePrediction {
            estimated_speedup: speedup_multiplier,
            memory_usage_change: memory_efficiency,
            binary_size_estimate: binary_size,
            compilation_time_estimate: compilation_time,
        })
    }

    fn count_lines_of_code(&self, code: &str) -> u32 {
        code.lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("//"))
            .count() as u32
    }

    fn calculate_cyclomatic_complexity(&self, code: &str, language: &str) -> Result<u32> {
        let complexity_patterns = match language {
            "rust" | "ruchy" => vec![
                r"\bif\b",
                r"\bmatch\b",
                r"\bwhile\b",
                r"\bfor\b",
                r"\bloop\b",
                r"\|\|",
                r"&&",
                r"\?",
                r"\.unwrap_or",
                r"\.map\(",
                r"\.filter\(",
            ],
            "python" => vec![
                r"\bif\b",
                r"\belif\b",
                r"\bwhile\b",
                r"\bfor\b",
                r"\btry\b",
                r"\bexcept\b",
                r"\band\b",
                r"\bor\b",
            ],
            "javascript" => vec![
                r"\bif\b",
                r"\bwhile\b",
                r"\bfor\b",
                r"\bswitch\b",
                r"\bcatch\b",
                r"\|\|",
                r"&&",
                r"\?",
                r"\.map\(",
                r"\.filter\(",
            ],
            "go" => vec![
                r"\bif\b",
                r"\bfor\b",
                r"\bswitch\b",
                r"\bselect\b",
                r"\|\|",
                r"&&",
            ],
            "c" => vec![
                r"\bif\b",
                r"\bwhile\b",
                r"\bfor\b",
                r"\bswitch\b",
                r"\|\|",
                r"&&",
                r"\?",
            ],
            _ => return Err(anyhow!("Unsupported language for complexity analysis: {}", language)),
        };

        let mut complexity = 1; // Base complexity
        
        for pattern in complexity_patterns {
            let re = Regex::new(pattern)?;
            complexity += re.find_iter(code).count() as u32;
        }

        Ok(complexity)
    }

    fn calculate_cognitive_complexity(&self, code: &str, language: &str) -> Result<u32> {
        // Simplified cognitive complexity calculation
        // In reality, this would need proper AST parsing
        let nested_patterns = match language {
            "rust" | "ruchy" => vec![
                (r"\bif\b.*\{[^}]*\bif\b", 2),
                (r"\bmatch\b.*\{[^}]*\bmatch\b", 2),
                (r"\bfor\b.*\{[^}]*\bfor\b", 2),
                (r"\bwhile\b.*\{[^}]*\bwhile\b", 2),
            ],
            "python" => vec![
                (r"\bif\b.*:\s*\n\s+.*\bif\b", 2),
                (r"\bfor\b.*:\s*\n\s+.*\bfor\b", 2),
                (r"\bwhile\b.*:\s*\n\s+.*\bwhile\b", 2),
            ],
            _ => vec![],
        };

        let mut cognitive_score = 0;

        // Base complexity from control structures
        let base_complexity = self.calculate_cyclomatic_complexity(code, language)?;
        cognitive_score += base_complexity;

        // Additional penalty for nesting
        for (pattern, penalty) in nested_patterns {
            let re = Regex::new(pattern)?;
            cognitive_score += re.find_iter(code).count() as u32 * penalty;
        }

        Ok(cognitive_score)
    }

    fn estimate_big_o_complexity(&self, code: &str, _language: &str) -> Result<String> {
        // Very simplified Big O estimation based on loop patterns
        let nested_loops = Regex::new(r"for.*\{[^}]*for.*\{")?;
        let triple_nested = Regex::new(r"for.*\{[^}]*for.*\{[^}]*for.*\{")?;
        let single_loops = Regex::new(r"\bfor\b|\bwhile\b")?;
        // Note: Rust regex doesn't support backreferences, so we check for recursion differently
        let function_call = Regex::new(r"fn\s+(\w+)")?;

        if triple_nested.is_match(code) {
            Ok("O(n³)".to_string())
        } else if nested_loops.is_match(code) {
            Ok("O(n²)".to_string())
        } else if function_call.is_match(code) && code.contains("recursive") {
            // Very basic recursive detection - would need proper analysis
            if code.contains("fibonacci") || code.contains("fib") {
                Ok("O(2^n)".to_string())
            } else {
                Ok("O(log n)".to_string())
            }
        } else if single_loops.find_iter(code).count() > 0 {
            Ok("O(n)".to_string())
        } else {
            Ok("O(1)".to_string())
        }
    }

    fn identify_complexity_hotspots(&self, code: &str, language: &str) -> Result<Vec<String>> {
        let mut hotspots = Vec::new();

        // Look for deeply nested structures
        if language == "rust" || language == "ruchy" {
            if code.contains("match") {
                let match_count = Regex::new(r"\bmatch\b")?.find_iter(code).count();
                if match_count > 2 {
                    hotspots.push("Multiple match statements may increase complexity".to_string());
                }
            }

            if code.contains("unwrap()") {
                hotspots.push("Consider using proper error handling instead of unwrap()".to_string());
            }
        }

        // Long functions
        let lines = code.lines().count();
        if lines > 50 {
            hotspots.push(format!("Function is {} lines long, consider breaking it down", lines));
        }

        // Deep nesting detection (simplified)
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();
        if open_braces > 5 && open_braces == close_braces {
            hotspots.push("Deep nesting detected, consider refactoring".to_string());
        }

        Ok(hotspots)
    }

    fn get_language_speedup_factor(&self, source_language: &str) -> Result<f64> {
        // Estimated speedup factors when translating to Ruchy
        // Based on typical performance characteristics
        match source_language {
            "python" => Ok(15.0), // Python is typically 10-20x slower than compiled languages
            "javascript" => Ok(3.0), // V8 is quite optimized, but compiled code is still faster
            "rust" => Ok(0.98), // Ruchy aims for 95-105% of Rust performance
            "go" => Ok(1.2), // Go is slightly slower than Rust due to GC
            "c" => Ok(0.95), // C can be slightly faster due to manual optimizations
            "ruchy" => Ok(1.0), // No change
            _ => Err(anyhow!("Unknown language for speedup estimation: {}", source_language)),
        }
    }

    fn estimate_memory_efficiency(&self, _original_code: &str, ruchy_code: &str) -> Result<f64> {
        // Estimate memory usage change (negative = less memory, positive = more)
        // This is a simplified estimation based on code patterns
        
        let has_vectors = ruchy_code.contains("Vec<") || ruchy_code.contains("vec!");
        let has_strings = ruchy_code.contains("String") || ruchy_code.contains("&str");
        let has_heap_allocations = ruchy_code.contains("Box<") || ruchy_code.contains("Rc<");

        let base_efficiency = 0.0; // Neutral starting point

        let mut efficiency_change = base_efficiency;

        if has_vectors {
            efficiency_change += 0.1; // Vectors may use more memory than needed
        }

        if has_strings {
            efficiency_change += 0.05; // String handling overhead
        }

        if has_heap_allocations {
            efficiency_change += 0.15; // Heap allocations have overhead
        }

        // Ruchy's zero-cost abstractions should minimize overhead
        efficiency_change *= 0.8; // 20% efficiency improvement from Ruchy optimizations

        Ok(efficiency_change)
    }

    fn estimate_binary_size(&self, ruchy_code: &str) -> Result<u64> {
        // Estimate binary size in KB based on code characteristics
        let base_size = 50; // Base Ruchy runtime size in KB
        let loc = self.count_lines_of_code(ruchy_code) as u64;
        
        // Rough estimation: ~0.5KB per line of code for simple programs
        let estimated_size = base_size + (loc * 500) / 1000; // Convert bytes to KB
        
        Ok(estimated_size)
    }

    fn estimate_compilation_time(&self, ruchy_code: &str) -> Result<f64> {
        // Estimate compilation time in seconds
        let loc = self.count_lines_of_code(ruchy_code) as f64;
        let base_time = 0.1; // Base compilation overhead
        
        // Ruchy aims for fast compilation: ~1000 lines per second
        let compile_time = base_time + (loc / 1000.0);
        
        Ok(compile_time)
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_of_code_count() {
        let analyzer = CodeAnalyzer::new();
        let code = r#"
            // This is a comment
            fn main() {
                let x = 42;
                println!("Hello");
            }
            
            // Another comment
        "#;

        let loc = analyzer.count_lines_of_code(code);
        assert_eq!(loc, 4); // Only non-comment, non-empty lines
    }

    #[test]
    fn test_cyclomatic_complexity_rust() {
        let analyzer = CodeAnalyzer::new();
        let code = r#"
            fn test() {
                if x > 0 {
                    if y > 0 {
                        return true;
                    }
                }
                for i in 0..10 {
                    match i {
                        0 => println!("zero"),
                        _ => println!("other"),
                    }
                }
            }
        "#;

        let complexity = analyzer.calculate_cyclomatic_complexity(code, "rust").unwrap();
        assert!(complexity > 1); // Should detect multiple decision points
    }

    #[test]
    fn test_big_o_estimation() {
        let analyzer = CodeAnalyzer::new();
        
        let linear_code = "for i in 0..n { println!(\"{}\", i); }";
        let big_o = analyzer.estimate_big_o_complexity(linear_code, "rust").unwrap();
        assert_eq!(big_o, "O(n)");

        let quadratic_code = "for i in 0..n { for j in 0..n { println!(\"{} {}\", i, j); } }";
        let big_o = analyzer.estimate_big_o_complexity(quadratic_code, "rust").unwrap();
        assert_eq!(big_o, "O(n²)");

        let constant_code = "let x = 42; println!(\"{}\", x);";
        let big_o = analyzer.estimate_big_o_complexity(constant_code, "rust").unwrap();
        assert_eq!(big_o, "O(1)");
    }

    #[test]
    fn test_performance_prediction() {
        let analyzer = CodeAnalyzer::new();
        let python_code = "print('hello')";
        let ruchy_code = "println(\"hello\");";
        
        let prediction = analyzer.predict_performance(python_code, ruchy_code, "python").unwrap();
        assert!(prediction.estimated_speedup > 1.0); // Should be faster than Python
        assert!(prediction.binary_size_estimate > 0);
        assert!(prediction.compilation_time_estimate > 0.0);
    }
}