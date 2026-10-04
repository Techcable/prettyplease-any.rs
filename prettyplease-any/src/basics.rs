use proc_macro2::Ident;
use syn::{Expr, ExprPath, Path};

use crate::PrettyPlease;
use crate::internal::Sealed;

impl Sealed for Ident {}
impl PrettyPlease for Ident {
    fn pretty_print(&self) -> String {
        self.to_string()
    }
}

impl Sealed for Path {}
impl PrettyPlease for Path {
    fn pretty_print(&self) -> String {
        self.clone().pretty_print_owned()
    }
    fn pretty_print_owned(self) -> String
    where
        Self: Sized,
    {
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            path: self,
            qself: None,
        })
        .pretty_print_owned()
    }
}
