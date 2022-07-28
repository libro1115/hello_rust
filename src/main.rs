use std::io;//入出力ライブラリ
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);//1～100間での乱数を生成
    loop{
        println!("please input number");
        let mut guess = String::new();//letは変数宣言mutは変数が可変であることを示す
        io::stdin()
            .read_line(&mut guess)//mutがないと参照先で不変になる
            .expect("Failed to read line");//失敗時処理
        let guess:u32 = guess//シャドーイング（名前の再利用。元の変数は消えてない
                        .trim()//文字列から\n等を消す
                        .parse()//数値に変換
                        .expect("Please type a number");//エラー処理

        println!("you guessd: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
            {
                println!("Jackpot!");
                break;
            },
        }
    }
}
