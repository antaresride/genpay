use lexer::Lexer;

/// Reads the source file into a Heap-allocated String
pub fn file_reader(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the source code
    let source = file_reader("a.pay")?;
    println!("--- Source Code ---\n{}\n-------------------", source);

    // 2. Initialize the Lexer (Borrowing the source)
    let lexer = Lexer::new(&source);

    // 3. Iterate directly.
    // The 'for' loop calls .next() automatically, starting from the first token.
    for token in lexer {
        let (start, end) = token.span();
        println!("[{:>3}..{:>3}] {:?}", start, end, token);
    }

    Ok(())
}

//let mut iter = source.char_indices();

/* while let Some(tok) = next_token(&source, &mut iter) {
    println!("{:?}", tok);
}*/
