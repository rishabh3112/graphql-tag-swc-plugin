use std::collections::HashMap;
use swc_ecma_ast::Expr;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub import_sources: Vec<String>,
    pub gql_tag_identifiers: Vec<String>,
}

pub struct TransformVisitor {
    pub active_gql_tag_identifiers: Vec<String>,
    pub expr_def_map: HashMap<String, Expr>,
    pub config: Config,
}
