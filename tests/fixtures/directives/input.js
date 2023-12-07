// Simple directive
const SIMPLE_DIRECTIVE = gql`
  query testQuery {
    getEntity {
      id @directive
    }
  }
`;

// directives with arguments
const ARGUMENTS_DIRECTIVE = gql`
  query testQuery {
    getEntity {
      id
      name @directive(arg: "argVal")
    }
  }
`;

// directives with selection set
const DIRECTIVES_SELECTION_SET = gql`
  query testQuery {
    getEntity {
      id
      user @directive(arg: "argVal") {
        id
        name
      }
    }
  }
`;
