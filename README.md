## Description
**Genpay** - a statically-typed compiling programming language for smart contracts and system tools. <br><br>

##  Features
*  **Powerful**. The language syntax is easy to read and write.
*  **Fast**. The compiler uses LLVM as a backend generating WASM, Binary and LLVM-IR.
*  **Strict**. Analyzers and checkers will prevent most compile-time errors.

## Technical Details
- **Language:** Rust
- **Build Systems:** Cargo
- **Backend:** LLVM
- **Errors:** thiserror
- **Error Reporting:** miette, colored
- **Command Line Interface:** clap
- **Arena Allocation:** bumpalo
- **Memory Allocation:** Minimalloc

### Modular Design
- `genpay` - Combines all submodules into the main process.
- `lexer` - Converts source code into abstract data types (Tokens).
- `parser` - Analyzes and converts tokens into an Abstract Syntax Tree.
- `semantic` - Recursively checks the AST for type and principle matching.
- `codegen` - Recursively compiles the AST.
- `linker` -  Compiles the module to differents objects file and links it.

