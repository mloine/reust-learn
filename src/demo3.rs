use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
// 该属性用于隐藏对未使用代码的警告。
//#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(input: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = input;

    (y2 - y1).abs() * (x2 - x1).abs()
}

fn square(input: Point, len: f32) -> Rectangle {
    let x2 = input.x + len;
    let y2 = input.y - len;
    Rectangle {
        top_left: input,
        bottom_right: Point { x: x2, y: y2 },
    }
}

// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 },
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}


impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
    
    fn sp(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => 1+ x + y,
            Self::Subtract => 1+ x - y,
        }
    }
}


enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}


// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


fn main() {
    //    ----------------------------------------------------------------------------------
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    //    // 使用简单的写法初始化字段，并创建结构体
    //    let name = String::from("Peter");
    //    let age = 27;
    //    let peter = Person { name, age };
    //
    //    // 以 Debug 方式打印结构体
    //    println!("{:?}", peter);
    //
    //    // 实例化结构体 `Point`
    //    let point: Point = Point { x: 10.3, y: 0.4 };
    //
    //    // 访问 point 的字段
    //    println!("point coordinates: ({}, {})", point.x, point.y);
    //
    //    // 使用结构体更新语法创建新的 point，
    //    // 这样可以用到之前的 point 的字段
    //    let bottom_right = Point { x: 5.2, ..point };
    //    //    let bottom_right = Point { x: 5.2,y: 0.1 };
    //
    //    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    //    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    //
    //    // 使用 `let` 绑定来解构 point
    //    let Point {
    //        x: left_edge,
    //        y: top_edge,
    //    } = point;
    //
    //    let _rectangle = Rectangle {
    //        // 结构体的实例化也是一个表达式
    //        top_left: Point {
    //            x: left_edge,
    //            y: top_edge,
    //        },
    //        bottom_right: bottom_right,
    //    };
    //
    //    // 实例化一个单元结构体
    //    let _unit = Unit;
    //
    //    // 实例化一个元组结构体
    //    let pair = Pair(1, 0.1);
    //
    //    // 访问元组结构体的字段
    //    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    //
    //    // 解构一个元组结构体
    //    let Pair(integer, decimal) = pair;
    //
    //    println!("pair contains {:?} and {:?}", integer, decimal);
    //
    //    println!("rect_area: {}", rect_area(square(point, 5f32)))

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    
    // 创建一个类型别名
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    // 我们可以通过别名引用每个枚举变量，避免使用又长又不方便的枚举名字
    let x = Operations::Add;
    println!("1+2={}",Operations::Add.run(1,2));
    println!("1+2 + 1={}",Operations::Add.sp(1,2));
    
    
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
    
    
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
