// Simple directive
const SIMPLE_DIRECTIVE = {
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
                                    "kind": "Field",
                                    "name": {
                                        "kind": "Name",
                                        "value": "id"
                                    },
                                    "arguments": [],
                                    "directives": [
                                        {
                                            "kind": "Directive",
                                            "name": {
                                                "kind": "Name",
                                                "value": "directive"
                                            }
                                        }
                                    ]
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
        "end": 67,
        "source": {
            "body": "\n  query testQuery {\n    getEntity {\n      id @directive\n    }\n  }\n"
        }
    }
};
// directives with arguments
const ARGUMENTS_DIRECTIVE = {
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
                                    "directives": [
                                        {
                                            "kind": "Directive",
                                            "name": {
                                                "kind": "Name",
                                                "value": "directive"
                                            },
                                            "arguments": [
                                                {
                                                    "kind": "Argument",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "arg"
                                                    },
                                                    "value": {
                                                        "kind": "StringValue",
                                                        "value": "argVal"
                                                    }
                                                }
                                            ]
                                        }
                                    ]
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
        "end": 93,
        "source": {
            "body": '\n  query testQuery {\n    getEntity {\n      id\n      name @directive(arg: "argVal")\n    }\n  }\n'
        }
    }
};
// directives with selection set
const DIRECTIVES_SELECTION_SET = {
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
                                        "value": "user"
                                    },
                                    "arguments": [],
                                    "directives": [
                                        {
                                            "kind": "Directive",
                                            "name": {
                                                "kind": "Name",
                                                "value": "directive"
                                            },
                                            "arguments": [
                                                {
                                                    "kind": "Argument",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "arg"
                                                    },
                                                    "value": {
                                                        "kind": "StringValue",
                                                        "value": "argVal"
                                                    }
                                                }
                                            ]
                                        }
                                    ],
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
                            ]
                        }
                    }
                ]
            }
        }
    ],
    "loc": {
        "start": 0,
        "end": 127,
        "source": {
            "body": '\n  query testQuery {\n    getEntity {\n      id\n      user @directive(arg: "argVal") {\n        id\n        name\n      }\n    }\n  }\n'
        }
    }
};
