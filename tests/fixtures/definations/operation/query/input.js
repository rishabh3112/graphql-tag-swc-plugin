import { gql } from "@apollo/client";

const QUERY = gql`
  query testQuery {
    getEntity {
      id
      name
    }
  }
`;
