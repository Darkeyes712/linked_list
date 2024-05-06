mod core;

use core::SimpleLinkedList;

fn main() {
    let mut l = SimpleLinkedList { list: vec![2] };

    println!("{:?}", l);

    println!("{:?}", l.is_empty());
    println!("{:?}", l.len());
    l.push(3);
    println!("{:?}", l);
    l.pop();
    println!("{:?}", l);
    println!("{:?}", l.peek());
    l.push(3);
    println!("{:?}", l.peek());

    println!("{:?}", l.rev());
}
