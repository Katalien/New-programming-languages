//* 1. EXPRESSIONS */ 
// Example 1
// fn main(){
//     //statement 1
//     //statement 2
//     //statement 3
// }

// Example 2
// fn main(){
//     let x = 5; // statement
//     // expressions
//     x;
//     x+1; 
//     15;
// }

// Example 3
//error: the operator returns nothing
// fn main() {
//     let x = (let y = 6);
// }

//Example 4
// fn main() {
//     let y  = {
//         let mut x = 3;
//         x += 1;
//         x // expression
//     }; 
//     println!("The value of y is: {y}");
// }

//* 2. INPUT */ 
// Example 0
// fn main() {
//     println!("Hello, world!");
//     println!("{}", example_fn(2,3));
// }
// fn example_fn(p1: i32, p2: i32) -> i32 {
//     p1 + p2
// }

//Example 1
// fn main(){
//     let mut num = 5;
//     take_by_value(num);
//     println!("Main: num = {}", num);
//     modify_int(num);
//     println!("Main: num = {}", num);
//     modify_int_by_reference(&mut num);
//     println!("Main: num = {}", num);
// }
// fn take_by_value(x: i32) {
//     println!("take_by_value: x = {}", x);
// }
// fn modify_int(mut x: i32) {
//     x += 1;
//     println!("modify_by_value: x = {}", x);
// }
// fn modify_int_by_reference(x: &mut i32){
//     *x += 1;
//     println!("modify_by_reference: x = {}", x);
// }

//Example 2
// fn main(){
//     let mut my_string = String::from("Hello");
//     modify_string(&mut my_string);
//     println!("{}", my_string);
// }
// // access error 
// fn modify_string(s: mut String) {
//     s.push_str(" World"); 
// }

//* 3. () TYPE */ 
// Example 1
// fn main(){
//     let empty_tuple: () = ();
//     println!("Empty tuple: {:?}", empty_tuple);
//     log_message("Hello, world!");
//     log_message2("Hello, world!");
// }
// fn log_message(messaage: &str) -> (){
//     println!("{}", messaage);
// }
// fn log_message2(messaage: &str){
//     println!("{}", messaage);
// }

// Example 2
// fn main(){
//     let x = 42;
//     match x {
//         0 => println!("Zero"),
//         1 => println!("One"),
//         // The _ is a placeholder for any other value
//         _ => println!("Other"), 
//     }
// }

//* 4. OUTPUT */ 
// Example 
// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {x}");
//     let y = plus_one2(5);
//     println!("The value of x is: {y}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// // error: must return i32
// // but statement doesn't return anything
// fn plus_one2(x: i32) -> i32 {
//     x + 1;
// }

//* 5. TASKS
//Test task 1
// fn main(){
//     let mut a:i32 = 10;
//     f2(&mut a);

//     fn f1(a: &mut i32){
//         println!("{}", *a);
//     }
    
//     fn f2(a: &mut i32){
//         *a = 20;
//         f1(a);
//     }
// }

//Test task 2
// fn main(){
//     let mut a:i32 = 10;
//     f2(&mut a);

//     fn f1(a: &mut i32){
//         println!("{}", *a);
//     }
    
//     fn f2(a: &mut i32){
//         *a = 20;
//         return f1(a);
//     }
//     fn f(a: &mut i32){
//         f2(a);
//     }
//     f(&mut a);
// }

//* 6. RECURSION */ 
// fn factorial(n: u64) -> u64 {
//     if n == 0 {
//         1
//     } else {
//         n * factorial(n - 1)
//     }
// }

// fn main() {
//     let n = 5;
//     let result = factorial(n);
//     println!("Factorial of {} is: {}", n, result);
// }

//* 7. CLOSURES */ 
// fn main() {
//     let num = 5;
//     let add_one = |x| x + 1;
//     let result = add_one(num);
//     println!("Result: {}", result);
// }

fn main() {
    fn  function            (i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    println!("Function: {}", function(i));
    println!("Closure with type indication: {}", closure_annotated(i));
    println!("Closure with type output: {}", closure_inferred(i));

    // no arguments, but returns `i32`
    // type is identified automatically 
    let one = || 1;
    println!("Closure returning one: {}", one());

}