import { gql } from "@apollo/client";

const getQuery = function () {
  return gql`
    query testQuery($a: String!) {
      testQueryName(a: $a) @apple {
        a
        b
        c
      }
    }
  `;
};
