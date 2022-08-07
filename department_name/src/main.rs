// add the person to partment list:
// like this: Add Sally to Engineering  Or  Add Amir to Sales

use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    loop {
        println!("1 Please add the person and partment");
        println!("2  enter 'exit' to break");
        println!("3 enter 'list' to view list");
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("Error reading");

        let line = line.trim();

        if line == "exit" {
            break;
        } else if line == "list" {
            println!("{:?}", map);
            continue;
        }

        let word_interval: Vec<usize> = interval_word(&line);
        // println!("{:?}", word_interval);
        let name = &line[word_interval[0] + 1..word_interval[1]];
        // println!("{:?}", name);
        let partment = &line[word_interval[2] + 1..];
        // println!("{:?}", partment);
        map.insert(name.to_string(), partment.to_string());
    }
}

fn interval_word(s: &str) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            list.push(i);
        }
    }
    return list;
}
