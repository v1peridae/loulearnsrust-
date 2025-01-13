fn main() {
    //Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    //theres also 
    let v = vec![1,2,3];
    println!("{:?}", v);

    // ive already done strings

    //Hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Monkichi"), 10);
    scores.insert(String::from("Keroppi"), 50);
    

}
