import { gql } from "@apollo/client";
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
                    },
                    "defaultValue": {
                        "kind": "StringValue",
                        "value": "apple"
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
        "end": 47,
        "source": {
            "body": 'query testQuery($one:String="apple"){getEntity}'
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
        "end": 40,
        "source": {
            "body": "query testQuery($one:String!){getEntity}"
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
        "end": 41,
        "source": {
            "body": "query testQuery($one:[String]){getEntity}"
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
        "end": 42,
        "source": {
            "body": "query testQuery($one:[String!]){getEntity}"
        }
    }
};
