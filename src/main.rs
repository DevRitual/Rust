fn main() {
   let x= 10;//here x is the owner of 10 
   let a = 5;//primitive type
let _b = a;//primitive type

println!("{}", a); // it will print a but dosnt work in heap
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s2); // compile-time error if you take s1

}//here x goes out of scope


