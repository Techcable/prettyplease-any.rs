use syn::{Expr, ExprLit, Lit, Stmt};

use crate::PrettyPlease;
use crate::internal::{Sealed, pretty_each_ast_enum};

impl Sealed for Expr {}
impl PrettyPlease for Expr {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String {
        Stmt::Expr(self, None).pretty_print_owned()
    }
}

pretty_each_ast_enum! {
    base => Expr,
    ignore => [
        Verbatim,
    ],
    variants => [
        Array,
        Assign,
        Async,
        Await,
        Binary,
        Block,
        Break,
        Call,
        Cast,
        Closure,
        Const,
        Continue,
        Field,
        ForLoop,
        Group,
        If,
        Index,
        Infer,
        Let,
        Lit,
        Loop,
        Macro,
        Match,
        MethodCall,
        Paren,
        Path,
        Range,
        RawAddr,
        Reference,
        Repeat,
        Return,
        Struct,
        Try,
        TryBlock,
        Tuple,
        Unary,
        Unsafe,
        While,
        Yield,
    ],
}

impl Sealed for Lit {}
impl PrettyPlease for Lit {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String {
        Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: self,
        })
        .pretty_print_owned()
    }
}
pretty_each_ast_enum!(
    base => Lit,
    ignore => [Verbatim],
    variants => [
        Bool,
        Byte,
        ByteStr,
        Char,
        CStr,
        Float,
        Int,
        Str,
    ],
);

#[cfg(test)]
mod test {
    use syn::{Expr, parse_quote};

    use crate::PrettyPlease;

    #[test]
    fn basic_expr() {
        #[track_caller]
        fn check(expr: Expr, expected: &str) {
            similar_asserts::assert_eq!(expr.pretty_print_owned(), expected);
        }
        check(parse_quote!(3 + 4), "3 + 4");
        check(
            parse_quote!({
                fn foo() -> String {
                    3 + 7
                }
            }),
            indoc::indoc! {"{
                fn foo() -> String {
                    3 + 7
                }
            }"},
        );
    }
}
