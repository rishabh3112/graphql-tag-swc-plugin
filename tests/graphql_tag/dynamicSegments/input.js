import { gql } from "@apollo/client";

const NAME = "LOL";

const DYNAMIC_FRAGMENT = gql`
  fragment name on ${NAME} {
    id
  }
`;

const QUERY_WITH_DYNAMIC_SEGMENT = gql`
  query testQuery {
    getEntity {
      ... on ${NAME}{
        lol
      }
    }
  }

  ${DYNAMIC_FRAGMENT}
`;

const QUERY_WITH_DYNAMIC_FRAGMENT_SPREAD = gql`
  query testQuery {
    getEntity {
      ...${NAME}
    }
  }

  ${DYNAMIC_FRAGMENT}
`;

const STATIC_QUERY = gql`
  query testQuery {
    getEntity {
      ... on LOL {
        lol
      }
    }
  }

  ${DYNAMIC_FRAGMENT}
`;
