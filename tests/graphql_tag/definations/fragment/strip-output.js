import { gql } from "@apollo/client";
const FRAGMENT = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "FragmentDefinition",
            "name": {
                "kind": "Name",
                "value": "TestFragment"
            },
            "directives": [],
            "typeCondition": {
                "kind": "NamedType",
                "name": {
                    "kind": "Name",
                    "value": "Entity"
                }
            },
            "selectionSet": {
                "kind": "SelectionSet",
                "selections": [
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "id"
                        },
                        "arguments": [],
                        "directives": []
                    },
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "name"
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
        "end": 40,
        "source": {
            "body": "fragment TestFragment on Entity{id name}"
        }
    }
};
