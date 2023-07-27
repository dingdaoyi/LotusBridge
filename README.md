# LotusBridge

[![许可证](https://img.shields.io/badge/许可证-GPL3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

LotusBridge 是一个基于 Rust 的项目，旨在为边缘计算设备提供统一的解决方案。它允许您在边缘设备上处理数据，并将计算结果传送到云平台。该项目既是 Rust 学习项目，又是边缘计算在物联网场景中的实际解决方案。

## 功能特点

- **边缘计算**：LotusBridge 能够在边缘设备上进行数据处理和计算，减少与云端的频繁通信。
- **统一处理**：该项目提供了一种统一的方式来处理不同类型的边缘设备，使您能够屏蔽设备特定操作的复杂性。
- **

云平台集成**：LotusBridge 与云平台无缝集成，允许您将处理过的数据传输到云端进行进一步的分析和存储。
- **可扩展性**：该项目被设计为可扩展的，可以支持日益增长的边缘设备数量，并且适用于大规模的边缘计算部署。

## 快速入门

要开始使用 LotusBridge，请按照以下步骤进行操作：

1. **安装**：克隆该仓库并使用 Cargo（Rust 的包管理器）构建项目。

```bash
git clone https://github.com/dingdaoyi/LotusBridge.git
cd LotusBridge
cargo build
```

2. **配置**：通过修改`config`目录下的配置文件来配置项目。根据您的边缘设备和云平台的需求进行自定义设置。

3. **使用**：运行 LotusBridge 可执行文件启动边缘计算过程。

```bash
cargo run
```

有关更详细的使用说明和 API 文档，请参阅[文档](docs/README.md)。

## 贡献

欢迎和鼓励各种贡献！如果您有兴趣为 LotusBridge 做出贡献，请按照[贡献指南](CONTRIBUTING.md)开始。
## 原型设计
原型设计由@zxc-411 提供
http://axure.zd178.com/byjs/#id=gsrjql&p=%E5%90%91%E5%8D%97%E9%93%BE%E6%8E%A5&g=1
## UI设计
原型设计由@annei 提供
https://mastergo.com/goto/syA7JxUe?page_id=M&file=100489162530367
## 许可证

LotusBridge 是开源项目，遵循 [GPL-3.0 许可证](LICENSE)。

## 联系方式

如有任何问题或反馈，请联系 [yanbing26@qq.com](mailto:yanbing26@qq.com)。