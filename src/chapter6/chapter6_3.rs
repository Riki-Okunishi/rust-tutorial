
pub fn chapter6_3() {
    if_let_controll();
}

fn if_let_controll() {
    let some_u8_value = Some(0u8);

    // match によるじれったい書き方
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // ここには None も含まれる
    }

    // if let 記法による簡潔な書き方(マッチしたとき以外を無視する match 式の糖衣構文と等価)
    if let Some(3) = some_u8_value { // if let 記法．Golang や Python の代入式や，C や Java の代入式も値を返す仕様による条件式とは少し違う．
        println!("three");
    } else {
        // else でそれ以外の場合に飛べる
    }
}