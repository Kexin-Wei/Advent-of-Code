use std::env;
use std::fs;

pub fn read_data(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) {
    // read doc
    let data_file_name = "data.txt";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let data_path = current_dir.parent().unwrap().join(data_file_name);
    println!("the data file is {}", data_path.display());
    let data_file = fs::read_to_string(data_path).expect("Should have been able to read the file");

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1: Vec<i32> = Vec::new();
        let mut vec2: Vec<i32> = Vec::new();
        read_data(&mut vec1, &mut vec2);
        assert_eq!(vec1.len(), 1000);
        assert_eq!(vec2.len(), 1000);
    }
}
