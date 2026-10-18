#[derive(logos::Logos)]
#[logos(extras = ())]
enum SyntaxKind {
    #[regex(r"a", priority = 1)]
    #[regex(r"b", priority = 2)]
    FloatLiteral,
}
