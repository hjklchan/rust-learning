/// 异步并发
#[allow(unused)]
struct Description;

fn main() {

}

/// async / await 关键字
/// 
/// 可以用顺序的逻辑书写方式来写出异步执行的代码
/// 
/// ## async 函数
/// ```rust
/// async fn foo() {}
/// ```
/// ## async 代码块
/// ```
/// async fn foo() {
///     async {
///
///     }.await
/// }
/// 
/// 
/// async fn foo() {
///     /** 明确的将环境变量 move 进来 **/
///     async move {
///
///     }.await
/// }
/// ```
#[allow(unused)]
fn code_example01() {
    ()
}

/// async 被视作为一个 Future 对象  
/// 类似 Js 的 Promise, async 用来定义这个 Future 对象  
/// 定义好的这个异步代码不会自动执行, 需要 async 配对的 .await 驱动
/// 
/// ! await 关键字只能在 async 块或函数里使用!
#[allow(unused)]
fn code_example02() {
    async fn foo() {
        // 用 .await 驱动异步代码去执行
        let a = async {
            // do something here...
        };
        a.await;

        // 或是更简洁的写法
        async {}.await;
    }
}

/// 程序是从 main 函数开始执行的  
/// ! 但是 main 函数不能用 async 修饰!
///
/// 所以必然引入一个**外部驱动机制**, 比如有一种辅助函数,
/// 它可以接收一个 Future 并驱动它执行, 而不需要 .await  
/// ## For Example:
/// ```
/// let a = async {};
/// /** 驱动执行异步代码 **/
/// block_on(a);
/// ```
/// **block_on** 并不是一个普通函数, 必须是一个运行时的入口函数  
/// 目前 Rust 标准库还没有内置官方的异步 Runtime, 但是 Rust 生态中有很多第三方的 Runtime 实现库, 比如:
/// - tokio
/// - async-std
/// - futures
/// 
/// 而 **tokio** 在第三方异步 Runtime 的激烈竞争中胜出, 可以说它现在已经成为了 Rust 生态中异步运行时事实上的标准
/// 
/// ### Q1: 什么是异步运行时?
/// 不说, 看官方文档
#[allow(unused)]
fn code_example03() {
    ()
}
