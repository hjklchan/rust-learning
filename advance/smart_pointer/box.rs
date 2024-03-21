
/// 进阶之 Box 堆对象分配
fn main() {
    example01();
    example02();
}

/// # example01 使用 Box<T> 将数据存储在堆上
#[allow(unused)]
fn example01() {
    let a = Box::new(1);
    println!("{}", *a);
}

/// # example02 特征对象
#[allow(unused)]
fn example02() {
    // 定义 Drawer 特征
    trait Drawer {
        fn draw(&self);
    }

    // 定义 Button 结构体
    struct Button {
        bid: u32,
    }

    // 为 Button 实现 Drawer 特征
    impl Drawer for Button {
        fn draw(&self) {
            println!("这是屏幕上第 {} 个按钮", self.bid);
        }
    }

    // 定义 Select 结构体
    struct Select {
        sid: i32,
    }

    // 为 Select 实现 Drawer 特征
    impl Drawer for Select {
        fn draw(&self) {
            println!("这个选择框贼难用 {}", self.sid);
        }
    }

    // 使用 vec 存放多个特征对象
    let elements: Vec<Box<dyn Drawer>> = vec![
        Box::new(Select { sid: 101 }),
        Box::new(Button { bid: 1 }),
        Box::new(Select { sid: 0 }),
    ];
    // 遍历每个特征对象的 draw 方法
    for elem in elements.into_iter() {
        elem.draw();
    }
}
