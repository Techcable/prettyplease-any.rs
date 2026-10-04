use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::{Block, Expr, ExprBlock, ItemConst, Stmt, Type, TypeTuple};

use crate::PrettyPlease;
use crate::internal::{Sealed, head, pretty_each_ast_enum, tail};

impl Sealed for Stmt {}
impl PrettyPlease for Stmt {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String {
        let expr = Box::new(Expr::Block(ExprBlock {
            attrs: Vec::new(),
            label: None,
            block: Block {
                brace_token: Default::default(),
                stmts: vec![self],
            },
        }));
        let item = ItemConst {
            expr,
            ident: Ident::new("PRETTY", Span::call_site()),
            attrs: Default::default(),
            vis: syn::Visibility::Inherited,
            modifiers: Default::default(),
            generics: Default::default(),
            colon_token: Default::default(),
            eq_token: Default::default(),
            semi_token: Default::default(),
            ty: Box::new(Type::Tuple(TypeTuple {
                attrs: Vec::new(),
                paren_token: Default::default(),
                elems: Punctuated::new(),
            })),
            const_token: Default::default(),
        };
        let pretty = item.pretty_print();
        let stripped = crate::internal::strip_pretty_prefix_and_suffix(&pretty, "const PRETTY: () = {", "};");
        let lines = stripped.lines().collect::<Vec<_>>();
        match &*lines {
            &[] => unreachable!("zero lines"),
            &[single] => {
                let stripped = single.strip_prefix(' ')
                    .and_then(|s| s.strip_suffix(' '))
                    .unwrap_or_else(|| panic!(
                        "Expected single-line pretty expression to start/end with spaces (prefix = {:?}, suffix = {:?})",
                        head(single, 30),
                        tail(single, 30),
                    ));
                stripped.into()
            }
            [first, inner @ ..] => {
                assert!(
                    first.chars().all(char::is_whitespace),
                    "In multi-line mode, first line should be blank",
                );
                const EXPECTED_INDENT: &str = "    ";
                let mut result = String::new();
                for (offset, &line) in inner.iter().enumerate() {
                    let lineno = offset + 2;
                    if !result.is_empty() {
                        result.push('\n');
                    }
                    if let Some(stripped) = line.strip_prefix(EXPECTED_INDENT) {
                        result.push_str(stripped);
                    } else {
                        panic!("Line #{lineno} doesn't have expected prefix: {line:?}")
                    }
                }
                result
            }
        }
    }
}

pretty_each_ast_enum! {
    base => Stmt,
    ignore => [
        // syn uses (Expr, Option<Semicolon>) instead of a dedicated `StmtExpr` type
        Expr,
        // we handle Item separately
        Item,
    ],
    variants => [
        Local(syn::Local),
        Macro,
    ]
}
