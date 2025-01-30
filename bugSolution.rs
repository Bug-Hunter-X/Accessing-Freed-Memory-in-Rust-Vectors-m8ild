fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    println!("First element: {:?}", first);
    if let Some(second) = vec.get(1){
        println!("Second element: {:?}", second);
    } else {
        println!("Vector does not contain a second element");
    }
} 