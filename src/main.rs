use std::io;

fn main() {
    let mut input_type = String::new();
    println!("実行する操作の数値を入力してください（1:結果登録、2:集計）");
    io::stdin().read_line(&mut input_type).unwrap();
    let input_type: u8 = input_type.trim().parse().expect("数値を入力してください");

    if input_type == 1 {
        println!("結果登録");
    } else if input_type == 2 {
        println!("集計");
    }
}
