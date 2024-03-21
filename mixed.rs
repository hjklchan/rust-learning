#[allow(unused_imports)]
use std::ops::Deref;

fn main() {
    drop_example01();
    drop_example02();
}

#[allow(unused)]
fn drop_example02() {
    // 手动回收内存
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo");
        }
    }

    let foo = Foo;

    drop(foo);
}

#[allow(unused)]
fn drop_example01() {
    struct HasDrop1;
    struct HasDrop2;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }
    struct HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    }
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }

    // 到花括号结束后调用 drop 方法
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    // 到花括号结束后调用 drop 方法
    let _foo = Foo;
    println!("Running!");
}

#[allow(unused)]
fn deref_example01() {
    struct GoodBox<T>(T);

    impl<T> GoodBox<T> {
        fn new(t: T) -> GoodBox<T> {
            GoodBox(t)
        }
    }

    impl<T> std::ops::Deref for GoodBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> std::ops::DerefMut for GoodBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            return &mut self.0;
        }
    }

    let s = GoodBox::new(String::from("hello, rust!"));

    // assert_eq!(1, *i1);

    fn got_str(_s: &str) {
        println!("{_s}");
    }

    let se: String = String::from("value");
    let sep: &str = &se[..];

    got_str(&(*s)[..]);

    let mut gb1 = GoodBox::new(1);
    *gb1 = 2;
    println!("{}", *gb1);
}

/// deref
#[allow(unused)]
fn defer_regular() {
    let x = 6;
    let y = &x;
    assert_eq!(6, x);
    assert_eq!(6, *y);
}

#[allow(unused)]
fn box_in_vec() {
    let arr = vec![Box::new(2), Box::new(0)];
    let (first, second) = (&arr[0], &arr[1]);
    let v = (*first).to_owned();
}

#[allow(unused)]
fn trait_object() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("这是屏幕上 ID 为 {} 的按钮", self.id);
        }
    }

    struct Select {
        id: u32,
    }

    impl Draw for Select {
        fn draw(&self) {
            println!("这个选择框贼难用 {}", self.id);
        }
    }

    let elements: Vec<Box<dyn Draw>> = vec![Box::new(Select { id: 3 }), Box::new(Button { id: 0 })];
    for elem in elements.into_iter() {
        elem.draw();
    }
}

#[allow(unused)]
fn data_copy() {
    let arr: [i32; 1000] = [0; 1000];
    let arr_cpy = arr;

    println!("{:?}", arr.len());
    println!("{:?}", arr_cpy.len());
}

#[allow(unused)]
fn enum_example01() {
    enum AtomicNumber {
        HYDROGEN = 1,
        HELIUM = 2,
        // ...
        IRON = 26,
    }
}

#[allow(unused)]
fn generic_type_of_fixed_size<T: Sized>(_t: T) {}

#[allow(unused)]
fn impl_iterator_trait_example01() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
}

#[allow(unused)]
fn iterators_are_lazy() {
    let v = vec![1, 10, 78];
    let col: i32 = v
        .iter()
        .map(|x| {
            println!("map: {x}");
            x + 1
        })
        .filter(|x| {
            println!("filter: {x}");
            x % 2 == 1
        })
        .sum();
    println!("{col:?}");
}

#[allow(unused)]
fn consume_iter_example01() {
    let v1 = vec![1, 4, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{total}");
}

#[allow(unused)]
fn into_iter_example01() {
    let arr = [1, 4, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(Some(1), arr_iter.next());
    assert_eq!(Some(4), arr_iter.next());
    assert_eq!(Some(3), arr_iter.next());

    println!("{:?}", arr);
}

#[allow(unused)]
fn fn_once_example() {
    let mut s = String::from("value");
    let mut f = |substr| s.push_str(substr);
    f(" key");
    println!("{}", s);
}
