pub fn mean(x: &[i32]) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }

    let v = Vec::from(x);

    let sum: i32 = v.iter().sum();
    let len = v.len();

    (sum as f64) / (len as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 空の配列が与えられた場合に、NaNを返すことを確認する
    fn test_mean_empty() {
        let x = [];

        let result = mean(&x);

        assert!(result.is_nan());
    }

    #[test]
    // 正の整数が含まれる配列が与えられた場合に、平均値を正しく計算できることを確認する
    fn test_mean_positive_integers() {
        let x = [1, 2, 3, 4, 5];

        let result = mean(&x);

        assert_eq!(result, 3.0);
    }

    #[test]
    // 負の整数が含まれる配列が与えられた場合に、平均値を正しく計算できることを確認する
    fn test_mean_negative_integers() {
        let x = [-1, -2, -3, -4, -5];

        let result = mean(&x);

        assert_eq!(result, -3.0);
    }

    #[test]
    // 正の整数と負の整数が混在する配列が与えられた場合に、平均値を正しく計算できることを確認する
    fn test_mean_mixed_integers() {
        let x = [1, 2, -3, -4, 5];

        let result = mean(&x);

        assert_eq!(result, 0.2);
    }

    #[test]
    // 単一要素の配列が与えられた場合に、正しく平均値を計算できることを確認する
    fn test_mean_single_element() {
        let x = [1];

        let result = mean(&x);

        assert_eq!(result, 1.0);
    }

    #[test]
    // 非常に大きな配列が与えられた場合に、正しく平均値を計算できることを確認する
    fn test_mean_large_array() {
        let x = [1; 100000];

        let result = mean(&x);

        assert_eq!(result, 1.0);
    }
}
