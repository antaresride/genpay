use crate::Token; // Use the re-exports from your crate root
use ariadne::{Color, Label, Report, ReportKind, Source};

pub fn report_error(filename: &str, source: &str, msg: &str, token: &Token) {
    let (start, end) = token.span();

    // Ensure non-zero width for EOF and single-char positions
    let range = start..end.max(start + 1);

    Report::build(ReportKind::Error, filename, start)
        .with_message("Syntax Error")
        .with_label(
            Label::new((filename, range))
                .with_message(msg)
                .with_color(Color::Red),
        )
        .finish()
        .print((filename, Source::from(source)))
        .unwrap_or_else(|e| eprintln!("Failed to print error report: {}", e));
}
#[cfg(test)]
mod tests {
    use super::*; // Pulls in report_error and Token
    use crate::Keyword; // Pull in Keyword specifically for the tests

    #[test]
    fn ariade_report() {
        let source = "let x = 42";
        // Arguments match: (Keyword, &str, start, end)
        let token = Token::Keyword(Keyword::Let, 0, 3);
        report_error("main.gp", source, "unexpected keyword here", &token);
    }

    #[test]
    fn end_of_line_report() {
        let source = "";
        let token = Token::EOF(0);
        report_error("main.gp", source, "unexpected end of file", &token);
    }
}
