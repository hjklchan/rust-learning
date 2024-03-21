
// Rc 与 Arc 实现 1vN 所有权机制
// Rc: 引用计数 (Reference counting)
// Arc: 原子引用计数 (Atomic Reference counting)
fn main() {
    example01();
    example02();
    example03(); // todo
}

/// # example01 所有权被转移导致的错误示例
#[allow(unused)]
fn example01() {
    // 以下是经典的所有权被转移导致的错误
    let s = String::from("hello world!");
    let a = Box::new(s);
    // ! 此处的 s 的所有权已转移给 a
    // let b = s;
}

/// # example02 使用 Rc<T> 解决问题
#[allow(unused)]
fn example02() {
    use std::rc::Rc;

    // 智能指针 Rc<T> 创建时会将引用计数 + 1
    let a = Rc::new(String::from("hello world!"));
    println!("创建 Rc 后的初始引用计数器为 {}", Rc::strong_count(&a));
    // 克隆后引用计数 + 1
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
}

/// # example03 使用 Rc::clone
#[allow(unused)]
fn example03() {
    todo!();
}