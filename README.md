# cluster-api-rs

**cluster-api-rs** is an auto-generated Rust API types derived from [Cluster API](https://github.com/kubernetes-sigs/cluster-api) CRDs. It provides a Rust interface for interacting with Cluster API resources, allowing to build new CAPI integrations in Rust.

Definitions are generated using [`kopium`](https://github.com/kube-rs/kopium).

Library can be used as a dependency:

1. Add it to your `Cargo.toml`:
```toml
[dependencies]
cluster-api-rs = "1.8.5"
```

## Features

- Automatically generated from upstream Kubernetes Cluster API CRDs on each new release.
- Type-safe Rust bindings for Cluster API resources.

## Contributing

We welcome contributions to enhance the project! To contribute:

1. Fork this repository.
2. Create a new branch for your feature or fix.
3. Submit a pull request with a detailed explanation of your changes.

## License

This project is licensed under the **Apache 2.0 License**. See the [LICENSE](https://github.com/capi-samples/cluster-api-rs/blob/main/LICENSE) for details.
