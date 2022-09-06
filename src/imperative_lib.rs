// Assumptions:
// - No gql imports using `require()`
// - Not removing duplicate declaration
// - No Directives are used
// - All declarations are defined as variable declaration
// - tag name is every where => gql - no need to test import statements for alias
// - Discussion on loc object
// - Need to decide time frame for the work to be done in this PR

use apollo_parser::{
    ast::{Definition, Document, FragmentDefinition, InlineFragment, Selection, SelectionSet},
    Parser,
};

use swc_common::Span;
use swc_core::{
    ast::*,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    testing_transform::test,
    visit::{as_folder, FoldWith, VisitMut},
};

pub struct TransformVisitor;

fn get_key_value_prop(key: String, value: Expr) -> Box<Prop> {
    Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Str(key.into()),
        value: Box::new(value),
    }))
}

fn get_string_literal_expr(str: String) -> Expr {
    Expr::Lit(Lit::Str(str.into()))
}

fn add_to_object_props(mut object: ObjectLit, value: Box<Prop>) -> ObjectLit {
    object.props.push(PropOrSpread::Prop(value));
    object
}

fn build_name(name_str: String, span: Span) -> ObjectLit {
    let mut name = ObjectLit {
        span,
        props: vec![],
    };

    name = add_to_object_props(name, get_key_value_prop("kind".into(), "name".into()));
    name = add_to_object_props(name, get_key_value_prop("value".into(), name_str.into()));

    name
}

fn build_selection_set(selection_set: SelectionSet, span: Span) -> ObjectLit {
    let mut selection_set_object = ObjectLit {
        span,
        props: vec![],
    };

    selection_set_object = add_to_object_props(
        selection_set_object,
        get_key_value_prop(
            "kind".into(),
            get_string_literal_expr("SelectionSet".into()),
        ),
    );

    let mut selections_array = ArrayLit {
        span,
        elems: vec![],
    };

    for selection in selection_set.selections() {
        let mut selection_object = ObjectLit {
            span,
            props: vec![],
        };

        match selection {
            Selection::Field(field) => {
                selection_object = add_to_object_props(
                    selection_object,
                    get_key_value_prop("kind".into(), "Field".into()),
                );

                let name = build_name(field.name().unwrap().text().as_str().into(), span);

                selection_object = add_to_object_props(
                    selection_object,
                    get_key_value_prop("name".into(), Expr::Object(name)),
                );

                if field.arguments().is_some() {
                    todo!();
                }

                if field.directives().is_some() {
                    todo!();
                }

                if field.selection_set().is_some() {
                    selection_object = add_to_object_props(
                        selection_object,
                        get_key_value_prop(
                            "selectionSet".into(),
                            Expr::Object(build_selection_set(field.selection_set().unwrap(), span)),
                        ),
                    )
                }

                selections_array.elems.push(Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Object(selection_object)),
                }));
            }
            Selection::FragmentSpread(_) => todo!(),
            Selection::InlineFragment(inline_fragment) => {
                let inline_fragment_object =
                    build_fragment_inline_fragment(inline_fragment, span, "InlineFragment".into());
                selections_array.elems.push(Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Object(inline_fragment_object)),
                }));
            }
        }
    }

    selection_set_object = add_to_object_props(
        selection_set_object,
        get_key_value_prop("selections".into(), Expr::Array(selections_array)),
    );

    selection_set_object
}

fn build_fragment_declaration(frag_def: FragmentDefinition, span: Span, kind: String) -> ObjectLit {
    let mut defination = ObjectLit {
        span,
        props: vec![],
    };

    defination = add_to_object_props(
        defination,
        get_key_value_prop("kind".into(), get_string_literal_expr(kind.into())),
    );

    // defination.name
    let name = build_name(
        frag_def
            .fragment_name()
            .unwrap()
            .name()
            .unwrap()
            .text()
            .as_str()
            .into(),
        span,
    );

    defination = add_to_object_props(
        defination,
        get_key_value_prop("name".into(), Expr::Object(name)),
    );

    // defination.typeCondition
    if frag_def.type_condition().is_some() {
        let mut type_condition = ObjectLit {
            span,
            props: vec![],
        };

        type_condition = add_to_object_props(
            type_condition,
            get_key_value_prop("kind".into(), get_string_literal_expr("NamedType".into())),
        );

        let mut type_condition_name = ObjectLit {
            span,
            props: vec![],
        };

        type_condition_name = add_to_object_props(
            type_condition_name,
            get_key_value_prop("kind".into(), "Name".into()),
        );

        type_condition_name = add_to_object_props(
            type_condition_name,
            get_key_value_prop(
                "value".into(),
                frag_def
                    .type_condition()
                    .unwrap()
                    .named_type()
                    .unwrap()
                    .name()
                    .unwrap()
                    .text()
                    .as_str()
                    .into(),
            ),
        );

        type_condition = add_to_object_props(
            type_condition,
            get_key_value_prop("name".into(), Expr::Object(type_condition_name)),
        );

        defination = add_to_object_props(
            defination,
            get_key_value_prop("typeCondition".into(), Expr::Object(type_condition)),
        );
    }

    if frag_def.directives().is_some() {
        // DIRECTIVE_TODO: Skipping as discussion
        println!("Hello");
    } else {
        defination = add_to_object_props(
            defination,
            get_key_value_prop(
                "directives".into(),
                Expr::Array(ArrayLit {
                    span,
                    elems: vec![],
                }),
            ),
        );
    }

    if frag_def.selection_set().is_some() {
        defination = add_to_object_props(
            defination,
            get_key_value_prop(
                "selectionSet".into(),
                Expr::Object(build_selection_set(frag_def.selection_set().unwrap(), span)),
            ),
        );
    }

    defination
}

fn build_fragment_inline_fragment(
    inline_frag: InlineFragment,
    span: Span,
    kind: String,
) -> ObjectLit {
    let mut defination = ObjectLit {
        span,
        props: vec![],
    };

    defination = add_to_object_props(
        defination,
        get_key_value_prop("kind".into(), get_string_literal_expr(kind.into())),
    );

    // defination.typeCondition
    if inline_frag.type_condition().is_some() {
        let mut type_condition = ObjectLit {
            span,
            props: vec![],
        };

        type_condition = add_to_object_props(
            type_condition,
            get_key_value_prop("kind".into(), get_string_literal_expr("NamedType".into())),
        );

        let mut type_condition_name = ObjectLit {
            span,
            props: vec![],
        };

        type_condition_name = add_to_object_props(
            type_condition_name,
            get_key_value_prop("kind".into(), "Name".into()),
        );

        type_condition_name = add_to_object_props(
            type_condition_name,
            get_key_value_prop(
                "value".into(),
                inline_frag
                    .type_condition()
                    .unwrap()
                    .named_type()
                    .unwrap()
                    .name()
                    .unwrap()
                    .text()
                    .as_str()
                    .into(),
            ),
        );

        type_condition = add_to_object_props(
            type_condition,
            get_key_value_prop("name".into(), Expr::Object(type_condition_name)),
        );

        defination = add_to_object_props(
            defination,
            get_key_value_prop("typeCondition".into(), Expr::Object(type_condition)),
        );
    }

    if inline_frag.directives().is_some() {
        // DIRECTIVE_TODO: Skipping as discussion
        println!("Hello");
    } else {
        defination = add_to_object_props(
            defination,
            get_key_value_prop(
                "directives".into(),
                Expr::Array(ArrayLit {
                    span,
                    elems: vec![],
                }),
            ),
        );
    }

    if inline_frag.selection_set().is_some() {
        defination = add_to_object_props(
            defination,
            get_key_value_prop(
                "selectionSet".into(),
                Expr::Object(build_selection_set(
                    inline_frag.selection_set().unwrap(),
                    span,
                )),
            ),
        );
    }

    defination
}

fn parse_gql_string(body: String, span: Span) -> ObjectLit {
    let parser = Parser::new(&body);
    let ast = parser.parse();
    assert_eq!(0, ast.errors().len());

    let doc = ast.document();

    let mut swc_ast = ObjectLit {
        span,
        props: vec![],
    };

    // Add kind: "Document" KeyValueProp
    swc_ast = add_to_object_props(
        swc_ast,
        get_key_value_prop("kind".into(), get_string_literal_expr("Document".into())),
    );

    // Add "definations": [] prop
    let mut definations = vec![];

    // TODO: convert doc to KeyProp representation
    for def in doc.definitions() {
        // TODO: handle all Defination types
        if let Definition::FragmentDefinition(frag_def) = def {
            let defination =
                build_fragment_declaration(frag_def, span, "FragmentDefinition".into());
            definations.push(Some(ExprOrSpread {
                expr: Box::new(Expr::Object(defination)),
                spread: None,
            }));
        }
    }

    swc_ast = add_to_object_props(
        swc_ast,
        get_key_value_prop(
            "definitions".into(),
            Expr::Array(ArrayLit {
                span,
                elems: definations,
            }),
        ),
    );

    swc_ast
}

impl VisitMut for TransformVisitor {
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        let decls = &mut node.decls;
        for mut decl in decls {
            if let Some(initial) = &mut decl.init {
                if let Some(tag_tpl) = initial.as_mut_tagged_tpl() {
                    if let Some(tag) = tag_tpl.tag.as_mut_ident() {
                        if &tag.sym != "gql" {
                            return;
                        }

                        if tag_tpl.tpl.quasis.len() == 0 {
                            return;
                        }

                        let template = &mut tag_tpl.tpl;
                        let quasi = &mut template.quasis[0];
                        let data = &mut quasi.raw;
                        let gql_body = data.to_string();

                        // TODO: parse gql and insert it here
                        let gql_swc_ast = parse_gql_string(gql_body.clone(), tag_tpl.span);

                        decl.init = Some(Box::new(Expr::Object(gql_swc_ast)));
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    valid,
    // Input codes
    r#"const a = gql`
      fragment barFragment on Foo {
        field1
        field2
      }
    `"#,
    // Output codes after transformed with plugin
    r#"const a = "apple""#
);
