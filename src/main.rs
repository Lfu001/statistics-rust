mod functions;
use functions::{mean, median, mode};

fn main() {
    // あるクラスのテストの点数が与えられたとする
    let scores = [80, 90, 75, 85, 90, 92, 88, 80, 78];
    println!("このクラスの点数は{:?}です。", scores);

    // 平均点を求める
    let mean = mean(&scores);
    println!("このクラスの平均点は{}点です。", mean);

    // 中央値を求める
    let median = median(&scores);
    println!("このクラスの中央値は{}点です。", median);

    // 最頻値を求める
    let mode = mode(&scores);
    match mode {
        Some(m) => println!("このクラスの最頻値は{}点です。", m),
        None => println!("このクラスに最頻値はありません。"),
    }
}
