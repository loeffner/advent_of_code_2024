use std::fs::File;

pub fn total_distance(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> i32 {
    arr1.sort_unstable();
    arr2.sort_unstable();

    arr1.iter()
        .zip(arr2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

// fn similarity_score_naive(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> i32 {
//     let mut score = 0;
//     for i in arr1.iter() {
//         let mut cnt = 0;
//         for j in arr2.iter() {
//             if i == j {
//                 cnt += 1;
//             }
//         }
//         score += i * cnt;
//     }
//     return score;
// }

fn similarity_score_sort(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> i32 {
    let mut score = 0;

    arr1.sort_unstable();
    arr2.sort_unstable();

    let mut arr2_iter = arr2.iter();
    let mut elem_arr2 = arr2_iter.next();
    let mut arr_2_elem_cnt = 0;

    for (arr1_idx, elem_arr1) in arr1.iter().enumerate() {
        if arr1_idx > 0 && elem_arr1 == &arr1[arr1_idx - 1] {
            score += elem_arr1 * arr_2_elem_cnt;
            continue;
        }
        arr_2_elem_cnt = 0;
        while let Some(&val_arr2) = elem_arr2 {
            if val_arr2 > *elem_arr1 {
                score += elem_arr1 * arr_2_elem_cnt;
                break;
            }
            if val_arr2 == *elem_arr1 {
                arr_2_elem_cnt += 1;
            }
            elem_arr2 = arr2_iter.next();
        }
    }
    return score;
}

pub fn similarity_score(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> i32 {
    similarity_score_sort(arr1, arr2)
}

pub fn vectors_from_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse number"))
            .collect();

        if numbers.len() == 2 {
            vec1.push(numbers[0]);
            vec2.push(numbers[1]);
        }
    }
    Ok((vec1, vec2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let mut arr1 = vec![3, 4, 2, 1, 3, 3];
        let mut arr2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(total_distance(&mut arr1, &mut arr2), 11);
    }
    #[test]
    fn test_total_distance_different_length_arrays() {
        let mut arr1 = vec![3, 4, 2, 1, 3, 3];
        let mut arr2 = vec![4, 3, 5, 3];
        assert_eq!(total_distance(&mut arr1, &mut arr2), 6);
    }
    #[test]
    fn test_total_distance_zero_length_array() {
        let mut arr1 = vec![3, 4, 2, 1, 3, 3];
        let mut arr2 = vec![];
        assert_eq!(total_distance(&mut arr1, &mut arr2), 0);
    }
    #[test]
    fn test_total_distance_from_file() {
        let filename = "tests/input.txt";

        let result = vectors_from_file(filename);
        assert!(result.is_ok());
        let (mut vec1, mut vec2) = result.unwrap();

        let result = total_distance(&mut vec1, &mut vec2);
        assert_eq!(result, 11);
    }
    #[test]
    fn test_similarity_score() {
        let mut arr1 = vec![3, 4, 2, 1, 3, 3];
        let mut arr2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&mut arr1, &mut arr2), 31);
    }
}
