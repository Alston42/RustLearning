## Rust

每 6 周发布一版的一个好处是下一班火车即将到来。如果一个功能在特定版本中错过了也无需担心：另一个版本很快就会到来！这有助于减少在发布截止日期前匆忙加入可能未完善的功能的压力。

Rust项目支持最新的稳定版本。当一个新的稳定版本发布时，旧版本就达到了其生命周期（EOL）。这意味着每个版本都支持六周。

## cargo

### 模块系统(the module system)

#### crate
`crate`: all types of rust source code(rs files) after compiled, 包括：

`binary crate`: executable

`library crate`: libxx.so in `C/C++`

其表现各模块的树形结构

#### 包（package） 
提供一系列功能的一个或者多个 crate。包中包含的 `Cargo.toml` ，阐述如何去构建这些 `crate`。

Cargo 遵循的一个约定：
- 至多 **只能** 包含一个 `library crate`， `src/lib.rs`；
- 可包含任意多个二进制 `binary crate`, `src/main.rs`, `src/bin/xx.rs`；
- 包中至少包含一个 `crate`，无论是`lib`还是`binary`。
- `src/main.rs` 是一个与包同名的 `binary crate` 的 `crate root`
- `src/lib.rs`，是一个与包同名的 `library crate`的 `crate root`
- `Rust` 编译器以`crate root`为起始点，构成名为 `crate` 的模块，且其位于 `模块树` 的模块结构的根部，并由此派生出所有`crate`定义的各模块
- 可将一个 crate 的功能保持在其自身的作用域(`scope`/`namespace`)中
- 外部包引入后将接到`crate root`下
- `标准库(std)`也是外部 `crate`，但因其随 `Rust` 语言一同分发，无需修改 `Cargo.toml` 来引入 `std`

#### 模块(mod)
将一个 crate 中的代码进行分组，以提高可读性与重用性。

控制项的 `私有性`，即项是可以被外部代码使用的(`public`)，还是作为一个内部实现的内容，不能被外部代码使用（`private`）。

相当于C/C++的`class`

#### 路径(path)
用于引用模块树中模块树中某特定项
- `绝对路径(absolute path)`: 从 `crate root` 开始，以`crate::`开头
- `相对路径(relative path)`: 从当前模块开始
    - `super`: 从父模块开始的相对路径
    - `self`: 从当前模块开始的相对路径，可省略

`Rust` 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的

父模块不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项，兄弟模块间无法相互使用其私有项

过编译原则: 路径正确地指向被引用者且引用者具有访问被引用者的权限

##### pub
`pub`: 创建公共项，使子模块的内部部分暴露给上级模块。

`use`创建`软链接(symbolic link，又称符号连接)`
习惯上，`use`将函数所在父模块引入作用域，调用函数时需指定父模块；而引入结构体、枚举和其他项时则指定其完整路径

`as`: `use __path__ as __sign__`

`重导出(reexporting)`: `pub use __path__`，除引入到当前作用域外，还可导出路径到父模块，获取位于一个位置的公有项并将其公开到另一个位置。

枚举定义前`pub`，则其所有成员变为`public`

结构体定义前`pub`，将其变作`public`，但其`field`仍为`private`，对此类结构体需提供一个`public`的关联构造函数来构造其实例

`嵌套路径`: `use std::{cmp::Ordering, io};`，`use std::io::{self, Write};`

`*(glob operator)`: 将某路径下**所有公有项**引入作用域，`use std::collections::*`

### build

```bash
rustc filename  # compile
cargo new package_name # init a package
cargo build package_name # compile
@params: --release : executable file derived into release folder not default `debug` folder, compile slow but generate effective executable file
cargo run package_name # compile if changes and then run  
cargo check package_name # no compile, just check if it pass compilation, for debug
```

`runtime error`
`compile error`

### package version

`^0.8.3`: >= 0.8.3 && < 0.9.0

第一次`cargo build`时，`Cargo`，Cargo 计算出所有符合要求的依赖版本并写入 `Cargo.lock` 文件。项目会持续使用各依赖指定版本直到显式地升级，即`cargo update`.

### 发布配置(release profiles)

预定义的、可自定义定制的带有不同选项的配置，允许开发者更灵活地控制代码编译的多种选项

运行 `cargo build` 时采用的 `dev` 配置，被定义为开发时的好的默认配置

运行 `cargo build --release` 的 `release` 配置。良好的发布构建的默认配置。

output:
```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

当项目的 `Cargo.toml` 文件中没有任何 `[profile.*]` 部分的时候，`Cargo` 会对每一个配置都采用默认设置。

通过在 `[profile.*]` 对应的部分中增加任何定制的配置，可以覆盖任意默认设置的子集

`opt-level`: 设置控制 Rust 会对代码进行何种程度的优化，取值从0到3，integer

### 发布crate到Crates.io

#### 文档注释

`文档注释(documentation comments)`: 采用`///`(函数/结构体等项的注释)或`//!`(项的结构，模块描述，模块树...如用于`crate root`文件)以支持`Markdown`标记来格式化文本

`cargo doc`: 生成文档注释的HTML文档
`cargo doc --open`: 构建并于浏览器中打开

- Panics：这个函数可能会 panic! 的场景
- Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误
- Safety：如果这个函数使用 unsafe 代码，这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。

`cargo test` 可调用文档中的示例函数，形如:
```
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
```

公有API信息可由重导出默认实现

#### remote

`cargo login API_token` first

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

# ...
```

`cargo publish`

`cargo yank --vers 1.0.1`
`cargo yank --vers 1.0.1 --undo`: undo yanking operation

### 工作空间(workspace)

in `$WORKSPACE`: `touch Cargo.toml`

write in:
```toml
[workspace]

members = [
    "member1",
    "lib1"
    # ......
]
```
then `cargo new member1`

`cargo new lib1 --lib`

├── Cargo.lock<br>
├── Cargo.toml<br>
├── lib1<br>
│   ├── Cargo.toml<br>
│   └── src<br>
│       └── lib.rs<br>
├── member1<br>
│   ├── Cargo.toml<br>
│   └── src<br>
│       └── main.rs<br>
└── target<br>

add path dependency, in `member1/Cargo.toml`:
```toml
[dependencies]

lib1 = { path = "../lib1" }
```

`cargo build` to build

`cargo run -p selected_package` to run

`cargo test -p selected_package` to test

`cargo test` to test all packages


必须进入每一个 `crate` 的目录并运行 `cargo publish` 来向 `crates.io` 发布工作空间中的每一个 crate。

#### add outer dependencies

in `member1/Cargo.toml` && `$WORKSPACE/Cargo.toml`:
```toml
[dependencies]
rand = "0.5.5"
```

#### install

`cargo install package`: default download into `$HOME/.cargo/bin`, this dictionary need to be added into `$PATH`


#### custom extended command

`cargo list`: show all the cargo command

every binary file named `cargo-something` in `$PATH` could be launched by `cargo something`

## 通用编程概念

`mut` : mutability of a variable

`let`: declaration of a variable

```rust
struct Result<T, E> {
    Ok(T),
    Err(E),
}
```
`T`: 成功返回值的类型

`E`: 返回失败原因的类型

`.expect(message)`: `Ok`时返回原值，`Err`时`panic!`，输出`message`

### 数字标识: 
`1_000_000u32`，以`_`任意分割数字，末尾添上数据类型如`u32`

`usize`/`isize`，`size`相应于系统架构`arch`变量，为`32位`或`64位`

`0xff`: 十六进制

`0o77`: 八进制

`0b1111_0000`: 二进制

`b'A'`: 字节，仅限于`u8`

使用 `--release` 参数进行发布（release）模式构建时，`Rust` 不检测会导致 `panic` 的整型溢出。相反当检测到整型溢出时，Rust 会进行一种被称为 `二进制补码包裹（two’s complement wrapping）` 的操作：

如对于`u8`: $ x = x - k \cdot 256 \quad when \quad k \cdot 256 \leq x < (k + 1) * 256 $

要显式处理溢出的可能性，可以使用标准库针对原始数字类型提供的以下一系列方法：
- `wrapping_*`: 在所有模式下进行包裹，如 `wrapping_add`
- `checked_*`: 若发生溢出，则返回 None 值
- `overflowing_*`: 返回该值和一个指示是否存在溢出的布尔值
- `saturating_*`: 使溢出时返回最小值或最大值

### `遮蔽(shadow)`
之前声明过的变量名经由重复声明关联到新的数据类型与值
- `const`: 命名全大写，shadow unpermitted，只能声明一次，类型与值不可再更改
- `裸let`: shadow permitted, 可声明多次，以此更改类型甚至值
- `let mut`: 声明一个类型不变，值可变的变量

`内部作用域(inner scope)`可访问外作用域变量，遮蔽只在同一作用域内生效，不能跨作用域，或看作是遮蔽了一个为访问外部变量而传入内部作用域的局部变量（浅拷贝？所有权移交给谁？），

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // [type; length]
let a = [3; 5]; // [3, 3, 3, 3, 3]
let tup: (i32, f64, u8) = (500, 6.4, 1);    // 解构(destructuring)
let (x, y, z) = tup;
let five_hundred = tuple.0;
let six_point_four = tuple.1;
```
### expression ? statement
Rust 是一门`基于表达式（expression-based）`的语言: 
- `语句（statement）`: 执行一些操作但不返回值的指令
- `表达式（expression）`: 计算并产生一个值。

Wrong Example: `let x = (let y = 6)`;

`代码块(code block)`即`{...}`亦可作为`expression`:
```rust
{
    let x = 3;
    x + 1 // without `;`
}
```
Rust 并不对函数返回值命名，但要在`箭头（->）`后声明它的类型。

函数**隐式**返回函数体**最后**一个表达式的值。

若使用 `return` 关键字，可以从函数中**提前显式**返回；

```rust
let number = if condition { 5 } else { "six" }  // uncompiled
```
`range`函数：`(1..4).rev()`，`.rev` 即`reverse`

### 循环

`loop {...}`

`循环标签(loop label)`: `'label: loop {...}`

`while constraint {...}`

`for element in array {...}`

### 内存管理/所有权系统
`栈`: `后进先出（last in, first out）`

`堆`: `缺乏组织`

入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。

访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。

调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

就字符串字面值来说，编译器在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。

对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。

这意味着：
- 必须在运行时向内存分配器（memory allocator）请求内存。
- 需要一个当我们处理完 String 时将内存返回给分配器的方法。

第一部分在各编程语言中是非常通用的。

然而，第二部分实现起来就各有区别了。

在有 `垃圾回收（garbage collector，GC）` 的语言中，`GC` 记录并清除不再使用的内存，而开发者并不需要关心它；

在大部分没有 `GC` 的语言中，开发者识别出不再使用的内存并调用代码显式释放。

`Rust` 则是: 内存在拥有它的所有权的变量离开作用域后就被自动释放，此时自动调用了函数`drop`

`length/size`: 当前内容使用了多少字节的内存

`capacity(容量)`: 从分配器总共获取了多少字节的内存


为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效

Compile Error:
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");
```

`浅拷贝(shallow copy)`：仅拷贝引用

`深拷贝(deep copy)`：拷贝引用指向的内容 => `.clone()`关键字 或 直接赋值为`栈`上的数据

`移动(move)`：不拷贝内容，拷贝引用并删除旧的，即移动旧的到新的引用上

Rust 的所有权规则：
- 每块内存在任意时刻只能有一个所有者，所有权移动后旧所有者会失去所有权，以防多拷贝一份引用在最终释放同一块内存时造成的`double free(二次释放)` 的内存污染的漏洞。
- 当持有堆中数据值的变量离其作用域时，其内存将通过 `drop` 被释放掉，除非在此之前数据被移动为另一个变量所有。

### 引用
`引用(reference)`: `&`，浅拷贝，但最终所有权仍在原有者上，即只可访问无法修改

`可变引用`: `&mut`，浅拷贝，可访问可修改

`借用(borrowing)`：create a reference
- 借用规则：
    - 在任意时刻，同一块内存可以同时拥有多个不可变引用，或者只能拥有一个可变引用（防止数据竞争[data race]）
    - 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。区别于普通变量
    - 引用必须总是有效的

- 数据竞争:
    - 两个或更多指针同时访问同一数据。
    - 至少有一个指针被用来写入数据。
    - 没有同步数据访问的机制。

`悬垂指针（dangling pointer）`: 释放内存后却仍错误地保留指向它的野指针，此时其指向一块无效内存；而后续进程恰巧再次将原先这块空余内存分配给新的持有者时（且新内存区段也可能只是与旧的相部分重叠），两指针相冲突！

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
    // 正确的是： s
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

`字符串 slice` 的类型声明写作 `&str`，是不可变引用：

字符串字面值被储存在二进制文件中，例如`let s = "Hello, world!";`中 `s` 的类型是 `&str`：是一个指向二进制程序特定位置的 slice，是不可变引用。

### 结构体(structure)
和元组一样，结构体的每一部分可以是不同类型。

不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义，也因此比元组更灵活：不需要依赖顺序来指定或访问实例中的值。

定义每一部分数据的`键名`和`类型`，称其为 `字段（field）`，以`字段`替称`键名`很常见

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Rust 并不允许只将某个字段标记为可变

#### 字段初始化简写语法
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
若字段名同参数名重名，无需重复`field: input_data`的模式

#### 结构体更新语法
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```
`..user1` 必须放在最后

此语法移动了user1

#### 元组结构体
无键，按顺序存储值
```rust
struct Color(i32, i32, i32);
```
#### 类单元结构体
无字段的，类似于元组的unit类型，即`()`，用于想要在某个类型上实现 trait 但不需要在类型中存储数据的时候

#### 结构体数据的所有权
结构体存储另一复合数据类型，且拥有其所有权，则需使用**自身拥有所有权**的数据类型，如`String` 类型而不是 `&str` `字符串 slice` 类型。

`生命周期(lifetimes)`: 可使结构体存储被其他对象拥有的数据的引用。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。

println! 宏能处理很多类型的格式，不过，{} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。

```rust
#[derive(Debug)]
'{xx:?}'
```

### 方法(method)
在结构体、枚举、trait对象上下文中被定义，首参数总是为`self`，指向调用该方法的结构体实例

`impl`: implementation，`impl` 块中内容与相应类型所关联，即 `C++` 中的成员函数

`&self`为`self: &Self`缩写，在一个 `impl` 块中，`Self` 类型是 `impl` 块的类型的别名

果想要在方法中改变调用方法的实例，需要将第一个参数改为 `&mut self`

`getters`: 与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。可以把字段变成私有的，但方法是公共的，进而把对字段的只读访问作为该类型公共 API 的一部分

#### 自动引用和解引用
C++(手动): `object->something()` 等若于 `(*object).something()`

Rust: object.something() 调用方法时，会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配

#### 关联函数
在 `impl` 块中被定义的函数，包括方法

非方法的关联函数，需以ClassName::functionName()调用

### 枚举

#### enum
成员间互斥，择一

数据关联，任意类型数据可嵌入枚举中
```rust
enum Message {
    Quit,   // 无关联数据
    Move { x: i32, y: i32 },    // 关联匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32), // 关联元组
}
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            _ => println!("Other"),
        }
    }
}
```

#### Option
`null`: 空值是一个因为某种原因目前无效或缺失的值。

`Rust` 并没有空值，而以编码存在或不存在概念的枚举以替代：
```rust
enum Option<T> {
    Some(T),
    None,
}
let absent_number: Option<i32> = None;
let some_number = Some(5);
```
`None`值需指定类型

其他语言中声明某类型`T`的变量，`null`值与有效值都可赋值为用，需后续区分辨别

而在`Rust`中`Option<T>` 和 `T`是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`，在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`，期间便能自动捕获假设非空有效值但实际上为空的情况，而在之后任意`T`类型的值使用都一定是有效的，因而也没有设定`null`值的必要。

简而言之：只要一个值不是 `Option<T>` 类型，你就 可以 安全地认定它的值不为空。

### 模式匹配
#### match控制流运算符

一个 match 表达式由分支（arm） 构成。一个分支包含一个用于匹配的模式（pattern），给到 match 的值与分支模式相匹配时，应该执行对应分支的代码。Rust 获取提供给 match 的值并逐个检查每个分支的模式。

`Rust` 中的匹配是`穷举式的（exhaustive）`：必须穷举到最后的可能性来使代码有效。Rust 防止我们忘记明确的处理 None 的情况，这让我们免于假设拥有一个实际上为空的值。

##### 多模式匹配

```rust
let x = 1;
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    4..=8 => println!("four through eight"),
    'a'..='j' => println!("early ASCII letter");
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("anything else"),
}
```

`|`: 或(or)，匹配此分支任一模式

`..=`: 匹配一个闭区间范围的值，只允许用于数字和`char`值，因为编译器会在编译时检查范围不为空。而 `char` 和数字值是 `Rust` 仅有的可以判断范围是否为空的类型

##### 通配模式
`other`: 匹配所有值，并绑定到`other`变量

`_`: `占位符(placeholder)`，匹配所有值，但不会绑定到变量，即忽略了值

`_ => ()`：匹配所有值，但不会绑定到变量，且不执行任何操作

#### if let 简单控制流
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```
```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```
增加简洁度，但失去强制的穷尽性检查

`if let` 是 `match` 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

`if`, `if let`, `else if`, `else if let`, `else` 可混用
```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

#### while let 条件循环
```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

#### for 循环

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

#### let语句

`let x = 5;` 等若于 `let PATTERN = EXPRESSION`

变量名`x`或说实际上是 “将任何值绑定到变量 x，不管值是什么” 这一模式

**命名变量是匹配任何值的不可反驳模式**

再如 `let (x, y, z) = (1, 2, 3);`

#### 函数参数的模式匹配

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
let point = (3, 5);
print_coordinates(&point);
```

值`&(3, 5)` 匹配模式`&(x, y)`;

#### Refutability(可反驳性)
`不可反驳的(irrefutable)`: 能匹配任何传递的可能值的模式，如`let x = 5;`中的`x`
`可反驳的(refutable)`: 对某些可能的值进行匹配会失败的模式，如`if let Some(x) = a_value`

函数参数、 `let` 语句和 `for` 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作

`if let`，`while let`，`match`表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。（在只有一个匹配分支的 match 中可使用不可反驳模式）

`let Some(x) = some_option_value;` 如在 `let` 语句中使用可反驳形式无法通过编译

可过编译
```rust
if let x = 5 {
    println!("{}", x);
};
```
#### 匹配命名变量

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y);
```
`match` 新开了一个作用域，作为模式的`Some(y)`的一部分而声明的变量`y`，此时覆盖了`match`结构之外的同名变量`y`，由于没有赋值，`y`作为模式可匹配任何值

#### match guard

指定于某一`match`分支模式之后的额外`if`条件

可修复上述变量覆盖问题

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {}", x, y);
```
`match`内部作用域中，y没有作为命名变量的模式，因而也没有重新声明

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```
优先级顺序如下: 
`(4 | 5 | 6) if y => ...`

#### 解构结构体

```rust
struct Point {
    x: i32,
    y: i32,
}
let p = Point { x: 0, y: 7 };
let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);
```

创建了变量 `a` 和 `b` 来匹配结构体 `p` 中的 `x` 和 `y` 字段。

匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称：
```rust
let Point { x, y } = p;
```

甚至将字面量作为结构体模式的一部分解构，其余字段创建的(同名)变量可匹配任意值
```rust
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

#### 解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
        println!(
            "Move in the x direction {} and in the y direction {}",
            x,
            y
        );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
        println!(
            "Change the color to red {}, green {}, and blue {}",
            r,
            g,
            b
        )
    }
}
```
#### 复杂解构

解构嵌套枚举/结构体:

```rust
enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!(
            "Change the color to red {}, green {}, and blue {}",
            r,
            g,
            b
        )
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h,
            s,
            v
        )
    }
    _ => ()
}
```

解构混合数据:
`let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });`

#### 忽略值模式

##### _

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
```

实现 `trait` 时，当你需要特定类型签名但是函数实现并不需要某个参数时。此时编译器就不会警告说存在未使用的函数参数

`let _x = 5;`: 于变量名前以`_`开头，编译器会忽略未使用的变量并不警告

###### 嵌套_

在一个模式内部使用 `_` 忽略部分值
```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}
println!("setting is {:?}", setting_value);
```

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```
##### ..

`..` 模式会忽略模式中剩余的任何没有显式匹配的值部分，可自动隐式推断

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    },
}
```

#### @

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```
at 运算符（`@`）允许在创建一个存放值的变量的同时测试其值是否匹配模式。

### 集合(collections)

集合指向的数据是储存在堆上的，数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小

#### Vec
是泛型实现

`vec!`宏： `vec![1, 2, 3]`

添加: `v.push()`

##### 读取
`let ele: &T =  &v[idx]`，返回引用

`let ele = v.get(idx)`，返回`Option<T>`

在 `vector` 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中，从而使对原 `vector` 元素的引用成了野指针

##### 遍历
```rust
for i in &mut v {
    *i += 50;
}
for i in &v {
    println!("{}", i);
}
```

##### 多存储
`vector` 只能储存相同类型的值

枚举的成员都被定义为相同的枚举类型

故可在vector中存储已由枚举预先定义好的有限种不同类型的数据。

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

#### 字符串
`Rust` 的核心语言中只有一种字符串类型：`str`，`字符串 slice`，它通常以被借用的形式出现，`&str`，是一些储存在别处的 `UTF-8` 编码字符串数据的引用

`&String[..]` == `&str`

称作 `String` 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

`to_string` 方法从字符串字面量创建 `String`

`push_str` 方法来附加字符串 `slice`，从而使 `String` 变长

`push` 方法被定义为获取一个单独的字符作为参数，并附加到 `String` 中

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
```
`fn add(self, s: &str) -> String {`

之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。当 add 函数被调用时，Rust 使用了一个被称为 解引用强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。

String == Vec<u8>

`format!`: `format!("balabala...{}, {}, {}", var1, var2, var3)`，provide a format string which fuse multiple variables that realize **trait** `Display`

不能使用索引字符串

索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
`&hello[0..1]`报错

##### 遍历
```rust
for c in "नमस्ते".chars() {  // 单独的 Unicode 标量值
    println!("{}", c);
}
for b in "नमस्ते".bytes() {  // 返回每一个原始字节
    println!("{}", b);
}
```
####  HashMap
哈希 map 将它们的数据储存在堆上，是同质的：所有的键必须是相同类型，值也必须都是相同类型。
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对

如果队伍的名字和初始分数分别在两个 vector 中，可以使用 zip 方法来创建一个元组的 vector

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
`HashMap<_, _>`必要的类型推断，`_`可自动推断类型

对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者，

如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

`insert`可插入，可覆盖

`scores.entry(String::from("Yellow")).or_insert(50);`只在键没有对应值时插入，返回键相应值的可变引用

## 错误处理
`可恢复错误（recoverable）`: `Result<T, E>`，通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。

`不可恢复错误（unrecoverable）`: `panic!`，遇到错误时停止程序执行，即bug，比如尝试访问超过数组结尾的位置

`panic!宏` mode: 
- `展开（unwinding）`: Rust 会回溯栈并清理它遇到的每一个函数的数据
- `终止（abort）`: 不清理数据就退出程序，那么程序所使用的内存需要由操作系统来清理

```rust [Cargo.toml]
[profile.release]
panic = 'abort'
```
可使最终二进制文件大小变小

序列索引越界: `缓冲区溢出（buffer overhead）`

```bash
$ RUST_BACKTRACE=1 cargo run
```

`&[i32]`: 变长度数组

`[i32, i32]`: 定长度数组

## 泛型
`泛型（generics）`: 是具体类型或其他属性的抽象替代。

在 `impl` 之后声明泛型 `T` ，这样 Rust 就知道 `Class`对象类型的尖括号中的类型是泛型而不是具体类型。

泛型定义：
`struct ClassName<T>`

方法实现：
`impl<T> ClassName<T>`

编译时泛型代码经过了`单态化（monomorphization）`处理来保证效率。

`单态化`: 编译器在编译时为填充了泛型类型参数的每一个具体类型生成了非泛型的函数和方法实现，这些代码将进行 `静态分发(static dispatch)`，得名于编译器在编译时就知晓调用了什么方法

使用泛型时相比重复编写多特定类型的代码没有额外的`runtime overhead(运行时开销)`，

## trait
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

`trait`: (某一抽象类型)实现某些目的所必须的功能/行为的集合，由多种具体类型所共享
类似于`接口（interfaces）`

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

`相干性（coherence）`限制 / `孤儿规则（orphan rule）`: 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。其得名于不存在父类型。确保了其他人编写的代码不会破坏你代码。

### 默认实现
为某实例使用 `trait` 的默认实现时，可在 `trait` 的定义中将函数签名写为要实现的默认函数定义，并通过`impl trait_name for ClassName {}` 指定一个空的 `impl` 块

`grep`: Globally search a Regular Expression and Print.

### Trait as Params

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
以 `item: impl trait_name` 指定具有 `trait` 抽象特性的类型作为泛型函数的传入参数

### Trait Bound

```rust
pub fn notify<T: Summary>(item: T {
    item.summarize();
}
```
`pub fn notify(item1: impl Summary, item2: impl Summary) {`

`pub fn notify<T: Summary>(item1: T, item2: T) {`，限制了传递给参数 `item1` 和 `item2` 值的具体类型必须同为泛型 `T`。

### Multiple Trait Bounds

```rust
pub fn notify(item: impl Summary + Display) {
pub fn notify<T: Summary + Display>(item: T) {
```

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

### 返回实现了trait的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

```
省略实际的冗长的类型，设计 `闭包` 和 `迭代器` 时

为使用不同类型的值而设计的 `trait` 对象

### example

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest : &T = &list[0];

    for &item in list.iter() {
        if item > *largest {
            largest = item;
        }
    }

    largest
}
```

### blanket implementations(为实现了特定trait的泛型实现方法)
有条件的

```rust
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### 关联类型(associated types)

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```
```rust
impl Iterator<u32> for Counter  {
    // ...
}
```

使用泛型的trait时，可为单一对象类型实现不同的trait泛型参数，但不得不在每一个实现中标注trait泛型参数

关联类型只能为单个对象类型关联一种具体类型参数的trait

### 默认泛型类型参数

为泛型类型指定默认类型的语法是在声明泛型类型时使用 `<PlaceholderType=ConcreteType>`

### 运算符重载(Operators Overloading)

Rust 并不允许创建自定义运算符或重载任意运算符，只有在 `std::ops` 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}


trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

`RHS`: right hand side

### 完全限定语法与消歧义：调用不同trait相同名称的方法

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```
```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();  // 调用直接实现在 Human 上的 fly 方法
    // Human::fly(&person);
}
```
如果有两个方法都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现。

若无 self 参数：
```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

可用完全限定语法 `<Dog as Animal>::baby_name()`，指定调用的是 Dog 上 Animal trait 实现中的 baby_name 函数

一般地：`<Type as Trait>::function(receiver_if_method, next_arg, ...);`

### 父(超)trait (supertrait)

某个trait的实现依赖于另一个trait的功能，被依赖的trait即父trait，因而要求实现了父trait的类型

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}
```
类似于为 trait 增加 trait bound

### newtype模式

绕开孤儿原则的限制，可在外部类型上实现外部trait

在一个元组结构体中创建一个新类型，其带有一个字段作为希望实现外部trait的外部类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现外部trait

`Newtype`: 源自 Haskell 编程语言。使用这个模式没有运行时性能消耗，这个封装类型在编译时就被省略了。

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；想像 Vec<T> 那样对待 Wrapper，可为封装类型实现 Deref trait，代理到self.0，这是一种解决方案。

## 高级类型

### newtype模式

- 静态的确保某值不被混淆
- 用来表示一个值的单元
- 抽象掉一些类型的实现细节

### 类型别名(type alias)

`type Kilometers = i32;`: 意味着 `Kilometers` 是 `i32` 的 **同义词（synonym）**

主要用途是为很长的类型重命名，减少重复

`type Result<T> = std::result::Result<T, std::io::Error>;`

### never type

`!`: empty type，因其没有值，又称 never type，在函数从不返回的时候充当返回值，相当于 C/C++ 的 void

`发散函数(diverging functions)`: 发散函数

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
`continue`返回`!`，把控制权交回上层循环

描述 `!` 的行为的正式方式是 never type 可以**强转**为任何其他类型，故Rust可自行推断`guess`的类型为`u32`

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```
`panic!` 是 `!` 类型，被强转为 `T` 类型，故整个 `match` 表达式的结果为 `T` 类型

### 动态大小类型

dynamically sized types，DST,unsized types

允许我们处理只有在运行时才知道大小的类型

```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```
Rust 需要知道应该为特定类型的值分配多少内存，同时所有同一类型的值必须使用相同数量的内存，这对于str无法实现！

虽然 &T 是一个储存了 T 所在的内存位置的单个值，但&str则是**两个**值：str的地址和其长度。这样，&str 就有了一个在编译时可以知道的大小：它是 usize(地址信息) + usize(长度信息)

这是 Rust 中 `DST` 的常规用法：它们有一些额外的元信息来储存动态信息的大小
 
每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型，例如为了将 trait 用于 trait 对象，必须将他们放入指针之后，比如 `&dyn Trait` 或 `Box<dyn Trait>`，`Rc<dyn Trait>`

`Sized` trait: 为了处理 DST，Rust 内置的确定一个类型的大小是否在编译时可知的trait

```rust
fn generic<T>(t: T) {
    // --snip--
}
```
实际上应当为
```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

泛型函数默认只能用于在编译时已知大小的类型，可如下放宽此限制:
```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```
`?Sized` trait bound 指 `T` 可能是也可能不是 `Sized` 的，此语法只能用于 Sized ，而不能用于其他 trait

另外将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后

## lifetime
`生命周期（lifetime）`： 引用保持有效的作用域

### 借用检查器（borrow checker）
比较作用域来确保所有的借用都是有效的


### 标注
```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```
单个生命周期标注本身没有多少意义，在函数中，多个引用的泛型生命周期参数若相同则意味着它们至少拥有相同的生命周期，即传入参数后的输入变量的生命周期都削减到了与相同者当中的最小者(即`'a`)/相重叠部分一致（函数调用完毕后恢复原样），而返回量的生命周期则相同标注者的生命周期一致。

过编译原则：返回引用的生命周期应确定，而非有多种可能，避免悬垂引用，进而所有输入的引用的生命周期都应当可确定。

即，**为满足 `生命周期省略原则(lifetime elision rules)` 的前提，编译器自行隐式推断且添上人为的生命周期标注后不存在没有计算出生命周期的引用**

#### 生命周期省略原则
适用于 fn 定义，以及 impl 块。
`输入生命周期(input lifetime)`原则：
1. 每一个输入参数的引用都有其自己的**输入生命周期**参数，

有一个引用输入参数的函数有一个输入生命周期参数：`fn foo(x: & i32)` => `fn foo<'a>(x: &'a i32)`

有两个引用输入参数的函数有两个不同的生命周期参数: `fn foo(x: & i32, y: & i32)` => `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此类推。

`输出生命周期(output lifetime)`原则：

2. 若只有一种输入生命周期参数，则所有**输出生命周期**参数与之相同

`fn foo(x: & i32) -> & i32` => `fn foo<'a>(x: &'a i32) -> &'a i32`

`fn foo(x: & i32, y: & i32) -> & i32` => `fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'??? i32` 推断失败

适于`方法`:

3. 若有多个输入生命周期参数，且其一为 `&self` 或 `&mut self`，则说明是某对象的**方法**，那么所有**输出生命周期**参数与 `self` 的相同

这意味着经常不需要在方法签名中标注生命周期

`fn foo<'a>(x: &'a i32, y: & i32) -> &'a i32` => `fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32` 推断成功

推断成功后还需判断函数实现中返回值的生命周期是否同标注中相符合，如：

```rust
fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    y
}
```

以及若在调用函数后有无于生命周期外使用返回值，如：

```rust
let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is {}", result);
```

当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用*没有*指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，因而它将会在函数结束时离开作用域，从而是一个`悬垂引用`：

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

`静态生命周期`: `'static`，其生命周期能够存活于整个程序期间。

#### 结构体定义中的生命周期标注
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```
这个标注意味着 `ImportantExcerpt` 的实例不能比其 `part` 字段中的引用存在的更久。


#### 结合泛型类型参数、trait bounds 和生命周期
```rust
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

`fn foo<'a>(x； &'a str, y: &'a str, c: impl Display) -> &'a str`
`fn foo<'a, T: Display>(x； &'a str, y: &'a str, c: T) -> &'a str`
```rust
fn foo<'a, T>(x: &'a str, y: &'a str, c: T) -> &'a str
    where T: Display
```
泛型类型参数，生命周期参数放在`<...>`列表里，再在签名末尾由`where`定义`trait bounds`

## Auto Test

测试函数标注：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        // ...
    }
}
```

执行编译测试：`cargo test xxx`

### 正确测试

检查代码是否返回期望的正确的值

测试语句：
`assert!()`: pass when the assertion is true

`assert_eq!()`

`assert_ne!()`

`assert_eq!`，`assert_ne!` 宏在底层分别使用了 `==` 和 `!=`，这意味着待比较的类型需实现 `PartialEq` 的 `trait`才能断言两者值是否相等，需实现 `Debug` 的 `trait` 才能在断言失败时打印他们的值。因为这两个 `trait` 都是派生 `trait`，通常可以直接在结构体或枚举上添加 `#[derive(PartialEq, Debug)]` 

### 错误测试

检查代码是否按照期望处理错误

`#[should_panic]`: 函数中的代码 `panic` 时 `pass`，全程跑通则 `failed`

`#[should_panic(expected = "some message")]` `expected` 参数提供的值是 `panic!(Err_message)` 信息`Err_message`的子串

`expected` 信息的选择取决于 `panic` 信息有多独特或动态，和你希望测试有多准确。

### 重定向输出

部分终端都提供了两种输出：`标准输出（standard output，stdout）` 对应一般信息，`标准错误（standard error，stderr）` 则用于错误信息。

`$ cargo run > output.txt`: `>` 重定向标准输出流写入到 `output.txt`文件中而不是终端屏幕上。
错误信息发送到标准错误流，而在命令行终端中显示

## 函数式编程(Functional Programming)
函数式编程风格通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。


闭包（Closures），一个可以储存在变量里的类似函数的结构
迭代器（Iterators），一种处理元素序列的方式
如何使用这些功能来改进第 12 章的 I/O 项目
这两个功能的性能（剧透警告： 他们的速度超乎你的想象！）

### 闭包(closures)

可以保存进变量或作为参数传递给其他函数

可以捕获环境的匿名函数，闭包被定义时周围的作用域被称为其`环境(environment)`，而非闭包被调用时调用者附近的作用域

若强制闭包获取其使用的环境值的所有权，可在参数列表前使用 `move` 关键字

```rust
| param1, param2, ... | {
    // ...
}
```
or 

`| param1, param2, ...| ...`

闭包体最后一行的返回值作为调用闭包时的返回值

`fn` 函数中必要的参数和返回值类型标注是因为其为暴露给用户的显式接口的一部分，然而闭包储存在变量中并被使用，不用命名或暴露给库的用户调用，也故而得名。

类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型标注

```rust

#![allow(unused)]
use std::thread;
use std::time::Duration;
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

闭包定义会为每个参数和返回值推断一个具体类型。如果尝试对同一闭包使用不同类型则会得到类型错误。

`memoization`/`lazy evaluation(惰性求值)`: 可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。

闭包从环境中捕获一个值，会在闭包体中储存这个值以供使用，会使用内存并产生额外的开销

闭包可以通过三种方式捕获其环境，这直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用，三者被编码为标准库中 Fn 系列 trait:
- `FnOnce` **消费**从周围作用域捕获的变量，闭包必须获取其所有权并在**定义闭包**时将其**移动**进闭包。

其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。

- `FnMut` 获取可变的借用值因此可以改变其环境
- `Fn` 从其环境获取不可变的借用值

#### 返回闭包

闭包表现为 trait，这意味着不能直接返回闭包。

对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值

但是这不能用于闭包，因为他们没有一个可返回的具体类型，`Fn`/`fn`不允许作为返回值类型

```rust
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```
错误又一次指向了 Sized trait！Rust 并不知道需要多少空间来储存闭包

可修改为:
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

### 迭代器(iterator)

负责遍历序列中的每一项和决定序列何时结束的逻辑。

在 `Rust` 中，迭代器是 `惰性的（lazy）`，即在调用方法使用迭代器之前它都不会有效果

原型: 
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
`next`: `Iterator` 实现者唯一被要求必须定义的方法，按顺序一次返回迭代器中的一个项，封装在 `Some` 中，当迭代器结束时，它返回 `None`

`Item`: 迭代器返回元素的类型

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {}", val);
}
```
`for` 循环会获取 `v1_iter` 所有权(不可变借用)，并在后台使`v1_iter`可变，无需自行使 `v1_iter` 可变

```rust
let v1 = vec![1, 2, 3];

let mut v1_iter = v1.iter();

assert_eq!(v1_iter.next(), Some(&1));
assert_eq!(v1_iter.next(), Some(&2));
assert_eq!(v1_iter.next(), Some(&3));
assert_eq!(v1_iter.next(), None);
```
在迭代器上调用 `next` 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 `消费（consume）`了，或使用了迭代器，因而`v1_iter`需要是可变的；

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

let total: i32 = v1_iter.sum();
```

调用 `sum` 之后不再允许使用 `v1_iter` 因为调用 `sum` 时它会获取迭代器的所有权。

`into_iter`: 创建一个获取 `vector` 所有权的迭代器

#### 迭代器适配器(iterator adaptors)

`Iterator` trait 中定义的方法

在迭代器链式遍历过程中多次调用迭代器适配器以生成新的迭代器，不过由于所有迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

`filter` 方法: 筛选迭代器中符合闭包体中定义条件的项并保留到新迭代器中，否则放弃掉

##### 自定义迭代适配器

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

#### 零成本抽象

`零开销(zero-overhead)`: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

迭代器是 `Rust` 的 `零成本抽象(zero-cost abstractions)` 之一，意味着抽象并不会引入运行时开销，因为其被编译成了与手写的底层汇编代码大体一致性能的代码。

what the fuck?
```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```
遍历 `coefficients` 的值完全用不到循环：`Rust` 知道这里会迭代 12 次，所以它**展开**了循环

`展开(unroll)`: 一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

### 函数指针

向函数传递常规函数

`Fn`: 闭包 trait，需声明一个带有 `Fn` 作为 trait bound 的泛型参数。

`fn`: 函数指针这一类型，可直接指定作为参数

函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数，因而倾向于编写使用泛型和闭包 trait 的函数

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)   // 完全限定语法
    .collect();
```

### 宏(Macro)

包括:
- 使用 `macro_rules!` 的 **声明***（Declarative）*宏
- 三种**过程***（Procedural）*宏：
    - 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
    - 类属性（Attribute-like）宏定义可用于任意项的自定义属性
    - 类函数宏看起来像函数不过作用于作为参数传递的 token

从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的 **元编程***（metaprogramming）*

宏可以在编译器翻译代码前展开

一个函数标签必须声明函数参数个数和类型。相比之下，宏能够接受不同数量的参数

#### 声明宏

声明宏允许我们编写一些类似 Rust `match` 表达式的代码

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
`$x:expr`: 匹配 Rust 的任意表达式
`$( $x:expr ) `: 捕获符合括号内模式的值
`*`: 该模式匹配零个或更多个`*`之前的模式
`,`: 匹配逗号

#[macro_export] 标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的

#### 过程宏

`TokenStream` 类型由包含在 Rust 中的 `proc_macro` crate 定义，宏操作的源代码构成了输入 TokenStream，宏产生的代码是输出 TokenStream

参考项目文件

## 面向对象编程（Object-Oriented Programming，OOP）
一种模式化编程方式。所共享的一些特性往往是对象、封装和继承三要素

### 对象

**一个 `对象` 包含数据和操作这些数据的过程，这些过程通常被称为 `方法`**

Rust 刻意不将结构体与枚举称为 “对象”，因为其字段中的数据和 impl 块中定义的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中。

类似于enum，struct，trait等实现

不能向 trait 对象增加数据，trait 允许对通用行为进行抽象

### 封装（encapsulation）

- **对象的实现细节不能被使用对象的代码获取到，唯一与对象交互的方式是通过对象提供的公有 `API`；**
- **使用对象的代码无法深入到对象内部并直接改变数据或者行为**

封装使得改变和重构对象的内部时无需改变使用对象的代码。

### 继承（Inheritance）
**一个对象可以定义为继承另一个对象的定义，从而可以获得父对象的数据和行为，而无需重新定义**

选择继承有两个主要的原因:
- 重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。
相反，`Rust` 可以使用默认 `trait` 方法实现来进行共享

- 与类型系统有关：多态（polymorphism），表现为子类型可以用于父类型被使用的地方，即如果多种对象共享特定的属性，则可以相互替代使用。

    很多人将多态描述为继承的同义词。不过它是一个有关可以用于多种类型的代码的更广泛的概念。
    
    对于继承来说，这些类型通常是子类。`Rust` 则通过**泛型**来对不同的可能类型进行抽象，并通过 `trait bounds` 对这些类型所必须提供的内容施加约束，即称作 `bounded parametric polymorphism`

近来继承作为一种语言设计的解决方案在很多语言中失宠了，因为其时常带有共享多于所需的代码的风险。子类不应总是共享其父类的所有特征，但是继承却始终如此。如此会使程序设计更为不灵活，并引入无意义的子类方法调用，或由于方法实际并不适用于子类而造成错误的可能性。

某些语言还只允许子类继承一个父类，进一步限制了程序设计的灵活性。

### 为不同类型而设计的trait对象

存储多个涵盖了有限多种类型的枚举的向量，实现混合储存的容器，然而无法扩展有效的类型集合，所有可能的类型都被预先定义好了

`GUI(Graphical User Interface)`: 图形用户接口，通过遍历列表并调用每一个项目的 draw 方法来将其绘制到屏幕上

in src/lib.rs:
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T> {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

不同于
```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
```
trait bound限制了components中各元素类型必须一致，而关键字`dyn`标明的 trait 对象，允许在运行时替代实现了该trait的多种具体类型

in src/lib.rs:
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

in src/main.rs:
```rust
use gui::{Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```
调用外部库的开发者，除了预先编写在里的类型之外，自行额外自定义了新的类型 `Select Box`，然而只要它实现了 `Draw` trait，就可以像库里的`Button`一样由 `Screen` 操作并绘制这个新类型

trait 对象，只关心类型所反映的特征信息而不是具体类型本身，类似于动态类型语言中 `鸭子类型(duck typing)` 的概念: 如果它走起来像一只鸭子，叫起来像一只鸭子，那么它就是一只鸭子！

`动态分发(dynamic dispatch)`: 编译器在编译时无法知晓所有可能用于 trait 对象代码的类型，无法知晓调用了什么方法，因而会生成只有在运行时(使用trait对象中的指针)才能确定要调用什么方法的代码。

只有 `对象安全(object safe)` 的 trait 才可以组成 trait 对象

对象安全原则:

一个 trait 中所有的方法都满足: 
- 返回值类型不为 Self

如果 trait 方法返回具体的 Self 类型，但是 trait 对象忘记了其真正的类型，那么方法不可能使用已经忘却的原始具体类型

- 方法没有任何泛型类型参数

同理对于泛型类型参数来说，当使用 trait 时其会放入具体的类型参数：此具体类型变成了实现该 trait 的类型的一部分。当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么。

Example:
```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

### 状态模式(state pattern)

该模式的关键在于一个值有某些内部状态，体现为一系列的状态对象，同时值的行为随着其内部状态而改变。状态对象共享功能：当然，在 Rust 中使用结构体和 trait 而不是对象和继承。每一个状态对象负责其自身的行为，以及该状态何时应当转移至另一个状态。持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情。

如果要创建一个不使用状态模式的替代实现，则可能会在 Post 的方法中，或者甚至于在 main 代码中用到 match 语句，来检查博文状态并在这里改变其行为。这意味着需要查看很多位置来理解处于发布状态的博文的所有逻辑！这在增加更多状态时会变得更糟：每一个 match 语句都会需要另一个分支。

对于状态模式来说，Post 的方法和使用 Post 的位置无需 match 语句，同时增加新状态只涉及到增加一个新 struct 和为其实现 trait 的方法。

状态模式的一个缺点是因为状态实现了状态之间的转换，一些状态会相互联系。

另一个缺点是会发现一些重复的逻辑

## 智能指针
### Box<T>
允许将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针

- 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
- 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
- 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候


`递归类型(recursive type)`: 其值的一部分可以是相同类型的另一个值。这种值的嵌套理论上可以无限的进行下去

`cons list`: 每一项都包含两个元素，当前项的值和下一项，其最后一项值包含一个叫做 Nil 的值且没有下一项, 它宣布列表的终止

```rust
enum List {
    Cons(i32, List),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

实现无法计算 `List` 实际大小

Rust 需要在编译时知道类型占用多少空间，在类型定义中给入已知大小的 box 指针指向下一同类型的值即可实现

enum只会使用一个成员，所以其需要的最大空间是存储其最大成员所需的空间大小

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, List, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

此时任何 `List` 值最多需要一个 `i32` 加上 `box` 指针数据的大小(usize)。

`Box<T>` 类型是一个智能指针，因为它实现了 `Deref trait`，它允许 `Box<T>` 值被当作引用对待

#### Deref trait

实现 `Deref trait` 允许重载(不可变引用的)`解引用运算符（dereference operator）*`

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

`*y` == `*(y.deref())`

`* 运算符` 实际被替换成了先调用 `deref` 方法再接着使用 `*` 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换

函数和方法的隐式解引用强制转换（deref coercions），发生于被传递的值的引用与函数或方法中定义的参数类型不匹配时，这时会有一系列的 `deref` 方法被调用，把我们提供的参数类型转换成函数或方法需要的参数类型，无需增加过多显式使用 & 和 * 的引用和解引用的负担。

mutability: 
T: Deref<Target=U> ：从 &T 到 &U。
T: DerefMut<Target=U> ：从 &mut T 到 &mut U。
T: Deref<Target=U> ：从 &mut T 到 &U。

可变引用可强转为不可变引用，但反之是 **不可能** 的

因为将不可变引用转换为可变引用则需要内存只能有一个不可变引用，然而这点无法保证

#### Drop trait

`Box<T>` 值离开作用域时，由于 `Box<T>` 类型 `Drop trait` 的实现，box 所指向的堆数据也会被清除，不需要在程序中到处编写在实例结束时清理这些变量的代码 

```rust
impl Drop for ClassName {
    fn drop(&mut self) {
        // ...
    }
}
```

##### std::mem::drop

提早在作用域结束之前强制释放变量

### Rc<T>

引用计数(**r**eference **c**ounting)，记录一个内存被引用的数量来知晓其是否仍在被使用，倘若有零个引用，代表无任何有效引用并自动清理

通过不可变引用， `Rc<T>` 允许在程序的多个部分之间**只读**地共享数据

只用于单线程场景

```rust
let a = Cons(5,
    Box::new(Cons(10,
        Box::new(Nil))));
let b = Cons(3, Box::new(a));
let c = Cons(4, Box::new(a));
```
创建 List b 时，a已经被移动进了b

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use crate::List::{Cons, Nil};

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));
```

也可`a.clone()`，但`Rc::clone`只会增加引用计数，更快

`Rc::strong_count(Rc_ptr)`: 获取引用计数

`Drop trait` 的实现当 `Rc<T>` 值离开作用域时自动减少引用计数

### RefCell<T>

在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响

在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。静态分析，正如 `Rust` 编译器，是天生保守的。但代码的一些属性不可能通过分析代码发现：其中最著名的就是 `停机问题（Halting Problem）`

- `Rc<T>` 允许相同数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者。
- `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>` 仅允许在编译时执行不可变借用检查，倘若违反会触发编译错误；`RefCell<T>` 允许在运行时执行不可变或可变借用检查，倘若违反则会触发panic并退出。

`内部可变性(Interior mutability)`： 可以在即便 `RefCell<T>` 自身是不可变的情况下修改其内部的值。令一个值在其方法内部能够修改自身，而在值方法外部的代码中仍视为不可变

`测试替身(test double)`: 代表一个在测试中替代某个类型的类型，如 `mock` 对象

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

不能修改 `MockMessenger` 来记录消息，因为 `send` 方法获取了 `self` 的不可变引用

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

当创建不可变和可变引用时，分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow(返回 Ref<T> 类型的智能指针) 和 borrow_mut(返回 RefMut<T> 类型的智能指针) 方法

RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一

RefCell<T> 在任何时候内部计数只允许有多个不可变借用或一个可变借用

### Rc<T> 和 RefCell<T> 实现多个可变借用

Rc<T> 允许对相同内存有多个所有者，不过只能提供内存的不可变访问，只需实现储存了 RefCell<T> 的 Rc<T>，就可以得到有多个所有者并且可以修改的内存

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

### 内存泄露

`内存泄露(memory leak)`: 永远也不会被清理的内存

`Rust` 并不保证完全地避免内存泄漏，这意味着内存泄漏在 `Rust` 被认为是内存安全的

可从`Rc<T>` 和 `RefCell<T>` 看出：创建**引用循环**的可能性是存在的。这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

#### 引用循环

```rust
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```
```rust
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

println!("a initial rc count = {}", Rc::strong_count(&a));
println!("a next item = {:?}", a.tail());

let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

println!("a rc count after b creation = {}", Rc::strong_count(&a));
println!("b initial rc count = {}", Rc::strong_count(&b));
println!("b next item = {:?}", b.tail());

if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
}

println!("b rc count after changing a = {}", Rc::strong_count(&b));
println!("a rc count after changing a = {}", Rc::strong_count(&a));

// Uncomment the next line to see that we have a cycle;
// it will overflow the stack
// println!("a next item = {:?}", a.tail());
```
对于单向链表，从new head开始释放，后一项在前一项释放后的引用计数减1为0，进而链式释放
对于上例，Rust 首先丢弃 b，使 b 中 Rc<List> 实例的引用计数减 1。然而，因为 a 仍然引用 b 中的 Rc<List>，Rc<List> 的引用计数是 1 而不是 0，所以 b 中的 Rc<List> 在堆上的内存不会被丢弃。接下来 Rust 会丢弃 a，同理这会将 a 中 Rc<List> 实例的引用计数从 2 减为 1。这个实例的内存也不能被丢弃。这些列表的内存将永远保持未被回收的状态
在末尾注释段解注后，对应Some(ref)类型会经过一系列隐式解引用强制转换最终由 Display trait 方法实现输出，然而列表已形成了闭环，最终只会不断循环下去直至爆栈

一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。

##### Weak<T>

调用 `Rc::downgrade` 创建 `弱引用（weak reference）`，`Weak<T>` 类型的智能指针，并将 `weak_count` 加1

强引用代表如何共享 `Rc<T>` 实例的所有权，但弱引用并不属于所有权关系。任何弱引用的循环无需使 `weak_count` 为0，只需在其相关的 `strong_count` 为 0 时即可被打断，从而使 `Rc<T>` 实例被清理

`Weak<T>` 可描述内部连接结构
而`Rc<T>` 则是外部调用内部引用的接口，倘若全被清理，内部的数据也应得到清理

#####  树结构

父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃，即父节点有对子节点的强引用

然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在，即子节点有对父节点的弱引用

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```
没有无限的输出表明这段代码并没有造成引用循环

Quiz:
```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

## 无畏并发(fearless concurrency)

多线程: 
- `并发编程(Concurrent programming)`: 程序的不同部分相互独立的执行，异步
- `并行编程(parallel programming)`: 程序不同部分于同时执行，同步

通过利用所有权和类型检查，在 Rust 中很多并发错误都是 编译时 错误，而非运行时错误，因而无需花费大量时间尝试重现运行时并发 bug 出现的特定情况，Rust 会拒绝编译不正确的代码

在大部分现代操作系统中，已执行程序的代码在一个 `进程(process)` 中运行，操作系统则负责管理多个进程。在程序内部，也可有多个同时运行的独立部分，即 `线程（threads）`

多线程可改善运行速度，但因无法预先保证不同线程中的代码的执行顺序，会引发下列问题：
- 竞争状态(Race conditions): 多个线程以不一致的顺序访问数据或资源
- `死锁(Deadlocks)`: 两个线程相互等待对方停止使用其所拥有的资源，这会阻止它们继续运行
- 只会发生在特定情况且难以稳定重现和修复的 bug

`1:1`线程模式: 由编程语言调用操作系统（OS）内置 API 创建 OS 线程，一个 OS 线程对应一个语言线程，故而得名

`M:N`线程模式: 编程语言提供的线程，称作**绿色(green)**线程，在不同数量的 OS 线程的上下文中得到执行，`M`个绿色线程对应`N`个 OS 线程

`运行时`: 概念模糊，在不同语境中含义不同，此处指二进制文件中包含的由语言自身提供的代码。这些代码根据语言的不同可大可小，任何非汇编语言都会有一定数量的运行时代码

由于 Rust 是较为底层的语言，Rust 标准库只提供了 1:1 线程模型实现。

如果愿意牺牲性能来换取抽象，以获得对线程运行更精细的控制及更低的上下文切换成本，可使用实现了 M:N 线程模型的 crate

编程语言提供的线程被称为 绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。为此，绿色线程模式被称为 M:N 模型：M 个绿色线程对应 N 个 OS 线程，这里 M 和 N 不必相同。

默认**并发和(或)并行**为**并发**，此处不做区分

### 创建线程

`thread::spawn` 新建线程，返回值类型 `JoinHandle`，拥有所有权

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        } // 若闭包调用完毕，则该线程结束
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 等待所有线程结束
}
```
这两线程可能会轮流运行，不过并不保证如此：这依赖操作系统如何调度线程

输出大体上如下：
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!

这是因为当主线程(main)结束时，新线程也会结束，而不管其是否执行完毕

`join` 方法: 等待自身线程结束。
`阻塞(Blocking)`: 意味着阻止当前线程执行工作或退出

于主线程(main)中调用handle.join()，阻塞主线程，等到handle所代表的线程结束，此处即等到所有线程结束

### move 闭包

经常与 `thread::spawn` 一起使用，因为它允许在一个线程中使用另一个线程的数据，捕获了定义该闭包的线程的环境

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    // uncomment the next line to see v aborted;
    // it will cause a dangling reference error in the spawned thread
    // drop(v); 

    handle.join().unwrap();
}
```
Rust 会 **推断** 如何捕获 v，然而 Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

### 消息传递(message passing)实现在线程间传送数据

不要通过共享内存来通讯；而是通过通讯来共享内存

任何编程语言中的通道都类似于单所有权，因为一旦将一个值传送到通道中，将无法再使用这个值。

共享内存类似于多所有权：多个线程可以同时访问相同的内存位置，存有风险

**发送者(transmitter) -> 通道(channel) -> 接收者(receiver)**
ROS2: 发布者(publisher) -> 主题(topic) -> 订阅者(subscriber)
当发送者或接收者任一被丢弃时可以认为通道被 *关闭（closed）* 了

`std::sync::mpsc`: multiple producer, single consumer

`mpsc::channel`: create a channel, return a tuple of a transmitter and a receiver

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Uncomment the next line to panic;
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

send函数获取`val`值的所有权并移动其值归到receiver所有，以防原线程对值可能的修改导致的数据竞争现象，这也是 `mpsc` 要求只能有一个receiver的原因

#### 多发布

```rust
// --snip--
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

#### 共享状态并发

`互斥器(mutex)`: mutual exclusion，任意时刻，其只允许一个线程访问某些数据

`锁(lock)`: 互斥器通过锁系统 `保护(guarding)` 其数据。其记录了哪个线程有互斥器数据的排他访问权

- 线程应在使用数据之前尝试获取锁。
- 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

`std::sync::Mutex<T>` 是一个智能指针，调用`lock`方法返回`MutexGuard`类型的智能指针，其实现了 `Deref` trait 来指向其内部数据；也提供了 `Drop` trait 实现当 `MutexGuard` 离开作用域时自动释放锁

`Rc<T>` 并不能安全的在线程间共享，因为其并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断

`Arc<T>`: atomically reference counted，原子引用计数，可安全用于并发环境的`Rc<T>`类型

线程安全带有性能惩罚，故并未为所有原始类型实现原子性

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
`Arc<T>`虽是不可变引用，但`Mutex<T>`的`lock`方法返回一个可修改内部值的可变引用，提供了内部可变性

也就是说，`RefCell<T>` 之于 `Rc<T>`，等若于并发环境下 `Mutex<T>` 之于 `Arc<T>`

确保所使用的类型可以用于并发环境的 trait 之一


### 使用Sync和Send trait的可拓展并发

`std::marker::Send` 标记 trait 表明类型的所有权可以在线程间传递，

Rust 几乎所有基本类型都是 `Send` 的，任何完全由 `Send` 的类型组成的类型也会自动被标记为 `Send`，除了`Rc<T>`，裸指针(raw pointer)……

`std::marker::Sync` 标记 trait 表明一个实现了 `Sync` 的类型可以安全的在多个线程中拥有其值的引用。对于任意类型 `T`，如果 `&T` 是 `Send` 的话 `T` 就是 `Sync` 的

几乎所有基本类型是 `Sync` 的，完全由 `Sync` 的类型组成的类型也是 `Sync` 的，除了`Rc<T>`，`RefCell<T>`，`Cell<T>`……

通常并不需要手动实现 Send 和 Sync trait，手动实现这些标记 trait 涉及到编写不安全的 Rust 代码
那么如何得到发布的博文呢？我们希望强制执行的规则是草案博文在可以发布之前必须被审核通过。等待审核状态

## 不安全Rust(unsafe Rust)

不安全 Rust 之所以存在:
- 静态分析本质上是保守的。
- 底层计算机硬件固有的不安全性。Rust 需要能够直接与操作系统交互，甚至于能编写操作系统这样的底层系统！

能力: 
- 解引用裸指针
- 调用不安全的函数或方法
- 访问或修改可变静态变量
- 实现不安全 trait
- 访问 union 的字段

`unsafe` 只提供了上述那五个不会被编译器检查内存安全的功能，而并不会关闭借用检查器或禁用任何其他 Rust 安全检查

通过要求这五类操作必须位于标记为 `unsafe` 的块中，就能够知道任何与内存安全相关的错误必定位于 `unsafe` 块内

### 解引用裸指针

裸指针与引用和智能指针的区别：
- 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
- 不保证指向有效的内存
- 允许为空
- 不能实现任何自动清理功能

`*mut T`: 可变裸指针
`*const T`: 不可变裸指针，不可变意味着指针解引用之后不能直接赋值（，但指针指向地址可更改？）

可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针

`as`: 