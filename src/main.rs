#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    //b存储在栈上 5存储在堆上 b指向5
    let b = Box::new(12);
    use List::Cons;
    use List::Nil;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}&{:?}", b, list);
    println!("{:?}&", b);
}
