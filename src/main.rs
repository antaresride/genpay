use lexer::Lexer;

#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "use_cranelift")]
    println!("{}", codegen::identify_architecture_aarch64());
    // 1. Load string to heap
    let content = Lexer::file_reader("a.pay")?;
    // 2. Initialize Lexer pointing to that heap memory
    let lexer = Lexer::new(&content);
    println!("{:?}", lexer);
    //parser::mock_parse_declaration();

    Ok(())
}
