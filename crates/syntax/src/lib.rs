mod error;
mod tokens;

// Re-export the main types
pub use tokens::Keyword;
pub use tokens::Literal;
pub use tokens::Symbol;
pub use tokens::Token;

// To allow using variants (like Identifier, Keyword, etc.) directly:
pub use tokens::Token::*;
