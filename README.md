# Gesha

⚠️ **Note:** Gesha is in an experimental stage.  
Features may change, and breaking changes can occur. Use with caution.

## Overview

Gesha is a tool that generates Rust code from an OpenAPI specification.  
It helps developers quickly create Rust applications that adhere to OpenAPI standards.

## Features

- Supports OpenAPI v3.0 Schema Object.
- Accurately expands inline-defined types.
- Correctly interprets `required` and `nullable` attributes.
- Comprehensively covers examples and their tests.

## Installation

You can install the generator using Cargo:

```sh
cargo install gesha
```

Alternatively, you can build from source:

```sh
git clone https://github.com/x7c1/gesha.git
cd gesha
cargo build --release
```

## Usage

To generate Rust code from an OpenAPI specification:

```sh
gesha --schema ./openapi.yaml --output ./generated.rs
```

### Options

- `--schema <FILE>` : Path to the OpenAPI specification file (YAML format).
- `--output <FILE>` : Path to the generated Rust code file.

## Example

Given an OpenAPI specification (`openapi.yaml`):

```yaml
openapi: 3.0.4
info:
  title: Example API
  version: 1.0.0
paths:
  /pets:
    get:
      operationId: getPets
      responses:
        '200':
          description: A list of pets
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Pet'
components:
  schemas:
    Pet:
      type: object
      required:
        - id
        - name
      properties:
        id:
          type: integer
        name:
          type: string
```

Run the generator:

```sh
gesha --schema ./openapi.yaml --output ./generated.rs
```

This will generate Rust code in the `generated.rs` file.

```sh
cat ./generated.rs
```

You will see something like this:

```rust
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        pub name: String,
    }
}
```

## Further Examples

For more detailed examples, check out the following directory:

- [examples/v3_0/src/components/schemas](./examples/v3_0/src/components/schemas)

## Contributing

Contributions are not yet fully accepted, as the project is still under development and far from complete.
We really appreciate your interest in contributing, and please check back later!

## License

This project is licensed under the MIT License.
