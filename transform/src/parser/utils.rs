use apollo_parser::ast::OperationType;
use swc_ecma_ast::*;

pub fn get_key_value_node(key: String, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Str(key.into()),
        value: Box::new(value),
    })))
}

pub fn get_operation_token(operation_type: Option<OperationType>) -> String {
    let opr_token = operation_type.unwrap();

    if opr_token.query_token().is_some() {
        return opr_token.query_token().unwrap().text().into();
    }

    if opr_token.mutation_token().is_some() {
        return opr_token.mutation_token().unwrap().text().into();
    }

    if opr_token.subscription_token().is_some() {
        return opr_token.subscription_token().unwrap().text().into();
    }

    "query".into()
}
