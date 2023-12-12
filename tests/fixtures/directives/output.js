import { gql } from "@apollo/client";
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
        "end": 40,
        "source": {
            "body": "query testQuery{getEntity{id@directive}}"
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
        "end": 59,
        "source": {
            "body": 'query testQuery{getEntity{id name@directive(arg:"argVal")}}'
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
        "end": 68,
        "source": {
            "body": 'query testQuery{getEntity{id user@directive(arg:"argVal"){id name}}}'
        }
    }
};
