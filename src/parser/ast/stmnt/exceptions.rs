use super::NodeStmnt;
use crate::common::Type;
use crate::error::{span::Span, ChalError};
use crate::lexer::{Delimiter, Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeVarCall};
use crate::parser::{LineReader, TokenReader};

/// The structure, denoting a `try-catch` block. Any exceptions received under
/// the `try` block will be handled inside the `catch` block.
///
/// Chalcedony introduces the concept of `unsafety` inside scripting languages.
/// Functions which can raise an exception, which is not guarded inside a `try-
/// catch` block, must end with a `!` at the edn (e.g. `tg!()`). Thus calling
/// a potentially errorous function is easily distinguishable and enforced by
/// the interpreter itself.
///
/// Syntax:
/// `try`:
///     \<statements\>
/// `catch`(\<var-name\>: exception):
///     \<safe-statements\>
///
/// where `<safe-statements>` denotes the use of any code, which does not have
/// the potential to raise any exception.
#[derive(Debug, PartialEq)]
pub struct NodeTryCatch {
    pub try_body: Vec<NodeStmnt>,
    pub try_span: Span,
    pub exception_var: NodeVarCall,
    pub catch_body: Vec<NodeStmnt>,
}

/// The structure, denoting the raising of an exception.
///
/// Exceptions in Chalcedony are similar to the `error` type in GoLang. Both
/// types boil down to the usage of strings, e.g.:
/// ```go
/// package main
///
/// import ("errors", "fmt")
///
/// func example() (res int, err error) {
///     err = errors.New("some invalid value")
///     return
/// }
///
/// func main() {
///     res, err := example()
///     if err != nil {
///         fmt.Printf("Encountered the error: %v", err)
///     }
/// }
/// ```
///
/// In Chalcedony a similar result can be expressed in a quite similar manner:
/// ```
/// fn example!() -> int:
///     throw "some invalid value"
///     return 0
///
/// try:
///     let res = example!()
/// catch (exc: exception):
///     print("Encountered the error: " + exc)
/// ```
///
/// <br> <br>
/// Syntax:
/// `throw` \<str-expr\>
///
/// where `<str-expr>` means an expression, which results in a string
#[derive(Debug, PartialEq)]
pub struct NodeThrow(pub NodeExpr);

impl NodeTryCatch {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        // try:                                         | try header
        //     let a = unsafe_func!()                   > try body
        //     let b = other_unsafe_func!()             > try body
        //     print("Sum: " + (a + b))                 > try body
        //
        // catch (exc: exception):                      | catch header
        //     print("Encountered the error: " + exc)   > catch body
        //

        let mut try_header = reader.advance_reader()?;
        let try_span = try_header.current();
        try_header.expect_exact(TokenKind::Keyword(Keyword::Try))?;
        try_header.expect_exact(TokenKind::Special(Special::Colon))?;
        try_header.expect_exact(TokenKind::Newline)?;

        let try_body = reader.advance_until(|ln| {
            let Some(front) = ln.front_tok() else {
                return false;
            };
            front.kind == TokenKind::Keyword(Keyword::Catch)
        })?;

        let mut catch_header = reader.advance_reader()?;
        catch_header.expect_exact(TokenKind::Keyword(Keyword::Catch))?;
        catch_header.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        let exception_var_span = catch_header.current();
        let exception_var = catch_header.expect_ident()?;

        catch_header.expect_exact(TokenKind::Special(Special::Colon))?;
        catch_header.expect_exact(TokenKind::Type(Type::Exception))?;
        catch_header.expect_exact(TokenKind::Delimiter(Delimiter::ClosePar))?;
        catch_header.expect_exact(TokenKind::Special(Special::Colon))?;
        catch_header.expect_exact(TokenKind::Newline)?;

        Ok(NodeTryCatch {
            try_span,
            try_body: LineReader::new(try_body, reader.spanner()).try_into()?,
            exception_var: NodeVarCall {
                name: exception_var,
                span: exception_var_span,
            },
            catch_body: reader.try_into()?,
        })
    }
}

impl NodeThrow {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        reader.expect_exact(TokenKind::Keyword(Keyword::Throw))?;

        let expr_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let expr_reader = TokenReader::new(expr_raw, Span::from(reader.spanner()));
        let expr = NodeExpr::new(expr_reader)?;

        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeThrow(expr))
    }
}
