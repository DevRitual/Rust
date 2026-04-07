fn main() {
    println!("Hello world");
    let mut collector: Vec<i32> = (1..10).collect();
    println!("Collection of vector form 1 to 10 is : {:?}", collector);
    let mut xs = vec![1i32, 2, 3, 4];
    xs.push(5);
    println!("we pushed 5 in xs : {:?}", xs);
    collector.push(0);
    println!("{:?}", collector)
}
