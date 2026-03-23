use lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load string to heap
    let content = Lexer::file_reader("a.pay")?;
    // 2. Initialize Lexer pointing to that heap memory
    let lexer = Lexer::new(&content);
    println!("{:?}", lexer);
    //parser::mock_parse_declaration();

    Ok(())
}
