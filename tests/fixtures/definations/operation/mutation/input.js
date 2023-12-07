const MUTATION = gql`
  mutation testMutation($entity: String) {
    updateEntity(entity: $entity)
  }
`;
