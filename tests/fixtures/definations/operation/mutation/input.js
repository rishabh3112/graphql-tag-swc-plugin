import { gql } from "@apollo/client";

const MUTATION = gql`
  mutation testMutation($entity: String) {
    updateEntity(entity: $entity)
  }
`;
