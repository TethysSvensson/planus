use codespan::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LexicalError {
    pub err: String,
    pub span: Span,
}
impl std::fmt::Display for LexicalError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.span.start().0 == 0 && self.span.end().0 == 0 {
            write!(fmt, "{}", self.err)
        } else {
            write!(fmt, "{} at {:?}", self.err, self.span)
        }
    }
}
impl LexicalError {
    pub fn new<E: ToString>(err: E, span: Span) -> Self {
        LexicalError {
            err: err.to_string(),
            span,
        }
    }
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct ErrorKind: u32 {
        const DECLARATION_PARSE_ERROR = 0x1;
        const UNKNOWN_IDENTIFIER = 0x2;
        const TYPE_ERROR = 0x4;
        const NUMERICAL_RANGE_ERROR = 0x8;
        const NUMERICAL_PARSE_ERROR = 0x10;
        const MISC_SEMANTIC_ERROR = 0x20;
        const TYPE_DEFINED_TWICE = 0x40;
        const FIELD_DEFINED_TWICE = 0x80;
        const FILE_ORDER = 0x100;
        const NOT_SUPPORTED = 0x200;
    }
}
