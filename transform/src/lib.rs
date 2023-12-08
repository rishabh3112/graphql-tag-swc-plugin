mod parser;
pub mod structs;

use structs::{Config, TransformVisitor};

use std::collections::HashMap;

use regex::Regex;
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

impl TransformVisitor {
    pub fn new(config: Config) -> Self {
        Self {
            active_gql_tag_identifiers: vec![],
            skip_tag: true,
            expr_def_map: HashMap::new(),
            config,
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_program(&mut self, node: &mut Program) {
        node.visit_mut_children_with(self);
        self.skip_tag = true;
        self.active_gql_tag_identifiers.clear()
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
                    if self.config.gql_tag_identifiers.contains(&local_name_string) {
                        gql_tag_local_name = Some(local_name_string);
                    }
                    break;
                }

                ImportSpecifier::Namespace(_) => break,
            }
        }

        if gql_tag_local_name.is_none() {
            return;
        }

        self.skip_tag = !self
            .config
            .import_sources
            .contains(&node.src.value.to_string());

        self.active_gql_tag_identifiers
            .push(gql_tag_local_name.unwrap());
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        if let Some(tag_tpl) = node.as_mut_tagged_tpl() {
            if let Some(tag) = tag_tpl.tag.as_mut_ident() {
                if self.skip_tag
                    || !self
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
                let no_gql_line_regex = Regex::new(r#"(^\$\{.*\}$)"#).unwrap();

                let mut data: String = "".into();
                for quasi in &mut template.quasis {
                    data += &quasi.raw;
                }

                let gql_raw_string = data.to_string();

                let gql_text = gql_raw_string
                    .lines()
                    .filter(|line| !no_gql_line_regex.is_match(line.trim()))
                    .map(|line| String::from(line) + "\n")
                    .collect();

                let gql_swc_ast_result = parser::parse_graphql_tag(
                    gql_text,
                    tag_tpl.span,
                    expressions,
                    &mut self.expr_def_map,
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
