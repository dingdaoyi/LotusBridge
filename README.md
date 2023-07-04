# Yanbing-Edge

[![License](https://img.shields.io/badge/license-GPL3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

Yanbing-Edge is a Rust-based project that aims to provide a unified solution for edge computing devices. It allows you to process data on edge devices and transmit the computational results to the cloud platform. This project serves as both a learning project for Rust and a practical solution for edge computing in IoT scenarios.

## Features

- **Edge Computing**: Yanbing-Edge enables data processing and computation on edge devices, reducing frequent communication with the cloud.
- **Unified Handling**: The project provides a unified way to handle different types of edge devices, shielding you from the complexity of device-specific operations.
- **Cloud Platform Integration**: Yanbing-Edge seamlessly integrates with cloud platforms, allowing you to transmit processed data to the cloud for further analysis and storage.
- **Scalability**: The project is designed to be scalable, supporting an increasing number of edge devices and suitable for large-scale edge computing deployments.

## Getting Started

To start using Yanbing-Edge, follow these steps:

1. **Installation**: Clone the repository and build the project using Cargo, the package manager for Rust.

```bash
git clone https://github.com/dingdaoyi/yanbing-edge.git
cd yanbing-edge
cargo build
```

2. **Configuration**: Customize the project by modifying the configuration files in the `config` directory according to your edge device and cloud platform requirements.

3. **Usage**: Run the Yanbing-Edge executable to start the edge computing process.

```bash
cargo run
```

For more detailed usage instructions and API documentation, refer to the [documentation](docs/README.md).

## Contributing

Contributions of all kinds are welcome and encouraged! If you're interested in contributing to Yanbing-Edge, please refer to the [contribution guidelines](CONTRIBUTING.md) to get started.

## License

Yanbing-Edge is an open-source project released under the [GPL-3.0 License](LICENSE).

## Contact

For any questions or feedback, please contact [yanbing26@qq.com](mailto:yanbing26@qq.com).

---

# Yanbing-Edge

[![许可证](https://img.shields.io/badge/许可证-GPL3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

Yanbing-Edge 是一个基于 Rust 的项目，旨在为边缘计算设备提供统一的解决方案。它允许您在边缘设备上处理数据，并将计算结果传送到云平台。该项目既是 Rust 学习项目，又是边缘计算在物联网场景中的实际解决方案。

## 功能特点

- **边缘计算**：Yanbing-Edge 能够在边缘设备上进行数据处理和计算，减少与云端的频繁通信。
- **统一处理**：该项目提供了一种统一的方式来处理不同类型的边缘设备，使您能够屏蔽设备特定操作的复杂性。
- **

云平台集成**：Yanbing-Edge 与云平台无缝集成，允许您将处理过的数据传输到云端进行进一步的分析和存储。
- **可扩展性**：该项目被设计为可扩展的，可以支持日益增长的边缘设备数量，并且适用于大规模的边缘计算部署。

## 快速入门

要开始使用 Yanbing-Edge，请按照以下步骤进行操作：

1. **安装**：克隆该仓库并使用 Cargo（Rust 的包管理器）构建项目。

```bash
git clone https://github.com/dingdaoyi/yanbing-edge.git
cd yanbing-edge
cargo build
```

2. **配置**：通过修改`config`目录下的配置文件来配置项目。根据您的边缘设备和云平台的需求进行自定义设置。

3. **使用**：运行 Yanbing-Edge 可执行文件启动边缘计算过程。

```bash
cargo run
```

有关更详细的使用说明和 API 文档，请参阅[文档](docs/README.md)。

## 贡献

欢迎和鼓励各种贡献！如果您有兴趣为 Yanbing-Edge 做出贡献，请按照[贡献指南](CONTRIBUTING.md)开始。

## 许可证

Yanbing-Edge 是开源项目，遵循 [GPL-3.0 许可证](LICENSE)。

## 联系方式

如有任何问题或反馈，请联系 [yanbing26@qq.com](mailto:yanbing26@qq.com)。