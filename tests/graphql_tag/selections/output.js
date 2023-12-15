import { gql } from "@apollo/client";
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
        "end": 35,
        "source": {
            "body": "query testQuery{getEntity{id name}}"
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
        "end": 84,
        "source": {
            "body": "query testQuery{getEntity{id...on User{name address}...on Baby{name parentAddress}}}"
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
        "end": 46,
        "source": {
            "body": "fragment UserFragment on User{id name address}"
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
        "end": 52,
        "source": {
            "body": "fragment BabyFragment on Baby{id name parentAddress}"
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
        "end": 58,
        "source": {
            "body": "query testQuery{getEntity{...UserFragment...BabyFragment}}"
        }
    }
};
