// libs
use apollo_parser::cst::{CstChildren, Selection, SelectionSet};
use swc_common::Span;
use swc_ecma_ast::*;

// modules
mod selection;

// helpers
use crate::parser::utils::get_key_value_node;
use selection::create_selection;

pub fn create_selection_set(selection_set: Option<SelectionSet>, span: Span) -> Expr {
    if selection_set.is_none() {
        let sel_set = ObjectLit {
            span,
            props: vec![],
        };
        return Expr::Object(sel_set);
    }
    let unwrapped_selection_set = selection_set.unwrap();
    let kind = get_key_value_node("kind".into(), "SelectionSet".into());
    let selections = get_key_value_node(
        "selections".into(),
        create_selections(unwrapped_selection_set.selections(), span),
    );

    let sel_set = ObjectLit {
        span,
        props: vec![kind, selections],
    };
    Expr::Object(sel_set)
}

fn create_selections(selections: CstChildren<Selection>, span: Span) -> Expr {
    let mut all_selections = vec![];
    for selection in selections {
        all_selections.push(create_selection(selection, span));
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_selections,
    })
}
