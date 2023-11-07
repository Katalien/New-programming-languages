//* 1. Code duplicating */
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(*result, 100);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     assert_eq!(*result, 6000);
// }


// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }

// fn largest<T>(list: &[T]) -> &T {
// }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

//* 2. Structures */
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

//! Not compile

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }


// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }


//* 3. Enums */

// enum Option<T> {
//     Some(T),
//     None,
// }
    

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


//* 4. Methods */
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// fn trait_param(){
//     pub trait Summary { // trait
//         fn summarize(&self) -> String;
//     }

//     pub struct NewsArticle {
//         pub headline: String,
//         pub location: String,
//         pub author: String,
//         pub content: String,
//     }

//     impl Summary for NewsArticle {
//         fn summarize(&self) -> String {
//             format!("{}, by {} ({})", self.headline, self.author, self.location)
//         }
//     }

//     pub struct Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool,
//     }

//     impl Summary for Tweet {
//         fn summarize(&self) -> String {
//             format!("{}: {}", self.username, self.content)
//         }
//     }

//     pub fn notify(item: &impl Summary) { // trait in args
//         println!("Breaking news! {}", item.summarize());
//     }
    
//     pub fn notify2(item1: &impl Summary, item2: &impl Summary) { // traits in args
//         println!("Breaking news! {} {]", item1.summarize(), item2.summarize());
//     }
    
//     pub fn notify2<T: Summary>(item1: &T, item2: &T) { // same notify2
//         println!("Breaking news! {} {]", item1.summarize(), item2.summarize());    
//     }
    
//     pub fn notify3(item: &(impl Summary + Display)) { // multi traited arg
//         println!("Breaking news! {}", item.summarize());
//     }
    
//     pub fn notify3<T: Summary + Display>(item: &T) { // same notify3
//         println!("Breaking news! {}", item.summarize());
//     }
// }

// fn returnable_trait(){
//     pub trait Summary {
//         fn summarize(&self) -> String;
//     }

//     pub struct NewsArticle {
//         pub headline: String,
//         pub location: String,
//         pub author: String,
//         pub content: String,
//     }

//     impl Summary for NewsArticle {
//         fn summarize(&self) -> String {
//             format!("{}, by {} ({})", self.headline, self.author, self.location)
//         }
//     }

//     pub struct Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool,
//     }

//     impl Summary for Tweet {
//         fn summarize(&self) -> String {
//             format!("{}: {}", self.username, self.content)
//         }
//     }

//     fn returns_summarizable() -> impl Summary {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
