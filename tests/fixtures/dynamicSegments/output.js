import { gql } from "@apollo/client";
const NAME = "LOL";
const DYNAMIC_FRAGMENT = gql`
  fragment name on ${NAME} {
    id
  }
`;
const QUERY_WITH_DYNAMIC_SEGMENT = gql`
  query testQuery {
    getEntity {
      ... on ${NAME}{
        lol
      }
    }
  }

  ${DYNAMIC_FRAGMENT}
`;
const STATIC_QUERY = {
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
                        "directives": [],
                        "selectionSet": {
                            "kind": "SelectionSet",
                            "selections": [
                                {
                                    "kind": "InlineFragment",
                                    "directives": [],
                                    "typeCondition": {
                                        "kind": "NamedType",
                                        "name": {
                                            "kind": "Name",
                                            "value": "LOL"
                                        }
                                    },
                                    "selectionSet": {
                                        "kind": "SelectionSet",
                                        "selections": [
                                            {
                                                "kind": "Field",
                                                "name": {
                                                    "kind": "Name",
                                                    "value": "lol"
                                                },
                                                "arguments": [],
                                                "directives": []
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        }
    ].concat(DYNAMIC_FRAGMENT.definitions),
    "loc": {
        "start": 0,
        "end": 90,
        "source": {
            "body": "\n  query testQuery {\n    getEntity {\n      ... on LOL {\n        lol\n      }\n    }\n  }\n\n  \n"
        }
    }
};
