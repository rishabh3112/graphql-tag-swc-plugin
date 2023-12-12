// libs
use apollo_parser::cst::{ListType, NamedType, NonNullType, Type, TypeCondition};
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{nodes::name::create_name, utils::get_key_value_node};

fn create_not_null_type(not_null_type: NonNullType, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "NonNullType".into());

    let type_expr: Expr;
    if not_null_type.named_type().is_some() {
        type_expr = create_named_type(not_null_type.named_type().unwrap(), span);
    } else {
        type_expr = create_list_type(not_null_type.list_type().unwrap(), span);
    }

    let type_def = get_key_value_node("type".into(), type_expr);

    let type_object = ObjectLit {
        span,
        props: vec![kind, type_def],
    };

    Expr::Object(type_object)
}

fn create_named_type(named_type: NamedType, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "NamedType".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(named_type.name().unwrap().text().as_str().into(), span),
    );

    let type_object = ObjectLit {
        span,
        props: vec![kind, name],
    };

    Expr::Object(type_object)
}

fn create_list_type(list_type: ListType, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "ListType".into());

    let mut type_object = ObjectLit {
        span,
        props: vec![kind],
    };

    if list_type.ty().is_some() {
        let type_def = get_key_value_node("type".into(), create_type_node(list_type.ty(), span));
        type_object.props.push(type_def);
    }

    Expr::Object(type_object)
}

pub fn create_type_node(type_def: Option<Type>, span: Span) -> Expr {
    if type_def.is_none() {
        let type_object = ObjectLit {
            span,
            props: vec![],
        };

        return Expr::Object(type_object);
    }
    let unwrapped_type_def = type_def.unwrap();

    match unwrapped_type_def {
        Type::NamedType(named_type) => create_named_type(named_type, span),
        Type::ListType(list_type) => create_list_type(list_type, span),
        Type::NonNullType(not_null_type) => create_not_null_type(not_null_type, span),
    }
}

pub fn create_type_condition(type_condition: Option<TypeCondition>, span: Span) -> Expr {
    if type_condition.is_none() {
        let type_cond = ObjectLit {
            span,
            props: vec![],
        };
        return Expr::Object(type_cond);
    }

    let unwrapped_type_condition = type_condition.unwrap();

    let kind = get_key_value_node("kind".into(), "NamedType".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(
            unwrapped_type_condition
                .named_type()
                .unwrap()
                .name()
                .unwrap()
                .text()
                .as_str()
                .into(),
            span,
        ),
    );

    let type_cond = ObjectLit {
        span,
        props: vec![kind, name],
    };
    Expr::Object(type_cond)
}
