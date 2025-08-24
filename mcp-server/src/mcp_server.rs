//! MCP Server implementation for Rosetta Ruchy translation service

use anyhow::Result;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    analyzer::CodeAnalyzer, language_detector::LanguageDetector, ruchy_tooling::RuchyToolchain,
    translator::CodeTranslator,
};

#[derive(Clone)]
pub struct MCPServer {
    host: String,
    port: u16,
    state: Arc<ServerState>,
}

struct ServerState {
    translator: CodeTranslator,
    analyzer: CodeAnalyzer,
    ruchy_toolchain: RuchyToolchain,
    language_detector: LanguageDetector,
}

// MCP Protocol Types
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub source_code: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>, // Always "ruchy" for now
    pub options: Option<TranslationOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationOptions {
    pub optimize: bool,
    pub verify: bool,
    pub include_analysis: bool,
    pub complexity_check: bool,
}

impl Default for TranslationOptions {
    fn default() -> Self {
        Self {
            optimize: true,
            verify: true,
            include_analysis: true,
            complexity_check: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub id: String,
    pub ruchy_code: String,
    pub source_language: String,
    pub ast_analysis: Option<serde_json::Value>,
    pub provability_score: Option<f64>,
    pub quality_score: Option<f64>,
    pub performance_prediction: Option<PerformancePrediction>,
    pub verification_status: Option<VerificationStatus>,
    pub optimization_suggestions: Vec<String>,
    pub complexity_metrics: Option<ComplexityMetrics>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub estimated_speedup: f64,
    pub memory_usage_change: f64,
    pub binary_size_estimate: u64,
    pub compilation_time_estimate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationStatus {
    pub verified: bool,
    pub proof_score: f64,
    pub safety_guarantees: Vec<String>,
    pub potential_issues: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub lines_of_code: u32,
    pub estimated_big_o: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub code: String,
    pub language: Option<String>,
    pub analysis_type: AnalysisType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisType {
    Complexity,
    Performance,
    Verification,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    pub name: String,
    pub version: String,
    pub supported_languages: Vec<String>,
    pub capabilities: Vec<String>,
    pub endpoints: HashMap<String, String>,
}

impl MCPServer {
    pub fn new(host: String, port: u16, ruchy_path: String) -> Self {
        let state = Arc::new(ServerState {
            translator: CodeTranslator::new(),
            analyzer: CodeAnalyzer::new(),
            ruchy_toolchain: RuchyToolchain::new(ruchy_path),
            language_detector: LanguageDetector::new(),
        });

        Self { host, port, state }
    }

    pub async fn start(self) -> Result<()> {
        let host = self.host.clone();
        let port = self.port;
        let app = self.create_router();

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;

        info!("MCP Server listening on {}:{}", host, port);
        info!(
            "Capabilities endpoint: http://{}:{}/api/v1/capabilities",
            host, port
        );

        axum::serve(listener, app).await?;

        Ok(())
    }

    pub fn create_router(self) -> Router {
        Router::new()
            .route("/", get(root_handler))
            .route("/health", get(health_handler))
            .route("/api/v1/capabilities", get(capabilities_handler))
            .route("/api/v1/translate", post(translate_handler))
            .route("/api/v1/analyze", post(analyze_handler))
            .route("/api/v1/benchmark", post(benchmark_handler))
            .route("/api/v1/verify", post(verify_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .with_state(self.state)
    }
}

// Handlers
async fn root_handler() -> &'static str {
    "Rosetta Ruchy MCP Server v1.0.0 - Code Translation to Ruchy"
}

async fn health_handler() -> ResponseJson<serde_json::Value> {
    ResponseJson(serde_json::json!({
        "status": "healthy",
        "service": "rosetta-ruchy-mcp",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn capabilities_handler() -> ResponseJson<CapabilitiesResponse> {
    let mut endpoints = HashMap::new();
    endpoints.insert("translate".to_string(), "/api/v1/translate".to_string());
    endpoints.insert("analyze".to_string(), "/api/v1/analyze".to_string());
    endpoints.insert("benchmark".to_string(), "/api/v1/benchmark".to_string());
    endpoints.insert("verify".to_string(), "/api/v1/verify".to_string());

    ResponseJson(CapabilitiesResponse {
        name: "rosetta-ruchy-translator".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        supported_languages: vec![
            "rust".to_string(),
            "python".to_string(),
            "javascript".to_string(),
            "go".to_string(),
            "c".to_string(),
        ],
        capabilities: vec![
            "code_translation".to_string(),
            "performance_analysis".to_string(),
            "formal_verification".to_string(),
            "quality_assessment".to_string(),
            "complexity_analysis".to_string(),
            "benchmark_comparison".to_string(),
        ],
        endpoints,
    })
}

async fn translate_handler(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<TranslationRequest>,
) -> Result<ResponseJson<TranslationResponse>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    // Detect source language if not provided
    let source_language = match request.source_language {
        Some(lang) => lang,
        None => match state.language_detector.detect(&request.source_code) {
            Ok(lang) => lang,
            Err(e) => {
                warn!("Failed to detect language: {}", e);
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Could not detect source language: {}", e),
                ));
            }
        },
    };

    info!(
        "Translating {} code to Ruchy (request: {})",
        source_language, id
    );

    // Translate to Ruchy
    let ruchy_code = match state
        .translator
        .translate_to_ruchy(&request.source_code, &source_language)
    {
        Ok(code) => code,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Translation failed: {}", e),
            ));
        }
    };

    let options = request.options.unwrap_or_default();
    let mut response = TranslationResponse {
        id,
        ruchy_code: ruchy_code.clone(),
        source_language: source_language.clone(),
        ast_analysis: None,
        provability_score: None,
        quality_score: None,
        performance_prediction: None,
        verification_status: None,
        optimization_suggestions: Vec::new(),
        complexity_metrics: None,
    };

    // Run Ruchy advanced tooling if requested
    if options.include_analysis {
        if let Ok(analysis) = state.ruchy_toolchain.analyze_ast(&ruchy_code).await {
            response.ast_analysis = Some(analysis);
        }
    }

    if options.verify {
        if let Ok(provability) = state.ruchy_toolchain.check_provability(&ruchy_code).await {
            response.provability_score = Some(provability.score);
            response.verification_status = Some(VerificationStatus {
                verified: provability.verified,
                proof_score: provability.score,
                safety_guarantees: provability.safety_guarantees,
                potential_issues: provability.potential_issues,
            });
        }

        if let Ok(quality) = state.ruchy_toolchain.get_quality_score(&ruchy_code).await {
            response.quality_score = Some(quality);
        }
    }

    if options.optimize {
        if let Ok(suggestions) = state
            .ruchy_toolchain
            .get_optimization_suggestions(&ruchy_code)
            .await
        {
            response.optimization_suggestions = suggestions;
        }
    }

    if options.complexity_check {
        if let Ok(metrics) = state.analyzer.analyze_complexity(&ruchy_code, "ruchy") {
            response.complexity_metrics = Some(ComplexityMetrics {
                cyclomatic_complexity: metrics.cyclomatic,
                cognitive_complexity: metrics.cognitive,
                lines_of_code: metrics.loc,
                estimated_big_o: metrics.big_o_estimate,
            });
        }
    }

    // Generate performance prediction
    if let Ok(prediction) =
        state
            .analyzer
            .predict_performance(&request.source_code, &ruchy_code, &source_language)
    {
        response.performance_prediction = Some(prediction);
    }

    Ok(ResponseJson(response))
}

async fn analyze_handler(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<AnalysisRequest>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, String)> {
    let language = match request.language {
        Some(lang) => lang,
        None => match state.language_detector.detect(&request.code) {
            Ok(lang) => lang,
            Err(e) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Could not detect language: {}", e),
                ))
            }
        },
    };

    match request.analysis_type {
        AnalysisType::Complexity => {
            match state.analyzer.analyze_complexity(&request.code, &language) {
                Ok(metrics) => Ok(ResponseJson(serde_json::to_value(metrics).unwrap())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        AnalysisType::Performance => {
            // Performance analysis placeholder
            Ok(ResponseJson(serde_json::json!({
                "analysis_type": "performance",
                "language": language,
                "status": "not_implemented"
            })))
        }
        AnalysisType::Verification => {
            // Only available for Ruchy code
            if language == "ruchy" {
                match state.ruchy_toolchain.check_provability(&request.code).await {
                    Ok(provability) => Ok(ResponseJson(serde_json::to_value(provability).unwrap())),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    "Verification only available for Ruchy code".to_string(),
                ))
            }
        }
        AnalysisType::All => Ok(ResponseJson(serde_json::json!({
            "analysis_type": "all",
            "language": language,
            "status": "not_implemented"
        }))),
    }
}

async fn benchmark_handler() -> ResponseJson<serde_json::Value> {
    ResponseJson(serde_json::json!({
        "status": "not_implemented",
        "message": "Benchmark comparison endpoint will be implemented in Phase 2"
    }))
}

async fn verify_handler(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<serde_json::Value>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, String)> {
    let code = request
        .get("code")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'code' field".to_string()))?;

    match state.ruchy_toolchain.check_provability(code).await {
        Ok(provability) => Ok(ResponseJson(serde_json::to_value(provability).unwrap())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
