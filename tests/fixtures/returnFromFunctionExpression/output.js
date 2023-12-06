const getQuery = function() {
    return {
        "kind": "Document",
        "definitions": [
            {
                "kind": "OperationDefinition",
                "name": {
                    "kind": "Name",
                    "value": "testQuery"
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
                                "value": "a"
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
                                "value": "testQueryName"
                            },
                            "arguments": [
                                {
                                    "kind": "Argument",
                                    "name": {
                                        "kind": "Name",
                                        "value": "a"
                                    },
                                    "value": {
                                        "kind": "Variable",
                                        "name": {
                                            "kind": "Name",
                                            "value": "a"
                                        }
                                    }
                                }
                            ],
                            "directives": [
                                {
                                    "kind": "Directive",
                                    "name": {
                                        "kind": "Name",
                                        "value": "apple"
                                    }
                                }
                            ],
                            "selectionSet": {
                                "kind": "SelectionSet",
                                "selections": [
                                    {
                                        "kind": "Field",
                                        "name": {
                                            "kind": "Name",
                                            "value": "a"
                                        },
                                        "arguments": [],
                                        "directives": []
                                    },
                                    {
                                        "kind": "Field",
                                        "name": {
                                            "kind": "Name",
                                            "value": "b"
                                        },
                                        "arguments": [],
                                        "directives": []
                                    },
                                    {
                                        "kind": "Field",
                                        "name": {
                                            "kind": "Name",
                                            "value": "c"
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
        ],
        "loc": {
            "start": 0,
            "end": 119,
            "source": {
                "body": "\n    query testQuery($a: String!) {\n      testQueryName(a: $a) @apple {\n        a\n        b\n        c\n      }\n    }\n  \n"
            }
        }
    };
};
