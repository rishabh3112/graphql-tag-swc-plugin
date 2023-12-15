import { gql } from "@apollo/client";
import { unique } from "lol";

const QUERY = gql`
  query testQuery {
    getEntity {
      id
      name
    }
  }
`;

const useTestQuery = () => {
  return useQuery(QUERY);
};
