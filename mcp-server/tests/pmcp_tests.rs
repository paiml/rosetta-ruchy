//! PMCP Integration Tests
//!
//! Tests the PMCP (Protocol for MCP) integration module for interactive
//! translation capabilities.
//!
//! Coverage target: mcp-server/src/pmcp_integration.rs (0/167 lines → ~140/167)

use rosetta_ruchy_mcp::pmcp_integration::*;

/// Test: Create new interactive translation session
#[test]
fn test_create_interactive_session() {
    let session = InteractiveTranslationSession {
        id: "test-session-1".to_string(),
        source_code: "def hello(): print('Hello')".to_string(),
        source_language: "python".to_string(),
        current_step: 0,
        total_steps: 5,
        partial_ruchy_code: String::new(),
        step_explanations: vec![],
        user_feedback: vec![],
        verification_results: vec![],
    };

    assert_eq!(session.id, "test-session-1");
    assert_eq!(session.source_language, "python");
    assert_eq!(session.current_step, 0);
    assert_eq!(session.total_steps, 5);
}

/// Test: User feedback creation
#[test]
fn test_user_feedback_approval() {
    let feedback = UserFeedback {
        step: 1,
        feedback_type: FeedbackType::Approval,
        content: "Looks good!".to_string(),
        timestamp: "2025-10-14T12:00:00Z".to_string(),
    };

    assert_eq!(feedback.step, 1);
    assert!(matches!(feedback.feedback_type, FeedbackType::Approval));
    assert_eq!(feedback.content, "Looks good!");
}

/// Test: User feedback suggestion
#[test]
fn test_user_feedback_suggestion() {
    let feedback = UserFeedback {
        step: 2,
        feedback_type: FeedbackType::Suggestion,
        content: "Consider using immutable variable".to_string(),
        timestamp: "2025-10-14T12:01:00Z".to_string(),
    };

    assert_eq!(feedback.step, 2);
    assert!(matches!(feedback.feedback_type, FeedbackType::Suggestion));
}

/// Test: User feedback question
#[test]
fn test_user_feedback_question() {
    let feedback = UserFeedback {
        step: 3,
        feedback_type: FeedbackType::Question,
        content: "Why this approach?".to_string(),
        timestamp: "2025-10-14T12:02:00Z".to_string(),
    };

    assert_eq!(feedback.step, 3);
    assert!(matches!(feedback.feedback_type, FeedbackType::Question));
}

/// Test: User feedback rejection
#[test]
fn test_user_feedback_rejection() {
    let feedback = UserFeedback {
        step: 4,
        feedback_type: FeedbackType::Rejection,
        content: "This won't work for my use case".to_string(),
        timestamp: "2025-10-14T12:03:00Z".to_string(),
    };

    assert_eq!(feedback.step, 4);
    assert!(matches!(feedback.feedback_type, FeedbackType::Rejection));
}

/// Test: Step verification result - syntax check
#[test]
fn test_verification_syntax_check_passed() {
    let verification = StepVerificationResult {
        step: 1,
        verification_type: VerificationType::SyntaxCheck,
        passed: true,
        details: "Syntax is valid".to_string(),
        suggestions: vec![],
    };

    assert_eq!(verification.step, 1);
    assert!(matches!(verification.verification_type, VerificationType::SyntaxCheck));
    assert!(verification.passed);
}

/// Test: Step verification result - type check
#[test]
fn test_verification_type_check_failed() {
    let verification = StepVerificationResult {
        step: 2,
        verification_type: VerificationType::TypeCheck,
        passed: false,
        details: "Type mismatch found".to_string(),
        suggestions: vec!["Add type annotation".to_string()],
    };

    assert_eq!(verification.step, 2);
    assert!(matches!(verification.verification_type, VerificationType::TypeCheck));
    assert!(!verification.passed);
    assert_eq!(verification.suggestions.len(), 1);
}

/// Test: Step verification result - provability check
#[test]
fn test_verification_provability_check() {
    let verification = StepVerificationResult {
        step: 3,
        verification_type: VerificationType::ProvabilityCheck,
        passed: true,
        details: "Function is provably correct".to_string(),
        suggestions: vec![],
    };

    assert!(matches!(verification.verification_type, VerificationType::ProvabilityCheck));
    assert!(verification.passed);
}

/// Test: Step verification result - performance check
#[test]
fn test_verification_performance_check() {
    let verification = StepVerificationResult {
        step: 4,
        verification_type: VerificationType::PerformanceCheck,
        passed: true,
        details: "O(n) complexity detected".to_string(),
        suggestions: vec!["Could optimize to O(log n)".to_string()],
    };

    assert!(matches!(verification.verification_type, VerificationType::PerformanceCheck));
    assert_eq!(verification.suggestions.len(), 1);
}

/// Test: Step verification result - quality check
#[test]
fn test_verification_quality_check() {
    let verification = StepVerificationResult {
        step: 5,
        verification_type: VerificationType::QualityCheck,
        passed: true,
        details: "Quality score: 0.95".to_string(),
        suggestions: vec![],
    };

    assert!(matches!(verification.verification_type, VerificationType::QualityCheck));
    assert!(verification.passed);
}

/// Test: PMCP translation request - function level
#[test]
fn test_pmcp_request_function_level() {
    let request = PMCPTranslationRequest {
        source_code: "def add(a, b): return a + b".to_string(),
        source_language: Some("python".to_string()),
        interactive: true,
        step_size: StepSize::Function,
        verification_level: VerificationLevel::Standard,
    };

    assert_eq!(request.source_language, Some("python".to_string()));
    assert!(request.interactive);
    assert!(matches!(request.step_size, StepSize::Function));
}

/// Test: PMCP translation request - statement level
#[test]
fn test_pmcp_request_statement_level() {
    let request = PMCPTranslationRequest {
        source_code: "x = 10\ny = 20\nz = x + y".to_string(),
        source_language: None, // Auto-detect
        interactive: true,
        step_size: StepSize::Statement,
        verification_level: VerificationLevel::Comprehensive,
    };

    assert_eq!(request.source_language, None);
    assert!(matches!(request.step_size, StepSize::Statement));
    assert!(matches!(request.verification_level, VerificationLevel::Comprehensive));
}

/// Test: PMCP translation request - expression level
#[test]
fn test_pmcp_request_expression_level() {
    let request = PMCPTranslationRequest {
        source_code: "result = (a + b) * (c - d)".to_string(),
        source_language: Some("python".to_string()),
        interactive: true,
        step_size: StepSize::Expression,
        verification_level: VerificationLevel::Basic,
    };

    assert!(matches!(request.step_size, StepSize::Expression));
    assert!(matches!(request.verification_level, VerificationLevel::Basic));
}

/// Test: Interactive session with feedback and verification
#[test]
fn test_session_with_feedback_and_verification() {
    let mut session = InteractiveTranslationSession {
        id: "session-2".to_string(),
        source_code: "def factorial(n): return 1 if n == 0 else n * factorial(n-1)".to_string(),
        source_language: "python".to_string(),
        current_step: 0,
        total_steps: 3,
        partial_ruchy_code: String::new(),
        step_explanations: vec![],
        user_feedback: vec![],
        verification_results: vec![],
    };

    // Add user feedback
    session.user_feedback.push(UserFeedback {
        step: 1,
        feedback_type: FeedbackType::Approval,
        content: "Good start".to_string(),
        timestamp: "2025-10-14T12:00:00Z".to_string(),
    });

    // Add verification result
    session.verification_results.push(StepVerificationResult {
        step: 1,
        verification_type: VerificationType::SyntaxCheck,
        passed: true,
        details: "Syntax valid".to_string(),
        suggestions: vec![],
    });

    assert_eq!(session.user_feedback.len(), 1);
    assert_eq!(session.verification_results.len(), 1);
}

/// Test: Session serialization/deserialization
#[test]
fn test_session_serialization() {
    let session = InteractiveTranslationSession {
        id: "session-3".to_string(),
        source_code: "print('test')".to_string(),
        source_language: "python".to_string(),
        current_step: 1,
        total_steps: 2,
        partial_ruchy_code: "println!(\"test\");".to_string(),
        step_explanations: vec!["Converting print to println".to_string()],
        user_feedback: vec![],
        verification_results: vec![],
    };

    // Serialize to JSON
    let json = serde_json::to_string(&session).unwrap();
    assert!(json.contains("session-3"));
    assert!(json.contains("python"));

    // Deserialize back
    let deserialized: InteractiveTranslationSession = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, session.id);
    assert_eq!(deserialized.current_step, session.current_step);
}

/// Test: Feedback type serialization
#[test]
fn test_feedback_type_serialization() {
    let feedback = UserFeedback {
        step: 1,
        feedback_type: FeedbackType::Suggestion,
        content: "Test".to_string(),
        timestamp: "2025-10-14T12:00:00Z".to_string(),
    };

    let json = serde_json::to_string(&feedback).unwrap();
    assert!(json.contains("suggestion"));

    let deserialized: UserFeedback = serde_json::from_str(&json).unwrap();
    assert!(matches!(deserialized.feedback_type, FeedbackType::Suggestion));
}

/// Test: Verification type serialization
#[test]
fn test_verification_type_serialization() {
    let verification = StepVerificationResult {
        step: 1,
        verification_type: VerificationType::ProvabilityCheck,
        passed: true,
        details: "Test".to_string(),
        suggestions: vec![],
    };

    let json = serde_json::to_string(&verification).unwrap();
    assert!(json.contains("provability_check"));

    let deserialized: StepVerificationResult = serde_json::from_str(&json).unwrap();
    assert!(matches!(deserialized.verification_type, VerificationType::ProvabilityCheck));
}
