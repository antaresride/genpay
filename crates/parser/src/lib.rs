mod expressions;
pub use expressions::Expressions;
mod statements;
pub use statements::Statements;

#[cfg(test)]
#[cfg(feature = "use_arena")]
#[allow(dead_code)]
mod arena_benchmarks {
    // use super::*;
    use bumpalo::Bump;

    use std::time::Instant;

    // A simple recursive expression tree
    enum Expression<'a> {
        Literal(i32),
        Add(&'a Expression<'a>, &'a Expression<'a>),
    }

    /*  fn build_large_tree_standard(depth: u32) -> Box<Expression<'static>> {
        // Note: Standard Box allocation for recursive types
        // (Simplified for illustration; true recursive Boxed enums require more boilerplate)
        unimplemented!("Standard heap allocation is often 10-20x slower for this pattern")
    }*/

    #[test]
    fn bench_bumpalo_speed() {
        let arena = Bump::new();
        let start = Instant::now();

        // Simulate 1,000,000 allocations of an AST node
        for i in 0..1_000_000 {
            let _node = arena.alloc(Expression::Literal(i));
        }

        let duration = start.elapsed();
        println!("\n[Arena] 1M allocations took: {:?}", duration);

        // Validation: The arena is dropped all at once here
        // No individual 'Drop' calls for the 1M nodes.
    }
    #[test]
    fn mock_parse_declaration(){
        // Source: let x = 100;
        // Explicitly use the struct from the syntax module
         let tokens:&[syntax::Token<'_>] = &[

            syntax::Key(syntax::Keyword::Let),
            syntax::Token::Identifier(syntax::Data::try_from_str("x").expect("Failed to create identication")),
            syntax::Symb(syntax::Symbol::Equal),
            syntax::Lit(syntax::Literal::Int(100)),

        ];
        //assert!(tokens[0],"let");
        println!("{:?} Printing the tokens ",tokens);

    }
}
