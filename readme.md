# Orange Short Url(OSU)

基于 axum + diesel 短链服务

# 获取短链

POST 请求 /url/api {"url":"http://www.baidu.com"}
返回一个 code

# 短链跳转

访问 /j/{code} 即可跳转至原页面