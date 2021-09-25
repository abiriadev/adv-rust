// use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Millimeters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

struct Meters(i32);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        let Point { x, y } = self;
        let Point { x: x2, y: y2 } = other;

        Point {
            x: x + x2,
            y: y + y2,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dug;

impl Dug {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dug {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let string = self.to_string();

        let len = string.len();

        println!("{}", "*".repeat(len + 2));
        println!("*{}*", string);
        println!("{}", "*".repeat(len + 2));
    }
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

type Kilometers = i32;

type Res<T> = Result<T, std::io::Error>;

fn _never() -> ! {
    panic!("panic!")
}

fn _never2() -> ! {
    loop {}
}

fn _unsized_f<T: ?Sized>(_t: &T) {}

fn add_one(inp: i32) -> i32 {
    inp + 1
}

fn do_twice(f: fn(i32) -> i32, v: i32) -> i32 {
    f(v) + f(v)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    _Stop,
}

fn ret_cl(a: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |b| a + b)
}

fn main() {
    let p1 = Point { x: 1, y: 5 };
    let p2 = Point { x: 6, y: 7 };

    let p3 = p1 + p2;

    assert_eq!(p3, Point { x: 7, y: 12 });

    println!("{:?}", Millimeters(120) + Meters(5));

    let human = Human;

    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    println!("{}", Dug::baby_name());
    println!("{}", <Dug as Animal>::baby_name());

    p3.outline_print();

    let wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("wrapper: {}", wrapper);

    let k: Kilometers = 44;

    println!("distance: {}K", k + 7);

    let res: Res<&str> = Ok("happy");

    if let Ok(v) = res {
        println!("{}", v);
    }

    let res = do_twice(add_one, 5);

    println!("res: {:?}", res);

    let list_of_string: Vec<String> = vec![1, 2, 3].iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_string);

    let list_of_status: Vec<Status> = (0u32..10).map(Status::Value).collect();

    println!("{:?}", list_of_status);

    println!("{:?}", (*ret_cl(2))(3));

    let v = vec2![1, 2, 4];

    println!("{:?}", v);

    let f = tup![1; 2];

    println!("{:?}", f);
}

#[macro_export]
macro_rules! vec2 {
    ($($x:expr),*)=> {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! tup {
    ($x:expr;$y:expr) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        ($x, $y)
        // temp_vec
    }};
}
