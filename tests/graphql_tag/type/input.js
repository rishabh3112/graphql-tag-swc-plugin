import { gql } from "@apollo/client";

const NAMED_TYPE = gql`
  query testQuery($one: String = "apple") {
    getEntity
  }
`;

const NAMED_TYPE_NOT_NULL = gql`
  query testQuery($one: String!) {
    getEntity
  }
`;

const LIST_TYPE = gql`
  query testQuery($one: [String]) {
    getEntity
  }
`;

const LIST_TYPE_NOT_NULL = gql`
  query testQuery($one: [String!]) {
    getEntity
  }
`;
