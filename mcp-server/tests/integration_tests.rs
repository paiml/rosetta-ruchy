//! Integration tests for the Rosetta Ruchy MCP Server

use anyhow::Result;
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use rosetta_ruchy_mcp::{
    analyzer::CodeAnalyzer,
    language_detector::LanguageDetector,
    translator::CodeTranslator,
    mcp_server::{TranslationRequest, TranslationOptions, AnalysisRequest, AnalysisType},
};
use serde_json::json;
use tower::ServiceExt;

// Mock Ruchy toolchain for testing
#[allow(dead_code)]
struct MockRuchyToolchain;

#[tokio::test]
async fn test_health_endpoint() -> Result<()> {
    let app = create_test_app().await;

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;
    
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["service"], "rosetta-ruchy-mcp");

    Ok(())
}

#[tokio::test]
async fn test_capabilities_endpoint() -> Result<()> {
    let app = create_test_app().await;

    let response = app
        .oneshot(Request::builder().uri("/api/v1/capabilities").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert_eq!(json["name"], "rosetta-ruchy-translator");
    assert!(!json["supported_languages"].as_array().unwrap().is_empty());
    assert!(json["capabilities"].as_array().unwrap().contains(&json!("code_translation")));

    Ok(())
}

#[tokio::test]
async fn test_translation_endpoint_rust_to_ruchy() -> Result<()> {
    let app = create_test_app().await;

    let request_body = TranslationRequest {
        source_code: r#"
            fn main() {
                let x: i32 = 42;
                println!("Hello, world!");
            }
        "#.to_string(),
        source_language: Some("rust".to_string()),
        target_language: Some("ruchy".to_string()),
        options: Some(TranslationOptions {
            optimize: true,
            verify: true,
            include_analysis: true,
            complexity_check: true,
        }),
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/translate")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert!(json["ruchy_code"].as_str().unwrap().contains("fun main"));
    assert_eq!(json["source_language"], "rust");
    assert!(json["quality_score"].as_f64().unwrap() > 0.0);

    Ok(())
}

#[tokio::test]
async fn test_translation_endpoint_python_to_ruchy() -> Result<()> {
    let app = create_test_app().await;

    let request_body = TranslationRequest {
        source_code: r#"
def hello_world():
    print("Hello, world!")
    
if __name__ == "__main__":
    hello_world()
        "#.to_string(),
        source_language: Some("python".to_string()),
        target_language: Some("ruchy".to_string()),
        options: None, // Test default options
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/translate")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert!(json["ruchy_code"].as_str().unwrap().contains("fun hello_world"));
    assert_eq!(json["source_language"], "python");

    Ok(())
}

#[tokio::test]
async fn test_translation_endpoint_language_detection() -> Result<()> {
    let app = create_test_app().await;

    let request_body = TranslationRequest {
        source_code: r#"
function main() {
    const x = 42;
    console.log("Hello, world!");
}
        "#.to_string(),
        source_language: None, // Test language detection
        target_language: Some("ruchy".to_string()),
        options: Some(TranslationOptions::default()),
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/translate")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert_eq!(json["source_language"], "javascript");
    assert!(json["ruchy_code"].as_str().unwrap().contains("fun main"));

    Ok(())
}

#[tokio::test]
async fn test_analysis_endpoint_complexity() -> Result<()> {
    let app = create_test_app().await;

    let request_body = AnalysisRequest {
        code: r#"
            fn complex_function(n: i32) -> i32 {
                if n <= 1 {
                    return n;
                }
                
                let mut result = 0;
                for i in 0..n {
                    if i % 2 == 0 {
                        result += i;
                    } else {
                        result -= i;
                    }
                }
                
                match result {
                    x if x > 100 => x / 2,
                    x if x < -100 => x * 2,
                    _ => result,
                }
            }
        "#.to_string(),
        language: Some("rust".to_string()),
        analysis_type: AnalysisType::Complexity,
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/analyze")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert!(json["cyclomatic"].as_u64().unwrap() > 1);
    assert!(json["loc"].as_u64().unwrap() > 0);

    Ok(())
}

#[tokio::test]
async fn test_verify_endpoint() -> Result<()> {
    let app = create_test_app().await;

    let request_body = json!({
        "code": r#"
            fun fibonacci(n: i32) -> i32 {
                if n <= 1 {
                    n
                } else {
                    fibonacci(n - 1) + fibonacci(n - 2)
                }
            }
        "#
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/verify")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert!(json["score"].as_f64().unwrap() >= 0.0);
    assert!(json["score"].as_f64().unwrap() <= 1.0);

    Ok(())
}

#[tokio::test]
async fn test_error_handling_invalid_json() -> Result<()> {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/translate")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{invalid json"))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn test_error_handling_unsupported_language() -> Result<()> {
    let app = create_test_app().await;

    let request_body = TranslationRequest {
        source_code: "some code".to_string(),
        source_language: Some("brainfuck".to_string()),
        target_language: Some("ruchy".to_string()),
        options: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/translate")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body)?))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    Ok(())
}

#[tokio::test]
async fn test_benchmark_endpoint_not_implemented() -> Result<()> {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/benchmark")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let json: serde_json::Value = serde_json::from_slice(&body)?;

    assert_eq!(json["status"], "not_implemented");

    Ok(())
}

// Helper function to create test app
async fn create_test_app() -> axum::Router {
    use rosetta_ruchy_mcp::mcp_server::MCPServer;
    
    let server = MCPServer::new(
        "127.0.0.1".to_string(),
        8080,
        "mock-ruchy".to_string(), // Use mock ruchy path for tests
    );

    // Create the router for testing
    server.create_router()
}

// Unit tests for individual components
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_language_detector() {
        let detector = LanguageDetector::new();

        let rust_code = "fn main() { println!(\"Hello\"); }";
        assert_eq!(detector.detect(rust_code).unwrap(), "rust");

        let python_code = "def main(): print(\"Hello\")";
        assert_eq!(detector.detect(python_code).unwrap(), "python");

        let js_code = "function main() { console.log(\"Hello\"); }";
        assert_eq!(detector.detect(js_code).unwrap(), "javascript");
    }

    #[test]
    fn test_code_translator() {
        let translator = CodeTranslator::new();

        let rust_code = "fn main() { println!(\"Hello\"); }";
        let ruchy_code = translator.translate_to_ruchy(rust_code, "rust").unwrap();
        
        assert!(ruchy_code.contains("fun main"));
        assert!(ruchy_code.contains("println("));

        // Test passthrough for Ruchy code
        let original_ruchy = "fun main() { println(\"Hello\"); }";
        let passthrough = translator.translate_to_ruchy(original_ruchy, "ruchy").unwrap();
        assert_eq!(passthrough, original_ruchy);
    }

    #[test]
    fn test_code_analyzer() {
        let analyzer = CodeAnalyzer::new();

        let simple_code = "fn main() { println!(\"Hello\"); }";
        let analysis = analyzer.analyze_complexity(simple_code, "rust").unwrap();
        
        assert_eq!(analysis.cyclomatic, 1); // Simple function
        assert!(analysis.loc > 0);
        assert_eq!(analysis.big_o_estimate, "O(1)");

        let complex_code = r#"
            fn complex(n: i32) -> i32 {
                for i in 0..n {
                    if i % 2 == 0 {
                        for j in 0..i {
                            println!("{}", j);
                        }
                    }
                }
            }
        "#;
        let complex_analysis = analyzer.analyze_complexity(complex_code, "rust").unwrap();
        assert!(complex_analysis.cyclomatic > analysis.cyclomatic);
        assert_eq!(complex_analysis.big_o_estimate, "O(n²)");
    }

    #[tokio::test]
    async fn test_ruchy_toolchain_mock() {
        use rosetta_ruchy_mcp::ruchy_tooling::RuchyToolchain;
        
        let toolchain = RuchyToolchain::new("mock-ruchy".to_string());
        let code = "fun main() { println(\"Hello\"); }";

        // Test AST analysis (should work with mock)
        let ast_result = toolchain.analyze_ast(code).await;
        assert!(ast_result.is_ok());

        // Test provability check (should work with mock fallback)
        let provability_result = toolchain.check_provability(code).await;
        assert!(provability_result.is_ok());

        let provability = provability_result.unwrap();
        assert!(provability.score >= 0.0);
        assert!(provability.score <= 1.0);
    }
}

// Performance tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_language_detection_performance() {
        let detector = LanguageDetector::new();
        let code = "fn main() { println!(\"Hello, world!\"); }".repeat(100);

        let start = Instant::now();
        for _ in 0..1000 {
            let _ = detector.detect(&code);
        }
        let duration = start.elapsed();

        // Should be able to detect language in less than 1ms on average
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_translation_performance() {
        let translator = CodeTranslator::new();
        let code = r#"
            fn fibonacci(n: i32) -> i32 {
                if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
            }
        "#;

        let start = Instant::now();
        for _ in 0..100 {
            let _ = translator.translate_to_ruchy(code, "rust");
        }
        let duration = start.elapsed();

        // Translation should be fast
        assert!(duration.as_millis() < 100);
    }
}