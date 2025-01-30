fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    println!("First element: {:?}", first);
    //This will panic because of accessing freed memory
    println!("Second element: {:?}", vec.get(1));
}