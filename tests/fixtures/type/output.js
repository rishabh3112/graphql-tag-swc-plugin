const NAMED_TYPE = {
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
                            "value": "one"
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
        "end": 53,
        "source": {
            "body": "\n  query testQuery($one: String) {\n    getEntity\n  }\n"
        }
    }
};
const NAMED_TYPE_NOT_NULL = {
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
                            "value": "one"
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
        "end": 54,
        "source": {
            "body": "\n  query testQuery($one: String!) {\n    getEntity\n  }\n"
        }
    }
};
const LIST_TYPE = {
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
                            "value": "one"
                        }
                    },
                    "type": {
                        "kind": "ListType",
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
        "end": 55,
        "source": {
            "body": "\n  query testQuery($one: [String]) {\n    getEntity\n  }\n"
        }
    }
};
const LIST_TYPE_NOT_NULL = {
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
                            "value": "one"
                        }
                    },
                    "type": {
                        "kind": "ListType",
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
        "end": 56,
        "source": {
            "body": "\n  query testQuery($one: [String!]) {\n    getEntity\n  }\n"
        }
    }
};
