use crate::tokens::Token;
use std::rc::Rc;
#[derive(Debug, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Rc<Token<'a>>,
}
