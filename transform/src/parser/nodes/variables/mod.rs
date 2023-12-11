// libs
use apollo_parser::ast::{DefaultValue, Variable, VariableDefinition, VariableDefinitions};
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{
        directive::create_directives, name::create_name, types::create_type_node,
        value::create_value,
    },
    utils::get_key_value_node,
};

pub fn create_variable_definitions(variable_defs: Option<VariableDefinitions>, span: Span) -> Expr {
    if variable_defs.is_none() {
        return Expr::Array(ArrayLit {
            span,
            elems: vec![],
        });
    }

    let mut all_variable_definitions = vec![];
    for variable_def in variable_defs.unwrap().variable_definitions() {
        all_variable_definitions.push(create_variable_definition(variable_def, span))
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_variable_definitions,
    })
}

fn create_variable_definition(
    variable_def: VariableDefinition,
    span: Span,
) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "VariableDefinition".into());
    let directives = get_key_value_node(
        "directives".into(),
        create_directives(variable_def.directives(), span),
    );
    let variable = get_key_value_node(
        "variable".into(),
        create_variable_value(variable_def.variable().unwrap(), span),
    );

    let mut var_def = ObjectLit {
        span,
        props: vec![kind, directives, variable],
    };

    if variable_def.ty().is_some() {
        let type_def = get_key_value_node("type".into(), create_type_node(variable_def.ty(), span));
        var_def.props.push(type_def);
    }

    if variable_def.default_value().is_some() {
        let default_value = get_key_value_node(
            "defaultValue".into(),
            create_default_value(variable_def.default_value(), span),
        );
        var_def.props.push(default_value);
    }

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(var_def)),
    })
}

fn create_default_value(default_value: Option<DefaultValue>, span: Span) -> Expr {
    if default_value.is_none() {
        return Expr::Object(ObjectLit {
            span,
            props: vec![],
        });
    }

    let unwrapped_default_value = default_value.unwrap();
    create_value(unwrapped_default_value.value(), span)
}

pub fn create_variable_value(var: Variable, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "Variable".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(var.name().unwrap().text().as_str().into(), span),
    );
    let variable = ObjectLit {
        span,
        props: vec![kind, name],
    };

    Expr::Object(variable)
}
