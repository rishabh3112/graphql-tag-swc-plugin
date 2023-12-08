import { gql as lol } from "@apollo/client";
import baseGql from "graphql-tag";
import { gql } from "@lol/client";

// should not compile
const NEGATIVE_CASE = gql`
  query testQuery {
    getEntity
  }
`;

// should  compile
const POSITIVE_CASE_1 = lol`
  query testQuery {
    getEntity
  }
`;

// should compile
const POSITIVE_CASE_2 = baseGql`
  query testQuery {
    getEntity
  }
`;
