

struct User { // 構造体を定義．構造は C で型注釈は Python
    username: String, // String ではなく &str にすると，「ライフタイム指定子が必要」とコンパイルエラーになる．
    email: String,// インスタンスが存在する間に，保持している参照先が破棄されるかもしれないため．ライフタイムはおそらくスコープを無視して変数の存続期間を定めるためのもの．一旦保留
    sign_in_count: u64,
    active: bool,
}

struct _UnitFormedStruct {} // ユニット様構造体．フィールドは持たないが，トレイトを実装する場合に使う．

// タプル構造体
struct Color (i32, i32, i32); // Golang の 'type' のよる別名定義に近い
struct Point(i32, i32, i32); // 間のスペースはなくてよい

pub fn chapter5_1() {
    init_struct();
    tuple_struct();
}

fn init_struct() {
    let _user1 = User{ // 構造体の初期化．Golang っぽい．全フィールド明示的に初期化が必要なのは C++ っぽい．
        username: String::from("John"),
        email: String::from("john@example.com"),
        active: true,
        sign_in_count: 1, // 初期化を記述する順序は自由
    }; // 全体で文なので，セミコロンで閉じる(ここは Golang とは違う)

    let mut user2 = User { // 可変変数にすると，インスタンス全体が可変になる(フィールドの一部だけ可変，とはできない)．
        username: String::from("Bob"),
        email: String::from("bob@email.com"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("bob2@email.com"); // 可変変数であれば，フィールドの変更も可能

    let user3 = build_user(String::from("David"), String::from("david@gmail.com")); // 関数を使ってインスタンスを生成．関数内部で生成しても，ムーブしているので問題なし

    let _user4 = User { // 別のインスタンスによるインスタンスの生成の愚直な方法
        username: user3.username,
        email: user3.email,
        active: user3.active,
        sign_in_count: user3.sign_in_count,
    };

    let _user5 = User { // 構造体更新記法での初期化
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
        ..user3 // 「残りのフィールドは変数 user3 と同じ」の意
    };

}

fn build_user(email: String, username: String) -> User {
    User { // インスタンス生成は式なので，関数の最後でインスタンス生成すると，そのインスタンスが返る
        email, // 全く同じ変数名であれば，対応するフィールドの初期値と認識される
        username, // 同上
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    print_color(black);
    // print_color(origin); // Color も Point もタプルとしては全く同じ構成だが，Color を取る関数の引数に Point は使えない
}

fn print_color(c: Color) {
    println!("Point c is ({}, {}, {})", c.0, c.1, c.2);
}