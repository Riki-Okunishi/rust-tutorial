
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    _Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin { // match では，条件式(=bool)ではなく，任意の型をとれる
        Coin::Penny => { // '{}' は式なので，複数行の処理も各アームで走らせられる
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // enum Coin の Quarter は，UsState 型に紐付けられている．match の結果 Quarter だった場合，紐付けられた UsState の値が得られる
            println!("State quarter from {:?}!", state);
            25
        },
        // もしすべての可能性を検査していなければコンパイルエラー
    }
}

pub fn chapter6_2() {
    use_match_for_coin();
    use_match_for_int();
}

fn use_match_for_coin() {
    let c1 = Coin::Penny;
    let q1 = Coin::Quarter(UsState::Alabama);
    let coins = vec![c1, Coin::Dime, Coin::Nickel, q1];

    println!("check coins");
    for c in coins {
        let value = value_in_cents(c);
        println!("value is {}", value);
    }
}

fn use_match_for_int() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 「それ以外」は '_' で示す．実行結果として何もしたくなければ，'()' というユニット値を返せば何もしない
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // None のときは match の結果も None
        Some(i) => Some(i + 1), // Some(T) のときは T(=i32) の値に +1 した Some を match の結果として返す
    } // match は式なので，match の結果が返される
}