use std::env;
use std::fs::read_to_string;
use std::path::Path;

// main function
fn main() {
    //macro for printing into console
    println!("Hello World !");
    
    //get cli parameters
    //args[0] being the binary
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing input filepath");
        return;
    }
    //filepath only, not opening
    let path = Path::new(&args[1]);
    //gets the given filepath
    let display = path.display();

    let mut _left_list: Vec<i32> = [].to_vec();
    let mut _right_list: Vec<i32> = [].to_vec();

    //read each line
    for line in read_to_string(&display.to_string()).unwrap().lines() {
        //removes spaces at beginning and at end
        let line = line.trim();
        let mut splitted = line.split_whitespace();

        //if line is empty stop it
        if line.len() == 0 {
            break;
        }

        //next uses the iterator to get current index. I would have loved to have a .first() method
        //in this case
        //last gets the last element
        let id_left = splitted.next();
        let id_right = splitted.last();
        //.unwrap() method **consumes** the element, which means they are removed from the vector
        let parsed_left: i32 = id_left.unwrap().parse().unwrap();
        let parsed_right: i32 = id_right.unwrap().parse().unwrap();
        _left_list.push(parsed_left);
        _right_list.push(parsed_right);
    }
    _left_list.sort();
    _right_list.sort();
    let mut result = 0;
    //Even if it's an iterator, we must tell rust it's mutable as we will iterate over
    let mut right_iter = _right_list.iter();

    for nb in _left_list {
        //gets iterator and gets its next element
        let nb2 = right_iter.next().unwrap();
        let mut calc: i32 = nb - nb2;
        if calc < 0 {
            calc *= -1;
        }
        result += calc;
    }
    println!("{:?}", result)
}
