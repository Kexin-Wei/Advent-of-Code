use utils::read_data;
fn main() {
    let start = std::time::Instant::now();
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    read_data(&mut vec1, &mut vec2);

    vec1.sort();
    vec2.sort();

    let vec3 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| (b - a).abs())
        .collect::<Vec<i32>>();

    // get sum of vec3
    let sum: i32 = vec3.iter().sum();
    let end = start.elapsed();
    println!("result: {}, time: {}ms", sum, end.as_millis());
}
