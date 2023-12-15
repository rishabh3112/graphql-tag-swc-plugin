import { gql as lol } from "@apollo/client";
import baseGql from "graphql-tag";
import { gql } from "@lol/client";
// should not compile
const NEGATIVE_CASE = gql`
  query testQuery {
    getEntity
  }
`;
// should  compile
const POSITIVE_CASE_1 = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "OperationDefinition",
            "name": {
                "kind": "Name",
                "value": "testQuery"
            },
            "directives": [],
            "variableDefinitions": [],
            "operation": "query",
            "selectionSet": {
                "kind": "SelectionSet",
                "selections": [
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "getEntity"
                        },
                        "arguments": [],
                        "directives": []
                    }
                ]
            }
        }
    ],
    "loc": {
        "start": 0,
        "end": 39,
        "source": {
            "body": "\n  query testQuery {\n    getEntity\n  }\n"
        }
    }
};
// should compile
const POSITIVE_CASE_2 = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "OperationDefinition",
            "name": {
                "kind": "Name",
                "value": "testQuery"
            },
            "directives": [],
            "variableDefinitions": [],
            "operation": "query",
            "selectionSet": {
                "kind": "SelectionSet",
                "selections": [
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "getEntity"
                        },
                        "arguments": [],
                        "directives": []
                    }
                ]
            }
        }
    ],
    "loc": {
        "start": 0,
        "end": 39,
        "source": {
            "body": "\n  query testQuery {\n    getEntity\n  }\n"
        }
    }
};
