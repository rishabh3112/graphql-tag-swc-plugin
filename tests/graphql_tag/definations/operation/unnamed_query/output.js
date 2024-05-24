import gql from 'graphql-tag';
const foo = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "OperationDefinition",
            "directives": [],
            "variableDefinitions": [
                {
                    "kind": "VariableDefinition",
                    "directives": [],
                    "variable": {
                        "kind": "Variable",
                        "name": {
                            "kind": "Name",
                            "value": "foo"
                        }
                    },
                    "type": {
                        "kind": "NonNullType",
                        "type": {
                            "kind": "NamedType",
                            "name": {
                                "kind": "Name",
                                "value": "String"
                            }
                        }
                    }
                }
            ],
            "operation": "query",
            "selectionSet": {
                "kind": "SelectionSet",
                "selections": [
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "foo1"
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
        "end": 27,
        "source": {
            "body": "query($foo: String!) {foo1}"
        }
    }
};