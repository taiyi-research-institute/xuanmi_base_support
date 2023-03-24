# 介绍

此库提供四类功能：异常处理、http post收发对象、格式转换、语法糖。

在 `Cargo.toml` 里添加如下依赖项

```
[dependencies.xuanmi_base_support]
git = "https://github.com/taiyi-research-institute/xuanmi_base_support"
branch = "main"
```

如果你在中国大陆，`git` 链接可以改成

```
ssh://git@github.com/taiyi-research-institute/xuanmi_base_support
```

如果你需要运行单元测试，推荐使用如下命令。

```
cargo test -- --show-output
```

# 异常处理

当你使用此库提供的异常处理机制时，笔者推荐你养成如下几个习惯。按照这样的习惯来处理异常，你将得到类似于Java Exception的报错信息。

1. 如果你自己写的函数有可能执行失败，那么这个函数的返回类型应该指定为 `Outcome<T>` 。
2. 如果你调用别人的函数，且该函数有可能失败，那么你要在函数调用的后面跟 `.catch(name, context)?` 。
   这是用来取代`.unwrap()` 或 `.expect()` 的。
3. 把你写过的 `panic!(...);` 替换成 `throw!(...);` 。类似地，把你写过的 `assert!(cond, ...);` 替换成 `if cond { throw!(...); }` 。
4. 给Exception起名时，建议传符号，不建议传字符串字面量。

例1：调用别人的函数，该函数有可能失败。**要点：Outcome, catch**

```rust
#[macro_use] use xuanmi_base_support::*; 
use serde_json;

pub fn objectToJson<T>(
    obj: &T
) -> Outcome<String> where T: Serialize {
    let json: String = serde_json::to_string(obj)
    .catch( // 取代unwrap
        // EXN是模块名. 我在模块EXN里定义了许多Exception名称
        EXN::SerializationException, 
        // 上下文信息
        &format!("Failed to convert object of type `{}` to JSON string", std::any::type_name::<T>()),
    )?;
    Ok(json)
}
```

例2：你写一个可能失败的函数。**要点：Outcome, catch**

```rust
#[macro_use] use xuanmi_base_support::*; 

fn div() -> Outcome<f64> {
    let (a, b): (f64, f64) = (1.0, 0.0);
    let eps: f64 = 1.0 / 4096.0;
    if b.abs() < eps {
        throw!( // 取代panic或assert
            name=EXN::ArithmeticException,
            ctx=&format!("Cannot divide a={:.4} by b={:.4}", a, b)
        );
    } else {
        return Ok(a/b);
    }
}
```

关于宏 `throw!` ，你只能给它传零对到若干对键值参数。通过传递这些参数，你可以定制报错信息。支持的参数有

* `name` - 错误的名称。笔者喜欢仿照Java Exception的名称。你也可以用错误代码。
* `ctx`, `context` - 详细的报错信息。通常需要配合Rust语言提供的 `format!` 宏。
* `src`, `cause` - 错误对象，可以是任何实现了 `std::error::Error` 的对象。用于给已有的错误对象（比如 `match` 里的 `Err(e)` 里的 `e`）再套一层。

> 除非你清楚此库异常处理的原理，否则不建议设定以下参数。

* `file` - 源码文件的路径。不传则为 `throw!` 被调用的文件路径。
* `line` - 源码行号。不传或传0则为 `throw!` 被调用的行号。
* `col`, `column` - 源码列号。不传或传0则为 `throw!` 被调用时，首字母左侧光标的偏移量。


# http post收发对象

此库提供了 `http_post` 函数。这个函数把一个Rust对象序列化成JSON，作为POST请求体，发送给一个URL；之后，这个函数把响应体当成JSON，反序列化成另一个Rust对象。注意：（1）这个函数每1000ms重试一次，一共重试3次，即最多请求4次；（2）服务端对 `content-type` 要么不过滤, 要么允许 `application/json` .

例子：

首先定义收发结构体。结构体需实现Clone, Serialize, Deserialize。

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyRequest {
    uname: String,
    email: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyResponse {
    a: String,
    b: String,
}
```

定义好结构体就可以收发了。

```rust
#[macro_use] use xuanmi_base_support::*;

fn test_http_post() -> Outcome<MyResponse> {
    let req = MyRequest {
        uname: "xuanmi".to_string(),
        email: "luban@example.com".to_string(),
    };
    let url = "http://localhost:50000/test";
    let resp: MyResponse = http_post(url, &req).catch(
    	name=EXN::HttpPostException,
    	ctx=&format!("Failed to post to {}", url)
    )?;
    // do something else ...
    Ok(resp)
}
```

# 格式转换

To be continued...

# 语法糖

To be continued...
