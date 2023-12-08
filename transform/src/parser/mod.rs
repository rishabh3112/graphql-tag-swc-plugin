use std::collections::HashMap;

use apollo_parser::{
    ast::{
        Argument, Arguments, AstChildren, BooleanValue, DefaultValue, Definition, Directive,
        Directives, Document, EnumValue, Field, FloatValue, FragmentDefinition, FragmentSpread,
        InlineFragment, IntValue, ListType, ListValue, NamedType, NonNullType, NullValue,
        ObjectField, ObjectValue, OperationDefinition, Selection, SelectionSet, StringValue, Type,
        TypeCondition, Value, Variable, VariableDefinition, VariableDefinitions,
    },
    SyntaxTree,
};

use swc_common::Span;
use swc_ecma_ast::*;

use regex::Regex;

mod utils;

fn create_variable_definitions(variable_defs: Option<VariableDefinitions>, span: Span) -> Expr {
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
    let kind = utils::get_key_value_node("kind".into(), "VariableDefinition".into());
    let directives = utils::get_key_value_node(
        "directives".into(),
        create_directives(variable_def.directives(), span),
    );
    let variable = utils::get_key_value_node(
        "variable".into(),
        create_variable_value(variable_def.variable().unwrap(), span),
    );

    let mut var_def = ObjectLit {
        span,
        props: vec![kind, directives, variable],
    };

    if variable_def.ty().is_some() {
        let type_def =
            utils::get_key_value_node("type".into(), create_type_node(variable_def.ty(), span));
        var_def.props.push(type_def);
    }

    if variable_def.default_value().is_some() {
        let default_value = utils::get_key_value_node(
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

fn create_type_node(type_def: Option<Type>, span: Span) -> Expr {
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

fn create_not_null_type(not_null_type: NonNullType, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "NonNullType".into());

    let type_expr: Expr;
    if not_null_type.named_type().is_some() {
        type_expr = create_named_type(not_null_type.named_type().unwrap(), span);
    } else {
        type_expr = create_list_type(not_null_type.list_type().unwrap(), span);
    }

    let type_def = utils::get_key_value_node("type".into(), type_expr);

    let type_object = ObjectLit {
        span,
        props: vec![kind, type_def],
    };

    Expr::Object(type_object)
}

fn create_named_type(named_type: NamedType, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "NamedType".into());
    let name = utils::get_key_value_node(
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
    let kind = utils::get_key_value_node("kind".into(), "ListType".into());

    let mut type_object = ObjectLit {
        span,
        props: vec![kind],
    };

    if list_type.ty().is_some() {
        let type_def =
            utils::get_key_value_node("type".into(), create_type_node(list_type.ty(), span));
        type_object.props.push(type_def);
    }

    Expr::Object(type_object)
}

fn create_name(name: String, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "Name".into());
    let value = utils::get_key_value_node("value".into(), name.into());
    let name = ObjectLit {
        span,
        props: vec![kind, value],
    };
    Expr::Object(name)
}

fn create_type_condition(type_condition: Option<TypeCondition>, span: Span) -> Expr {
    if type_condition.is_none() {
        let type_cond = ObjectLit {
            span,
            props: vec![],
        };
        return Expr::Object(type_cond);
    }

    let unwrapped_type_condition = type_condition.unwrap();

    let kind = utils::get_key_value_node("kind".into(), "NamedType".into());
    let name = utils::get_key_value_node(
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

fn create_directive(directive: Directive, span: Span) -> Option<ExprOrSpread> {
    let kind = utils::get_key_value_node("kind".into(), "Directive".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(directive.name().unwrap().text().as_str().into(), span),
    );

    let mut directive_object = ObjectLit {
        span,
        props: vec![kind, name],
    };

    if directive.arguments().is_some() {
        let arguments_prop = utils::get_key_value_node(
            "arguments".into(),
            create_arguments(directive.arguments(), span),
        );

        directive_object.props.push(arguments_prop)
    }

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(directive_object)),
    })
}

fn create_directives(directives: Option<Directives>, span: Span) -> Expr {
    if directives.is_none() {
        return Expr::Array(ArrayLit {
            span,
            elems: vec![],
        });
    }

    Expr::Array(ArrayLit {
        span,
        elems: directives
            .unwrap()
            .directives()
            .into_iter()
            .map(|directive| create_directive(directive, span))
            .collect(),
    })
}

fn create_selection_set(selection_set: Option<SelectionSet>, span: Span) -> Expr {
    if selection_set.is_none() {
        let sel_set = ObjectLit {
            span,
            props: vec![],
        };
        return Expr::Object(sel_set);
    }
    let unwrapped_selection_set = selection_set.unwrap();
    let kind = utils::get_key_value_node("kind".into(), "SelectionSet".into());
    let selections = utils::get_key_value_node(
        "selections".into(),
        create_selections(unwrapped_selection_set.selections(), span),
    );

    let sel_set = ObjectLit {
        span,
        props: vec![kind, selections],
    };
    Expr::Object(sel_set)
}

fn create_selections(selections: AstChildren<Selection>, span: Span) -> Expr {
    let mut all_selections = vec![];
    for selection in selections {
        all_selections.push(create_selection(selection, span));
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_selections,
    })
}

fn create_selection(selection: Selection, span: Span) -> Option<ExprOrSpread> {
    match selection {
        Selection::Field(field) => create_field(field, span),
        Selection::FragmentSpread(frag_spread) => create_fragment_spread(frag_spread, span),
        Selection::InlineFragment(inline_frag) => create_inline_fragment(inline_frag, span),
    }
}

fn create_field(field: Field, span: Span) -> Option<ExprOrSpread> {
    let kind = utils::get_key_value_node("kind".into(), "Field".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let arguments = utils::get_key_value_node(
        "arguments".into(),
        create_arguments(field.arguments(), span),
    );
    let directives = utils::get_key_value_node(
        "directives".into(),
        create_directives(field.directives(), span),
    );

    let mut sel: ObjectLit = ObjectLit {
        span,
        props: vec![kind, name, arguments, directives],
    };

    if field.selection_set().is_some() {
        let sel_set = utils::get_key_value_node(
            "selectionSet".into(),
            create_selection_set(field.selection_set(), span),
        );

        sel.props.push(sel_set);
    }

    if field.alias().is_some() {
        let alias = utils::get_key_value_node(
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
    let kind = utils::get_key_value_node("kind".into(), "FragmentSpread".into());
    let name = utils::get_key_value_node(
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
    let directives = utils::get_key_value_node(
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
    let kind = utils::get_key_value_node("kind".into(), "InlineFragment".into());
    let directives = utils::get_key_value_node(
        "directives".into(),
        create_directives(inline_frag.directives(), span),
    );

    let mut inline_frag_object = ObjectLit {
        span,
        props: vec![kind, directives],
    };

    if inline_frag.type_condition().is_some() {
        let type_condition = utils::get_key_value_node(
            "typeCondition".into(),
            create_type_condition(inline_frag.type_condition(), span),
        );

        inline_frag_object.props.push(type_condition);
    }

    if inline_frag.selection_set().is_some() {
        let sel_set = utils::get_key_value_node(
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

fn create_arguments(arguments: Option<Arguments>, span: Span) -> Expr {
    if arguments.is_none() {
        let args = ArrayLit {
            span,
            elems: vec![],
        };
        return Expr::Array(args);
    }

    let unwrapped_arguments = arguments.unwrap().arguments();
    let mut all_arguments = vec![];
    for argument in unwrapped_arguments {
        all_arguments.push(create_argument(argument, span));
    }
    return Expr::Array(ArrayLit {
        span,
        elems: all_arguments,
    });
}

fn create_argument(argument: Argument, span: Span) -> Option<ExprOrSpread> {
    let kind = utils::get_key_value_node("kind".into(), "Argument".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(argument.name().unwrap().text().as_str().into(), span),
    );
    let value = utils::get_key_value_node("value".into(), create_value(argument.value(), span));
    let arg = ObjectLit {
        span,
        props: vec![kind, name, value],
    };
    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(arg)),
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

fn create_variable_value(var: Variable, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "Variable".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(var.name().unwrap().text().as_str().into(), span),
    );
    let variable = ObjectLit {
        span,
        props: vec![kind, name],
    };

    Expr::Object(variable)
}

fn create_string_value(str: StringValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "StringValue".into());

    let mut string_token = str.to_string();
    let re = Regex::new(r#""(?P<str>[^"]*)""#).unwrap();
    for cap in re.captures_iter(string_token.clone().as_str()) {
        string_token = cap[0].to_string();
    }
    let value = utils::get_key_value_node(
        "value".into(),
        string_token[1..string_token.len() - 1].into(),
    );

    let str_value = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(str_value)
}

fn create_float_value(float: FloatValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "FloatValue".into());
    let value =
        utils::get_key_value_node("value".into(), float.float_token().unwrap().text().into());

    let float_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(float_val)
}

fn create_int_value(int: IntValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "IntValue".into());
    let value = utils::get_key_value_node("value".into(), int.int_token().unwrap().text().into());

    let int_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(int_val)
}

fn create_boolean_value(bool: BooleanValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "BooleanValue".into());
    let value = utils::get_key_value_node(
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
    let kind = utils::get_key_value_node("kind".into(), "EnumValue".into());
    let value = utils::get_key_value_node("value".into(), enum_val.text().as_str().into());

    let enum_val_obj = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(enum_val_obj)
}

fn create_null_value(_null: NullValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "NullValue".into());

    let null_val = ObjectLit {
        span,
        props: vec![kind],
    };

    Expr::Object(null_val)
}

fn create_list_value(list: ListValue, span: Span) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "ListValue".into());
    let values = utils::get_key_value_node(
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
    let kind = utils::get_key_value_node("kind".into(), "ObjectValue".into());
    let fields = utils::get_key_value_node(
        "fields".into(),
        create_object_fields(object.object_fields(), span),
    );

    let object_val = ObjectLit {
        span,
        props: vec![kind, fields],
    };

    Expr::Object(object_val)
}

fn create_object_fields(object_fields: AstChildren<ObjectField>, span: Span) -> Expr {
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
    let kind = utils::get_key_value_node("kind".into(), "ObjectField".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let value = utils::get_key_value_node("value".into(), create_value(field.value(), span));

    let object_field_value = ObjectLit {
        span,
        props: vec![kind, name, value],
    };

    Expr::Object(object_field_value)
}

fn create_list_value_values(values: AstChildren<Value>, span: Span) -> Expr {
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

fn create_value(value: Option<Value>, span: Span) -> Expr {
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

// -------

fn create_operation_definition(definition: OperationDefinition, span: Span) -> Box<Expr> {
    let kind = utils::get_key_value_node("kind".into(), "OperationDefinition".into());
    let name = utils::get_key_value_node(
        "name".into(),
        create_name(definition.name().unwrap().text().as_str().into(), span),
    );
    let variable_definitions = utils::get_key_value_node(
        "variableDefinitions".into(),
        create_variable_definitions(definition.variable_definitions(), span),
    );
    let directives = utils::get_key_value_node(
        "directives".into(),
        create_directives(definition.directives(), span),
    );

    let operation = utils::get_key_value_node(
        "operation".into(),
        utils::get_operation_token(definition.operation_type()).into(),
    );

    let mut opr_def = ObjectLit {
        span,
        props: vec![kind, name, directives, variable_definitions, operation],
    };

    if definition.selection_set().is_some() {
        let selection_set = utils::get_key_value_node(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        opr_def.props.push(selection_set);
    }

    Box::new(Expr::Object(opr_def))
}

fn create_fragment_definition(definition: FragmentDefinition, span: Span) -> Box<Expr> {
    let kind = utils::get_key_value_node("kind".into(), "FragmentDefinition".into());
    let name = utils::get_key_value_node(
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

    let directives = utils::get_key_value_node(
        "directives".into(),
        create_directives(definition.directives(), span),
    );

    let mut frag_def = ObjectLit {
        span,
        props: vec![kind, name, directives],
    };

    if definition.type_condition().is_some() {
        let type_condition = utils::get_key_value_node(
            "typeCondition".into(),
            create_type_condition(definition.type_condition(), span),
        );

        frag_def.props.push(type_condition);
    }

    if definition.selection_set().is_some() {
        let selection_set = utils::get_key_value_node(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        frag_def.props.push(selection_set);
    }

    Box::new(Expr::Object(frag_def))
}

fn create_definition(definition: Definition, span: Span) -> Option<ExprOrSpread> {
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

fn create_definitions(definitions: AstChildren<Definition>, span: Span) -> Expr {
    let mut all_definitions = vec![];
    for def in definitions {
        all_definitions.push(create_definition(def, span));
    }

    Expr::Array(ArrayLit {
        span,
        elems: all_definitions,
    })
}

fn create_loc(body: String, span: Span) -> Expr {
    let start = utils::get_key_value_node("start".into(), Expr::Lit(Lit::Num(Number::from(0))));
    let end =
        utils::get_key_value_node("end".into(), Expr::Lit(Lit::Num(Number::from(body.len()))));
    let source_body = utils::get_key_value_node("body".into(), Expr::Lit(Lit::Str(body.into())));
    let source_expr = Expr::Object(ObjectLit {
        span,
        props: vec![source_body],
    });
    let source = utils::get_key_value_node("source".into(), source_expr);

    Expr::Object(ObjectLit {
        span,
        props: vec![start, end, source],
    })
}

fn create_document(
    document: Document,
    span: Span,
    body: String,
    expressions: Vec<Box<Expr>>,
    _expr_def_map: &mut HashMap<String, Expr>,
) -> Expr {
    let kind = utils::get_key_value_node("kind".into(), "Document".into());
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

    let definitions = utils::get_key_value_node(
        "definitions".into(),
        if expressions.len() > 0 {
            concat_call_expr
        } else {
            definitions_expr
        },
    );

    let loc = utils::get_key_value_node("loc".into(), create_loc(body, span));

    let document_object_lit = ObjectLit {
        span,
        props: vec![kind, definitions, loc],
    };

    Expr::Object(document_object_lit)
}

pub fn parse_graphql_tag(
    body: String,
    span: Span,
    expressions: Vec<Box<Expr>>,
    expr_def_map: &mut HashMap<String, Expr>,
) -> Result<Expr, SyntaxTree> {
    let parser = apollo_parser::Parser::new(&body);
    let ast = parser.parse();

    if ast.errors().len() != 0 {
        Err(ast)
    } else {
        let doc = ast.document();
        Ok(create_document(doc, span, body, expressions, expr_def_map))
    }
}
