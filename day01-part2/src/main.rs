use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Expects an input file in parameter of this program");
        return;
    }

    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut left_list: Vec<u32> = vec![];

    //args[0] being the binary.
    let filepath: &String = &args[1];
    for line in read_to_string(filepath).unwrap().lines() {
        let line = line.trim();
        let mut splitted = line.split_whitespace();

        if line.len() == 0 {
            break;
        }

        left_list.push(splitted.next().unwrap().parse::<u32>().unwrap());
        let right: u32 = splitted.last().unwrap().parse().unwrap();
        if map.contains_key(&right) {
            map.insert(right, map.get(&right).clone().unwrap() + 1);
        } else {
            map.insert(right, 1);
        }
    }

    let mut res: u32 = 0;
    for v in left_list {
        if !map.contains_key(&v) {
            continue;
        }
        res += v * map.get(&v).unwrap();
    }
    println!("{:?}", res);
}
