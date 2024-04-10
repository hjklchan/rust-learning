/// 错误处理

fn main() {
    // example02();
    // example04();
    example06();
}

// 组合器
// 设计模式 - 组合器模式
// 常见的组合器
//
/// # example01
// or() 表达式按顺序求值, 若任意一个表达式结果是 Some 或 Ok, 则该值立即返回
// and() 若两个表达式都是 Some 或 Ok, 则第二个表达式的值返回
#[allow(unused)]
fn example01() {
    let _s1 = Some("some1");
    let _s2 = Some("some2");
    let _non: Option<&str> = None;

    let _r1: Result<&str, &str> = Result::Ok("ok1");
    let _r2: Result<&str, &str> = Result::Ok("ok2");
    let _e1: Result<&str, &str> = Result::Err("err1");
    let _e2: Result<&str, &str> = Result::Err("err2");

    // 参考: https://course.rs/advance/errors.html#or-和-and
}

/// # example02 or_else() 和 and_then()
// 与 or 和 and 不同的是她们的第二个参数是闭包
#[allow(unused)]
fn example02() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some3");

    let non: Option<&str> = None;
    let fn_non = || None;

    assert_eq!(s1.or_else(fn_some), Some("some1"));
    assert_eq!(s1.or_else(fn_non), Some("some1"));

    assert_eq!(non.or_else(fn_some), Some("some3"));
    assert_eq!(non.or_else(fn_non), non);

    // 其他参考: https://course.rs/advance/errors.html#or_else-%E5%92%8C-and_then
}

/// # example03 filter
/// filter 用于对 Option 进行过滤
#[allow(unused)]
fn example03() {
    let s1 = Some(3);
    let s2 = Some(6);

    let n: Option<i8> = None;

    let fn_is_even = |x: &i8| *x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n);
    assert_eq!(s2.filter(fn_is_even), s2);
}

/// # example04 map() 和 map_err()
/// map 可以将 Some 和 Ok 中的值映射为另一个
#[allow(unused)]
fn example04() {
    let s1 = Some("abcde");
    let s2 = Some(6);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let r1: Result<&str, &str> = Ok("cdefg");
    let r2: Result<usize, &str> = Ok(5);

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), Some(5));
    // 其他参考: https://course.rs/advance/errors.html#map-%E5%92%8C-map_err

    // map_err 例子
    let ok: Result<&str, &str> = Ok("everything is ok");
    let err: Result<&str, &str> = Err("404");

    let ok2: Result<&str, i32> = Ok("everything is ok");
    let err2: Result<&str, i32> = Err(404);

    let f1 = |_s: &str| _s.parse::<i32>().unwrap();
    assert_eq!(ok.map_err(f1), ok2);
    assert_eq!(err.map_err(f1), err2);
}

/// # example05 自定义错误类型
/// 自定义的错误类型实现 `Error: Debug + Display` 后
/// 该类型可以作为 Err 使用
/// Debug 特征往往无需手动实现, 可以直接通过 derive 派生
#[allow(unused)]
fn example05() {
    // 最简单的错误
    #[derive(Debug)] // 为 AppError 派生 Debug 特征
    struct AppError;
    // 为 AppError 实现 Display 特征
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "An error occurred, please try again!")
        }
    }
    // 实例函数用于产生 AppError 错误
    fn generate_err() -> Result<(), AppError> {
        Err(AppError)
    }
    // 匹配错误
    match generate_err() {
        Err(err) => {
            eprintln!("{}", err);
        }
        _ => {
            println!("It's ok");
        }
    }
    eprintln!("{:?}", generate_err());
}

/// # example06 更详细的错误
/// 上个示例中无法获得更多信息
/// 以下新的错误中包含错误码和错误信息等等
#[allow(unused)]
fn example06() {
    // 自定义错误
    struct AppError {
        code: usize,
        message: String,
    }
    // 为自定义错误实现 Display 特征
    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let errmsg = match self.code {
                404 => "Sry, Cannot find the route.",
                _ => "Sry, something is wrong!",
            };
            return write!(f, "{}", errmsg);
        }
    }
    // 为自定义错误实现 Debug 特征
    impl std::fmt::Debug for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AppError {{ code: {}, message: {} }}",
                   self.code, self.message
            )
        }
    }
    // 生成一个错误示例
    fn generate_err() -> Result<(), AppError> {
        Err(AppError {
            code: 404,
            message: "找不到路由".to_string(),
        })
    }
    // Test usecase
    match generate_err() {
        Err(msg) => println!("{msg:?}"),
        _ => println!("其他错误"),
    };
}

/// # example07 错误转换 From 特征
/// 将其他错误转成自定义错误类型可以使用 std::convert::From 特征
#[allow(unused)]
fn example07() {
    struct AppError {
        kind: String,    // 错误类型
        message: String, // 错误信息
    }
    // 为 AppError 实现 std::convert::From 特征, From 包含在 std::prelude 中
}