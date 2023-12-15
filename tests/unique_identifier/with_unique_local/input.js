import { gql } from "@apollo/client";

const QUERY = gql`
  query testQuery {
    getEntity {
      id
      name
    }
  }
`;

const useTestQuery = () => {
  const unique = () => "unique";
  return useQuery(QUERY);
};
