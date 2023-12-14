// built-ins
use std::collections::HashMap;

use swc_common::comments::Comments;
// libs
use swc_ecma_ast::Expr;

pub struct GraphQLTagConfig {
    pub import_sources: Vec<String>,
    pub gql_tag_identifiers: Vec<String>,
    pub strip: bool,
    pub unique_fn_name: String,
    pub unique_fn_used: bool,
}

pub struct TransformVisitor<C>
where
    C: Comments,
{
    pub active_gql_tag_identifiers: Vec<String>,
    pub expr_def_map: HashMap<String, Expr>,
    pub config: GraphQLTagConfig,
    pub comments: C,
    pub unique_fn_used: bool,
}
