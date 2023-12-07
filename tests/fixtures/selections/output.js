const NORMAL_SELECTION = {
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
        "end": 67,
        "source": {
            "body": "\n  query testQuery {\n    getEntity {\n      id\n      name\n    }\n  }\n"
        }
    }
};
const INLINE_FRAGMENT_SELECTION = {
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
                                    "kind": "InlineFragment",
                                    "directives": [],
                                    "typeCondition": {
                                        "kind": "NamedType",
                                        "name": {
                                            "kind": "Name",
                                            "value": "User"
                                        }
                                    },
                                    "selectionSet": {
                                        "kind": "SelectionSet",
                                        "selections": [
                                            {
                                                "kind": "Field",
                                                "name": {
                                                    "kind": "Name",
                                                    "value": "name"
                                                },
                                                "arguments": [],
                                                "directives": []
                                            },
                                            {
                                                "kind": "Field",
                                                "name": {
                                                    "kind": "Name",
                                                    "value": "address"
                                                },
                                                "arguments": [],
                                                "directives": []
                                            }
                                        ]
                                    }
                                },
                                {
                                    "kind": "InlineFragment",
                                    "directives": [],
                                    "typeCondition": {
                                        "kind": "NamedType",
                                        "name": {
                                            "kind": "Name",
                                            "value": "Baby"
                                        }
                                    },
                                    "selectionSet": {
                                        "kind": "SelectionSet",
                                        "selections": [
                                            {
                                                "kind": "Field",
                                                "name": {
                                                    "kind": "Name",
                                                    "value": "name"
                                                },
                                                "arguments": [],
                                                "directives": []
                                            },
                                            {
                                                "kind": "Field",
                                                "name": {
                                                    "kind": "Name",
                                                    "value": "parentAddress"
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
        "end": 176,
        "source": {
            "body": "\n  query testQuery {\n    getEntity {\n      id\n      ... on User {\n        name\n        address\n      }\n      ... on Baby {\n        name\n        parentAddress\n      }\n    }\n  }\n"
        }
    }
};
const USER_FRAGMENT = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "FragmentDefinition",
            "name": {
                "kind": "Name",
                "value": "UserFragment"
            },
            "directives": [],
            "typeCondition": {
                "kind": "NamedType",
                "name": {
                    "kind": "Name",
                    "value": "User"
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
                    },
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "address"
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
        "end": 67,
        "source": {
            "body": "\n  fragment UserFragment on User {\n    id\n    name\n    address\n  }\n"
        }
    }
};
const BABY_FRAGMENT = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "FragmentDefinition",
            "name": {
                "kind": "Name",
                "value": "BabyFragment"
            },
            "directives": [],
            "typeCondition": {
                "kind": "NamedType",
                "name": {
                    "kind": "Name",
                    "value": "Baby"
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
                    },
                    {
                        "kind": "Field",
                        "name": {
                            "kind": "Name",
                            "value": "parentAddress"
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
        "end": 73,
        "source": {
            "body": "\n  fragment BabyFragment on Baby {\n    id\n    name\n    parentAddress\n  }\n"
        }
    }
};
const FRAGMENT_SPREAD_SELECTION = {
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
                                    "kind": "FragmentSpread",
                                    "name": {
                                        "kind": "Name",
                                        "value": "UserFragment"
                                    },
                                    "directives": []
                                },
                                {
                                    "kind": "FragmentSpread",
                                    "name": {
                                        "kind": "Name",
                                        "value": "BabyFragment"
                                    },
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
        "end": 91,
        "source": {
            "body": "\n  query testQuery {\n    getEntity {\n      ...UserFragment\n      ...BabyFragment\n    }\n  }\n"
        }
    }
};
