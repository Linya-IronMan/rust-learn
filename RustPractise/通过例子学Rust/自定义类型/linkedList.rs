enum List {
    Node(u32, Box<List>),
    Nil
}

use List::{Node, Nil};

impl List {
    fn new() -> List {
        Nil
    }
    // &self 与 self 在这里有什么区别么？
    fn prepend(self, elem: u32) -> List {
        Node(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Node(_, ref tail)  => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Node(elem, ref tail ) => {
                format!("{} {}", elem, tail.stringify())
            },
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
