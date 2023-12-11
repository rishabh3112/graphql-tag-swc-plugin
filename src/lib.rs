use graphql_tag::structs::{GraphQLTagConfig, TransformVisitor};

use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_ast::Program;
use swc_ecma_visit::{as_folder, FoldWith};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub import_sources: Option<Vec<String>>,
    pub gql_tag_identifiers: Option<Vec<String>>,
}

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let default_config = GraphQLTagConfig {
        import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
        gql_tag_identifiers: vec!["gql".to_string()],
    };

    let config = match data.get_transform_plugin_config() {
        Some(config_str) => {
            let plugin_config = serde_json::from_str::<Config>(&config_str);
            match plugin_config {
                Ok(config) => GraphQLTagConfig {
                    import_sources: config
                        .import_sources
                        .unwrap_or(default_config.import_sources),
                    gql_tag_identifiers: config
                        .gql_tag_identifiers
                        .unwrap_or(default_config.gql_tag_identifiers),
                },
                Err(_) => {
                    println!("Got invalid config for graphql-tag-swc-plugin, using default config instead");
                    default_config
                }
            }
        }
        None => default_config,
    };

    program.fold_with(&mut as_folder(TransformVisitor::new(config)))
}
