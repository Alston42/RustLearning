```
rustc filename  # compile

cargo new package_name # init a package
cargo build package_name # compile
@params: --release : executable file derived into release folder not default `debug` folder, compile slow but generate effective executable file
cargo run package_name # compile if changes and then run  
cargo check package_name # no compile, just check if it pass compilation, for debug
```bash

mut : mutability of a variable
let: declaration of a variable, 


Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，且 Ok 内部包含成功生成的值。Err 成员则意味着操作失败，并且包含失败的前因后果。

Result 类型的值，就像任何类型的值一样，都有为其定义的方法。io::Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示传递给 expect 的参数。如果 read_line 方法返回 Err，则可能是底层操作系统引起的错误结果。如果 io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回，以便你可以使用它。在本例中，这个值是用户输入的字节数。


一个 match 表达式由分支（arm） 构成。一个分支包含一个用于匹配的模式（pattern），给到 match 的值与分支模式相匹配时，应该执行对应分支的代码。Rust 获取提供给 match 的值并逐个检查每个分支的模式。

binary crate -> executable
library crate -> libxx.so

^0.8.3 means >= 0.8.3 && < 0.9.0

when `cargo build` initially , Cargo 计算出所有符合要求的依赖版本并写入 Cargo.lock 文件。当将来构建项目时，Cargo 会发现 Cargo.lock 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。项目会持续使用 0.8.3 直到你显式地升级`cargo update`.

 Rust 允许用一个新值来遮蔽 （shadow） guess 之前的值


const 命名全大写，shadow unpermitted,  只能声明一次，类型与值不可再更改
裸let，shadow permitted, 可以声明多次，以此更改类型甚至值

遮蔽需于同一作用域内，不能跨作用域，局部变量与全局变量

Rust 中的可变性是针对引用的，而不是数据本身。这意味着你可以拥有多个指向同一数据的可变引用，但不能拥有任何指向同一数据的不可变引用。


runtime error
compile error


let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tuple.0;
let six_point_four = tuple.1;

Rust 是一门基于表达式（expression-based）的语言，
语句（statement）是执行一些操作但不返回值的指令。表达式（expression）计算并产生一个值。
(x):    let x = (let y = 6);
expression:
{
    let x = 3;
    x + 1 // without `;`
}

并不对函数返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数隐式返回函数体最后一个表达式的值。使用 return 关键字和指定值，可以从函数中提前显式返回；
