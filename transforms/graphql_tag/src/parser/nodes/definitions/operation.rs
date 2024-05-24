// libs
use apollo_parser::cst::OperationDefinition;
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{
        directive::create_directives, name::create_name, selection_set::create_selection_set,
        variables::create_variable_definitions,
    },
    utils::{get_key_value_node, get_operation_token},
};

pub fn create_operation_definition(definition: OperationDefinition, span: Span) -> Box<Expr> {
    let kind = get_key_value_node("kind".into(), "OperationDefinition".into());
    
    let variable_definitions = get_key_value_node(
        "variableDefinitions".into(),
        create_variable_definitions(definition.variable_definitions(), span),
    );
    let directives = get_key_value_node(
        "directives".into(),
        create_directives(definition.directives(), span),
    );

    let operation = get_key_value_node(
        "operation".into(),
        get_operation_token(definition.operation_type()).into(),
    );

    let mut opr_def = ObjectLit {
        span,
        props: vec![kind, directives, variable_definitions, operation],
    };

    if definition.name() != None {
        opr_def.props.insert(1,get_key_value_node(
            "name".into(),
            create_name(definition.name().unwrap().text().as_str().into(), span),
        ));
    }

    if definition.selection_set().is_some() {
        let selection_set = get_key_value_node(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        opr_def.props.push(selection_set);
    }

    Box::new(Expr::Object(opr_def))
}
