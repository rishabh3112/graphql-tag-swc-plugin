// built-ins
use std::collections::HashMap;

// libs
use miette::{Diagnostic, NamedSource, SourceSpan};
use swc_core::common::comments::Comments;
use swc_core::ecma::ast::Expr;
use thiserror::Error;

pub struct GraphQLTagConfig {
    pub import_sources: Vec<String>,
    pub gql_tag_identifiers: Vec<String>,
    pub strip: bool,
    pub file_path: String,
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

#[derive(Error, Debug, Diagnostic)]
#[error("{}", self.ty)]
#[diagnostic(code("GraphQL Error"))]
pub struct PrettyError {
    pub ty: String,
    #[source_code]
    pub src: NamedSource,
    #[label("{}", self.ty)]
    pub span: SourceSpan,
}
