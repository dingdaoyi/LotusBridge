# LotusBridge [中文](README-ZH.md)

[![License](https://img.shields.io/badge/license-GPL3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

LotusBridge is a Rust-based project that aims to provide a unified solution for edge computing devices. It allows you to process data on edge devices and transmit the computational results to the cloud platform. This project serves as both a learning project for Rust and a practical solution for edge computing in IoT scenarios.

## Features

- **Edge Computing**: LotusBridge enables data processing and computation on edge devices, reducing frequent communication with the cloud.
- **Unified Handling**: The project provides a unified way to handle different types of edge devices, shielding you from the complexity of device-specific operations.
- **Cloud Platform Integration**: LotusBridge seamlessly integrates with cloud platforms, allowing you to transmit processed data to the cloud for further analysis and storage.
- **Scalability**: The project is designed to be scalable, supporting an increasing number of edge devices and suitable for large-scale edge computing deployments.

## Getting Started

To start using LotusBridge, follow these steps:

1. **Installation**: Clone the repository and build the project using Cargo, the package manager for Rust.

```bash
git clone https://github.com/dingdaoyi/LotusBridge.git
cd LotusBridge
cargo build
```

2. **Configuration**: Customize the project by modifying the configuration files in the `config` directory according to your edge device and cloud platform requirements.

3. **Usage**: Run the LotusBridge executable to start the edge computing process.

```bash
cargo run
```

For more detailed usage instructions and API documentation, refer to the [documentation](docs/README.md).

## Contributing

Contributions of all kinds are welcome and encouraged! If you're interested in contributing to LotusBridge, please refer to the [contribution guidelines](CONTRIBUTING.md) to get started.

## License

LotusBridge is an open-source project released under the [GPL-3.0 License](LICENSE).

## Contact

For any questions or feedback, please contact [yanbing26@qq.com](mailto:yanbing26@qq.com).