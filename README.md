# graphql-tag-swc-plugin

SWC plugin alternative to `babel-plugin-graphql-tag`

## Installation

```
npm install -D graphql-tag-swc-plugin
```

## Usage

### SWC

Add following to swc config (below with default values):

```
jsc: {
  experimental: {
    plugins: [
      ["graphql-tag-swc-plugin",
        {
          importSources: ["@apollo/client", "graphql-tag"],
          gqlTagIdentifiers: ["gql"]
        },
      ],
    ],
  },
},
```

### Next js

Add following to next config (below with default values):

```
experimental: {
  swcPlugins: [
    ["graphql-tag-swc-plugin",
      {
        importSources: ["@apollo/client", "graphql-tag"],
        gqlTagIdentifiers: ["gql"]
      },
    ],
  ],
},
```

## Configuration

Following props are accepted by plugin currently, inline with babel alternative:

1.  `importSources`:
    > default: `["@apollo/client", "graphql-tag"]`

import paths from where gql tag is imported for which plugin should compile graphql

2. `gqlTagIdentifiers`:
   > default: `["gql"]`

Identifier by which graphql-tag will be imported.
Example: Here `import { gql } from "@apollo/client`, `"gql"` is the identifier.

## Contribution

All contributions are welcome!
For any bug / feature request please create a Pull request with fixture for the same whenever possible.

## Authors

- @rishabh3112 - Rishabh Chawla

## License

MIT
