use day01::{similarity_score, total_distance, vectors_from_file};

fn main() {
    let filename = "data/input.txt";
    let (mut vec1, mut vec2) = vectors_from_file(filename).unwrap();
    println!("File contains two lists of {:?} values each.", vec1.len());

    let result = total_distance(&mut vec1, &mut vec2);
    println!("The total distance between the lists is {:?}.", result);

    let result = similarity_score(&mut vec1, &mut vec2);
    println!("The similarity score is {:?}.", result);
}
