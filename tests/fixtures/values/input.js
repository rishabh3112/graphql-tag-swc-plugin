const ALL_BASIC_VALUES = gql`
  query testQuery($var: String) {
    getEntity(
      var: $var
      string: "Hello"
      float: 2.2
      int: 1
      bool: true
      nullVal: null
      enumVal: TestEnumValue
      objectValue: { a: "a", b: 1, c: { a: 1.1, b: [1, "a"] } }
      list: ["string", 1.1, 1, true, null, TestEnumValue, { a: "a" }]
    )
  }
`;
