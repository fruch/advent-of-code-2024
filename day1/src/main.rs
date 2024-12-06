use std::fs;
use std::collections::HashMap;
  
fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().expect("not a number");
        list_a.push(a);
        let b: i32 = parts.next().unwrap().parse().expect("not a number");
        list_b.push(b);
    }
    list_a.sort();
    list_b.sort();

    let mut f: i32 = 0;

    for (a, b) in list_a.iter().zip(list_b.iter()).collect::<Vec<_>>() {
        f += (a-b).abs();
        
    }
    println!("{}", f);


    let mut mapping = HashMap::new();
  
    for b in &list_b {
        mapping.insert(b.clone(),  list_b.iter().filter(|i| *i == b).count() as i32);
    }

    let mut t: i32 = 0;
    for a in &list_a {
        t += a * mapping.get(&a).unwrap_or(&0);
    }
    print!("{}", t);

}
