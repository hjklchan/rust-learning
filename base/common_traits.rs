// 常用的 Traits
// Commonly used Traits

fn main() {
    // default_example();
    // display_example();
    // to_string_example()
    // partial_eq_and_eq_example();
    // add_example()
    // clone_example();
    // copy_example();
    // to_owned_example();
    // deref_example();
    // drop_example();
    // from_and_into_example();
    // from_str_example();
    as_ref_example();
}

/// **Default** Trait 默认
/// 
/// *难度* ⭐
#[allow(unused)]
pub fn default_example() {
    #[derive(Debug)]
    struct Color(u8, u8, u8);

    impl Default for Color {
        fn default() -> Self {
            // 默认字段值
            Self(110, 120, 0)
        }
    }

    // test use case
    let _c = Color::default();
    let _c: Color = Default::default();

    fn default_color(t: Option<Color>) {
        // 可以使用 unwrap_or_default 解包出 Color 或者是 Color::default
        let _c = t.unwrap_or_default();
        println!("{:?}", _c);
    }
    default_color(None);

    /// NOTE: 支持部分更新语法
    /// ## Example
    /// ```
    /// struct Color { r: u8, g: u8, b: u8 }
    /// 
    /// impl Default for Color {
    ///     fn default() -> Self {
    ///         Self { r: 0, g: 100, b: 101 }
    ///     }
    /// }
    /// 
    /// let _c = Color::default();
    /// let _new_c = Color { r: 100, ..Color::default() }; /// <-
    /// ```
    fn _unless() {}
}

/// **Display** Trait 用于决定一个类型的值如何显示, 其意义就是如何使用字符串来表达
/// 
/// *难度* ⭐
///
/// Display Trait 对应格式化符合 `{}`, 如: `println!("{}", xxx)`
#[allow(unused)]
pub fn display_example() {
    struct Point {
        x: i32,
        y: i32,
    }
    // impl Display trait
    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Customize your display formatting
            write!(f, "Point ({} {})", self.x, self.y)
        }
    }

    // test use case
    let _p = Point {
        x: 10, y: 110,
    };
    println!("{}", _p); // > Point (10 110)
}

/// **ToString** Trait 用于将各种类型实例转为字符串方式显示
/// 
/// *难度* ⭐
#[allow(unused)]
pub fn to_string_example() {
    struct Person {
        name: String,
        age: u8,
    }
    // impl ToString trait
    impl ToString for Person {
        fn to_string(&self) -> String {
            // 格式化输出
            format!("{} 今年刚满 {} 岁", self.name, self.age)
        }
    }

    // test use case
    let _p = Person {
        name: String::from("Hjkl1"),
        age: 18,
    };
    // 将实例转为字符串输出
    println!("{}", _p.to_string()); // > Hjkl1 今年刚满 18 岁
}

/// **Debug** Trait 用于将各种类型实例转为字符串方式显示
/// 
/// *难度* ⭐
/// 
/// Debug 与 Display 很像, Display 的配对格式是 `{}`, 而 Debug 的 配对格式是 `{:?}`
/// 
/// 另外 Debug 还配套了美化版格式 `{:#?}`, 可将结构体输出格式更具结构化
#[allow(unused)]
fn debug_example() {
    display_example();
}

/// **PartialEq** 和 **Eq** Trait 值比较值特征
/// 
/// 若一个类型实现了 PartialEq, 那么就能比较两个值是否相等
/// 
/// - 对称性（symmetry): a == b 导出 b == a
/// - 传递性（transitivity): a == b && b == c 导出 a == c
/// 
/// 而 Eq 是 PartialEq 的子特征, 比较典型的是 Rust 中浮点数只实现了 PartialEq, 没有实现 Eq
/// 
/// 若一个类型的所有字段实现了 PartialEq
/// 
/// *难度* ⭐⭐
#[allow(unused)]
fn partial_eq_and_eq_example() {
    // 为 Point 结构体实现 PartialEq 特征
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 创建两个实例进行值比较
    assert_eq!(Point { x: 1, y: 2}, Point { x: 2, y: 2 }); // error: assertion `left == right` failed
}

/// **PartialOrd** 和 **Ord** Trait
/// 
/// 跟 PartialEq 差不多, PartialEq 只判断相等或不相等，PartialOrd 在这个基础上进一步判断是小于 / 小于等于 / 大于还是大于等于
/// 
/// PartialOrd 被定义为 PartialEq 的 sub trait
/// 
/// *由于 Ord 严格的顺序性, 如果一个类型实现了 Ord, 那么这个类型可以被用作 BTreeMap 或 BTreeSet 的 key*
/// 
/// *难度* ⭐⭐
#[allow(unused)]
fn partial_ord_and_ord_example() {
    // 可以通过过程宏一起实现
    #[derive(PartialEq, PartialOrd)]
    struct Point {
        x: i32, y: i32,
    }
    // 测试用例跳过
}

/// **Add** Trait 是对 (+) 做自定义的运算符重载
/// 
/// 默认输出类型是 `Self`
/// 
/// ### 同样的 Trait 还有 Sub Trait
/// 
/// *难度* ⭐
#[allow(unused)]
fn add_example() {
    #[derive(Debug)]
    struct Point {
        x: i32, y: i32,
    }
    // 为 Point 实现 Add Trait 能够让两个 Point 实例直接相加
    impl std::ops::Add for Point {
        type Output = Self;
        // ! 会消耗两实参的所有权
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    // test use case
    let p1 = Point {
        x: 10, y: 21,
    };
    let p2 = Point {
        x: 21, y: 11,
    };
    let res = p1 + p2;
    println!("{res:#?}");
    // 尝试打印 p1 和 p2 会报错
    // println!("{p1:#?}"); // ! error: value borrowed here after move
    // println!("{p2:#?}"); // ! error: value borrowed here after move
}

/// **Clone** Trait 用于完整的克隆实例
/// 
/// *使用 Clone 派生宏能方便的为目标类型实现 Clone Trait*
/// 
/// 一下会有两种情况:
/// 1. 第一种是已经拿到实例的所有权, 去 clone 一份生成一个新的所有权并被局部变量所持有
/// 2. 第二种是只拿到一个实例的引用, 想拿到它的所有权，如果这个类型实现了 Clone trait, 那么就可以 clone 一份拿到这个所有权
/// 
/// ### 克隆是对对象的深拷贝, 通常消耗比较大的负载, 而浅拷贝是按值拷贝一块连续的内存
/// 
/// *难度* ⭐
#[allow(unused)]
pub fn clone_example() {
    ()
}

/// **Copy** Trait 用于完整的克隆实例
/// 
/// Copy 是 Clone 的 sub trait, 仅仅是一个 Marker
/// 
/// 直接实现 Copy Trait 是不行的 👇
/// ```
/// impl Copy for Xxx {}
/// ```
/// 
/// 但是标准库提供 Copy 过程宏来自动为目标实现 Copy Trait
/// ```
/// #[derive(Copy, Clone)]
/// struct Xxx {}
/// ```
/// ## 注意:
/// 实现了 Copy Trait 就无法实现 Drop Trait!
/// 
#[allow(unused)]
pub fn copy_example() {
    // 既然 Copy 是 Clone 的 sub trait, 所以自然需要先实现 Clone
    // 将 Clone 和 Copy 一次性用过程宏派生出来
    #[derive(Clone, Copy, Debug)]
    struct Point {
        x: i32, y: i32
    }
    // use cases
    let p1 = Point {x: 10, y: 23};
    let mut p2 = p1;
    println!("before: {p1:#?}");
    println!("before: {p2:#?}");

    /// Copy Trait 只赋值固定尺寸的值
    // ? 为什么 Point 结构体里的字段都是固定尺寸的 (即复制语义), 那为什么不默认实现 Copy 呢?
    // > 因为 Rust 故意设计的, 在所有权的设计下, Rust 默认选择 Move 语义 (即所有权转移)
}

/// **ToOwned** Trait 为类型提供 to_owned() 方法, 可以将引用转换为所有权示例
#[allow(unused)]
pub fn to_owned_example() {
    let a: &str = "hjkl1!";
    let o: String = a.to_owned();
    println!("{}", o);
}

/// **Deref** Trait 用于将一个类型转换为另一个类型,
/// 但需要在引用符号 `&` 和点 `.` 操作符或其他智能指针的触发下才产生转换
///
/// ****
/// 比如标准库里常见的 `&String` 可以自动转换为 `&str`, 因为 String 实现了 Deref Trait  
/// 还有 `&Vec<T>` 转换成 `&[T]`
///
#[allow(unused)]
fn deref_example() {
    ()
}

/// **Drop** Trait 用于为类型值做自定义垃圾清理
/// 实现了这个 Trait 的类型实例在走出作用域时会触发 drop() 方法, 该调用发生在实例被摧毁之前
#[allow(unused)]
pub fn drop_example() {
    struct Point;
    // 为 Point 结构体实现 Drop Trait
    impl Drop for Point {
        fn drop(&mut self) {
            println!("Currently struct will be destroyed");
        }
    }
}

/// **From** 和 **Into** Trait 都是用于类型转换
/// - `From<T>` 用于将类型 T 转为自己
/// - `Into<T>` 用于将自己转为类型 T
///
/// ### 这两个 Trait 是互逆的
/// 实际上只要实现了 `From<T>` 就会自动实现 `Into<T>`, 看下面标准库实现:
/// ```
/// impl Into for Trait
/// where U: From,
/// {
///     fn into(self) -> U {
///         U::from(self)
///     }
/// }
///
/// ```
#[allow(unused)]
pub fn from_and_into_example() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 为 Point 实现 `From<(i32, i32)>` 特征 (将元组转换为 Point 实例)
    impl From<(i32, i32)> for Point {
        fn from(value: (i32, i32)) -> Self {
            Self {
                x: value.0,
                y: value.1,
            }
        }
    }
    // 为 Point 实现 `From<[i32; 2]>` 特征 (将数组转换为 Point 实例)
    impl From<[i32; 2]> for Point {
        fn from(value: [i32; 2]) -> Self {
            Self {
                x: value[0],
                y: value[1],
            }
        }
    }
    // use cases
    // 使用 from() 支持不同类型的转换
    let p1 = Point::from((10, 12));
    let p2 = Point::from([1, 2]);
    println!("{:?} {:?}", p1, p2);
    // 使用 into() 支持不同类型的转换
    let p3: Point = (11, 13).into();
    let p4: Point = (2, 4).into();
    println!("{:?} {:?}", p3, p4);
}

/// **TryFrom** 和 **TryInto** Trait 都是用于类型转换
/// 相比于 From 和 Into, TryXxx 是可失败的, 都会返回 Result
#[allow(unused)]
pub fn try_from_and_try_into_example() {
    ()
}

/// **FromStr** 用于从字符串类型转换到自身
///
/// ## FromStr Trait 标准库源码
/// ```
/// trait FromStr {
///     type Err;
///     fn from_str(s: &str) -> Result<Self, Self::Err>;
/// }
/// ```
/// 该 Trait 就是字符串 `parse()` 背后的 Trait
#[allow(unused)]
pub fn from_str_example() {
    todo!()
}

/// **AsRef** Trait 用于把自身引用转换为目标类型的引用
/// ## 和 Deref 的区别
/// `**deref()` 是隐式调用的, 而 `as_ref()` 需要显示的调用 `**`
#[allow(unused)]
fn as_ref_example() {
    struct Point {
        x: i32, y: i32,
    }
    // 为 Point 实现 `AsRef<T>` 特征
    impl AsRef<str> for Point {
        fn as_ref(&self) -> &str {
            "这是一个 Point 结构体"
        }
    }
    // use cases
    let p: Point = Point { x: 12, y: 32 };
    let s = p.as_ref();
    println!("{}", s);
}