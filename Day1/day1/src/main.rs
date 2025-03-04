use std::env;
use std::fs;

fn main() {
    // read doc
    let data_file_name = "data.txt";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let data_path = current_dir.parent().unwrap().join(data_file_name);
    println!("the data file is {}", data_path.display());
    let data_file = fs::read_to_string(data_path).expect("Should have been able to read the file");
    // print 10 lines
    for line in data_file.lines().take(10) {
        println!("{}", line);
    }

    // prepare two vec
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    let mut diff: i32 = 0;

    // read data
    for line in data_file.lines().take(10) {
        let parts: Vec<&str> = line.split(' ').collect();
        let num1 = parts[0].parse::<i32>().unwrap();
        let num2 = parts[1].parse::<i32>().unwrap();
        println!("{} {}", num1, num2);
        vec1.push(num1);
        vec2.push(num2);
    }
    vec1.sort();
    vec2.sort();

    let vec3 = vec2
        .iter()
        .zip(vec1.iter())
        .map(|(&a, &b)| a - b)
        .collect::<Vec<i32>>();

    for i in vec3 {
        diff += i;
    }
    println!("{}", diff);
}
