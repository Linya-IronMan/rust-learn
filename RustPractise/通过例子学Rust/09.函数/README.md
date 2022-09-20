# 9. 函数

## 9.1 方法

*   方法如果不对实例有所依赖或者干扰，一般叫做静态方法
*   函数内部通过 self.name 的方式访问绑定目标的结构体实例属性
*   \&self 是 self: \&self 的语法糖

```rust
struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，`Point` 的所有方法都在这里给出
impl Point {
    // 这是一个静态方法（static method）
    // 静态方法不需要被实例调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，需要两个参数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个实例方法（instance method）
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
    // 类型。在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！前面的 `destroy` 调用 “消耗了” `pair`
    //pair.destroy();
    // 试一试 ^ 将此行注释去掉
}
```

## 9.2 闭包

Rust 中的闭包也叫做 lambda表达式，能够捕获周围作用域中变量的函数。

*   声明时使用 `||` 替代 `()` 包裹输入参数
*   函数定界符（{}）在单个表达式的时候可选，其他情况必须加上
*   有能力捕获外部环境的变量
*   闭包是一种匿名函数
*   普通函数中无法访问外界的环境变量
*   闭包的语法可能会非常简洁：`let a = || 3;`
*   闭包不必书写返回的类型以及参数类型，它会自动推导
*   闭包会自动推导参数以及返回值类型。但是如果对同一个闭包多次调用的时候传递的参数不同，就会报错。
*   每一个闭包都有自己独有的匿名类型。即使两个闭包有着相同的签名，它们的类型仍然可以被认为是不同的

### 9.2.1 捕获

*   闭包优先(默认)通过引用来捕获变量，并且仅在需要时使用其他方式。

<!---->

*   一般情况下，传入闭包中的变量，虽然写法好像是使用的变量的本体，但是实际上是变量的引用

```rust

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color`  会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;

    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;
```

*   闭包中如果发生了数据的修改，那么闭包需要加上一个 mut

<!---->

*   这里需要闭包执行完最后一次调用之后，才能重新借用，否则会报错。

问题：如果是一个普通的函数，也会这样么？为什么会有这样的规则

```rust
    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;
```

```rust
fn main() {
    use std::mem;
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}
```

***

#### 9.2.2.2 闭包的捕获原理

闭包可以通过三种方式捕获其环境，它们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 `Fn` trait

*   `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被成为其环境。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实。所以，它只能被调用一次。
*   `FnMut` 获取可变的借用值，所以可以改变其环境
*   `Fn` 从其环境获取不可变的借用值

创建一个闭包时，Rust 根据其如何使用环境中的变量来推断我们希望如何引用环境。
由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 `FnOnce` 。
那些并没有移动被捕获变量所有权到闭包内的闭包也实现了`FnMut`，而不需要对捕获的变量进行可变访问的闭包也实现了`Fn`。

如果希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 `move` 关键字。

闭包所实现的trait 是由闭包所捕获了什么值而不是如何捕获所决定的。而 move 仅代表后者

***

#### 9.2.2.3 作为输入参数

闭包作为函数参数的时候，要求闭包是范型的，闭包的定义方式决定了这是必要的。

```rust
#![allow(unused)]
fn main() {
// `F` 必须是泛型的。
  fn apply<F>(f: F) where
      F: FnOnce() {
      f();
  }
}
```

闭包被定义的时候，编译器会隐式创建一个匿名类型的结构体，用以存储闭包捕获的变量，同时为这个未知类型的结构体实现函数功能，
通过 `Fn`, `FnMut`, `FnOnce` 三种 trait 中的一种。

如果使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是范型的。然而， 使用未限定类型的参数<T>过于不明确，并且是不被允许的。
事实上，指明为该结构体实现的是 `Fn` , `FnMut` 或 `FnOnce` 中的哪种trait，对于约束改结构体的类型而言就已经足够了。

```rust
// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}

```

*   函数也可以作为参数
*   函数作为参数的时候同样需要满足trait需求

既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数。

```rust
// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}
```

#### 9.2.2.4 闭包作为函数返回值

https://rustwiki.org/zh-CN/rust-by-example/fn/closures/output\_parameters.html

## 9.3 高阶函数

https://rustwiki.org/zh-CN/rust-by-example/fn/hof.html

## 9.4 发散函数

https://rustwiki.org/zh-CN/rust-by-example/fn/diverging.html
