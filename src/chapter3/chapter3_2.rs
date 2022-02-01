
// see https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html

pub fn chapter3_2() {
    /* 3.2 データ型 */
    // Rust は静的型付け言語．すべての型はスカラー型と複合型に分類される．

    /* スカラー型 */

    // 整数型 (i8 ~ i64, u8 ~ u64, isize, usize(32bitか64bit))
    let x_i8: i8 = 127; // -128 ~ 127 に収まらないとコンパイルエラーを吐いてくれる．
    let x_i64: i64 = -100_000_000; // 位取りに '_' を使える．Pythonっぽい．
    let x_u64: u64 = 0xFFff; // 16進数のアルファベットは大文字小文字どちらも可．
    let x_oct: u64 = 0o160; // 8進数
    let x_bin: u64 = 0b1000_1010;// 2進数
    let x_byte: u8 = b'A'; // バイト(u8だけ)
    println!("x_i8 = {}, x_i64 = {}, x_u64 = {}, x_oct ={}, x_bin = {}, x_byte ={}", x_i8, x_i64, x_u64, x_oct, x_bin, x_byte);


    // 浮動小数点型 IEEE-754規格 単精度：f32, 倍精度：f64
    let x = 2.0; // デフォルトは f64．
    let y: f32 = 4.5; // 型注釈をつければ f32．
    println!("x = {}, y ={}", x, y);


    // 数値演算
    // 足し算
    let mut sum = 5 + 10;
    sum += 4;
    println!("sum = {}", sum);

    // 引き算
    let mut difference = 95.5 - 4.3;
    difference -= 6.0;
    println!("difference = {}", difference);

    // 掛け算
    let mut product = 4 * 30;
    product *= 5;
    println!("product = {}", product);

    // 割り算
    let mut quotient = 56.7 / 32.2;
    quotient /= 4.0;
    println!("quotient = {}", quotient);

    // 余り
    let mut remainder = 47 % 10;
    remainder %= 3;
    println!("reminder = {}", remainder);


    // 論理値型
    let t = true;
    let f: bool = !t;
    println!("t = {}, f = {}", t, f);


    // 文字型 Unicode のスカラー値
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //ハート目の猫
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);



    /* 複合型 */
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1); // それぞれが自由な型を持てる．
    let (x, y, z) = tup; // パターンマッチングで「分配」できる．
    let (a, b, _) = tup; // '_' で捨てられる(いつ破棄される？)．
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("a = {}, b = {}", a, b);

    let first = tup.0; // 添字番号で直接アクセスできる．
    println!("tup.0 = {}", first);
    // let forth = tup.3; // 存在しない添字はコンパイルエラー．


    // 配列型
    let a = [1, 2, 3, 4, 5]; // すべて同じ型．長さは固定長．コンパイル時に長さが確定している必要がある．C言語っぽい．

    let first = a[0]; // 添字で要素アクセス．
    println!("aa[0] = {}", first);
    // let sixth = a[5]; // 存在しない添字にアクセスすると，"index out of bounds" のランタイムエラー．Rustでは「パニック」という．

}