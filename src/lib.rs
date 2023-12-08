use graphql_tag::structs::{Config, TransformVisitor};

use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_ast::Program;
use swc_ecma_visit::{as_folder, FoldWith};

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = match data.get_transform_plugin_config() {
        Some(config_str) => serde_json::from_str::<Config>(&config_str)
            .expect("Invalid options config passed for graphql-tag-swc-plugin"),
        None => Config {
            import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
            gql_tag_identifiers: vec!["gql".to_string()],
        },
    };

    program.fold_with(&mut as_folder(TransformVisitor::new(config)))
}
