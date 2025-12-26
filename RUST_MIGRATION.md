# Rust Migration Guide

This document describes the conversion of the LCI LOLCODE interpreter from C to Rust.

## Overview

The LCI project has been converted from C to Rust while maintaining compatibility with the LOLCODE 1.3 specification. This migration brings modern language features, improved safety, and better tooling to the project.

## Architecture

### Module Structure

The Rust implementation follows a modular design:

```
src/
├── error.rs        - Error types and handling
├── lexer.rs        - Lexical analysis (character → lexemes)
├── tokenizer.rs    - Token generation (lexemes → tokens)
├── parser.rs       - Syntax analysis (tokens → AST)
├── interpreter.rs  - Execution engine (AST → results)
├── lib.rs          - Library interface
└── main.rs         - CLI entry point
```

### Key Design Decisions

1. **Error Handling**
   - Uses `Result<T, E>` for all fallible operations
   - `thiserror` crate for ergonomic error definitions
   - Proper error propagation with the `?` operator
   - Exit codes preserved from original C implementation

2. **Memory Management**
   - No manual memory management needed
   - Rust's ownership system ensures safety
   - `Vec<T>` replaces dynamic arrays
   - `String` replaces `char*`
   - `HashMap` for variable storage

3. **Type Safety**
   - Strong typing with enums for tokens and values
   - Pattern matching for exhaustive handling
   - Compile-time guarantees prevent many runtime errors

4. **CLI Interface**
   - Modern `clap` v4 for argument parsing
   - Derive macros for ergonomic CLI definitions
   - Better help messages and error reporting

## Conversion Mapping

### C to Rust Equivalents

| C Pattern | Rust Equivalent |
|-----------|-----------------|
| `malloc/free` | Automatic (ownership) |
| `NULL` | `Option::None` |
| `typedef enum` | `enum` |
| `typedef struct` | `struct` |
| `char *` | `String` or `&str` |
| `int` | `i32` or `i64` |
| `void *` | Generic `T` or `Box<dyn Trait>` |
| Error codes | `Result<T, E>` |
| Manual bounds checking | Iterator methods |

### Example Conversions

#### Lexeme Structure

**C:**
```c
typedef struct {
    char *image;
    const char *fname;
    unsigned int line;
} Lexeme;
```

**Rust:**
```rust
pub struct Lexeme {
    pub image: String,
    pub fname: String,
    pub line: usize,
}
```

#### Error Handling

**C:**
```c
if (!lexeme) {
    perror("malloc");
    return NULL;
}
```

**Rust:**
```rust
// Allocation is automatic and infallible
// Errors use Result:
let lexeme = Lexeme::new(...)
    .ok_or(LciError::AllocationFailed)?;
```

## Dependencies

The Rust version uses these external crates:

- **clap** (4.5): Modern command-line argument parsing
- **thiserror** (2.0): Error handling macros

These are industry-standard, well-maintained crates that follow Rust best practices.

## Building

### Debug Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Documentation
```bash
cargo doc --open
```

## Performance

The Rust implementation offers:

- **Comparable or better performance** to C (thanks to LLVM optimization)
- **Zero-cost abstractions** - high-level code compiles to efficient machine code
- **Memory safety** without runtime overhead
- **Thread safety** built into the type system

## Future Improvements

Potential enhancements for the Rust version:

1. **Parallel Processing**: Use Rayon for parallel test execution
2. **Better Error Messages**: Leverage Rust's display traits for detailed errors
3. **Language Server Protocol**: IDE support for LOLCODE
4. **WASM Target**: Run LOLCODE in web browsers
5. **Extended Standard Library**: More built-in functions
6. **Debugging Support**: Integration with LLDB/GDB

## Compatibility

The Rust implementation aims to be 100% compatible with the original C implementation:

- Same command-line interface
- Same error codes
- Same output format
- Same LOLCODE 1.3 spec compliance

## Migration Benefits

### For Users

- **Easy Installation**: `cargo install` or download pre-built binaries
- **Better Error Messages**: Clear, actionable error reporting
- **Cross-Platform**: Rust makes it easier to build for multiple platforms
- **Active Maintenance**: Leverage Rust's modern tooling and community

### For Developers

- **Safety**: No segfaults, use-after-free, or buffer overflows
- **Productivity**: Less time debugging memory issues
- **Refactoring**: Compiler catches mistakes during refactoring
- **Testing**: Built-in test framework and benchmarking
- **Documentation**: Rustdoc generates beautiful API docs

## Contribution Guidelines

When contributing to the Rust version:

1. Run `cargo fmt` to format code
2. Run `cargo clippy` to catch common mistakes
3. Add tests for new features
4. Update documentation
5. Ensure `cargo test` passes

## License

The Rust implementation maintains the same GPL-3.0-or-later license as the original C version.

## Credits

- **Original Author**: Justin J. Meza (C implementation)
- **Rust Conversion**: Converted using modern Rust best practices
- **LOLCODE Spec**: http://lolcode.org

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [LOLCODE Specification](http://lolcode.org/specs/1.3)
