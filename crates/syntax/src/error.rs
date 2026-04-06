/*use ariadne::{Color, Label, Report, ReportKind, Source};

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

#[test]
fn ariade_report() {
    let source = "let x = 42";
    let token = Token::Key(Keyword::Let, 0, 3);
    report_error("main.gp", source, "unexpected keyword here", &token);
}

#[test]
fn end_of_line_report() {
    let source = "";
    let token = Token::EOF(0);
    report_error("main.gp", source, "unexpected keyword here", &token);
}
*/
