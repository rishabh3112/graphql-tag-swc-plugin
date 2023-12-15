import { gql } from "@apollo/client";
const MUTATION = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "OperationDefinition",
            "name": {
                "kind": "Name",
                "value": "testMutation"
            },
            "directives": [],
            "variableDefinitions": [
                {
                    "kind": "VariableDefinition",
                    "directives": [],
                    "variable": {
                        "kind": "Variable",
                        "name": {
                            "kind": "Name",
                            "value": "entity"
                        }
                    },
                    "type": {
                        "kind": "NamedType",
                        "name": {
                            "kind": "Name",
                            "value": "String"
                        }
                    }
                }
            ],
            "operation": "mutation",
            "selectionSet": {
                "kind": "SelectionSet",
                "selections": [
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "updateEntity"
                        },
                        "arguments": [
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "entity"
                                },
                                "value": {
                                    "kind": "Variable",
                                    "name": {
                                        "kind": "Name",
                                        "value": "entity"
                                    }
                                }
                            }
                        ],
                        "directives": []
                    }
                ]
            }
        }
    ],
    "loc": {
        "start": 0,
        "end": 67,
        "source": {
            "body": "mutation testMutation($entity:String){updateEntity(entity:$entity)}"
        }
    }
};
