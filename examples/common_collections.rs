use std::collections::HashMap;

fn main() {
    println!("\n--- STRING ---");
    let mut somestr = String::from("foo");
    somestr.push_str("bar");
    println!("String Example : {}", somestr);

    let mut s1 = String::from("Hello, ");
    s1 += "world!";
    println!("{}", s1);

    let hello = "ajsnqiuwgqjwc";
    let contoh = &hello[0..4];
    println!("{}", contoh);

    println!("\n--- VECTORS ---");
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    println!("{:?}", initial_scores);

    let somevec: Vec<i32> = vec![1,2,3,4,5,6];
    let three = somevec[3];
    println!("{}", somevec.get(3).unwrap());
    println!("{:?}", three);


    println!("\n--- HASHMAP ---");
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{}", scores["Blue"]);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let field_name = String::from("Favorite Number");
    let field_value: i32 = 137;
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{}", field_value);
}
