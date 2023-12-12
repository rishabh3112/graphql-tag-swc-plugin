// libs
use apollo_parser::cst::{Argument, Arguments};
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{name::create_name, value::create_value},
    utils::get_key_value_node,
};

pub fn create_arguments(arguments: Option<Arguments>, span: Span) -> Expr {
    if arguments.is_none() {
        let args = ArrayLit {
            span,
            elems: vec![],
        };
        return Expr::Array(args);
    }

    let unwrapped_arguments = arguments.unwrap().arguments();
    let mut all_arguments = vec![];
    for argument in unwrapped_arguments {
        all_arguments.push(create_argument(argument, span));
    }
    return Expr::Array(ArrayLit {
        span,
        elems: all_arguments,
    });
}

fn create_argument(argument: Argument, span: Span) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "Argument".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(argument.name().unwrap().text().as_str().into(), span),
    );
    let value = get_key_value_node("value".into(), create_value(argument.value(), span));
    let arg = ObjectLit {
        span,
        props: vec![kind, name, value],
    };
    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(arg)),
    })
}
