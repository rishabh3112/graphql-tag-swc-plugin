// built-ins
use std::collections::HashMap;

// libs
use apollo_parser::ast::Document;
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{nodes::definitions::create_definitions, utils::get_key_value_node};

fn create_loc(body: String, span: Span) -> Expr {
    let start = get_key_value_node("start".into(), Expr::Lit(Lit::Num(Number::from(0))));
    let end = get_key_value_node("end".into(), Expr::Lit(Lit::Num(Number::from(body.len()))));
    let source_body = get_key_value_node("body".into(), Expr::Lit(Lit::Str(body.into())));
    let source_expr = Expr::Object(ObjectLit {
        span,
        props: vec![source_body],
    });
    let source = get_key_value_node("source".into(), source_expr);

    Expr::Object(ObjectLit {
        span,
        props: vec![start, end, source],
    })
}

pub fn create_document(
    document: Document,
    span: Span,
    body: String,
    expressions: Vec<Box<Expr>>,
    _expr_def_map: &mut HashMap<String, Expr>,
) -> Expr {
    let kind = get_key_value_node("kind".into(), "Document".into());
    let definitions_expr = create_definitions(document.definitions(), span);

    let mut all_expressions = vec![];

    for _expression in expressions.clone() {
        let member_expr_for_definitions = MemberExpr {
            span,
            obj: _expression,
            prop: MemberProp::Ident(Ident::new("definitions".into(), span)),
        };

        all_expressions.push(ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Member(member_expr_for_definitions)),
        });
    }

    let concat_call_expr = Expr::Call(CallExpr {
        span,
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span,
            obj: Box::new(definitions_expr.clone()),
            prop: MemberProp::Ident(Ident::new("concat".into(), span)),
        }))),
        args: all_expressions,
        type_args: None,
    });

    let definitions = get_key_value_node(
        "definitions".into(),
        if expressions.len() > 0 {
            concat_call_expr
        } else {
            definitions_expr
        },
    );

    let loc = get_key_value_node("loc".into(), create_loc(body, span));

    let document_object_lit = ObjectLit {
        span,
        props: vec![kind, definitions, loc],
    };

    Expr::Object(document_object_lit)
}
