我推荐使用本项目中的 `exception.rs`. 其已被高频地使用一年, 与其他模块相比, 经受了最多的考验. 每当程序出bug时, `exception.rs`能帮我快速地找到报错位置和原因, 比`panic/unwrap/expect/assert`好用很多!

如果复制或改进其源码, 请注明出处.

## `exception.rs` 最佳实践

首先, 在你自己的`lib.rs`或`main.rs`里, 将`exception.rs`中的所有符号重新导出 (re-export) 为`crate::exception::*`. 必须如此, 两个宏才能被编译. 重新导出的方式有很多, 参考:

```rust
/* In lib.rs or main.rs */
/* pub */ mod exception {
   pub use xuanmi_base_support::{self,
      Exception, Outcome,
      TraitStdResultToOutcome, TraitStdOptionToOutcome
   };
   pub use xuami_base_support::{throw, assert_throw};
}
```

然后, 在实现任何 **可能失败的函数 (fallable function)** 时, 参考以下代码片:

```rust
use crate::exception::*;

/* async */ fn foo_may_fail(/* params */) -> Outcome</* ret-type */> {
   let a = foo_a_may_fail(params)/*.await*/.catch(
      "错误标题",
      &format!("错误原因. 有助于诊断的变量的值: {}", blabla)
   )?; // 别忘了问号.

   // 如果你确信 `foo_b_may_fail` 内部写了详细的错误信息, 那就不必再写一遍.
   let b = foo_b_may_fail(params)/*.await*/.catch_()?;

   let c = get_c(params)/*.await*/.ifnone(
      "错误标题",
      &format!("错误原因. 有助于诊断的变量的值: {}", blabla)
   )?;
   let d = get_d(params)/*.await*/.ifnone_()?;

   // 建议用 assert_throw! 取代 assert!, assert_eq!
   assert_throw!(a > b);
   assert_throw!(c > d, "&str类型的错误细节");
   assert_throw!(
      a+b == c*d,
      "&str类型的错误标题",
      "&str类型的错误细节"
   );

   Ok((a, b, c, d))
}
```


## `exception.rs` 原理简述

* `struct Exception`
   * 实现了`trait fmt::Display`, 使得程序遭遇异常时, 能够打印出类似于Java/Python那样的错误栈.
   * 具有一个`inner`字段, 用于维护错误栈.
* `type Outcome`, 是`std::Result<T, Box<Exception>>`的别名. 起名为`Outcome`是为了避免和`Result`撞名.
* `trait TraitStdResultToOutcome`, 给`Result<T, E>`挂接了`catch(self, name, ctx)`和`catch_(self)`两个函数. 调用任何一个函数, 会构造一个`Outcome`对象, 对象的`T`分支直接移动`self`的`T`分支, 对象的`E`分支把`inner`字段设定为`self`, 并设定名称、上下文、行号等字段.
* 类似地, `trait TraitStdOptionToOutcome`, 给`Option<T>`挂接了`ifnone(self, name, ctx)`和`ifnone_()`两个函数.
* 宏`throw!(name, ctx)`, 以及宏`assert_throw!(bool_expr, [[name], ctx])`, 构造一个没有内部错误的`Exception`对象, 并使宏的调用者返回该错误对象.
