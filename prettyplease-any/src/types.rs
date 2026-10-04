use proc_macro2::{Ident, Span};
use syn::{Item, ItemType, Type, Visibility, WhereClausePlacement};

use crate::PrettyPlease;
use crate::internal::{Sealed, pretty_each_ast_enum, strip_pretty_prefix_and_suffix};

impl Sealed for Type {}
impl PrettyPlease for Type {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String
    where
        Self: Sized,
    {
        let item = Item::Type(ItemType {
            attrs: Default::default(),
            vis: Visibility::Inherited,
            modifiers: Default::default(),
            type_token: Default::default(),
            ident: Ident::new("Pretty", Span::call_site()),
            generics: Default::default(),
            semi_token: Default::default(),
            ty: Box::new(self),
            eq_token: Default::default(),
            where_clause_placement: WhereClausePlacement::Early,
        });
        let pretty = item.pretty_print_owned();
        let stripped = strip_pretty_prefix_and_suffix(&pretty, "type Pretty = ", ";");
        stripped.trim().into()
    }
}
pretty_each_ast_enum! {
    base => Type,
    ignore => [Verbatim],
    variants => [
        Array,
        FnPtr,
        Group,
        ImplTrait,
        Infer,
        Macro,
        Never,
        Paren,
        Path,
        Ptr,
        Reference,
        Slice,
        TraitObject,
        Tuple,
    ]
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use syn::{Type, parse_quote};

    use crate::PrettyPlease;

    #[test]
    fn basic_types() {
        #[track_caller]
        fn check(expr: Type, expected: &str) {
            similar_asserts::assert_eq!(expr.pretty_print_owned(), expected);
        }
        check(parse_quote!((i8, i16, String)), "(i8, i16, String)");
        check(
            parse_quote!(
                ManyGenericParams<
                    Long, Long, Long, Long, Long, Long, Long, Long, Long, Long,
                    Long, Long, Long, Long, Long, Long, Long, Long, Long, Long
                >
            ),
            indoc! {"
            ManyGenericParams<
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
                Long,
            >"},
        );
    }
}
