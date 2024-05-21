# 技术栈
- axum
- mongodb
- redis
- jwt

# 使用流程
- 1.将.env-example文件名改为.env并改为自己的连接地址
- 2.main.rs 启动服务

# 功能实现介绍
- 1.统一封装了返回参数reponse、error结构
- 2.不同功能模块拆分，结构清晰
- 3.开箱即用，配好数据库连接，直接写对应的hanlder、service业务代码
- 4.mongodb使用原生的client执行crud操作，简单灵活，未加ORM框架

# 参考资料
- mongodb-driver文档：https://docs.rs/mongodb/2.8.2/mongodb/
- mongo orm wither 参考：https://github.com/thedodd/wither
- axum文档：
  https://github.com/tokio-rs/axum
  https://docs.rs/axum/latest/axum/
- redis : https://docs.rs/redis/latest/redis/

# 开发流程
- 1.写业务
- 2.单元测&集成测试
- 3.代码统一格式化
```shell
#格式化整个项目
cargo fmt 
# 格式化指定文件
rustfmt filename
```
若工具链未安装fmt
```shell
rustup component add rustfmt
```

日志级别：error > warn > info > debug > trace