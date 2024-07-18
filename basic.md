## cargo

### 模块系统(the module system)
- `模块（Modules）`和 `use`： 允许你控制作用域和路径的私有性。
- `路径（path）`：一个命名例如结构体、函数或模块等项的方式

#### crate
`crate`: all types of rust source code(rs files) after compiled, 包括：
`binary crate`: executable
`library crate`: libxx.so in `C/C++`
表现各模块的树形结构

#### 包（package） 
提供一系列功能的一个或者多个 crate。包中包含的 `Cargo.toml` ，阐述如何去构建这些 `crate`。
至多 **只能** 包含一个 `library crate`， `src/lib.rs`；
可包含任意多个二进制 `binary crate`, `src/main.rs`, `src/bin/xx.rs`；
包中至少包含一个 `crate`，无论是库的还是二进制的。

Cargo 遵循的一个约定：
`src/main.rs` 是一个与包同名的 `binary crate` 的 `crate root`
`src/lib.rs`，是一个与包同名的 `library crate`的 `crate root`
`Rust` 编译器以`crate root`为起始点，构成名为 `crate` 的模块，且其位于 `模块树` 的模块结构的根部，并由此派生出所有`crate`定义的各模块
可将一个 crate 的功能保持在其自身的作用域(`scope`/`namespace`)中
外部包引入后将接到`crate root`下
`标准库(std)`也是外部 `crate`，但因其随 `Rust` 语言一同分发，无需修改 `Cargo.toml` 来引入 `std`

#### 模块(mod)
将一个 crate 中的代码进行分组，以提高可读性与重用性。
控制项的 `私有性`，即项是可以被外部代码使用的(`public`)，还是作为一个内部实现的内容，不能被外部代码使用（`private`）。
相当于C/C++的`class`

#### 路径(path)
用于引用模块树中模块树中某特定项
`绝对路径(absolute path)`: 从 `crate root` 开始，以`crate::`开头
`相对路径(relative path)`: 从当前模块开始
    - `super`: 从父模块开始的相对路径
    - `self`: 从当前模块开始的相对路径，可省略

`Rust` 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
父模块及其子模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项，兄弟模块之间
使用 pub 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

被引用者的路径+引用者能否访问被引用者



结构体定义前`pub`，将其变作`public`，但其`field`仍为`private`，对此类结构体需提供一个`public`的关联构造函数来构造其实例
枚举定义前`pub`，则其所有成员变为`public`
`use`创建`软链接(symbolic link，又称符号连接)`
习惯上，`use`将函数所在父模块引入作用域，调用函数时需指定父模块；引入结构体、枚举和其他项时则指定其完整路径
`use path as sign`
`重导出(reexporting)`: `pub use path`，除引入到当前作用域外，还可导出路径到父模块
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

### package version

`^0.8.3`: >= 0.8.3 && < 0.9.0

when `cargo build` initially , Cargo 计算出所有符合要求的依赖版本并写入 Cargo.lock 文件。当将来构建项目时，Cargo 会发现 Cargo.lock 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。项目会持续使用 0.8.3 直到你显式地升级`cargo update`.

### debug

`runtime error`
`compile error`

## variables

`mut` : mutability of a variable
`let`: declaration of a variable

枚举`std::Result` 的成员是 `Ok` 和 `Err`，Ok 成员表示操作成功，且 Ok 内部包含成功生成的值。Err 成员则意味着操作失败，并且包含失败的前因后果。

Result 类型的值，就像任何类型的值一样，都有为其定义的方法。io::Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示传递给 expect 的参数。如果 read_line 方法返回 Err，则可能是底层操作系统引起的错误结果。如果 io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回，以便你可以使用它。在本例中，这个值是用户输入的字节数。

valid number sign: `1_000_000u32`，以`_`任意分割数字，末尾添上数据类型如`u32`
`usize`/`isize`，`size`相应于系统架构`arch`变量，为`32位`或`64位`

## key words

`遮蔽(shadow)`: 之前声明过的变量名经由重复声明关联到新的数据类型与值
- `const`: 命名全大写，shadow unpermitted，只能声明一次，类型与值不可再更改
- `裸let`: shadow permitted, 可声明多次，以此更改类型甚至值
`内部作用域(inner scope)`可访问外作用域变量，遮蔽只在同一作用域内生效，不能跨作用域，或看作是遮蔽了一个为访问外部变量而传入内部作用域的局部变量（浅拷贝？所有权移交给谁？），

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tuple.0;
let six_point_four = tuple.1;
```

Rust 是一门`基于表达式（expression-based）`的语言，
`语句（statement）`: 执行一些操作但不返回值的指令
`表达式（expression）`: 计算并产生一个值。
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
`range`函数：(1..4).rev() // .rev 即reverse

## 循环

`循环标签(loop label)`: `'label: loop {...}`

`loop {...}`
`while constraint {...}`
`for element in array {...}`

栈：后进先出（last in, first out）
堆是缺乏组织的
入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。
访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。
当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。
跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。
对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
必须在运行时向内存分配器（memory allocator）请求内存。
需要一个当我们处理完 String 时将内存返回给分配器的方法。
有借有还，再借不难。
第一部分在各编程语言中是非常通用的。
然而，第二部分实现起来就各有区别了。在有 垃圾回收（garbage collector，GC）的语言中，GC 记录并清除不再使用的内存，而我们并不需要关心它。在大部分没有 GC 的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。
Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。当变量离开作用域，Rust 为我们调用一个特殊的函数`drop`。

长度表示 String 的内容当前使用了多少字节的内存。容量是 String 从分配器总共获取了多少字节的内存


为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效
Compile Error:
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");
```

`浅拷贝(shallow copy)`：仅拷贝引用
`深拷贝(deep copy)`：拷贝引用指向的内容 => `.clone()`关键字 或 直接赋值为栈上的数据
`移动(move)`：不拷贝内容，拷贝引用并删除旧的，即移动旧的到新的引用上


Rust 的所有权规则：
每块内存在任意时刻只能有一个所有者，所有权移动后旧所有者会失去所有权，以防多拷贝一份引用在最终释放同一块内存时造成的double free二次释放的内存污染的漏洞。
当持有堆中数据值的变量离其作用域时，其内存将通过 drop 被释放掉，除非在此之前数据被移动为另一个变量所有。

## 引用
`引用(reference)`: `&`，浅拷贝，但最终所有权仍在原有者上，即只可访问无法修改
`可变引用`: `&mut`，浅拷贝，可访问可修改
`借用(borrowing)`：create a reference
- 借用规则：
    - 同一块内存可以同时拥有多个不可变引用，或者只能拥有一个可变引用（防止数据竞争[data race]）
    - 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。区别于普通变量
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

## 结构体(structure)
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

### 字段初始化简写语法
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

### 结构体更新语法
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```
`..user1` 必须放在最后
此语法移动了user1

### 元组结构体
无键，按顺序存储值
```rust
struct Color(i32, i32, i32);
```
### 类单元结构体
无字段的，类似于元组的unit类型，即`()`，用于想要在某个类型上实现 trait 但不需要在类型中存储数据的时候

### 结构体数据的所有权
结构体存储另一复合数据类型，且拥有其所有权，则需使用**自身拥有所有权**的数据类型，如`String` 类型而不是 `&str` `字符串 slice` 类型。

`生命周期(lifetimes)`: 可使结构体存储被其他对象拥有的数据的引用。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。

println! 宏能处理很多类型的格式，不过，{} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。
```rust
#[derive(Debug)]
'{xx:?}'
```

## 方法(method)
在结构体、枚举、trait对象上下文中被定义，首参数总是为`self`，指向调用该方法的结构体实例
`impl`: implementation，impl块中内容与相应类型所关联
即C++中的成员函数
`&self`为`self: &Self`缩写，在一个 `impl` 块中，`Self` 类型是 `impl` 块的类型的别名
如果想要在方法中改变调用方法的实例，需要将第一个参数改为 `&mut self`

`getters`: 与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。可以把字段变成私有的，但方法是公共的，进而把对字段的只读访问作为该类型公共 API 的一部分

## 自动引用和解引用
C++(手动): `object->something()` 等若于 `(*object).something()`
Rust: object.something() 调用方法时，会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配

## 关联函数
在 `impl` 块中被定义的函数，包括方法
非方法的关联函数，需以ClassName::functionName()调用

## 枚举

### enum
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

### Option
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

## 模式匹配
### match控制流运算符

一个 match 表达式由分支（arm） 构成。一个分支包含一个用于匹配的模式（pattern），给到 match 的值与分支模式相匹配时，应该执行对应分支的代码。Rust 获取提供给 match 的值并逐个检查每个分支的模式。

`Rust` 中的匹配是`穷举式的（exhaustive）`：必须穷举到最后的可能性来使代码有效。Rust 防止我们忘记明确的处理 None 的情况，这让我们免于假设拥有一个实际上为空的值。

#### 通配模式
`other`: 匹配所有值，并绑定到`other`变量
`_`: `占位符(placeholder)`，匹配所有值，但不会绑定到变量
`_ => ()`：匹配所有值，但不会绑定到变量，且不执行任何操作

### if let 简单控制流
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

## 集合(collections)

集合指向的数据是储存在堆上的，数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小

### Vec
是泛型实现
`vec!`宏： `vec![1, 2, 3]`
添加: `v.push()`
#### 读取
`let ele: &T =  &v[idx]`，返回引用
`let ele = v.get(idx)`，返回`Option<T>`

在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中，对原vector元素的引用成了野指针
#### 遍历
```rust
for i in &mut v {
    *i += 50;
}
for i in &v {
    println!("{}", i);
}
```

#### 多存储
vector 只能储存相同类型的值
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

### 字符串
Rust 的核心语言中只有一种字符串类型：`str`，`字符串 slice`，它通常以被借用的形式出现，`&str`，是一些储存在别处的 UTF-8 编码字符串数据的引用
`&String[..]` == `&str`
称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
to_string 方法从字符串字面量创建 String
push_str 方法来附加字符串 slice，从而使 String 变长
push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
```
`fn add(self, s: &str) -> String {`

之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。当 add 函数被调用时，Rust 使用了一个被称为 解引用强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。

String == Vec<u8>

`format!`

不能使用索引字符串
原因是，索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符
```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
`&hello[0..1]`报错

#### 遍历
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
Rust 并没有异常
`可恢复错误（recoverable）`: `Result<T, E>`，通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。
`不可恢复错误（unrecoverable）`: `panic!`，遇到错误时停止程序执行，即bug，比如尝试访问超过数组结尾的位置

`panic!宏`
`展开（unwinding）`: Rust 会回溯栈并清理它遇到的每一个函数的数据
`终止（abort）`: 不清理数据就退出程序，那么程序所使用的内存需要由操作系统来清理

```rust [Cargo.toml]
[profile.release]
panic = 'abort'
```
可使最终二进制文件小

序列索引越界：任何对应 vector 中这个元素的内存位置的值，甚至是这些内存并不属于 vector 的情况。这被称为 缓冲区溢出（buffer overread），并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取储存在数组后面不被允许的数据。

```bash
$ RUST_BACKTRACE=1 cargo run
```

如果代码 panic，就没有恢复的可能。选择返回 Result 值的话，就将选择权交给了调用。
