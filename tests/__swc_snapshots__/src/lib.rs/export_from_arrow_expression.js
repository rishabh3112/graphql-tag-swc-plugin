const a = ()=>{
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
                "variableDefinitions": [],
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
                            "arguments": [],
                            "directives": [],
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
            "end": 138,
            "source": {
                "body": "\n        query testQuery {\n            testQueryName {\n                a\n                b\n                c\n            }\n        }\n    \n"
            }
        }
    };
};
