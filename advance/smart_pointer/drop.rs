
// 进阶之 Drop 释放资源
fn main() {
    example01();
    example02();
}

/// # example01 手动回收内存
#[allow(unused)]
fn example01() {
    // 定义一个结构体 Foo
    struct Foo;
    // 为结构体 Foo 实现 Drop 特征
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!");
        }
    }
    
    let foo = Foo;
    // 手动释放内存
    // ! foo.drop()
    // 以上代码会报错, 因为编译器阻止我们调用 Drop 特征的 drop 方法
    // 不允许显式的调用析构函数

    // 但是可以使用 drop 释放内存
    // 其原理是移走目标值的所有权
    // 源码: `pub fn drop<T>(_x: T) {}`
    drop(foo);
}

/// # example02 Copy 和 Drop 是互斥的!
#[allow(unused)]
fn example02() {
    // ! 无法为一个类型同时实现 Drop 和 Copy 特征
    // ! Copy 特征是在栈上按位复制操作的, 而不是在堆上
    // ! 所以不能为实现了 Copy 特征的类型实现 Drop 特征
    // ! 所以实现了 Drop 特征调用 drop 是没有意义的
}