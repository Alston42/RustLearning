## cargo

### 模块系统(the module system)

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
父模块不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项，兄弟模块间无法相互使用其私有项
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


`&[i32]`: 变长度数组
`[i32, i32]`: 定长度数组

## 泛型
`泛型（generics）`: 是具体类型或其他属性的抽象替代。
在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。
Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程
我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。

### trait
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
类似于`接口（interfaces）`

如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

`相干性（coherence）`限制: `孤儿规则（orphan rule）`，其得名于不存在父类型。确保了其他人编写的代码不会破坏你代码。实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait

pub trait Summary {
    fn summa rize()
}

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```


#### realization


#### 默认实现
有时为 trait 中的某些或全部方法提供默认的行为，这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
如果想要对 NewsArticle 实例使用这个默认实现，而不是定义一个自己的实现，则可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块。

`grep`: Globally search a Regular Expression and Print.

#### Trait as Params

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### Trait Bound

```rust
pub fn notify<T: Summary>(item: T {
    item.summarize();
}
```
`pub fn notify(item1: impl Summary, item2: impl Summary) {`
`pub fn notify<T: Summary>(item1: T, item2: T) {`,泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。

#### Multiple Trait Bounds

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

#### Return Types which realize trait

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

为使用不同类型的值而设计的 trait 对象


#### 

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```


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


#### blanket implementations
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

## lifetime
Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域,大部分时候生命周期是隐含并可以推断的

生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据。

### 借用检查器（borrow checker）
比较作用域来确保所有的借用都是有效的


```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

单个生命周期标注本身没有多少意义，因为生命周期标注告诉 Rust 多个引用的泛型生命周期参数如何相互联系的。

如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。这两个生命周期标注意味着引用 first 和 second 必须与这泛型生命周期存在得一样久。
就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期
longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致

具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。


```rust
let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is {}", result);
```
 未显式指定生命周期时，按一般语言的思路，result的生命周期可能是string1的长周期，也可能是string2的短作用域
 向函数传入生命周期参数时，所有生命周期统一规范为最短者，无论返回哪个，都是生命周期统一过后的,此时仍可能报错

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域


### 结构体定义中的生命周期标注
这个标注意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。


被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）

函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。

编译器采用三条规则来判断引用何时不需要明确的标注。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。

第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
`fn first_word(s: &str) -> &str {`
infer to
`fn first_word<'a>(s: &'a str) -> &str {`

第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method),那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。这条规则意味着我们经常不需要在方法签名中标注生命周期。

`静态生命周期`: `'static`，其生命周期能够存活于整个程序期间。


这也就是为什么 Display trait bound 是必须的。因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。








## Auto Test

为了将一个函数变成测试函数，需要在 fn 行之前加上 #[test]。当使用 cargo test 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。

`#[cfg(test)]`
`#[test]`
`assert!()`: pass when the assertion is true
`assert_eq!()`
`assert_ne!()`

assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 PartialEq 和 Debug trait。所有的基本类型和大部分标准库类型都实现了这些 trait。对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。需要实现 Debug 才能在断言失败时打印他们的值。因为这两个 trait 都是派生 trait，通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 

除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的
`#[should_panic]`: 这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。
`#[should_panic(expected = "some message")]`expected 参数提供的值是 Guess::new 函数 panic 信息的子串
expected 信息的选择取决于 panic 信息有多独特或动态，和你希望测试有多准确。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

每 6 周发布一版的一个好处是下一班火车即将到来。如果一个功能在特定版本中错过了也无需担心：另一个版本很快就会到来！这有助于减少在发布截止日期前匆忙加入可能未完善的功能的压力。
Rust项目支持最新的稳定版本。当一个新的稳定版本发布时，旧版本就达到了其生命周期（EOL）。这意味着每个版本都支持六周。


部分终端都提供了两种输出：标准输出（standard output，stdout）对应一般信息，标准错误（standard error，stderr）则用于错误信息。

`$ cargo run > output.txt` 重定向标准输出流到该文件中,> 语法告诉 shell 将标准输出的内容写入到 output.txt 文件中而不是屏幕上。命令行程序被期望将错误信息发送到标准错误流，这样即便选择将标准输出流重定向到文件中时仍然能看到错误信息。

## 函数式编程(Functional Programming)
函数式编程风格通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。


闭包（Closures），一个可以储存在变量里的类似函数的结构
迭代器（Iterators），一种处理元素序列的方式
如何使用这些功能来改进第 12 章的 I/O 项目
这两个功能的性能（剧透警告： 他们的速度超乎你的想象！）

### 闭包：可以捕获环境的匿名函数
可以保存进变量或作为参数传递给其他函数
不同于函数，闭包允许捕获调用者作用域中的值??
可以捕获其环境并访问其被定义的作用域的变量??

闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|。

参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。在闭包的末尾，花括号之后，需要使用分号使 let 语句完整。因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（num）最后一行的返回值作为调用闭包时的返回值 。

闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型标注是因为他们是暴露给用户的显式接口的一部分。严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。
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

仍然把慢计算闭包调用了比所需更多的次数。解决这个问题的一个方法是在全部代码中的每一个需要多个慢计算闭包结果的地方，可以将结果保存进变量以供复用，这样就可以使用变量而不是再次调用闭包。但是这样就会有很多重复的保存结果变量的地方。。可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）。

Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

Tasks:
- 尝试修改 Cacher 存放一个哈希 map 而不是单独一个值。哈希 map 的 key 将是传递进来的 arg 值，而 value 则是对应 key 调用闭包的结果值。相比之前检查 self.value 直接是 Some 还是 None 值，现在 value 函数会在哈希 map 中寻找 arg，如果找到的话就返回其对应的值。如果不存在，Cacher 会调用闭包并将结果值保存在哈希 map 对应 arg 值的位置。

- 当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。比如说，我们可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多泛型参数来增加 Cacher 功能的灵活性。

当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：

    FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    FnMut 获取可变的借用值所以可以改变其环境
    Fn 从其环境获取不可变的借用值


## iterator

迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

在 Rust 中，迭代器是 惰性的（lazy），在调用方法使用迭代器之前它都不会有效果

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。

注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 消费（consume）了，或使用了迭代器。每


面向对象编程（Object-Oriented Programming，OOP）是一种模式化编程方式。所共享的一些特性往往是对象、封装和继承

一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。

封装（encapsulation）的思想：对象的实现细节不能被使用对象的代码获取到。唯一与对象交互的方式是通过对象提供的公有 API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。封装使得改变和重构对象的内部时无需改变使用对象的代码。

继承（Inheritance）一个对象可以定义为继承另一个对象的定义这使其可以获得父对象的数据和行为，而无需重新定义。
选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。相反 Rust 代码可以使用默认 trait 方法实现来进行共享
第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用。
很多人将多态描述为继承的同义词。不过它是一个有关可以用于多种类型的代码的更广泛的概念。对于继承来说，这些类型通常是子类。 Rust 则通过泛型来对不同的可能类型进行抽象，并通过 trait bounds 对这些类型所必须提供的内容施加约束。这有时被称为 bounded parametric polymorphism。
近来继承作为一种语言设计的解决方案在很多语言中失宠了，因为其时常带有共享多于所需的代码的风险。子类不应总是共享其父类的所有特征，但是继承却始终如此。如此会使程序设计更为不灵活，并引入无意义的子类方法调用，或由于方法实际并不适用于子类而造成错误的可能性。某些语言还只允许子类继承一个父类，进一步限制了程序设计的灵活性。