/// ## 聪明指针 (智能指针)
/// ## Smart Pointer
#[allow(unused)]
fn title() {
    ()
}

/// 如果一个变量, 里面存的是另一个变量的地址, 那么这个变量就叫做指针
///
/// 引用就是一种指针
#[allow(unused)]
fn ref_example() {
    let _str: &str = "hello";
    let _str: &String = &String::new();
    let _i: &i32 = &10;
}

/// 智能指针
///
/// *对一个结构体实现一些标准库提供的 Traits 就可以变为指针类型*
///
/// 这种指针可以在传统指针的基础上添加一些额外信息, 比如放在额外的一些字段中;
/// 也可以做一些额外操作, 比如管理引用计数, 资源自动回收等, 从而显得更加智能, 所以被叫做 **智能指针**
#[allow(unused)]
fn smart_pointer_example() {
    // 常见的智能指针有 Vec<T> 和 String
    ()
}

/// 回忆一段代码
#[allow(unused)]
fn code_example01() {
    fn foo() -> String {
        // 在函数中创建字符串实例
        // 该字符串实例在堆中分配
        // s 是拥有字符串的所有权
        let s = "abc".to_string();
        // 将 s 的所有权返回给外部的 _s 变量, foo() 变量调用完成后, 栈上的 s 被销毁
        s
    }
    // _s 拥有所有权
    let _s = foo();
}

/// 改造以上代码
///
/// 将 String 改为 &String
#[allow(unused)]
fn code_example02() {
    // ! 该字符串实例在堆中分配
    // ! 当调用结束后会回收 s 变量导致 s 是一个无效引用
    // fn foo() -> &String {
    //     let s = "abc".to_string();
    //     &s
    // }
}

/// 使用 **Box** 智能指针解决上面两个 code_example 的问题
#[allow(unused)]
fn code_example03() {
    // Box 是一个类型整体, 可以将资源强行创建在堆上, 并获得所有权
    // 其生命周期可以被精确的掌控
    // ! 注意: 堆上的资源默认与整个程序存在的时间一样久
    fn foo() -> Box<u32> {
        let i = 100u32;
        Box::new(i)
    }
    let _i = foo();
    println!("{}", *_i);
}

/// 从函数中返回结构体的 Box 指针
#[allow(unused)]
fn code_example04() {
    struct Point {
        x: i32,
        y: i32,
    }
    // 定义函数
    fn foo() -> Box<Point> {
        // 将结构体实例创建在栈上
        let p = Point { x: 32, y: 23 };
        // 在将栈上的 Point 实例强行复制一份放到堆上
        // 将 `Box<Point>` 实例的所有权返回
        let boxed = Box::new(p);
        // let q = p; // ! 提示已经被 Move 走
        boxed
    }
    // _p 获得 Box<Point> 的所有权
    let _p = foo();
}

/// `Box<T>`> 的解引用
///
/// `Box<T>` 通过解引用把堆里面的值再次移回栈上
#[allow(unused)]
fn code_example05() {
    let boxed = Box::new(5);
    // 堆上的值再次回到栈上
    let _value = *boxed;
    // ! 对于 copy 语义的 i32 类型来说, 解引用后的 boxed 依然能使用
    let _valye = *boxed;

    // 对于 move 语义的类型来说就不一样了, 会发生所有权转义
    let boxed: Box<String> = Box::new(String::new());
    // 堆上的值再次回到栈上
    let _s = *boxed;
    // let _t = *boxed; // ! error: 所有权发生转义
}

/// `Box<T>` 实现的 Traits
///
/// 标准库为 `Box<T>` 实现了 `Deref`/`Drop`/`AsRef<T>` 等 traits
#[allow(unused)]
fn code_example06() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn move_forward(&self) {
            println!("It's moving");
        }
    }
    // 将 Point 实例进行盒化
    let boxed = Box::new(Point { x: 32, y: 64 });
    // 点 `.` 操作触发解引用
    boxed.move_forward();
    println!("{:?}", boxed);
}

/// `Box<T>` 拥有 T 的所有权, 可以对 T 进行写操作
#[allow(unused)]
fn code_example07() {
    let mut boxed = Box::new(String::from("hjkl1"));
    // 进行写操作
    boxed.push_str("!!!");
    println!("{}", boxed);
}

/// `Box<T>` 的克隆
///
/// 能否克隆取决于 T 是否实现了 Clone Trait
#[allow(unused)]
fn code_example08() {
    let boxed = Box::new(String::from("hjkl1"));
    let mut another_boxed = boxed.clone();
    another_boxed.push_str("!");
    // 两值互不影响
    println!("{}", boxed == another_boxed);
}

/// `Box<T>` 作为函数参数
#[allow(unused)]
fn code_example09() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 作为函数参数
    fn foo(b: Box<Point>) {
        println!("{:?}", b);
    }
    let boxed = Box::new(Point { x: 0, y: 1 });
    foo(boxed);
    // println!("{:?}", boxed); // ! error: value borrowed here after move
}

/// `Box<T>` 作为类型也可以被引用,
/// 还可以被作为可变引用
#[allow(unused)]
fn code_example10() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn play(&self) {
            println!("playing something...");
        }
    }

    let boxed = Box::new(Point { x: 10, y: 11 });
    // 调用方法
    boxed.play();
    // 取 boxed 实例引用
    let ref_boxed = &boxed;
    ref_boxed.play();
    // 打印 boxed 实例的引用
    println!("{:?}", ref_boxed);

    // 将 boxed 作为可变引用
    let mut boxed = Box::new(Point { x: 0, y: 1 });
    let mut_boxed = &mut boxed;
    // 更新
    **mut_boxed = Point { x: 0, y: 0 };
    println!("{:?}", boxed);
}

/// `Box<Self>` 的三态
///
/// 三态中除了 self / &self/ &mut self 还有一种变体 `Self: Box<Self>`
#[allow(unused)]
fn code_example11() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn play_ref(&self) {
            println!("I'am play_ref of Point.");
        }

        fn play_mut_ref(&mut self) {
            println!("I'am play_mutref of Point.");
        }

        fn play_own(self) {
            println!("I'am play_own of Point.");
        }

        // ! 注意这里
        // 当 play_box_own() 被调用的时候会解引用 Box<Self>
        // 如果已经调用了 play_own() 移走所有权, 那么 play_box_own() 上的 Box<Self> 就失效了
        fn play_box_own(self: Box<Self>) {
            println!("I'am play_box_own of Point.");
        }
    }

    let mut boxed = Box::new(Point { x: 10, y: 20 });
    boxed.play_ref();
    boxed.play_mut_ref();
    boxed.play_own();
    // boxed.play_box_own(); // ! play_own() 和 play_box_own() 只能打开一个
}

/// `Box<dyn Trait>`
///
/// 特征对象往往代表一种类型, 可以代理一批其他的类型  
/// 但是 dyn trait 本身尺寸在编译器是未知的  
/// 所以 dyn trait 总是借助引用或者智能指针
///
/// ****
/// ### 而 `Box<dyn Trait>` 比 `&dyn Trait` 更常见  
/// 原因就是 Box 拥有所有权, 使用方便, 而 &dyn Trait 不拥有所有权, 有的时候就没那么方便
#[allow(unused)]
fn code_example12() {
    ()
}
