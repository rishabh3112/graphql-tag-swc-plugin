// libs
use apollo_parser::ast::FragmentDefinition;
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{
        directive::create_directives, name::create_name, selection_set::create_selection_set,
        types::create_type_condition,
    },
    utils::get_key_value_node,
};

pub fn create_fragment_definition(definition: FragmentDefinition, span: Span) -> Box<Expr> {
    let kind = get_key_value_node("kind".into(), "FragmentDefinition".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(
            definition
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
        create_directives(definition.directives(), span),
    );

    let mut frag_def = ObjectLit {
        span,
        props: vec![kind, name, directives],
    };

    if definition.type_condition().is_some() {
        let type_condition = get_key_value_node(
            "typeCondition".into(),
            create_type_condition(definition.type_condition(), span),
        );

        frag_def.props.push(type_condition);
    }

    if definition.selection_set().is_some() {
        let selection_set = get_key_value_node(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        frag_def.props.push(selection_set);
    }

    Box::new(Expr::Object(frag_def))
}
