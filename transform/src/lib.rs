use std::collections::HashMap;

use apollo_parser::{
    ast::{
        Argument, Arguments, AstChildren, BooleanValue, DefaultValue, Definition, Directive,
        Directives, Document, EnumValue, Field, FloatValue, FragmentDefinition, FragmentSpread,
        InlineFragment, IntValue, ListType, ListValue, NamedType, NonNullType, NullValue,
        ObjectField, ObjectValue, OperationDefinition, OperationType, Selection, SelectionSet,
        StringValue, Type, TypeCondition, Value, Variable, VariableDefinition, VariableDefinitions,
    },
    Parser,
};

use swc_common::Span;
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

use regex::Regex;

pub struct TransformVisitor {
    expr_def_map: HashMap<String, Expr>,
}

impl TransformVisitor {
    pub fn new() -> Self {
        Self {
            expr_def_map: HashMap::new(),
        }
    }
}

fn parse_gql_string(
    body: String,
    span: Span,
    expressions: Vec<Box<Expr>>,
    expr_def_map: &mut HashMap<String, Expr>,
) -> Expr {
    let parser = Parser::new(&body);
    let ast = parser.parse();
    assert_eq!(0, ast.errors().len());

    let doc = ast.document();

    create_document(doc, span, body, expressions, expr_def_map)
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        if let Some(tag_tpl) = node.as_mut_tagged_tpl() {
            if let Some(tag) = tag_tpl.tag.as_mut_ident() {
                if &tag.sym != "gql" {
                    return;
                }
                if tag_tpl.tpl.quasis.len() == 0 {
                    return;
                }

                let template = &mut tag_tpl.tpl;
                let mut data: String = "".into();

                for quasi in &mut template.quasis {
                    data += &quasi.raw;
                }

                let gql_raw_string = data.to_string();
                let no_gql_line_regex = Regex::new(r#"(^\$\{.*\}$)"#).unwrap();

                let gql_text = gql_raw_string
                    .lines()
                    .filter(|line| !no_gql_line_regex.is_match(line.trim()))
                    .map(|line| String::from(line) + "\n")
                    .collect();

                let expressions = template.exprs.clone();
                // TODO: parse gql and insert it here
                let gql_swc_ast =
                    parse_gql_string(gql_text, tag_tpl.span, expressions, &mut self.expr_def_map);

                *node = gql_swc_ast;
            }
        } else {
            node.visit_mut_children_with(self)
        }
    }
}

fn create_key_value_prop(key: String, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Str(key.into()),
        value: Box::new(value),
    })))
}

fn create_document(
    document: Document,
    span: Span,
    body: String,
    _expressions: Vec<Box<Expr>>,
    _expr_def_map: &mut HashMap<String, Expr>,
) -> Expr {
    let kind = create_key_value_prop("kind".into(), "Document".into());
    let definitions_expr = create_definitions(document.definitions(), span);

    let mut all_expressions = vec![];

    for _expression in _expressions.clone() {
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

    let definitions = create_key_value_prop(
        "definitions".into(),
        if _expressions.len() > 0 {
            concat_call_expr
        } else {
            definitions_expr
        },
    );

    let loc = create_key_value_prop("loc".into(), create_loc(body, span));

    let document_object_lit = ObjectLit {
        span,
        props: vec![kind, definitions, loc],
    };

    Expr::Object(document_object_lit)
}

fn create_loc(body: String, span: Span) -> Expr {
    let start = create_key_value_prop("start".into(), Expr::Lit(Lit::Num(Number::from(0))));
    let end = create_key_value_prop("end".into(), Expr::Lit(Lit::Num(Number::from(body.len()))));
    let source_body = create_key_value_prop("body".into(), Expr::Lit(Lit::Str(body.into())));
    let source_expr = Expr::Object(ObjectLit {
        span,
        props: vec![source_body],
    });
    let source = create_key_value_prop("source".into(), source_expr);

    Expr::Object(ObjectLit {
        span,
        props: vec![start, end, source],
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

fn create_operation_definition(definition: OperationDefinition, span: Span) -> Box<Expr> {
    let kind = create_key_value_prop("kind".into(), "OperationDefinition".into());
    let name = create_key_value_prop(
        "name".into(),
        create_name(definition.name().unwrap().text().as_str().into(), span),
    );
    let variable_definitions = create_key_value_prop(
        "variableDefinitions".into(),
        create_variable_definitions(definition.variable_definitions(), span),
    );
    let directives = create_key_value_prop(
        "directives".into(),
        create_directives(definition.directives(), span),
    );

    let operation = create_key_value_prop(
        "operation".into(),
        get_operation_token(definition.operation_type()).into(),
    );

    let mut opr_def = ObjectLit {
        span,
        props: vec![kind, name, directives, variable_definitions, operation],
    };

    if definition.selection_set().is_some() {
        let selection_set = create_key_value_prop(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        opr_def.props.push(selection_set);
    }

    Box::new(Expr::Object(opr_def))
}

fn create_fragment_definition(definition: FragmentDefinition, span: Span) -> Box<Expr> {
    let kind = create_key_value_prop("kind".into(), "FragmentDefinition".into());
    let name = create_key_value_prop(
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

    let directives = create_key_value_prop(
        "directives".into(),
        create_directives(definition.directives(), span),
    );

    let mut frag_def = ObjectLit {
        span,
        props: vec![kind, name, directives],
    };

    if definition.type_condition().is_some() {
        let type_condition = create_key_value_prop(
            "typeCondition".into(),
            create_type_condition(definition.type_condition(), span),
        );

        frag_def.props.push(type_condition);
    }

    if definition.selection_set().is_some() {
        let selection_set = create_key_value_prop(
            "selectionSet".into(),
            create_selection_set(definition.selection_set(), span),
        );

        frag_def.props.push(selection_set);
    }

    Box::new(Expr::Object(frag_def))
}

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
    let kind = create_key_value_prop("kind".into(), "VariableDefinition".into());
    let directives = create_key_value_prop(
        "directives".into(),
        create_directives(variable_def.directives(), span),
    );
    let variable = create_key_value_prop(
        "variable".into(),
        create_variable_value(variable_def.variable().unwrap(), span),
    );

    let mut var_def = ObjectLit {
        span,
        props: vec![kind, directives, variable],
    };

    if variable_def.ty().is_some() {
        let type_def = create_key_value_prop("type".into(), create_type(variable_def.ty(), span));
        var_def.props.push(type_def);
    }

    if variable_def.default_value().is_some() {
        let default_value = create_key_value_prop(
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

fn create_type(type_def: Option<Type>, span: Span) -> Expr {
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
    let kind = create_key_value_prop("kind".into(), "NonNullType".into());

    let type_expr: Expr;
    if not_null_type.named_type().is_some() {
        type_expr = create_named_type(not_null_type.named_type().unwrap(), span);
    } else {
        type_expr = create_list_type(not_null_type.list_type().unwrap(), span);
    }

    let type_def = create_key_value_prop("type".into(), type_expr);

    let type_object = ObjectLit {
        span,
        props: vec![kind, type_def],
    };

    Expr::Object(type_object)
}

fn create_named_type(named_type: NamedType, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "NamedType".into());
    let name = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "ListType".into());

    let mut type_object = ObjectLit {
        span,
        props: vec![kind],
    };

    if list_type.ty().is_some() {
        let type_def = create_key_value_prop("type".into(), create_type(list_type.ty(), span));
        type_object.props.push(type_def);
    }

    Expr::Object(type_object)
}

fn create_name(name: String, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "Name".into());
    let value = create_key_value_prop("value".into(), name.into());
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

    let kind = create_key_value_prop("kind".into(), "NamedType".into());
    let name = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "Directive".into());
    let name = create_key_value_prop(
        "name".into(),
        create_name(directive.name().unwrap().text().as_str().into(), span),
    );

    let mut directive_object = ObjectLit {
        span,
        props: vec![kind, name],
    };

    if directive.arguments().is_some() {
        let arguments_prop = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "SelectionSet".into());
    let selections = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "Field".into());
    let name = create_key_value_prop(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let arguments = create_key_value_prop(
        "arguments".into(),
        create_arguments(field.arguments(), span),
    );
    let directives = create_key_value_prop(
        "directives".into(),
        create_directives(field.directives(), span),
    );

    let mut sel: ObjectLit = ObjectLit {
        span,
        props: vec![kind, name, arguments, directives],
    };

    if field.selection_set().is_some() {
        let sel_set = create_key_value_prop(
            "selectionSet".into(),
            create_selection_set(field.selection_set(), span),
        );

        sel.props.push(sel_set);
    }

    if field.alias().is_some() {
        let alias = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "FragmentSpread".into());
    let name = create_key_value_prop(
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
    let directives = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "InlineFragment".into());
    let directives = create_key_value_prop(
        "directives".into(),
        create_directives(inline_frag.directives(), span),
    );

    let mut inline_frag_object = ObjectLit {
        span,
        props: vec![kind, directives],
    };

    if inline_frag.type_condition().is_some() {
        let type_condition = create_key_value_prop(
            "typeCondition".into(),
            create_type_condition(inline_frag.type_condition(), span),
        );

        inline_frag_object.props.push(type_condition);
    }

    if inline_frag.selection_set().is_some() {
        let sel_set = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "Argument".into());
    let name = create_key_value_prop(
        "name".into(),
        create_name(argument.name().unwrap().text().as_str().into(), span),
    );
    let value = create_key_value_prop("value".into(), create_value(argument.value(), span));
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

fn create_variable_value(var: Variable, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "Variable".into());
    let name = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "StringValue".into());

    let mut string_token = str.to_string();
    let re = Regex::new(r#""(?P<str>[^"]*)""#).unwrap();
    for cap in re.captures_iter(string_token.clone().as_str()) {
        string_token = cap[0].to_string();
    }
    let value = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "FloatValue".into());
    let value = create_key_value_prop("value".into(), float.float_token().unwrap().text().into());

    let float_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(float_val)
}

fn create_int_value(int: IntValue, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "IntValue".into());
    let value = create_key_value_prop("value".into(), int.int_token().unwrap().text().into());

    let int_val = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(int_val)
}

fn create_boolean_value(bool: BooleanValue, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "BooleanValue".into());
    let value = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "EnumValue".into());
    let value = create_key_value_prop("value".into(), enum_val.text().as_str().into());

    let enum_val_obj = ObjectLit {
        span,
        props: vec![kind, value],
    };

    Expr::Object(enum_val_obj)
}

fn create_null_value(_null: NullValue, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "NullValue".into());

    let null_val = ObjectLit {
        span,
        props: vec![kind],
    };

    Expr::Object(null_val)
}

fn create_list_value(list: ListValue, span: Span) -> Expr {
    let kind = create_key_value_prop("kind".into(), "ListValue".into());
    let values = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "ObjectValue".into());
    let fields = create_key_value_prop(
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
    let kind = create_key_value_prop("kind".into(), "ObjectField".into());
    let name = create_key_value_prop(
        "name".into(),
        create_name(field.name().unwrap().text().as_str().into(), span),
    );
    let value = create_key_value_prop("value".into(), create_value(field.value(), span));

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

fn get_operation_token(operation_type: Option<OperationType>) -> String {
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
