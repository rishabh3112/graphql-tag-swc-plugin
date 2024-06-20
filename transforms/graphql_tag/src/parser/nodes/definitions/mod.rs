// libs
use apollo_parser::cst::{CstChildren, Definition};
use swc_common::Span;
use swc_ecma_ast::*;

// modules
mod fragment;
mod operation;

// helpers
use fragment::create_fragment_definition;
use operation::create_operation_definition;

pub fn create_definition(
    definition: Definition,
    span: Span,
    assert_definition_name: bool,
) -> Option<ExprOrSpread> {
    if assert_definition_name {
        definition.name().expect("GraphQL query must have name.");
    }
    let def_expr = match definition {
        Definition::FragmentDefinition(frag_def) => create_fragment_definition(frag_def, span),
        Definition::OperationDefinition(operation_def) => {
            create_operation_definition(operation_def, span)
        }
        Definition::DirectiveDefinition(_) => todo!(),
        Definition::SchemaDefinition(_) => todo!(),
        Definition::ScalarTypeDefinition(_) => todo!(),
        Definition::ObjectTypeDefinition(_) => todo!(),
        Definition::InterfaceTypeDefinition(_) => todo!(),
        Definition::UnionTypeDefinition(_) => todo!(),
        Definition::EnumTypeDefinition(_) => todo!(),
        Definition::InputObjectTypeDefinition(_) => todo!(),
        Definition::SchemaExtension(_) => todo!(),
        Definition::ScalarTypeExtension(_) => todo!(),
        Definition::ObjectTypeExtension(_) => todo!(),
        Definition::InterfaceTypeExtension(_) => todo!(),
        Definition::UnionTypeExtension(_) => todo!(),
        Definition::EnumTypeExtension(_) => todo!(),
        Definition::InputObjectTypeExtension(_) => todo!(),
    };

    Some(ExprOrSpread {
        spread: None,
        expr: def_expr,
    })
}

pub fn create_definitions(definitions: CstChildren<Definition>, span: Span) -> Expr {
    let mut all_definitions = vec![];
    let is_multiple_definitions = definitions.clone().count() > 1;

    for def in definitions {
        all_definitions.push(create_definition(def, span, is_multiple_definitions));
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_definitions,
    })
}
