use syn::Item;

use crate::PrettyPlease;
use crate::internal::{Sealed, pretty_each_ast_enum};

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
