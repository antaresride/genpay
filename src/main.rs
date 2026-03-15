#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "use_cranelift")]
    println!("{}", codegen::identify_architecture_aarch64());
    println!("GenPayLabs!");
    let result = lexer::Lexer::read_from_slices("let a = 10;");
    let okay = lexer::Lexer::file_reader("a.pay");
    println!("Read from the File {:?}",okay);
    println!("Read from the Slice {:?}",result);
    Ok(())
}
