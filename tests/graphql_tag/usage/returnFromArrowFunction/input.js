import { gql } from "@apollo/client";

const getQuery = () => gql`
  query testQuery($a: String!) {
    testQueryName(a: $a) @apple {
      a
      b
      c
    }
  }
`;
