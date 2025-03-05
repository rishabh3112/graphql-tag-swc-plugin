// libs
use swc_core::common::Span;
use swc_core::ecma::ast::*;

// helpers
use crate::parser::utils::get_key_value_node;

pub fn create_name(name: String, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "Name".into());
    let value = get_key_value_node("value".into(), name.into());
    let name = ObjectLit {
        span,
        props: vec![kind, value],
    };
    Expr::Object(name)
}
