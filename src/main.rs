
use std::{cmp::max, collections::HashMap, fmt::{Display, LowerExp}, fs::File, io::{self, ErrorKind, Read, Write}, panic};

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    println!("Hello, world!");
    let mut x = String::from("123");
    // let y: u32 = x.parse().unwrap();
    let x_c = take(&mut x);
    // 只能借出去一次可变, 但是可以用多个作用域来操作
    // let x_b = &mut x;
    // let x_d = &mut x;
    // println!("x_b: {} x_c: {}", x_b, x_c);
    // print!("{}", y);
    println!("x: {}", x);
    // 尽可能的使用&str 而不是String作为参数
    // println!("{}", first_word(&s));
    // Struct
    let user1 = User {
        email: String::from(""),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 更新赋值，解构赋值
    let user2 = User {
        email: String::from(""),
        username: String::from("someusername456"),
        ..user1
        
    };
    println!("{}", user2.sign_in_count);
    // tuple struct
    struct Color(i32, i32, i32);
    let color = Color(0, 127, 255);
    println!("{}", color.2);
    // unit like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // struct
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    // :# 可以显示的更加优雅
    println!("{:#?}", rect1);
    // impl for struct
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    println!("{}", rect1.area());

    let s = Rectangle::square(3);
    println!("{}", s.area());
    // 枚举
    enum IpAddrKind {
        V4,
        V6,
    };
    struct Ipaddr{
        kind: IpAddrKind,
        address: String,
    }
    let home = Ipaddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    fn ip_route_message(ip_kind: Ipaddr) {
    
    }
    ip_route_message(home);

    // 可以在枚举的变量中假如任何的类型
    enum IpAddrKind2 {
        V4(String),
        V6(String),
    };
    // 可以对enum执行impl方法, 传入参数然后使用 =>
    impl IpAddrKind2 {
        fn route(&self) {
            match self {
                IpAddrKind2::V4(addr) => println!("{}", addr),
                _ => println!("not v4"),
            }
        }
    }
    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    home.route();
    // option 类型
    let x = Some(5);
    let y = 3;
    // 如何使x可以和y相加
    // let sum = x + y;
    let sum = match x {
        Some(num)  => num + y,
        None => y 
    };
    println!("{}", sum);
    // if let
    // 只关心一种匹配 并且必须放在等号后面需要匹配的值, 类型也必须一致
    // 并且可以搭配else使用
    let some_u8_value = Some("23");
    if let Some("231") = some_u8_value {
        println!("three")
    } else {
        println!("not three")
    }
    // hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // use tuple to create key value pair
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 可以自动推导而无需指定类型
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // string类型用于hashmap后会丢失所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // 使用get方法得到值，该方法需要使用一个引用
    let score = scores.get(&String::from("Blue"));
    match score {
        Some(s) => println!("{}", s),
        None => println!("not found"),
    }
    // 重复插入会覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 5220);
    println!("{:?}", scores);
    // entry 方法 可以避免重复插入
    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(10);
    scores.insert(String::from("Yellow"), 100);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);
    // hash example
    let test = "hello world hello world wonderful world";
    let mut maps = HashMap::new();

    for word in test.split_whitespace() {
        // 返回对应的值，而无需使用get方法
        let count = maps.entry(word).or_insert(0);
        // * 的作用是解引用
        *count += 1;
    } 
    println!("{:?}", maps);
    // pannic 错误处理
    // 闭包
    let x = 5;
    let equal_to_x = |z| z == x;
    let y = 5;
    assert!(equal_to_x(y));

    // panic!("crash");
    let vec = vec![1, 2, 3];
    // vec[99];

    // 使用result处理错误
    // let f = File::open("hello.txt");
    // let mut file = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("create failed: {:?}", e),
    //         },
    //         _ => panic!("other error: {:?}", error),
            
    //     },
    // };
    // let mut res = String::new();

    // let error = file.read_to_string(&mut res);
  
    // println!("{}", res);

    // let result = file.write(b"hello");
    // match result {
    //     Ok(n) => println!("{}", n),
    //     Err(e) => panic!("write failed: {:?}", e),
    // }

    // unwrap 是一个快速的match
    // let f = File::open("hello.txt").unwrap();
    // exception_result!
    // let f = File::open("hello.txt").expect("failed to open hello.txt");
    // ? 可以捕获错误
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    // let s = read_username_from_file();
    // 

    let numbers = vec![1, 2, 3];
    let max_num = numbers.iter().max().unwrap();
    println!("{}", max_num);
    // 泛型
    fn longest<T>(list:&[T]) -> usize {
        list.len()
    }
    let s = vec!["hello", "world"];
    println!("{}", longest(&s));
    let s = vec![1,2,3,4];
    println!("{}", longest(&s));
    // stuct 泛型
    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point<T, U> {
        fn mixup<A, B>(self, other: Point<A, B>) -> Point<T, B> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // trait 与其他类型共享的行为
    // 类似于虚函数

    trait Summary {
        fn summarize(&self) -> String;
        // 可以多个函数
        // fn summarize_author(&self) -> String;
    }
    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // 必须实现所有的trait方法如果使用impl
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    // 简单的情况使用trait作为参数
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    notify(&news);
    notify(&tweet);
    // trait bound
    fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    notify2(&news);
    notify2(&tweet);
    // 使用 + 实现多个trait
    fn notify3<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // 使用where clause
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    //     1
    // }
    // trait 作为返回
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    let s = returns_summarizable();
    println!("{}", s.summarize());

    // 生命周期
    let s1 = "I have a static lifetime.";
    let s2 = String::from("Hello");
    let r = longest_str(s1, s2.as_str());
    println!("The longest string is {}", r);
    // 生命周期的描述不会改变，但是 rust 会自动推导出生命周期
    // ‘a是表示生命周期比较小的一个
    // 如果函数不返回引用而是直接返回值，则表示函数将该值所有权交给了主函数，这也是rust的设计模式之一
    // struct 的生命周期，如果其中的值是引用，则必须标注生命周期
    struct important_excerpt<'a>{
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    {
        let i: important_excerpt<'_> = important_excerpt{ part: first_sentence};
        println!("{}", i.part);
    }
    // 生命周期的省略的三个规则
    // 1. 函数参数的生命周期，函数返回值的生命周期，函数内部使用的生命周期，可以省略
    // 2. 如果只有一个输入生命周期，则该生命周期与返回值的生命周期相同
    // 3. 如果有多个输入生命周期，则必须为所有生命周期都指定一个，并且该生命周期必须与返回值的生命周期相同


    // ’static 生命周期，表示该引用指向一个只读的静态字符串
    let s: &'static str = "I have a static lifetime.";
    // 他总是可以用的

}

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_sentence(text: &str) -> &str {
    let first = text.find('.').unwrap();
    &text[..first]
}
fn take(s:&mut String) -> String {
    s.push_str("4");
    s.clone()
}