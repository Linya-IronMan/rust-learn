enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

impl List {
    fn new () -> List {
        Nil
    }
    
    fn prepend(self, value: i32) -> List {
        Cons(value, Box::new(self))
    }

    fn len(&self) -> i32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {

        match *self {
            Cons(value, ref tail) => format!("{}, {}", value, tail.stringify()),
            Nil => format!("Nil")
        }
    }
    
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

