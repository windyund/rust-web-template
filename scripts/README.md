# 配置webhooks 步骤
- 1 服务器安装webhook(go) :https://github.com/adnanh/webhook

  1.1配置hooks.json
  ```json
  [
  {
    "id": "redeploy-webhook",
    "execute-command": "/data/smart-exam/scripts/deploy.sh",
    "command-working-directory": "/data/rust-web"
  }
  ]  
  ```
  1.2启动服务
  ```shell
  /path/to/webhook -hooks hooks.json -verbose
  # eg
  nohup ./webhook --hooks hooks.json  --verbose >hooks.log &
  ```
  1.3复制地址，配置代码库webhooks, 默认post请求
  ```shell
  http://yourserver:9000/hooks/redeploy-webhook
  ```
  todo: secret token配置
- 2.安装git
- 3.拉取代码
- 4.测试
