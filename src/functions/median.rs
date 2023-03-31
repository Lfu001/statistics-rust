pub fn median(x: &[i32]) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }

    let mut v = Vec::from(x);
    v.sort();

    let len = v.len();
    match len % 2 {
        0 => ((v[len / 2 - 1] + v[len / 2]) as f64) / 2.0,
        _ => v[len / 2] as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 空のスライスを渡した場合にNaNを返すことを確認する
    fn test_median_empty() {
        let x = [];

        let result = median(&x);

        assert!(result.is_nan());
    }

    #[test]
    // 要素が1つの配列を渡した場合に、その要素を中央値として返すことを確認する
    fn test_median_single() {
        let x = [1];

        let result = median(&x);

        assert_eq!(result, 1.0);
    }

    #[test]
    // 要素数が奇数の配列を渡した場合に、中央値を正しく計算することを確認する
    fn test_median_odd() {
        let x = [3, 1, 4, 1, 5];

        let result = median(&x);

        assert_eq!(result, 3.0);
    }

    #[test]
    // 要素数が偶数の配列を渡した場合に、中央値を正しく計算することを確認する
    fn test_median_even() {
        let x = [3, 1, 4, 1];

        let result = median(&x);

        assert_eq!(result, 2.0);
    }
}
