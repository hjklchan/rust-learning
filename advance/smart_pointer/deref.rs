
// 进阶之 Deref 解引用
fn main() {
    example01();
    example02();
    example03();
}

/// # example01 通过 * 获取引用背后的值
#[allow(unused)]
fn example01() {
    // 定义普通变量
    let x = 32;
    // 定义一个引用 变量 x 的变量
    let y = &x;

    // 将引用变量 y 解引用与字面量 32 作比较
    assert!(32 == *y); // ✔
}

/// # example02 智能指针的解引用
#[allow(unused)]
fn example02() {
    let x = Box::new(1024);
    // 只能指针被 * 解引用为 i32 类型的值
    // 然后被用于运算
    let _sum = *x >> 2;
    // println!("{}", _sum);
}

/// # example03 定义自己的智能指针
#[allow(unused)]
fn example03() {
    // 自定义自己的智能指针
    struct GoodBox<T>(T);
    // 为 GoodBox 实现 new 实例方法
    impl<T> GoodBox<T> {
        fn new(v: T) -> GoodBox<T> {
            GoodBox(v)
        }
    }

    // 定义自定义智能指针类型的变量 y
    let y = GoodBox::<i32>(1024);
    // ! 因为自定义智能指针为实现 deref 方法, 所以该变量 y 无法被解引用
    // assert_eq!(1024, *y);
}

/// # example04 改进自定义的智能指针
#[allow(unused)]
fn example04() {
    // 自定义自己的智能指针
    struct GoodBox<T>(T);
    // 为 GoodBox 实现 new 实例方法
    impl<T> GoodBox<T> {
        fn new(v: T) -> GoodBox<T> {
            GoodBox(v)
        }
    }
    // 为自定义的智能指针实现 Deref 特征
    impl<T> std::ops::Deref for GoodBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // 实现了 Deref 特征后的智能指针可以使用 * 解引用取得背后的值
    let y = GoodBox::<i32>::new(1023);
    // 使用 * 对 y 解引用取得类型为 i32 的值
    assert_eq!(1023, *y);
}

/// example05 函数和方法中的隐式 Deref 转换
#[allow(unused)]
fn example05() {
    // 定义一个类型为 String 的变量
    let s = String::from("value");
    display(&s);
    // 以上代码需要注意几点
    // 1. String 实现了 Deref 特征, 在需要时自动被转为 &str 类型
    // 2. &s 的实际类型是 &String, 当被传递到 display 函数时自动通过 Deref 转换为 &str
    // 3. 必须使用 &s 的方式触发 Deref (仅引用类型的实参才会自动触发解引用)

    fn display(_s: &str) {}
}

/// example06 连续的隐式 Deref 转换
#[allow(unused)]
fn example06() {
    let s = Box::new(String::from("hello world!"));
    // 注意: 当实参 Box 被 Deref 为 String 时无法满足 display 的参数要求
    // 但是编译器发现 String 的 Deref 之后可以解出 &str 匹配了参数
    display(&s);

    fn display(_s: &str) {}
}
