use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();
    //
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    //
    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);

    // let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }
    // for i in &mut v {
    //     *i += 50;
    // }

    // let mut s = String::new();

    // let data = "initial contents";
    // let s = data.to_string();
    //
    // let s = "initial contents".to_string();

    // let mut s = String::from("foo");
    // s.push_str("bar");
    //
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    //
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    //
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    //
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    //
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
