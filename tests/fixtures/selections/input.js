import { gql } from "@apollo/client";

const NORMAL_SELECTION = gql`
  query testQuery {
    getEntity {
      id
      name
    }
  }
`;

const INLINE_FRAGMENT_SELECTION = gql`
  query testQuery {
    getEntity {
      id
      ... on User {
        name
        address
      }
      ... on Baby {
        name
        parentAddress
      }
    }
  }
`;

const USER_FRAGMENT = gql`
  fragment UserFragment on User {
    id
    name
    address
  }
`;

const BABY_FRAGMENT = gql`
  fragment BabyFragment on Baby {
    id
    name
    parentAddress
  }
`;

const FRAGMENT_SPREAD_SELECTION = gql`
  query testQuery {
    getEntity {
      ...UserFragment
      ...BabyFragment
    }
  }
`;
