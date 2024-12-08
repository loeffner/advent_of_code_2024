fn check(a: i32, b: i32, monotony: i32) -> bool {
    let diff = b - a;
    if monotony * diff <= 0 {
        return false;
    } else if diff.abs() > 3 {
        return false;
    }
    return true;
}

pub fn check_safety(input: &Vec<i32>) -> bool {
    let monotony = input[1] - input[0];
    for elems in input.windows(2) {
        if !check(elems[0], elems[1], monotony) {
            return false;
        }
    }
    return true;
}

pub fn check_safety_dampened(input: &mut Vec<i32>) -> bool {
    let mut is_safe = false;
    for (i, _) in input.iter().enumerate() {
        let mut reduced_input = input.clone();
        reduced_input.remove(i);
        is_safe = check_safety(&reduced_input);
        if is_safe {
            return is_safe;
        }
    }
    return is_safe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_safe_all_decreasing() {
        let data = vec![7, 6, 4, 2, 1];
        assert_eq!(check_safety(&data), true);
    }
    #[test]
    fn test_is_safe_unsafe_negative_step_too_large() {
        let data = vec![1, 2, 7, 8, 9];
        assert_eq!(check_safety(&data), false);
    }
    #[test]
    fn test_is_safe_unsafe_positive_step_too_large() {
        let data = vec![9, 7, 6, 2, 1];
        assert_eq!(check_safety(&data), false);
        let mut data = data;
        assert_eq!(check_safety_dampened(&mut data), false);
    }
    #[test]
    fn test_is_safe_unsafe_not_monotonous() {
        let data = vec![1, 3, 2, 4, 5];
        assert_eq!(check_safety(&data), false);
    }
    #[test]
    fn test_is_safe_unsafe_not_strictly_monotonous() {
        let data = vec![8, 6, 4, 4, 1];
        assert_eq!(check_safety(&data), false);
    }
    #[test]
    fn test_is_safe_safe_all_increasing() {
        let data = vec![1, 3, 6, 7, 9];
        assert_eq!(check_safety(&data), true);
    }
    // is_safe_dampened
    #[test]
    fn test_is_safe_dampened_0() {
        let mut data = vec![42, 3, 6, 7, 32];
        assert_eq!(check_safety_dampened(&mut data), false);
    }
    #[test]
    fn test_is_safe_dampened_1() {
        let mut data = vec![3, 42, 6, 7, 9];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_2() {
        let mut data = vec![3, 6, 42, 7, 9];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_3() {
        let mut data = vec![0, 3, 6, 42, 9];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_4() {
        let mut data = vec![0, 3, 6, 7, 42];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_5() {
        let mut data = vec![0, 32, 32, 3, 4];
        assert_eq!(check_safety_dampened(&mut data), false);
    }
    #[test]
    fn test_is_safe_dampened_6() {
        let mut data = vec![0, -1, -2, 3, 4];
        assert_eq!(check_safety_dampened(&mut data), false);
    }
    #[test]
    fn test_is_safe_dampened_7() {
        let mut data = vec![0, -1, 3, -4, -5];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_8() {
        let mut data = vec![9, 10, 7, 6, 5];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
    #[test]
    fn test_is_safe_dampened_9() {
        let mut data = vec![2, 1, 2, 3, 4];
        assert_eq!(check_safety_dampened(&mut data), true);
    }
}
