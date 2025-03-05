// libs
use serde::Deserialize;
use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::{visit_mut_pass, VisitMutWith};
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind,
    plugin_transform,
    proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
};

// structs
use graphql_tag::structs::{GraphQLTagConfig, TransformVisitor};
use unique_identifier::UniqueIdentifierVisitor;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    import_sources: Option<Vec<String>>,
    gql_tag_identifiers: Option<Vec<String>>,
    strip: Option<bool>,
}

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let mut file_path: String = String::new();
    if let Some(name) = data.get_context(&TransformPluginMetadataContextKind::Filename) {
        file_path = name;
    }

    let mut program = program;
    let mut unique_visitor = UniqueIdentifierVisitor::new();

    program.visit_mut_with(&mut visit_mut_pass(&mut unique_visitor));

    let unique_fn_name = if unique_visitor.count > 0 {
        format!("{}{}", unique_visitor.identifier, unique_visitor.count)
    } else {
        unique_visitor.identifier
    };

    let default_config = GraphQLTagConfig {
        import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
        gql_tag_identifiers: vec!["gql".to_string()],
        strip: false,
        file_path: String::new(),
        unique_fn_name: unique_fn_name.clone(),
        unique_fn_used: false,
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
                    strip: config.strip.unwrap_or(false),
                    file_path,
                    unique_fn_name,
                    unique_fn_used: false,
                },
                Err(_) => {
                    println!("Got invalid config for graphql-tag-swc-plugin, using default config instead");
                    default_config
                }
            }
        }
        None => default_config,
    };

    program.visit_mut_with(&mut visit_mut_pass(TransformVisitor::new(
        config,
        PluginCommentsProxy,
    )));

    program
}
