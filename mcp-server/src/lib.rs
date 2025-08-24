//! Rosetta Ruchy MCP Server Library
//!
//! Provides real-time code translation and analysis services

#![allow(dead_code)]

pub mod analyzer;
pub mod language_detector;
pub mod mcp_server;
pub mod pmcp_integration;
pub mod ruchy_tooling;
pub mod translator;

pub use analyzer::CodeAnalyzer;
pub use language_detector::LanguageDetector;
pub use mcp_server::{MCPServer, TranslationRequest, TranslationOptions, AnalysisRequest, AnalysisType};
pub use translator::CodeTranslator;
pub use ruchy_tooling::RuchyToolchain;