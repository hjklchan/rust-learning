/// 进阶之全局变量

/// 静态常量
/// 全局常量可以和程序任何一部分使用
/// 但是如果定义在某个模块中, 需要引入对应的模块使用
/// 常量就是不可变的
#[allow(unused)]
const MAX_ID: usize = usize::MAX;
/// 常量和普通变量的区别
/// 常量必须指明类型
/// 定义常量时变量的命名规则一般时全大写
/// 常量可以用在任意作用域, 其生命周期贯穿整个程序的生命周期
/// 常量的赋值方式只能是常量表达式和数学表达式
/// 常量无法重复定义
/// ! 常量在编译时编译器会尽可能将其内联, 所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址

/// 静态变量
/// 静态变量允许声明一个全局的变量, 常用于全局数据统计
#[allow(unused)]
static mut REQUEST_RECV: usize = 0;
/// 静态变量必须使用 unsafe 代码块才能访问和修改值
/// ! 静态变量不会被内联, 在整个程序中, 静态变量只有一个实例, 所有的引用都会指向同一个地址

/// 原子类型
/// 想要全局计数器或者状态控制等等, 但又想线程安全, 可以使用原子类型
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};
/// 初始化原子类型
#[allow(unused)]
static REQUEST_RECV_NEW: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("该程序允许最大的用户数量 {}", MAX_ID);

    // 更新静态变量的值
    unsafe {
        REQUEST_RECV += 1;
    }

    // 使用原子计数器
    for _ in 0..100 {
        REQUEST_RECV_NEW.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户的请求数量是 {:?}", REQUEST_RECV_NEW);

    example01();
    example04();
}

/// # example01 全局 ID 生成器
#[allow(unused)]
fn example01() {
    struct Factory {
        factory_id: usize,
    }

    // 定义全局变量的计数器
    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    const MAX_ID: usize = usize::MAX / 2;

    /// generate_id 生成 ID 函数
    fn generate_id() -> usize {
        // 获取当前计数器的累计值
        let current_val: usize = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        // 第一次检查溢出情况
        if current_val > MAX_ID {
            panic!("Factory ids overflowed");
        }
        // 自增 ID
        GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        // 获取新的 ID 的值
        let new_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        // 再一次检查溢出情况
        if new_id > MAX_ID {
            panic!("Factory ids overflowed");
        }
        // 返回新的 ID
        new_id
    }

    impl Factory {
        /// # new 创建工厂实例
        /// 每次创建工厂实例时会被计数累计
        fn new() -> Factory {
            Factory {
                factory_id: generate_id(),
            }
        }
    }

    // Test usecase
    Factory::new();
    println!("{}", GLOBAL_ID_COUNTER.load(Ordering::Relaxed));
    Factory::new();
    Factory::new();
    println!("{}", GLOBAL_ID_COUNTER.load(Ordering::Relaxed));
}

/// TODO
/// # example02 运行期初始化
#[allow(unused_imports)]
use std::sync::Mutex;
// static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
#[allow(unused)]
fn example02() {}

/// # example03 Box::leak 可用于全局变量
/// FIXME 当不使用 lazy_static 和 Box::leak 的情况下
#[allow(unused)]
fn example03() {
    /// 配置
    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }
    // 全局静态变量
    static mut CONFIG: Option<&mut Config> = None;
    // 初始化全局配置
    unsafe {
        // FIXME 以下代码会发生错误
        // FIXME 此问题是试图将一个局部声明周期的变量赋值给全局变量的生命周期
        // CONFIG = Some(&mut Config {
        //     a: String::from("a of config"),
        //     b: String::from("b of config"),
        // });

        println!("{CONFIG:?}");
    }
}

/// # example04 Box::leak 可用于全局变量
/// FIXME 使用 Box::leak 可以将一个变量从内存中泄露
/// 然后再将其变为 'static 生命周期, 最终可以跟整个程序活的一样长
#[allow(unused)]
fn example04() {
    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }
    // 全局静态配置变量
    static mut CONFIG_NWE: Option<&mut Config> = None;
    // 开始运行时初始化静态配置变量
    unsafe {
        // 初始化
        CONFIG_NWE = Some(Box::leak(Box::new(Config {
            a: String::from("hello"),
            b: String::from("hjkl1"),
        })));
        // 查看配置
        println!("currently config {CONFIG_NWE:?}");
    }
}

/// # example05 从函数中返回全局变量
/// 依然可以使用 Box::leak 将局部变量从内存泄漏
#[allow(unused)]
fn example06() {
    #[derive(Debug)]
    struct Config {
        c: String,
        d: String,
    }
    static mut APP_CONFIG: Option<&mut Config> = None;
    // 定义初始化函数
    // 然后返回全局变量
    fn init() -> Option<&'static mut Config> {
        Some(Box::leak(Box::new(Config {
            c: String::from("halo"),
            d: String::from("world"),
        })))
    }
    // ...
    unsafe {
        APP_CONFIG = init();
        println!("currently config {APP_CONFIG:?}");
    }
}