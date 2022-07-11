# ApiBasics

全部Api服务的共用内容

## 特性

- [ ] `make_response(data: T) -> tide::Response`：制作一个200响应

- [ ] `make_error(name: AsRef<&str>, details: Option<T>, error: Option<Error>) -> tide::Response`

  如果Error不为None，则是服务器错误，响应500且包含`backtrace`

  如果Error为None，则是客户端错误，响应400且不包含`backtrace`

- [ ] 