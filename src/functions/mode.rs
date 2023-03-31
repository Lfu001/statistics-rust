use std::collections::HashMap;

pub fn mode(x: &[i32]) -> Option<i32> {
    let mut counter = HashMap::new();
    for i in x {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    counter
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(key, _)| *key)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 空の配列を渡すと、デフォルト値である0が返ることを確認する
    #[test]
    fn test_mode_empty() {
        let x = [];

        let result = mode(&x);

        assert_eq!(result, None);
    }

    // 配列内に1つの値がある場合、その値が最頻値であることを確認する
    #[test]
    fn test_mode_single_value() {
        let x = [3];

        let result = mode(&x);

        assert_eq!(result, Some(3));
    }

    // 配列内に重複する複数の値がある場合、最頻値が正しく計算されることを確認する
    #[test]
    fn test_mode_multiple_values() {
        let x = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4];

        let result = mode(&x);

        assert_eq!(result, Some(4));
    }

    // 配列内に最頻値が複数ある場合、いずれかの最頻値が返されることを確認する
    #[test]
    fn test_mode_multiple_modes() {
        let x = [1, 1, 2, 2, 3, 3];

        let result = mode(&x);

        assert!(result == Some(1) || result == Some(2) || result == Some(3));
    }
}
