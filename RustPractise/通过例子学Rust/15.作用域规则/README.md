
# 15. 作用域规则

- 所有权
- 借用
- 生命周期

告诉编译器什么时候借用是和发的、什么时候资源可以释放、以及变量何时被创建或销毁。

## 15.1 RALL

Rust 强制执行RALL(Resource Acquisistion Is Initiallization, 资源获取即初始化)，任何对象在离开作用域的时候，它的析构(destructor)函数就被调用
然后它占有的资源就被释放

这种行为避免了资源泄漏，再也不用手动释放内存或担心内存泄漏。

**注意** 可以使用 valgrind 对内存错误进行检查
```shell
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

### 15.1.1  析构函数

Rust 中的析构函数是通过 Drop trait 提供的。当资源离开作用域，就调用析构函数。
只需要为那些需要自己的析构函数逻辑的类型实现 Drop trait

```rust
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}

```

## 15.2 所有权和移动

- 因为变量要负责释放它们拥有的资源，所以资源只能有一个所有者。这也防止了资源的重复释放。
- 需要注意的是：并非所有变量都拥有资源。
- 赋值操作或通过值来传递函数参数的时候，资源的所有权会发生转移。这被称为所有权的移动
- 移动资源之后，原来的所有者不能再被使用，这可以避免悬挂指针(dangling pointer)的产生

```rust
let x = 5u32;
// 将x复制了一份到y，不存在紫玉啊你的移动。
let y = x;

let a = Box::new(5i32);
// 移动 a 到 b
let b = a;

fn destroy_box(c: Box<i32>) {
    println("{}", c)
}
// 此函数从 b 中取得堆分配的内存的所有权
destroy_box(b)
```

### 15.2.1 可变性

- 当所有权发生转移时，数据的可变性可能发生改变
- `let immutable_box = Box::new(5u32)` 这样创建获取的变量，实际上是一个引用。一个指向存储了 5u32 数据的堆空间的引用。
- 将一个不可变的变量重新复制到一个可变变量名上，就可以去改变数据了
- Box::new(5u32) 创建的变量，要改变数据需要先解引用 *immutable_box = 5;

```rust
let immutable_box = Box::new(5u32);

println!("immutable_box contains {}", immutable_box);

// 可变性错误
//*immutable_box = 4;

// *移动* box，改变所有权（和可变性）
let mut mutable_box = immutable_box;

println!("mutable_box contains {}", mutable_box);

// 修改 box 的内容
*mutable_box = 4;

println!("mutable_box now contains {}", mutable_box);
```

### 15.2.2 部分移动

- 对单个变量的解构中，可以同时使用 by-move 和 by-reference 模式绑定。这样就会导致只有变量的部分移动。这种情况下，
  后面不能整体使用父级变量，但是仍然可以使用只引用（而不移动）的部分。
- by-reference 方式进行解构需要在对应的变量名前加上 ref 关键字。

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);
}
```

## 15.3 借用

- 借用可以访问数据，同时不取得其所有权。
- 编译器（通过借用检查）静态保证了引用总是指向有效的对象。也就是说，当存在引用指向一个对象的时候，该对象不能被销毁。
- 函数自身就是一个作用域，因此在函数通过参数发生借用的时候，函数运行完毕之后，函数中临时创建的引用也就不存在了。

### 15.3.1 可变性

https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/mut.html

- `&mut T` 通过这种方式可以对可变数据进行可变引用。它可以使借用者能够 读/写 数据。


### 15.3.2 别名使用

- 数据可以多次不可变借用
- 不可变借用的同时，原始数据不能使用可变借用
- 同一时间内只允许一次可变借用。（不允许其他任何方式的借用）
  仅当最后一次使用可变引用之后，原始数据才可以再次借用

```rust
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);
    
    // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    // let mutable_borrow = &mut point;
    // TODO ^ 试一试去掉此行注释

    // 被借用的值在这里被重新使用
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
    // let y = &point.y;
    // TODO ^ 试一试去掉此行注释

    // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ 试一试去掉此行注释

    // 正常运行！可变引用能够以不可变类型传入 `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用不再用于其余的代码，因此可以重新借用
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
```

### 15.3.3 ref 模式

- 用于普通的赋值语句：左侧ref等价于右边的 & 符号
  ```rust
  let ref ref_c1 = c;
  let ref_c2 = &c;
  ```
- 解构结构体或者元组的时候，可以用来创建引用
- 需要注意解构结构体的方式，左侧需要指明解构类型
  ```rust
  let point = Point { x: 0, y: 0 };

  // 在解构一个结构体时 `ref` 同样有效。
  let _copy_of_x = {
      // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
      let Point { x: ref ref_to_x, y: _ } = point;

      // 返回一个 `point` 的 `x` 字段的拷贝。
      *ref_to_x
  };
  ```

## 15.4 生命周期

- 编译器（中的借用检查器）用生命周期来保证所有的借用都是有效的。
- 生命周期和作用域并不相同
- 生命周期由它生命的位置决定。

```rust

// 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期。
// `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和
// `borrow2` 的。`borrow1` 和 `borrow2` 的周期没有关联，
// 因为它们各不相交。
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

### 15.4.1 显式标注

- 使用撇号 `'` 可以显式标注生命周期

```rust
foo<'a>
```

- 和闭包类似，使用生命周期需要范型
- 下面的标注表示 foo 不能超出 `'a`  `'b` 中任何一个周期。
```rust
foo<'a, 'b>
```

```rust
// `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
// 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数的函数，不过有一个生命周期参数 `'a`。
fn failed_borrow<'a>() {
    let _x = 12;

    // 报错：`_x` 的生命周期不够长
    //let y: &'a i32 = &_x;
    // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
    // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
}

fn main() {
    // 创建变量，稍后用于借用。
    let (four, nine) = (4, 9);

    // 两个变量的借用（`&`）都传进函数。
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用者生存得更长。
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。

    failed_borrow();
    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。
}
```



