import { gql } from "@apollo/client";

export default gql`
  query testQuery($a: String!) {
    testQueryName(a: $a) @apple {
      a
      b
      c
    }
  }
`;
