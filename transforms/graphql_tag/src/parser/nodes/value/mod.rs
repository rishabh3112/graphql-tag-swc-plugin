// libs
use apollo_parser::cst::{
    BooleanValue, CstChildren, EnumValue, FloatValue, IntValue, ListValue, NullValue, ObjectField,
    ObjectValue, StringValue, Value,
};
use regex::Regex;
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{name::create_name, variables::create_variable_value},
    utils::get_key_value_node,
};

pub fn create_value(value: Option<Value>, span: Span) -> Expr {
    assert!(value.is_some());
    let unwrapped_value = value.unwrap();
    match unwrapped_value {
        Value::Variable(var) => create_variable_value(var, span),
        Value::StringValue(str) => create_string_value(str, span),
        Value::FloatValue(float) => create_float_value(float, span),
        Value::IntValue(int) => create_int_value(int, span),
        Value::BooleanValue(bool) => create_boolean_value(bool, span),
        Value::NullValue(null) => create_null_value(null, span),
        Value::EnumValue(enum_val) => create_enum_value(enum_val, span),
        Value::ListValue(list) => create_list_value(list, span),
        Value::ObjectValue(object) => create_object_value(object, span),
    }
}

fn create_string_value(str: StringValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "StringValue".into());

    let mut string_token: String = str.into();
    let re = Regex::new(r#""(?P<str>[^"]*)""#).unwrap();
    for cap in re.captures_iter(string_token.clone().as_str()) {
        string_token = cap[0].to_string();
    }
    let value = get_key_value_node("value".into(), string_token.into());

    let str_value = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(str_value)
}

fn create_float_value(float: FloatValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "FloatValue".into());
    let value = get_key_value_node("value".into(), float.float_token().unwrap().text().into());

    let float_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(float_val)
}

fn create_int_value(int: IntValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "IntValue".into());
    let value = get_key_value_node("value".into(), int.int_token().unwrap().text().into());

    let int_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(int_val)
}

fn create_boolean_value(bool: BooleanValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "BooleanValue".into());
    let value = get_key_value_node(
        "value".into(),
        (|| {
            if bool.true_token().is_some() {
                return Expr::Lit(Lit::Bool(true.into()));
            }
            return Expr::Lit(Lit::Bool(false.into()));
        })(),
    );

    let bool_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(bool_val)
}

fn create_enum_value(enum_val: EnumValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "EnumValue".into());
    let value = get_key_value_node("value".into(), enum_val.text().as_str().into());

    let enum_val_obj = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(enum_val_obj)
}

fn create_null_value(_null: NullValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "NullValue".into());

    let null_val = ObjectLit {
        span,
        props: vec![kind],
    };

    Expr::Object(null_val)
}

fn create_list_value(list: ListValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "ListValue".into());
    let values = get_key_value_node(
        "values".into(),
        create_list_value_values(list.values(), span),
    );

    let list_val = ObjectLit {
        span,
        props: vec![kind, values],
    };

    Expr::Object(list_val)
}

fn create_object_value(object: ObjectValue, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "ObjectValue".into());
    let fields = get_key_value_node(
        "fields".into(),
        create_object_fields(object.object_fields(), span),
    );

    let object_val = ObjectLit {
        span,
        props: vec![kind, fields],
    };

    Expr::Object(object_val)
}

fn create_object_fields(object_fields: CstChildren<ObjectField>, span: Span) -> Expr {
    let mut all_fields = vec![];
    for field in object_fields.into_iter() {
        all_fields.push(Some(ExprOrSpread {
            spread: None,
            expr: Box::new(create_object_field(field, span)),
        }));
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_fields,
    })
}

fn create_object_field(field: ObjectField, span: Span) -> Expr {
    let kind = get_key_value_node("kind".into(), "ObjectField".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let value = get_key_value_node("value".into(), create_value(field.value(), span));

    let object_field_value = ObjectLit {
        span,
        props: vec![kind, name, value],
    };

    Expr::Object(object_field_value)
}

fn create_list_value_values(values: CstChildren<Value>, span: Span) -> Expr {
    let mut all_values = vec![];
    for value in values.into_iter() {
        all_values.push(Some(ExprOrSpread {
            spread: None,
            expr: Box::new(create_value(Some(value), span)),
        }))
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_values,
    })
}
