/// `Arc<T>` 是共享所有权模型的智能指针
/// 而 `Box<T>` 是独占所有权的智能指针
/// 
/// `Arc<T>` 拥有所有权, 但不提供对 T 的修改能力  
/// 想要修改 Arc<T> 里面的 T, 必须配合锁才能完成，比如 Arc<Mutex<T>> 互斥锁
#[allow(unused)]
struct Description;

use std::sync::Arc;

fn main() {
    // code_example01();
    // code_example02();
    // code_example03();
}

/// `Arc<T>` 主要是与 clone() 配合使用  
/// - 在 Arc 的实例上每一次新的 clone() 操作总是会将资源的引用数 +1, 而保持原来那一份资源不动  
/// - 若 Arc 的实例走出作用域, 这个引用计数 -1, 直到引用计数器为 0 才会被销毁释放
///
/// *正因为 clone() 行为只会改变引用计数器, 所以 `Arc<T>` 不要求 T 实现 clone*
#[allow(unused)]
fn code_example01() {
    // 这里不需要目标类型实现 Clone trait
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        fn play(&self) {
            println!("I'am a method of Point.");
        }
    }
    // 创建 Arc 引用计数器实例
    let arced: Arc<Point> = Arc::new(Point { x: 10, y: 20 });
    // 引用计数 +1
    let another_arced = arced.clone();
    // 打印同一份值
    println!("{:?}", arced);
    println!("{:?}", another_arced);

    arced.play();
    another_arced.play();
}

/// 和 `Box<T>` 一样，`Arc<T>` 也可以用在方法中的 self 参数上面, 作为所有权 self 的一个变体形式
#[allow(unused)]
fn code_example02() {
    use std::sync::Arc;

    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn play_ref(&self) {
            println!("I'am play_ref of Point.");
        }
        fn play_mutref(&mut self) {
            println!("I'am play_mutref of Point.");
        }
        fn play_own(self) {
            println!("I'am play_own of Point.");
        }
        fn play_boxown(self: Box<Self>) {
            // 注意这里
            println!("I'am play_boxown of Point.");
        }
        fn play_arcown(self: Arc<Self>) {
            // 注意这里
            println!("I'am play_arcown of Point.");
        }
    }

    fn main() {
        let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
        boxed.play_ref();
        boxed.play_mutref();
        boxed.play_boxown();
        // boxed.play_own();  // play_boxown() 和 play_own() 只能同时打开一个

        let arced: Arc<Point> = Arc::new(Point { x: 10, y: 20 });
        arced.play_ref();
        // arced.play_mutref();  // 不能用
        // arced.play_own();     // 不能用, Arc<T> 中的 T 无法被移出
        arced.play_arcown();
    }
    // 输出
    // I'am play_ref of Point.
    // I'am play_mutref of Point.
    // I'am play_boxown of Point.
    // I'am play_ref of Point.
    // I'am play_arcown of Point.
}

/// `Arc<dyn Trait>`  
/// 可以將 `Box<dyn Trait>` 改成 `Arc<dyn Trait>`
/// ```
#[allow(unused)]
fn code_example03() {
    ()
}