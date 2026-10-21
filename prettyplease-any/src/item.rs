use assert_matches::assert_matches;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ImplItem, Item, TraitItem};

use crate::PrettyPlease;
use crate::internal::{Sealed, pretty_each_ast_enum, strip_ident, strip_pretty_prefix_and_suffix};

impl Sealed for Item {}
impl PrettyPlease for Item {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String {
        let file = syn::File {
            attrs: Vec::new(),
            frontmatter: None,
            shebang: None,
            items: vec![self.clone()],
        };
        prettyplease::unparse(&file)
    }
}
pretty_each_ast_enum!(
    base => Item,
    ignore => [Verbatim],
    variants => [
        Const,
        Enum,
        ExternCrate,
        Fn,
        ForeignMod,
        Impl,
        Macro,
        Mod,
        Static,
        Struct,
        Trait,
        TraitAlias,
        Type,
        Union,
        Use,
    ],
);

fn print_wrapped_items(type_name: &str, wrapper_prefix: &str, inner: &TokenStream) -> String {
    let wrapped = format!("{wrapper_prefix}{inner}}}");
    let file = syn::parse_file(&wrapped)
        .unwrap_or_else(|cause| panic!("could not roundtrip {type_name} back to file: {cause}"));
    let pretty = prettyplease::unparse(&file);
    let stripped = strip_pretty_prefix_and_suffix(&pretty, wrapper_prefix, "}").trim_end();
    let mut lines = stripped.lines();
    assert_matches!(
        lines.next(),
        Some(line) if line.trim().is_empty(),
        "first line must be blank"
    );
    strip_ident(lines, 1)
}

const IMPL_WRAPPER_PREFIX: &str = "impl __Dummy for __Dummy {";
const TRAIT_WRAPPER_PREFIX: &str = "trait __Dummy {";

macro_rules! impl_wrapped_items {
    ($($target:ident => $prefix:ident),+ $(,)?) => {$(
        impl Sealed for $target {}
        impl PrettyPlease for $target {
            fn pretty_print(&self) -> String where Self: Sized {
                print_wrapped_items(
                    stringify!($target),
                    $prefix,
                    &self.to_token_stream(),
                )
            }
        }
        pretty_each_ast_enum!(
            base => $target,
            ignore => [Verbatim],
            variants => [
                Const,
                Fn,
                Macro,
                Type,
            ],
        );
    )*};
}

impl_wrapped_items!(
    ImplItem => IMPL_WRAPPER_PREFIX,
    TraitItem => TRAIT_WRAPPER_PREFIX
);

#[cfg(test)]
mod test {
    use syn::parse::Parse;
    use syn::{ImplItem, TraitItem, parse_quote};

    use crate::PrettyPlease;

    #[track_caller]
    fn check<T: PrettyPlease>(expr: T, expected: &str) {
        similar_asserts::assert_eq!(expr.pretty_print_owned(), expected);
    }

    #[test]
    fn trait_items() {
        let check = check::<TraitItem>;
        check(
            parse_quote! {
                fn foo(&self) -> String;
            },
            "fn foo(&self) -> String;",
        );
        shared_items::<TraitItem>();
    }

    #[test]
    fn impl_items() {
        shared_items::<ImplItem>();
    }

    fn shared_items<T: PrettyPlease + Parse>() {
        let check = check::<T>;
        check(
            parse_quote! {
                const NUM: u32 = { 3 + 7 };
            },
            "const NUM: u32 = { 3 + 7 };",
        );
        check(
            parse_quote! {
                fn foo(&self) -> String {
                    format!("hello {self}")
                }
            },
            // "fn foo(&self) -> String {}",
            indoc::indoc! {r#"
            fn foo(&self) -> String {
                format!("hello {self}")
            }"#},
        );
    }
}
