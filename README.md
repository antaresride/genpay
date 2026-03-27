[Latest Release]: https://github.com/antaresride/genpay/releases/latest
<div align="center">
  <picture>
    <img src="https://github.com/antaresride/genpay/blob/main/assets/GenPayLogo.png" width="35%" />
  </picture>
  <div>
    <h1>Genpay Programming Language</h1>
    <i></i>
  </div>
  <br/> 
</div>

## Description
**Genpay** - A programming language for smart contracts and agents. <br><br>
See official documentation here: [Genpay Documentation](https://genpay-site.vercel.app/)
##  Features
*  **Powerful**. The language syntax is easy to read and write.
*  **Fast**. The compiler uses LLVM as a backend generating WASM, Binary and LLVM-IR.
*  **Strict**. Analyzers and checkers will prevent most compile-time errors.

## Technical Details
- **Language:** Rust
- **Build Systems:** Cargo
- **Backend:** LLVM or Cranelift
- **Errors:** 
- **Error Reporting:** ariadne
- **Command Line Interface:** clap
- **Arena Allocation:** bumpalo
- **Memory Allocation:** mimalloc

### Modular Design
- `genpay` - Combines all submodules into the main process.
- `lexer` - Converts source code into abstract data types (Tokens).
- `parser` - Analyzes and converts tokens into an Abstract Syntax Tree.
- `semantic` - Recursively checks the AST for type and principle matching.
- `codegen` - Recursively compiles the AST.
- `linker` -  Compiles the module to differents objects file and links it.
- `syntax` - Implements a Single-Pass Zero-Copy Lexical Analyzer using Reference Counting.
