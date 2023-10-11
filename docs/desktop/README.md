# Lotus Bridge Window 桌面管理软件

[![许可证](https://img.shields.io/badge/许可证-GPL3.0-blue.svg)](LICENSE)

Lotus Bridge Window 是使用 Flutter 编写的桌面应用，旨在管理 LotusBridge 项目。

GitHub 项目链接：[https://github.com/dingdaoyi/lotus_bridge_window](https://github.com/dingdaoyi/lotus_bridge_window)

## 启动项目

如果您只想下载执行文件，可以从 [Actions](https://github.com/dingdaoyi/lotus_bridge_window/actions) 下载：

```bash
git clone https://github.com/dingdaoyi/lotus_bridge_window.git
cd lotus_bridge_window

# Windows 打包
flutter build windows

# macOS 打包
flutter build macos
```

## 操作说明

### 1. 登录

![登录](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/login.png)

在登录时需要启动 LotusBridge 服务端，并配置好用户名和密码。默认用户名和密码从 config 文件中获取。如果服务端显示的地址不一致，可以在设置中进行修改。

### 2. 南向链接

![设备管理](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_manage.png)

南向链接涉及设备管理，展示了简单的设备列表、群组以及点位统计。

#### 2.1 设备添加

![设备添加](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_add.png)

在添加设备时需要关联协议。目前支持 Modbus 协议，后续将支持更多协议。选择协议后将显示对应协议的配置项，配置完成后点击保存即可。

#### 2.2 设备群组

![设备群组](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_group.png)

设备群组可以将设备分组，方便管理。

#### 2.3 设备点位

![设备点位](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/device_point.png)

点位对应设备的数据点。根据设备协议的不同，点位可以自动或手动创建。对于 Modbus 协议的点位，需要手动创建，点位地址规则详见 LotusBridge 文档。

### 3. 规则引擎

*待实现*

### 4. 北向应用

![推送管理](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/push_manage.png)

北向应用是设备数据的接收端，可以是云平台地址，也可以是本地其他计算节点或应用。目前已实现推送到消智云平台，推送以插件的形式实现。后续将支持更多插件。

北向应用需要添加设备群组，并关联设备群组的点位，将数据推送到对应的应用。

### 5. 插件管理

![插件管理](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/plugin_manage.png)

插件包括南向协议插件、北向应用插件和规则引擎插件。插件当前以 Rust features 的形式添加，后续会考虑支持插件的动态加载库文件。

### 6. 点位监控

![点位监控](https://raw.githubusercontent.com/dingdaoyi/lotus_bridge_window/main/doc/point_monitor.png)

## 许可证

LotusBridge 是开源项目，遵循 [GPL-3.0 许可证](LICENSE)。

## 联系方式

如有任何问题或反馈，请联系 [yanbing26@qq.com](mailto:yanbing26@qq.com)。