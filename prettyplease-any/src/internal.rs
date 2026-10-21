pub trait Sealed {}

macro_rules! pretty_each_ast_enum {
    (
        base => $target:ident,
        $(ignore => [$($ignored:ident),+ $(,)?],)?
        variants => [
            $($variant:ident $( ($tp:ty) )?),+
            $(,)?
        ] $(,)?
    ) => {
        const _: () = {
            #[remain::sorted] // use here to avoid Verbatim
            enum _VerifySorted {
                $($variant,)*
            }
            fn _verify_exhaustive(x: &$target) {
                #[cfg_attr(feature = "_internal_nightly_test", warn(non_exhaustive_omitted_patterns))]
                match x {
                    $($target::$variant(_) => {},)*
                    $($($target::$ignored { .. } => {},)*)?
                    #[allow(unreachable_patterns)] // possible if $target is not exhaustive
                    _ => {},
                }
            }
        };
        $($crate::internal::pretty_each_ast_enum! {
            @impl $target,
            $variant,
            $crate::internal::pretty_each_ast_enum!(@infer_type $target, $variant$(($tp))?)
        })*
    };
    (@infer_type $base:ident, $variant:ident) => (pastey::paste!(syn::[<$base $variant>]));
    (@infer_type $base:ident, $variant:ident($t:ty)) => ($t);
    (@impl $target:ident, $variant:ident, $inner:ty) => {
        impl crate::internal::Sealed for $inner {}
        impl PrettyPlease for $inner {
            fn pretty_print(&self) -> String {
                self.clone().pretty_print_owned()
            }
            fn pretty_print_owned(self) -> String {
                // existence of pretty_print_owned matters here,
                // as otherwise we would need two clones
                ($target::$variant(self)).pretty_print_owned()
            }
        }
    }
}

pub fn head(x: &str, amount: usize) -> String {
    x.chars().take(amount).collect::<String>()
}
pub fn tail(x: &str, amount: usize) -> String {
    x.chars().skip(x.len().saturating_sub(amount)).collect::<String>()
}

#[track_caller]
pub fn strip_pretty_prefix_and_suffix<'a>(pretty: &'a str, expected_prefix: &str, expected_suffix: &str) -> &'a str {
    let pretty = pretty.trim();
    let stripped = pretty
        .strip_prefix(expected_prefix)
        .unwrap_or_else(|| panic!("Unexpected prefix for pretty printed code: {:?}", head(pretty, 100)));
    stripped
        .strip_suffix(expected_suffix)
        .unwrap_or_else(|| panic!("Unexpected suffix for pretty printed code: {:?}", tail(pretty, 100)))
}

#[track_caller]
pub fn strip_ident<S: AsRef<str>>(lines: impl IntoIterator<Item = S>, levels: usize) -> String {
    let indent_size = levels.checked_mul(4).expect("overflow");
    let expected_indent = " ".repeat(indent_size);
    let mut result = String::new();
    for (offset, line) in lines.into_iter().enumerate() {
        let lineno = offset + 2;
        let line = line.as_ref();
        if !result.is_empty() {
            result.push('\n');
        }
        if let Some(stripped) = line.strip_prefix(&expected_indent) {
            result.push_str(stripped);
        } else {
            panic!("Line #{lineno} doesn't have expected indent (level {levels}): {line:?}")
        }
    }
    result
}

pub(crate) use pretty_each_ast_enum;
