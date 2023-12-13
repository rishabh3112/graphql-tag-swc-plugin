// built-ins
use std::{fs, path::PathBuf};

// libs
use swc_core::ecma::transforms::testing::{test_fixture, FixtureTestConfig, Tester};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_visit::as_folder;
use testing::{fixture, NormalizedOutput};

// structs
use graphql_tag::structs::{GraphQLTagConfig, TransformVisitor};
use unique_identifier::UniqueIdentifierVisitor;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/graphql_tag/**/input.js")]
fn graphql_tag_fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let output = dir.join("output.js");
    let strip_output = dir.join("strip-output.js");

    // With strip false
    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor::new(
                GraphQLTagConfig {
                    import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
                    gql_tag_identifiers: vec!["gql".to_string()],
                    strip: false,
                    unique_fn_name: "unique".into(),
                    unique_fn_used: false,
                    file_path: input.to_str().unwrap().into(),
                },
                _tr.comments.clone(),
            ))
        },
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            sourcemap: false,
        },
    );

    // With strip true
    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor::new(
                GraphQLTagConfig {
                    import_sources: vec!["@apollo/client".to_string(), "graphql-tag".into()],
                    gql_tag_identifiers: vec!["gql".to_string()],
                    strip: true,
                    unique_fn_name: "unique".into(),
                    unique_fn_used: false,
                    file_path: input.to_str().unwrap().into(),
                },
                _tr.comments.clone(),
            ))
        },
        &input,
        &strip_output,
        FixtureTestConfig {
            allow_error: true,
            sourcemap: false,
        },
    );
}

#[fixture("tests/unique_identifier/**/input.js")]
fn unique_identifier_fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let output = dir.join("output.txt");

    let expected: Result<String, std::io::Error> = fs::read_to_string(&output);
    let expected = expected.unwrap_or_default();

    let input_text = fs::read_to_string(input).unwrap();
    let actual_output = Tester::run(|tester| {
        let mut unique_visitor = UniqueIdentifierVisitor::new();
        tester.apply_transform(
            as_folder(&mut unique_visitor),
            "noop.js",
            syntax(),
            &input_text,
        )?;

        Ok(format!(
            "identifier: {}\ncount: {}",
            unique_visitor.identifier, unique_visitor.count
        ))
    });

    if actual_output != expected {
        NormalizedOutput::from(actual_output)
            .compare_to_file(output)
            .unwrap();
    }
}
