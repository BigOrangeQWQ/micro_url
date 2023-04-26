# Orange Short Url(OSU)

基于 axum + diesel 的短链生成服务

## 获取短链

- 请求方式: POST
- 请求地址: /url
- 请求参数: 
```json
{"url": "http://www.baidu.com"}
```
- 返回类型: string

## 短链跳转

- 请求方式: GET
- 请求地址: /j/{code}
- code 为获取短链时返回的 string


## TODO

- [x] 短连接的生成
- [x] 短连接的跳转
- [ ] 更好的生成交互
- [ ] 项目代码优化
- 注：本项目为个人学习产物