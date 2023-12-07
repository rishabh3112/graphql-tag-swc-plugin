const FRAGMENT = {
  "kind": "Document",
  "definitions": [
      {
          "kind": "FragmentDefinition",
          "name": {
              "kind": "Name",
              "value": "TestFragment"
          },
          "directives": [],
          "typeCondition": {
              "kind": "NamedType",
              "name": {
                  "kind": "Name",
                  "value": "Entity"
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
                  }
              ]
          }
      }
  ],
  "loc": {
      "start": 0,
      "end": 57,
      "source": {
          "body": "\n  fragment TestFragment on Entity {\n    id\n    name\n  }\n"
      }
  }
};