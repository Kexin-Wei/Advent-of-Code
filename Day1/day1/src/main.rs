use std::env;
use std::fs;

fn main() {
    // read doc
    let data_file_name = "data.txt";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let data_path = current_dir.parent().unwrap().join(data_file_name);
    println!("the data file is {}", data_path.display());
    let data_file = fs::read_to_string(data_path).expect("Should have been able to read the file");

    // timing the program
    let start = std::time::Instant::now();
    // prepare two vec
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    // read data
    for line in data_file.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let mut nums: Vec<i32> = Vec::new();
        for i in parts {
            // check whether can convert to number
            let converted: bool = i.parse::<i32>().is_ok();
            if !converted {
                continue;
            }
            nums.push(i.parse::<i32>().unwrap());
        }
        if nums.len() < 2 {
            continue;
        }
        vec1.push(nums[0]);
        vec2.push(nums[1]);
    }
    vec1.sort();
    vec2.sort();

    let vec3 = vec2
        .iter()
        .zip(vec1.iter())
        .map(|(&a, &b)| a - b)
        .collect::<Vec<i32>>();

    // get sum of vec3
    let sum: i32 = vec3.iter().sum();
    let end = start.elapsed();
    println!("result: {}, time: {}ms", sum, end.as_millis());
}
