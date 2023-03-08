use std::fmt::{Debug, Display};

fn main() {
    let x = return_debug1();
    println!("{:?}", x);

    let a = Animal { kind: Dog {} };
    a.info();
    println!("{}", a);
}

// 返回接口的实现类
fn return_debug() -> impl Debug {
    vec![0]
}

#[derive(Debug)]
struct Dog {}
// impl ToString for Dog {
//     fn to_string(&self) -> String {
//         String::from("wang wang wang...")
//     }
// }
// 实现了display接口的类型，会自动实现tostring
impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", "wang wang wang...");
        // println!("{}", self.to_string());// 在display中调用to_string会报错，递归了
        Result::Ok(())
    }
}
#[derive(Debug)]
struct Cat {}
// impl trait仅能在返回类型是一种的情况下使用
// 如果a、b同时实现接口Debug，return_debug可能返回a或者b，此时 impl Debug不能编译
// fn return_debug1() -> impl Debug {
//     if true {
//         return Dog {};
//     } else {
//         return Cat {};
//     }
// }
// dyn 关键字用于突出显示在关联的 Trait 上对方法的调用是动态调度的。 要以这种方式使用 trait，它必须是对象安全的。
// 与泛型参数或 impl Trait 不同，编译器不知道要传递的具体类型。即，类型为 erased。 因此，dyn Trait 引用包含 two 指针。 一个指针指向该数据 (例如，结构体的实例)。 另一个指针指向方法名称为函数指针的 map (称为虚拟方法表或 vtable)。
// 在运行时，当需要在 dyn Trait 上调用方法时，将查询 vtable 以获取函数指针，然后调用该函数指针。
// dyn 可实现返回对象的动态化
fn return_debug1() -> Box<dyn Debug> {
    if false {
        return Box::new(Dog {});
    } else {
        return Box::new(Cat {});
    }
}

#[derive(Debug)]
struct Animal<T> {
    kind: T,
}
// 仅为实现类Display接口的范型类型实现接口
// 指定条件下实现接口
impl<T: Display> Animal<T> {
    fn info(&self) {
        println!("{}", self.kind);
    }
}
impl<T: ToString> Display for Animal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", self.kind.to_string());
        Result::Ok(())
    }
}
