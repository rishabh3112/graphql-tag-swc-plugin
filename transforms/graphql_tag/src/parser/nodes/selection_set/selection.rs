// libs
use apollo_parser::cst::{Field, FragmentSpread, InlineFragment, Selection};
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use super::create_selection_set;
use crate::parser::{
    nodes::{
        arguments::create_arguments, directive::create_directives, name::create_name,
        types::create_type_condition,
    },
    utils::get_key_value_node,
};

pub fn create_selection(selection: Selection, span: Span) -> Option<ExprOrSpread> {
    match selection {
        Selection::Field(field) => create_field(field, span),
        Selection::FragmentSpread(frag_spread) => create_fragment_spread(frag_spread, span),
        Selection::InlineFragment(inline_frag) => create_inline_fragment(inline_frag, span),
    }
}

fn create_field(field: Field, span: Span) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "Field".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let arguments = get_key_value_node(
        "arguments".into(),
        create_arguments(field.arguments(), span),
    );
    let directives = get_key_value_node(
        "directives".into(),
        create_directives(field.directives(), span),
    );

    let mut sel: ObjectLit = ObjectLit {
        span,
        props: vec![kind, name, arguments, directives],
    };

    if field.selection_set().is_some() {
        let sel_set = get_key_value_node(
            "selectionSet".into(),
            create_selection_set(field.selection_set(), span),
        );

        sel.props.push(sel_set);
    }

    if field.alias().is_some() {
        let alias = get_key_value_node(
            "alias".into(),
            create_name(field.alias().unwrap().name().unwrap().text().into(), span),
        );

        sel.props.push(alias);
    }

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(sel)),
    })
}

fn create_fragment_spread(frag_spread: FragmentSpread, span: Span) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "FragmentSpread".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(
            frag_spread
                .fragment_name()
                .unwrap()
                .name()
                .unwrap()
                .text()
                .as_str()
                .into(),
            span,
        ),
    );
    let directives = get_key_value_node(
        "directives".into(),
        create_directives(frag_spread.directives(), span),
    );
    let fragment_spread = ObjectLit {
        span,
        props: vec![kind, name, directives],
    };

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(fragment_spread)),
    })
}

fn create_inline_fragment(inline_frag: InlineFragment, span: Span) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "InlineFragment".into());
    let directives = get_key_value_node(
        "directives".into(),
        create_directives(inline_frag.directives(), span),
    );

    let mut inline_frag_object = ObjectLit {
        span,
        props: vec![kind, directives],
    };

    if inline_frag.type_condition().is_some() {
        let type_condition = get_key_value_node(
            "typeCondition".into(),
            create_type_condition(inline_frag.type_condition(), span),
        );

        inline_frag_object.props.push(type_condition);
    }

    if inline_frag.selection_set().is_some() {
        let sel_set = get_key_value_node(
            "selectionSet".into(),
            create_selection_set(inline_frag.selection_set(), span),
        );

        inline_frag_object.props.push(sel_set);
    }

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(inline_frag_object)),
    })
}
