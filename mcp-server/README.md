# Rosetta Ruchy MCP Server

A high-performance Model Context Protocol (MCP) server that provides real-time code translation capabilities from various programming languages to Ruchy, with immediate formal verification and performance analysis.

[![Crates.io](https://img.shields.io/crates/v/rosetta-ruchy-mcp.svg)](https://crates.io/crates/rosetta-ruchy-mcp)
[![Documentation](https://docs.rs/rosetta-ruchy-mcp/badge.svg)](https://docs.rs/rosetta-ruchy-mcp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 **Real-time Code Translation**: Convert code from Rust, Python, JavaScript, Go, and C to Ruchy
- 🔍 **Advanced Analysis**: AST analysis, complexity metrics, and performance predictions
- 🛡️ **Formal Verification**: SMT solver integration for provability checking
- ⚡ **Quality Assessment**: Real-time code quality scoring and optimization suggestions
- 🌐 **HTTP API**: RESTful endpoints for easy integration
- 🔧 **PMCP Support**: Interactive step-by-step translation with user feedback
- 📊 **Performance Insights**: Benchmark predictions and memory usage analysis

## Quick Start

### Installation

#### Option 1: Install from crates.io
```bash
cargo install rosetta-ruchy-mcp
```

#### Option 2: Quick install script
```bash
curl -fsSL https://rosetta-ruchy.org/install.sh | sh
```

#### Option 3: Download pre-built binary
Visit the [releases page](https://github.com/rosetta-ruchy/rosetta-ruchy/releases) and download the appropriate binary for your platform.

### Starting the Server

```bash
# Start with default settings (localhost:8080)
rosetta-ruchy-mcp

# Custom host and port
rosetta-ruchy-mcp --host 0.0.0.0 --port 3000

# Specify custom Ruchy compiler path
rosetta-ruchy-mcp --ruchy-path /usr/local/bin/ruchy
```

### Verify Installation

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "rosetta-ruchy-mcp",
  "version": "1.0.0"
}
```

## API Reference

### Base URL
```
http://localhost:8080/api/v1
```

### Endpoints

#### 1. Get Server Capabilities
```http
GET /api/v1/capabilities
```

**Response:**
```json
{
  "name": "rosetta-ruchy-translator",
  "version": "1.0.0",
  "supported_languages": ["rust", "python", "javascript", "go", "c"],
  "capabilities": [
    "code_translation",
    "performance_analysis",
    "formal_verification",
    "quality_assessment"
  ],
  "endpoints": {
    "translate": "/api/v1/translate",
    "analyze": "/api/v1/analyze",
    "verify": "/api/v1/verify"
  }
}
```

#### 2. Translate Code to Ruchy
```http
POST /api/v1/translate
```

**Request Body:**
```json
{
  "source_code": "def fibonacci(n):\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2)",
  "source_language": "python",
  "options": {
    "optimize": true,
    "verify": true,
    "include_analysis": true,
    "complexity_check": true
  }
}
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "ruchy_code": "fun fibonacci(n: i32) -> i32 {\n    if n <= 1 {\n        n\n    } else {\n        fibonacci(n - 1) + fibonacci(n - 2)\n    }\n}\n\nmain()",
  "source_language": "python",
  "ast_analysis": {
    "functions": [{"name": "fibonacci", "complexity": 2}]
  },
  "provability_score": 0.95,
  "quality_score": 0.87,
  "performance_prediction": {
    "estimated_speedup": 15.0,
    "memory_usage_change": -0.1,
    "binary_size_estimate": 245,
    "compilation_time_estimate": 0.15
  },
  "verification_status": {
    "verified": true,
    "proof_score": 0.95,
    "safety_guarantees": [
      "Memory safety guaranteed",
      "No undefined behavior"
    ],
    "potential_issues": []
  },
  "optimization_suggestions": [
    "Consider using iterative approach for better performance"
  ],
  "complexity_metrics": {
    "cyclomatic_complexity": 2,
    "cognitive_complexity": 3,
    "lines_of_code": 7,
    "estimated_big_o": "O(2^n)"
  }
}
```

#### 3. Analyze Code Complexity
```http
POST /api/v1/analyze
```

**Request Body:**
```json
{
  "code": "fn complex_function(n: i32) -> i32 {\n    for i in 0..n {\n        if i % 2 == 0 {\n            println!(\"{}\", i);\n        }\n    }\n}",
  "language": "rust",
  "analysis_type": "complexity"
}
```

**Response:**
```json
{
  "cyclomatic_complexity": 3,
  "cognitive_complexity": 4,
  "lines_of_code": 7,
  "estimated_big_o": "O(n)",
  "hotspots": [
    "Consider extracting the conditional logic into a separate function"
  ]
}
```

#### 4. Verify Ruchy Code
```http
POST /api/v1/verify
```

**Request Body:**
```json
{
  "code": "fun safe_divide(a: f64, b: f64) -> Option<f64> {\n    if b != 0.0 {\n        Some(a / b)\n    } else {\n        None\n    }\n}"
}
```

**Response:**
```json
{
  "verified": true,
  "score": 1.0,
  "safety_guarantees": [
    "Memory safety guaranteed",
    "No undefined behavior",
    "Division by zero handled safely"
  ],
  "potential_issues": [],
  "proof_details": "100% pure functions, High Provability (100.0/100)"
}
```

## Usage Examples

### Python Script Translation

```python
import requests
import json

# Start the MCP server first: rosetta-ruchy-mcp

def translate_python_to_ruchy(python_code):
    url = "http://localhost:8080/api/v1/translate"
    payload = {
        "source_code": python_code,
        "source_language": "python",
        "options": {
            "optimize": True,
            "verify": True,
            "include_analysis": True
        }
    }
    
    response = requests.post(url, json=payload)
    
    if response.status_code == 200:
        result = response.json()
        print("✅ Translation successful!")
        print(f"📈 Quality Score: {result['quality_score']}")
        print(f"🚀 Estimated Speedup: {result['performance_prediction']['estimated_speedup']}x")
        print(f"\n🔧 Ruchy Code:\n{result['ruchy_code']}")
        return result
    else:
        print(f"❌ Translation failed: {response.text}")
        return None

# Example usage
python_code = '''
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)

if __name__ == "__main__":
    numbers = [3, 6, 8, 10, 1, 2, 1]
    sorted_numbers = quicksort(numbers)
    print(sorted_numbers)
'''

translate_python_to_ruchy(python_code)
```

### Rust Code Analysis

```bash
# Analyze Rust code complexity
curl -X POST http://localhost:8080/api/v1/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "code": "fn fibonacci(n: u32) -> u32 {\n    match n {\n        0 | 1 => n,\n        _ => fibonacci(n - 1) + fibonacci(n - 2),\n    }\n}",
    "language": "rust",
    "analysis_type": "complexity"
  }' | jq '.'
```

### JavaScript to Ruchy Translation

```bash
# Translate JavaScript function
curl -X POST http://localhost:8080/api/v1/translate \
  -H "Content-Type: application/json" \
  -d '{
    "source_code": "function isPrime(n) {\n    if (n <= 1) return false;\n    for (let i = 2; i * i <= n; i++) {\n        if (n % i === 0) return false;\n    }\n    return true;\n}",
    "source_language": "javascript"
  }' | jq '.ruchy_code'
```

## Language Support

| Language | Translation | Analysis | Verification |
|----------|-------------|----------|--------------|
| Rust     | ✅ Full     | ✅ Full  | ✅ Full      |
| Python   | ✅ Full     | ✅ Full  | ❌ N/A       |
| JavaScript | ✅ Full   | ✅ Full  | ❌ N/A       |
| Go       | ✅ Full     | ✅ Full  | ❌ N/A       |
| C        | ✅ Full     | ✅ Full  | ❌ N/A       |
| Ruchy    | ✅ Pass-through | ✅ Full | ✅ Full   |

> **Note**: Formal verification is currently only available for Ruchy code, as it requires the Ruchy compiler's advanced tooling suite.

## PMCP Interactive Translation

For step-by-step interactive translation with real-time feedback:

```bash
# Enable PMCP features
rosetta-ruchy-mcp --features pmcp

# Start interactive translation session
curl -X POST http://localhost:8080/api/v1/pmcp/start \
  -H "Content-Type: application/json" \
  -d '{
    "source_code": "complex_python_code_here",
    "interactive": true,
    "step_size": "function",
    "verification_level": "comprehensive"
  }'
```

## Integration with Claude Code

Add to your MCP configuration:

```json
{
  "mcp_servers": {
    "rosetta-ruchy-translator": {
      "command": "rosetta-ruchy-mcp",
      "args": ["--host", "127.0.0.1", "--port", "8080"],
      "capabilities": [
        "code_translation",
        "performance_analysis",
        "formal_verification",
        "quality_assessment"
      ]
    }
  }
}
```

## Configuration

### Environment Variables

- `RUST_LOG`: Set logging level (e.g., `info`, `debug`)
- `RUCHY_PATH`: Default path to Ruchy compiler
- `MCP_HOST`: Default host to bind to
- `MCP_PORT`: Default port to bind to

### Command Line Options

```
rosetta-ruchy-mcp [OPTIONS]

OPTIONS:
    --host <HOST>           Host to bind the server to [default: 127.0.0.1]
    --port <PORT>           Port to bind the server to [default: 8080]
    --ruchy-path <PATH>     Path to the ruchy compiler executable [default: ruchy]
    -h, --help             Print help information
    -V, --version          Print version information
```

## Performance

The MCP server is designed for high performance:

- **Language Detection**: < 1ms average
- **Translation**: < 10ms for typical functions
- **Analysis**: < 50ms for complex code
- **Memory Usage**: < 10MB baseline
- **Concurrent Requests**: Supports 1000+ concurrent translations

## Development

### Building from Source

```bash
git clone https://github.com/rosetta-ruchy/rosetta-ruchy.git
cd rosetta-ruchy/mcp-server
cargo build --release
```

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# With PMCP features
cargo test --features pmcp
```

### Docker

```bash
# Build image
docker build -t rosetta-ruchy-mcp .

# Run container
docker run -p 8080:8080 rosetta-ruchy-mcp
```

## Troubleshooting

### Common Issues

1. **"Ruchy compiler not found"**
   ```bash
   # Install Ruchy or specify custom path
   rosetta-ruchy-mcp --ruchy-path /path/to/ruchy
   ```

2. **"Port already in use"**
   ```bash
   # Use different port
   rosetta-ruchy-mcp --port 3000
   ```

3. **"Translation failed"**
   - Check that the source language is supported
   - Verify the source code syntax is valid
   - Check server logs with `RUST_LOG=debug`

### Debugging

Enable debug logging:
```bash
RUST_LOG=debug rosetta-ruchy-mcp
```

Check server health:
```bash
curl http://localhost:8080/health
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- **Language Support**: Add support for new source languages
- **Analysis Features**: Enhance code analysis capabilities
- **Performance**: Optimize translation speed and memory usage
- **Documentation**: Improve examples and guides
- **Testing**: Add more comprehensive test coverage

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Support

- **Documentation**: [https://rosetta-ruchy.org/docs](https://rosetta-ruchy.org/docs)
- **Issues**: [GitHub Issues](https://github.com/rosetta-ruchy/rosetta-ruchy/issues)
- **Discussions**: [GitHub Discussions](https://github.com/rosetta-ruchy/rosetta-ruchy/discussions)
- **Matrix**: [#rosetta-ruchy:matrix.org](https://matrix.to/#/#rosetta-ruchy:matrix.org)

## Acknowledgments

- Ruchy Language Team for the advanced compiler tooling
- MCP Protocol developers for the standardized protocol
- Contributors and the open-source community

---

*Built with ❤️ using Rust and the Toyota Way methodology for continuous improvement.*