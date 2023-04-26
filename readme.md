# Orange Short Url(OSU)

基于 axum + diesel 的短链生成服务

## 获取短链

POST 请求 /url/api {"url":"http://www.baidu.com"}
返回一个 code

## 短链跳转

访问 /j/{code} 即可跳转至原页面

## TODO

- [x] 短连接的生成
- [x] 短连接的跳转
- [ ] 更好的生成交互
- [ ] 项目代码优化
- 注：本项目为个人学习产物