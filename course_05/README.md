Rust 宏
=======

## 过程宏示例代码

> 模仿 `vec!` 宏实现一个 `pow10` 的 `Vec` 初始化

[link](./src/main.rs)

## 解释说明

宏(macro) 是 Rust 中的一种高级特性，Rust 中的宏分为两种：

- 声明式宏( declarative macros )
- 三种过程宏( procedural macros )）
    - #[derive]，在之前多次见到的派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征
    - 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性
    - 类函数宏(Function-like macro)，看上去就像是函数调用

从根本上来说，宏是通过一种代码来生成另一种代码。可以通过 `cargo expand` 来查询宏生成的代码。

在匹配器/Matcher中，$name部分定义了变量名，匹配结果将绑定到该变量以便应用到转码器/Transcriber中。

冒号后面的部分被称为选择器/Designator，用于声明我们要匹配的类型。
下面是一些常见的Rust宏选择器：
- item：条目，例如函数、结构、模块等
- block：代码块
- stmt：语句
- pat：模式
- expr：表达式
- ty：类型
- ident：标识符
- path：路径，例如 foo、 ::std::mem::replace, transmute::<_, int>, ...
- meta：元信息条目，例如 #[...]和 #![rust macro...] 属性
- tt：词条树

宏的编译过程是先将宏展开成实现特征的代码后，再被编译。上述示例代码中的宏展开之后如下：

```rust
fn main() {
    let test = {
        let mut temp_vec = Vec::new();
        temp_vec.push(10_u128.pow(2));
        temp_vec.push(10_u128.pow(4));
        temp_vec
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", test));
    };
}
```
