import { gql } from "@apollo/client";

const FRAGMENT = gql`
  fragment TestFragment on Entity {
    id
    name
  }
`;
