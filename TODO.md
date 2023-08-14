在crate xm_utils 里实现一个如下的宏

```Rust
#[macro_export]
macro_rules! init_tracer {
    ($path_prefix:expr, $level:expr) => {
        let tracer = { 
            /* initialize global logger to file */
        }
        tracing::subscriber::set_global(tracer).unwrap();
    }
}
```

在 crate lbm 里使用该宏

```Rust
init_tracer!("/var/log/lbm.log", "info");
```

编译时提示找不到crate tracing.

除了给lbm添加tracing依赖, 还有别的解决办法吗?
