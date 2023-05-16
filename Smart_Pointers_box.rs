fn main() {
    let var_i32 = 5;
    //stack
    let b = Box::new(var_i32);
    //heap
    println!("b = {}", b);
}
