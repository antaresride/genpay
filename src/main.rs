use lexer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = file_reader("a.pay")?;
    println!("{}", source);

    let mut iter = source.char_indices();

    while let Some(tok) = next_token(&source, &mut iter) {
        println!("{:?}", tok);
    }

    Ok(())
}
