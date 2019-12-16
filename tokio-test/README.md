# tokio-test

Tokio and Futures based testing utilities

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.

这个包主要是提供了一些测试工具,辅助其他地方写测试case更方便.
比如
1. block_on
2. Mock 一个基于缓冲区实现的异步读写,可以认为插入阻塞sleep
3. macros 提供了一些宏,比如assert_pending
4. task.rs 是一个模拟调度的,提供了一个自己实现的Waker