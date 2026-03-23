mod expressions;
pub use expressions::Expressions;
mod statements;
pub use statements::Statements;

pub fn mock_parse_declaration() {
    // Source: let x = 100;
    // Explicitly use the struct from the syntax module
    let tokens: &[syntax::Token<'_>] = &[
        syntax::Key(syntax::Keyword::Let),
        syntax::Token::Identifier(
            syntax::Data::try_from_str("x").expect("Failed to create identication"),
        ),
        syntax::Symb(syntax::Symbol::Equal),
        syntax::Lit(syntax::Literal::Int(100)),
    ];
    //assert!(tokens[0],"let");
    println!("{:?} Printing the tokens ", tokens);
}

#[cfg(test)]

mod arena_benchmarks {

    // A simple recursive expression tree
    enum Expression<'a> {
        Literal(i32),
        Add(&'a Expression<'a>, &'a Expression<'a>),
    }

    #[test]
    fn mock_parse_declaration() {
        // Source: let x = 100;
        // Explicitly use the struct from the syntax module
        let tokens: &[syntax::Token<'_>] = &[
            syntax::Key(syntax::Keyword::Let),
            syntax::Token::Identifier(
                syntax::Data::try_from_str("x").expect("Failed to create identication"),
            ),
            syntax::Symb(syntax::Symbol::Equal),
            syntax::Lit(syntax::Literal::Int(100)),
        ];
        //assert!(tokens[0],"let");
        println!("{:?} Printing the tokens ", tokens);
    }
}
