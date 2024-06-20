import gql from 'graphql-tag';
const foo = {
    "kind": "Document",
    "definitions": [
        {
            "kind": "OperationDefinition",
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
                            "value": "foo"
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
        "end": 5,
        "source": {
            "body": "{foo}"
        }
    }
};
