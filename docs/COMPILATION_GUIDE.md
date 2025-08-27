# Ruchy Compilation Guide

**Sprint 38**: Compilation & Transpilation Patterns  
**Date**: 2025-08-27  
**Ruchy Version**: v1.21.0

## Overview

This guide documents working patterns for compiling Ruchy code to native binaries and transpiling to Rust. While WebAssembly compilation is not yet implemented, native compilation works with some considerations.

## Working Features

### ✅ Native Compilation (`ruchy compile`)
- Generates standalone executables
- Supports Linux x86_64 targets
- Binary sizes ~3.8MB for simple programs
- Includes Rust runtime

### ✅ Rust Transpilation (`ruchy transpile`)
- Outputs valid Rust code
- Can be further optimized manually
- Useful for understanding compilation

### ✅ MCP Server (`ruchy mcp`)
- Real-time quality monitoring
- Configurable thresholds
- Streaming updates

## Compilation Patterns

### 1. Basic Hello World
```ruchy
// hello_world.ruchy
fun main() {
    println("Hello from compiled Ruchy!");
}
```

**Compile**: 
```bash
ruchy compile hello_world.ruchy --output hello_world
./hello_world
```

### 2. Avoiding Common Issues

#### ❌ Problem: `var` keyword
```ruchy
// This will fail compilation
fun broken() {
    var x = 0;
    while x < 10 {
        x = x + 1;
    }
}
```

#### ✅ Solution: Use recursion or let bindings
```ruchy
// This compiles successfully
fun working(x: i32) -> i32 {
    if x >= 10 {
        x
    } else {
        working(x + 1)
    }
}
```

### 3. Ownership Issues

#### ❌ Problem: Moving values
```ruchy
fun broken() {
    let nums = vec![1, 2, 3];
    let sum1 = process(nums);    // moves nums
    let sum2 = process(nums);    // ERROR: use after move
}
```

#### ✅ Solution: Clone or restructure
```ruchy
fun working() {
    let nums = vec![1, 2, 3];
    let result = process_once(nums);
}
```

## Transpilation to Rust

### Basic Usage
```bash
ruchy transpile input.ruchy > output.rs
```

### Generated Code Structure
```rust
// Ruchy input
fun add(a: i32, b: i32) -> i32 {
    a + b
}

// Rust output
fn add(a: i32, b: i32) -> i32 {
    { a + b }
}
```

### Common Transpilation Issues

1. **Extra braces**: Generated code has unnecessary `{ }`
2. **Unused imports**: Always includes `HashMap` even if unused
3. **Mutability**: Marks more variables as `mut` than needed

## MCP Server Deployment

### Starting the Server
```bash
ruchy mcp \
    --name "project-monitor" \
    --min-score 0.85 \
    --max-complexity 20 \
    --streaming \
    --verbose
```

### Configuration Options
| Option | Description | Default |
|--------|-------------|---------|
| `--name` | Server identifier | ruchy-mcp |
| `--min-score` | Minimum quality threshold | 0.8 |
| `--max-complexity` | Maximum complexity allowed | 10 |
| `--timeout` | Session timeout (seconds) | 3600 |
| `--streaming` | Enable real-time updates | false |

### Monitoring Dashboard Integration
```ruchy
// Connect to MCP server
fun connect_mcp() {
    // Server connection logic
    // Real-time quality metrics
}
```

## Build Scripts

### Makefile Integration
```makefile
compile:
    ruchy compile $(SOURCE) --output $(OUTPUT)
    
transpile:
    ruchy transpile $(SOURCE) > $(SOURCE:.ruchy=.rs)
    
mcp-server:
    ./scripts/start_mcp_server.sh
```

### Continuous Integration
```yaml
# .github/workflows/compile.yml
jobs:
  compile:
    steps:
      - name: Compile Ruchy
        run: ruchy compile main.ruchy --output app
      - name: Test Binary
        run: ./app
```

## Binary Distribution

### Size Optimization
- Basic programs: ~3.8MB
- Includes Rust runtime and stdlib
- No dynamic linking required

### Cross-Compilation
Currently supports:
- Linux x86_64 (native)
- Other targets pending

## Troubleshooting

### Common Errors

1. **"var not found in scope"**
   - Solution: Replace `var` with recursive patterns

2. **"use of moved value"**
   - Solution: Clone values or restructure logic

3. **"cannot infer type"**
   - Solution: Add explicit type annotations

### Debugging Compilation
```bash
# View transpiled Rust code
ruchy transpile file.ruchy

# Check for syntax errors
ruchy check file.ruchy

# Verify before compiling
ruchy score file.ruchy
```

## Best Practices

### For Successful Compilation
1. **Avoid `var` keyword** - Use recursion or functional patterns
2. **Handle ownership** - Be mindful of moves and borrows
3. **Type annotations** - Add when inference fails
4. **Test incrementally** - Start with simple programs
5. **Check transpilation** - Review generated Rust code

### Performance Tips
1. **Tail recursion** - Optimize recursive functions
2. **Avoid cloning** - Minimize unnecessary copies
3. **Use references** - When ownership not needed

## Examples Repository

### Working Examples
- `hello_world.ruchy` - Basic compilation test ✅
- `fibonacci.ruchy` - Recursive computation ✅
- `simple_math.ruchy` - Arithmetic operations ✅

### Known Issues
- `benchmark.ruchy` - Uses `var` keyword ❌
- `complex_loops.ruchy` - Ownership problems ❌

## Future Improvements

### Planned Features
- WebAssembly compilation (`ruchy wasm`)
- Cross-compilation targets
- Optimization flags
- Link-time optimization

### Current Limitations
- No WASM support yet
- Limited to x86_64 Linux
- No incremental compilation
- Binary size not optimized

## Conclusion

While Ruchy compilation has limitations, it successfully generates working binaries for well-structured programs. Key success factors:
- Avoid mutable state patterns
- Use functional programming style
- Test with transpilation first
- Monitor with MCP server

For production use, consider:
- Testing thoroughly before deployment
- Using MCP for quality monitoring
- Keeping binaries updated
- Following functional patterns

---

**Next Steps**: Continue exploring compilation patterns and document additional workarounds as discovered.