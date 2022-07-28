use std::io;//入出力ライブラリ

fn main() {
    println!("Guess the number");
    println!("please input number");
    let mut guess = String::new();//letは変数宣言mutは変数が可変であることを示す
    io::stdin()
        .read_line(&mut guess)//mutがないと参照先で不変になる
        .expect("Failed to read line");//失敗時処理

    println!("you guessd: {}", guess);
}
