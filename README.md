# LotusBridge

[![许可证](https://img.shields.io/badge/许可证-GPL3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

LotusBridge 是一个基于Rust的边缘计算设备网关项目。它可以让您在边缘设备上采集和处理数据，并将结果传送到云平台。该项目是 Rust
学习项目,正在完善中。

## 功能特点

- **边缘计算**：LotusBridge 能够在边缘设备上进行数据采集处理和计算，减少与云端的频繁通信。
- **统一处理**：该项目提供了一种统一的方式来处理不同类型的边缘设备，使您能够屏蔽设备特定操作的复杂性。
- **云平台集成**：LotusBridge 可以与云平台无缝集成，允许您将处理过的数据传输到云端进行进一步的分析和存储。
- **可扩展性**：南向设备采集、规则、北向数据推送均设计为可扩展feature模式，可以满足各种边缘设备集成，并且适用于大规模的边缘计算部署。

## 快速入门

要开始使用 LotusBridge，请按照以下步骤进行操作：

1. **安装**：克隆该仓库并使用 Cargo（Rust 的包管理器）构建项目。

```bash
git clone https://github.com/dingdaoyi/LotusBridge.git
cd LotusBridge
cargo build
```

2. **配置**：通过修改`config`目录下的配置文件来配置项目。

```yaml
# 数据库配置
database:
  database: sqlite.db
# 日志级别
logger:
  level: DEBUG
auth:
  username: admin
  password: 123456
  jwtSecret: aaabbbccc
  expireMinutes: 120 # 过期时间
server:
  port: 8000
```

3. **使用**：运行 LotusBridge。

```bash
cargo run
```

4. **前端项目**：LotusBridge 项目有一个关联的web管理项目,和一个桌面应用软件

前端项目地址:
https://github.com/dingdaoyi/lotus_bridge_web

桌面项目地址:
https://github.com/dingdaoyi/lotus_bridge_window

桌面项目展示:
![img.png](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_add.png)
![img.png](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_point.png)
![img.png](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/push_manage.png)
![img.png](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/plugin_manage.png)
![img.png](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/point_monitor.png)

有关更详细的使用说明和 API 文档，请参阅[文档](docs/README.md)。

## 项目进度和规划

[&check;] modbus-TCP 协议支持

[&check;] modbus-RTU 协议支持

[&check;] 消智云平台推送

[&cross;] 规则引擎

[&cross;] 其他协议和云上对接

## 贡献

欢迎和鼓励各种贡献！如果您有兴趣为 LotusBridge 做出贡献，请查看我们的[贡献指南](CONTRIBUTING.md)。

## 原型设计

原型设计由 [@zxc-411](https://github.com/zxc-411)提供
http://axure.zd178.com/byjs/#id=gsrjql&p=%E5%90%91%E5%8D%97%E9%93%BE%E6%8E%A5&g=1

## UI设计

UI设计由 [@annei](https://github.com/annei)提供
https://mastergo.com/goto/syA7JxUe?page_id=M&file=100489162530367

## 许可证

LotusBridge 是开源项目，遵循 [GPL-3.0 许可证](LICENSE)。

## 联系方式

如有任何问题或反馈，请联系 [yanbing26@qq.com](mailto:yanbing26@qq.com)。