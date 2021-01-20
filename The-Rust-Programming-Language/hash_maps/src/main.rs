// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// Hash maps store their data on the heap.
// Hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

use std::collections::HashMap;

fn main() {
    new_hash_maps();
    new_hash_maps_from_vectors();
    ownership();
    ownership_references();
    access_values();
    update_hash_maps();
    update_hash_maps_or_insert();
    pig_latin("First");
    pig_latin("apple");
    pig_latin("æpple");
    pig_latin("A");
    pig_latin("ear");
    pig_latin("ęar");
    pig_latin("žsß");

}

fn new_hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
}

fn new_hash_maps_from_vectors() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // can use _ to let Rust infers the types
    //let mut scores: HashMap<String, i32> = 
    let mut scores: HashMap<_, _> = 
        teams
            .into_iter()
            .zip(initial_scores.into_iter()) // return tuple
            .collect();
    scores.insert("Red".to_string(), 112);
    println!("{:?}", scores);
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let mut field_value = 10;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name is invalid at this point
    // field_value is cloned (i.e., i32 has Copy traits)
    //println!("{:?}", field_name);
    println!("{:?}", field_value);
    field_value = 20;
    println!("{:?}", field_value);
    println!("{:?}", map);
}

fn ownership_references() {
    let field_name = String::from("Favorite color");
    let field_value = 10;

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    // field_name is borrowed, it's fine here.
    println!("{:?}", field_name);
    println!("{:?}", field_value);
    println!("{:?}", map);
    //let _moved = field_name; // can't moved as it's borrowed by hash maps
    let borrowed = &field_name;
    println!("{} {:?}", borrowed, map);
}

fn access_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let blue = scores.get("Blue");
    println!("Blue: {:?}", blue);
    let green = scores.get("Green");
    println!("Green: {:?}", green);
}

fn update_hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    // overwrite
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // entry
    let yellow_exists = scores.entry("Yellow".to_string()); // mutable borrowed happen here
    println!("yellow_exists: {:?}", yellow_exists); // Entry(VacantEntry("Yellow"))
    let blue_exists = scores.entry("Blue".to_string());
    println!("blue_exists: {:?}", blue_exists); // Entry(OccupiedEntry { key: "Blue", value: 25 })
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // blue is not updated
    println!("{:?}", scores);
    
    let blue: &mut i32 = 
        scores
        .entry(String::from("Blue"))
        .or_insert(50); // return mutable reference
    *blue += 1; // dereference blue
    println!("{:?}", scores);

    scores.entry(String::from("Blue")).and_modify(|v| *v += 1); // blue is updated
    println!("{:?}", scores);
}

fn update_hash_maps_or_insert() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map.insert("test", 0);
    println!("{:?}", map);
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn pig_latin(str: &str) {
    use unicode_segmentation::UnicodeSegmentation;
    let vowels = vec!["a", "e", "i", "o", "u"];
    let mut map: HashMap<&str, bool> = HashMap::new();
    for v in vowels {
        map.insert(v, true);
    }
    //println!("map {:?}", map);
    let first = str.graphemes(true).nth(0);
    match first {
        Some (first) => {
            let first_lowercase = &first.to_ascii_lowercase()[..];
            let result = match map.get(first_lowercase) {
                Some(_) => {
                    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”)
                    format!("{}-hay", &str)
                },
                None => {
                    // The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
                    let mut word: String = "".to_string();
                    for (i, s) in str.graphemes(true).enumerate() {
                        if i != 0 {
                            word.push_str(s);
                        }
                      } 
                    format!("{}-{}ay", word, first)
                }
            };
            println!("first: {:?} {} -> {}", first, str, result);
        },
        None => {}
    }
}