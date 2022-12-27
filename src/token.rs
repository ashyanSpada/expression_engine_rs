use rust_decimal::Decimal;


pub struct Span(usize, usize);

enum Token<'a> {
    Default(Span),
    Operator((&'a str, Span)),
    Literal((Decimal, Span)),
    Comma((&'a str, Span)),
    Bool((bool, Span)),
    String((&'a str, Span)),
    Reference((&'a str, Span)),
    Function((&'a str, Vec<Token<'a>>, Span)),
    WhiteSpace(Span)
}
