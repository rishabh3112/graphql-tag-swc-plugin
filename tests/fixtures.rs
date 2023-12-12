// built-ins
use std::path::PathBuf;

// libs
use swc_core::ecma::transforms::testing::{test_fixture, FixtureTestConfig};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_visit::as_folder;
use testing::fixture;

// structs
use graphql_tag::structs::{GraphQLTagConfig, TransformVisitor};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixtures/**/input.js")]
fn graphql_tag_fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let output = dir.join("output.js");

    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor::new(GraphQLTagConfig {
                import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
                gql_tag_identifiers: vec!["gql".to_string()],
                strip: false,
            }))
        },
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            sourcemap: false,
        },
    );
}
