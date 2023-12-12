import { gql } from "@apollo/client";
const ALL_BASIC_VALUES = {
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
                            "value": "var"
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
                        "arguments": [
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "var"
                                },
                                "value": {
                                    "kind": "Variable",
                                    "name": {
                                        "kind": "Name",
                                        "value": "var"
                                    }
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "string"
                                },
                                "value": {
                                    "kind": "StringValue",
                                    "value": "Hello"
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "float"
                                },
                                "value": {
                                    "kind": "FloatValue",
                                    "value": "2.2"
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "int"
                                },
                                "value": {
                                    "kind": "IntValue",
                                    "value": "1"
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "bool"
                                },
                                "value": {
                                    "kind": "BooleanValue",
                                    "value": true
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "nullVal"
                                },
                                "value": {
                                    "kind": "NullValue"
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "enumVal"
                                },
                                "value": {
                                    "kind": "EnumValue",
                                    "value": "TestEnumValue"
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "objectValue"
                                },
                                "value": {
                                    "kind": "ObjectValue",
                                    "fields": [
                                        {
                                            "kind": "ObjectField",
                                            "name": {
                                                "kind": "Name",
                                                "value": "a"
                                            },
                                            "value": {
                                                "kind": "StringValue",
                                                "value": "a"
                                            }
                                        },
                                        {
                                            "kind": "ObjectField",
                                            "name": {
                                                "kind": "Name",
                                                "value": "b"
                                            },
                                            "value": {
                                                "kind": "IntValue",
                                                "value": "1"
                                            }
                                        },
                                        {
                                            "kind": "ObjectField",
                                            "name": {
                                                "kind": "Name",
                                                "value": "c"
                                            },
                                            "value": {
                                                "kind": "ObjectValue",
                                                "fields": [
                                                    {
                                                        "kind": "ObjectField",
                                                        "name": {
                                                            "kind": "Name",
                                                            "value": "a"
                                                        },
                                                        "value": {
                                                            "kind": "FloatValue",
                                                            "value": "1.1"
                                                        }
                                                    },
                                                    {
                                                        "kind": "ObjectField",
                                                        "name": {
                                                            "kind": "Name",
                                                            "value": "b"
                                                        },
                                                        "value": {
                                                            "kind": "ListValue",
                                                            "values": [
                                                                {
                                                                    "kind": "IntValue",
                                                                    "value": "1"
                                                                },
                                                                {
                                                                    "kind": "StringValue",
                                                                    "value": "a"
                                                                }
                                                            ]
                                                        }
                                                    }
                                                ]
                                            }
                                        }
                                    ]
                                }
                            },
                            {
                                "kind": "Argument",
                                "name": {
                                    "kind": "Name",
                                    "value": "list"
                                },
                                "value": {
                                    "kind": "ListValue",
                                    "values": [
                                        {
                                            "kind": "StringValue",
                                            "value": "string"
                                        },
                                        {
                                            "kind": "FloatValue",
                                            "value": "1.1"
                                        },
                                        {
                                            "kind": "IntValue",
                                            "value": "1"
                                        },
                                        {
                                            "kind": "BooleanValue",
                                            "value": true
                                        },
                                        {
                                            "kind": "NullValue"
                                        },
                                        {
                                            "kind": "EnumValue",
                                            "value": "TestEnumValue"
                                        },
                                        {
                                            "kind": "ObjectValue",
                                            "fields": [
                                                {
                                                    "kind": "ObjectField",
                                                    "name": {
                                                        "kind": "Name",
                                                        "value": "a"
                                                    },
                                                    "value": {
                                                        "kind": "StringValue",
                                                        "value": "a"
                                                    }
                                                }
                                            ]
                                        }
                                    ]
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
        "end": 222,
        "source": {
            "body": 'query testQuery($var:String){getEntity(var:$var string:"Hello" float:2.2 int:1 bool:true nullVal:null enumVal:TestEnumValue objectValue:{a:"a",b:1,c:{a:1.1,b:[1,"a"]}}list:["string",1.1,1,true,null,TestEnumValue,{a:"a"}])}'
        }
    }
};
