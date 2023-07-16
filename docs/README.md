## 原型设计
[README.md](design%2FREADME.md)
## UI设计
[README.md](ui%2FREADME.md)
## 开发中遇到的不会的问题
1,数字类型的数据用字符串方式表单提交后,后端没法解析

2,交叉编译没成功,工具链可能有问题

3,现在主程序采用不安全的方式启动的协议库,并且协议库需要在程序中waite等待,要不然会出现错误

4,在写接口时,参数要求先 path 再body 要不报错

## 交叉编译悟空派
armv7-unknown-linux-gnueabihf

armv7-unknown-linux-musleabihf

export SQLITE3_STATIC=1

CC=arm-linux-musleabihf-cc SQLITE3_STATIC=1 cargo build --target arm-unknown-linux-musleabihf

```