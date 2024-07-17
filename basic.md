```bash
rustc filename  # compile

cargo new package_name # init a package
cargo build package_name # compile
@params: --release : executable file derived into release folder not default `debug` folder, compile slow but generate effective executable file
cargo run package_name # compile if changes and then run  
cargo check package_name # no compile, just check if it pass compilation, for debug
```

`mut` : mutability of a variable
`let`: declaration of a variable, 


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

runtime error
compile error

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tuple.0;
let six_point_four = tuple.1;
```

Rust 是一门基于表达式（expression-based）的语言，
语句（statement）是执行一些操作但不返回值的指令。表达式（expression）计算并产生一个值。
(x):    let x = (let y = 6);
expression:
{
    let x = 3;
    x + 1 // without `;`
}

并不对函数返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数隐式返回函数体最后一个表达式的值。使用 return 关键字和指定值，可以从函数中提前显式返回；

```rust
let number = if condition { 5 } else { "six" }  // uncompiled
```
`range`函数：(1..4).rev() // .rev 即reverse

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
Compile Error
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");
```
浅拷贝(shallow copy)：仅拷贝指针、引用
深拷贝(deep copy)：拷贝指针指向的内容 => .clone() 或在栈上的数据直接赋值
移动(move)：不拷贝内容，拷贝指针并删除旧的，即移动旧的到新的引用上


Rust 的所有权规则：
每块内存在任意时刻只能有一个所有者，所有权移动后旧所有者会失去所有权，以防多拷贝一份引用在最终释放同一块内存时造成的double free二次释放的内存污染的漏洞。
当持有堆中数据值的变量离其作用域时，其内存将通过 drop 被释放掉，除非在此之前数据被移动为另一个变量所有。

## 引用(reference)
借用(borrowing)：create a reference

相当于浅拷贝，但最终所有权仍在原有者上，即只可访问无法修改

可变引用 &mut class，浅拷贝，可访问可修改
借用规则：
同一块内存可以同时拥有多个不可变引用，或者只能拥有一个可变引用（防止数据竞争[data race]）
一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。区别于普通变量

data race:
- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。旧数据无，旧指针有，后来同样区位有新数据、新指针，再次调用旧指针时会？？？

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

“字符串 slice” 的类型声明写作 &str，是不可变引用：
字符串字面值被储存在二进制文件中，
let s = "Hello, world!";
这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的

## 结构体(structure)
和元组一样，结构体的每一部分可以是不同类型。不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。
定义每一部分数据的名字和类型，称其为 字段（field）
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
无需重复username和email，若字段名同参数名重名

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
```rust
struct Color(i32, i32, i32);
```
### 类单元结构体
无字段的，类似于元组的unit类型，即`()`，用于想要在某个类型上实现 trait 但不需要在类型中存储数据的时候

### 结构体数据的所有权
在示例 5-1 中的 User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。

可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，

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