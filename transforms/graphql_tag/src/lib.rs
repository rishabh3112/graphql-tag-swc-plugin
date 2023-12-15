// built-ins
use std::collections::HashMap;

// libs
use swc_common::comments::Comments;
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

// modules
pub mod parser;
pub mod structs;
mod utils;

// helpers
use parser::utils::strip_ignored_characters;

// structs
use structs::{GraphQLTagConfig, TransformVisitor};
use utils::add_unique_fn_to_program;

impl<C> TransformVisitor<C>
where
    C: Comments,
{
    pub fn new(config: GraphQLTagConfig, comments: C) -> Self {
        Self {
            unique_fn_used: false,
            active_gql_tag_identifiers: vec![],
            expr_def_map: HashMap::new(),
            config,
            comments,
        }
    }
}

impl<C> VisitMut for TransformVisitor<C>
where
    C: Comments,
{
    fn visit_mut_program(&mut self, node: &mut Program) {
        node.visit_mut_children_with(self);
        self.active_gql_tag_identifiers.clear();

        if self.unique_fn_used {
            add_unique_fn_to_program(node, self.config.unique_fn_name.clone())
        }
    }

    fn visit_mut_import_decl(&mut self, node: &mut ImportDecl) {
        let mut gql_tag_local_name = None;
        for import_specifier in &mut node.specifiers {
            match import_specifier {
                ImportSpecifier::Named(specifier) => {
                    let local_name_string = specifier.local.sym.to_string();
                    let mut import_name = local_name_string.clone();

                    match &specifier.imported {
                        Some(import_export_name) => match import_export_name {
                            ModuleExportName::Ident(ident) => {
                                import_name = ident.clone().sym.to_string()
                            }
                            ModuleExportName::Str(_) => {}
                        },
                        None => {}
                    };

                    if self.config.gql_tag_identifiers.contains(&import_name) {
                        gql_tag_local_name = Some(local_name_string);
                        break;
                    }

                    continue;
                }

                ImportSpecifier::Default(specifier) => {
                    let local_name_string = specifier.local.sym.to_string();
                    gql_tag_local_name = Some(local_name_string);
                    break;
                }

                ImportSpecifier::Namespace(_) => break,
            }
        }

        if gql_tag_local_name.is_none() {
            return;
        }

        let valid_import_source = self
            .config
            .import_sources
            .contains(&node.src.value.to_string());

        if valid_import_source {
            self.active_gql_tag_identifiers
                .push(gql_tag_local_name.unwrap());
        }
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        if let Some(tag_tpl) = node.as_mut_tagged_tpl() {
            if let Some(tag) = tag_tpl.tag.as_mut_ident() {
                if !self
                    .active_gql_tag_identifiers
                    .contains(&tag.sym.to_string())
                {
                    return;
                }

                if tag_tpl.tpl.quasis.len() == 0 {
                    return;
                }

                let template = &mut tag_tpl.tpl;
                let expressions = template.exprs.clone();

                let mut data: String = "".into();
                for quasi in &mut template.quasis {
                    data += &quasi.raw;
                }

                let gql_raw_string = data.to_string();

                let gql_text = if self.config.strip {
                    strip_ignored_characters(gql_raw_string)
                } else {
                    gql_raw_string
                };

                let unique_fn_name = self.config.unique_fn_name.clone();
                let gql_swc_ast_result = parser::parse_graphql_tag(
                    gql_text,
                    tag_tpl.span,
                    expressions,
                    &mut self.expr_def_map,
                    unique_fn_name,
                    &mut self.unique_fn_used,
                    &mut self.comments,
                );

                match gql_swc_ast_result {
                    Ok(swc_ast) => *node = swc_ast,
                    Err(gql_ast) => {
                        for error in gql_ast.errors() {
                            println!(
                                "GraphQL Error: At index {}, {} got \"{}\" instead\n",
                                error.index(),
                                error.message(),
                                error.data()
                            )
                        }
                    }
                }
            }
        } else {
            node.visit_mut_children_with(self)
        }
    }
}
