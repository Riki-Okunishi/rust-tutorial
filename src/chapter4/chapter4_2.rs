
pub fn chapter4_2() {
    borrow_as_immutable_reference();
    borrow_as_mutable_reference();
    make_references();
    dangling_pointer();
}

fn borrow_as_immutable_reference() {
    let s1 = String::from("hello"); // 不変変数

    let len = calculate_length(&s1); // 不変参照を渡す．呼び出し側の書き方は C++ のポインタ渡しや Golang の参照渡しっぽい
    println!("The length of s1 '{}' is {}.", s1, len); // s1 はムーブされないので，calculate_length()を呼んだ後でも使える

    // change_given_muttable_reference(&mut s1); // 不変変数の 可変参照は作れない．"cannot borrow `s1` as mutable, as it is not declared as mutable"とエラー
}

fn borrow_as_mutable_reference() {
    let mut s2 = String::from("hello");

    let len = calculate_length(&s2); // 可変変数の 不変参照を渡せる．変更されないことを保証できる？．C++ のconst 引数に近い？
    calculate_length(&mut s2); // 不変参照を取る関数に，可変変数の 可変参照を渡す分には怒られない．変更可能だけどこの関数は変更しないことが確定しているから？(不変参照に変換しているかは不明．Rustは暗黙型変換嫌いそうだからしてなさそう)
    println!("The length of s2 '{}' is {}.", s2, len);

    change_given_muttable_reference(&mut s2); // 可変変数の 可変参照を借用すると，所有権を維持したまま値の変更が可能．
    println!("changed s2 = '{}'", s2); // 所有権を保っているので，s2 が使える．かつ，可変参照を使って 可変変数 s2 の値を書き換えたので，s2 の中身が変更されている
}

// 関数の引数も，参照を受け取ると明示する．関数の引数に参照を取って関数内で使用することを「借用する」という
fn calculate_length(s: &String) -> usize { // 引数の宣言は C++ の参照渡しっぽい．C++ でいう `const string &s`
    s.len()
}// ここで，sはスコープ外になる．けれど，参照しているものに対する所有権を持っているわけではないので，何も起こらない

// 引数宣言で，mutable な借用を明示する
fn change_given_muttable_reference(some_string: &mut String) { // C++ でいう `string &s`
    some_string.push_str(", world"); // 可変参照は 可変変数でしか作れないので，可変変数のみにできる操作が可能
} // スコープは抜けるが，借用なので値が変更された以外は何も起きない

fn make_references() {
    // 以下のデータの競合が起こる状況ではコンパイルが通らない．
    // 1. 可変参照が同一スコープに複数ある
    // 2. 不変参照と 可変参照の両方が同一スコープに存在する
    let mut s = String::from("hello");
    {
        let r1 = &mut s; // 可変参照型を変数に格納
        // let r2 = &mut s; // 同一変数に対する 2つ目の可変参照は作れない
        // println!("{}, {}", r1, r2); // 両方の 可変参照を使用するとコンパイルエラー
    }
    let r2 = &mut s; // 別のスコープであれば，同一変数の 可変参照を2つ作れる(同一スコープには1つしかない)
    
    {
        // let r5 = &mut s; // このローカルスコープでも r2 が有効なので，2つ以上 可変参照が存在するため作成不可
        // println!("{}, {}", r2, r5); // ここはコンパイルエラー
        let r3 = &s;
        let r4 = &s; // 同一変数の同一スコープ内での参照でも，不変参照なら複数作成可能．
        // let r5 = &mut s; // 既に 不変参照が存在するときに 可変参照は作れない
        // println!("{}, {}, {}", r3, r4, r5); // ここはコンパイルエラー
    }
}

fn dangling_pointer() {
    // let reference_to_nothing = dangle(); // スコープを抜け破棄された実体への参照は許さない
    let referece_to_no_dangled = no_dangle();
}

// fn dangle() -> &String { // dangle は String への参照を返す

//     let s = String::from("hello"); // sは新しいString

//     &s // String sへの参照を返す
// } // ここで，sはスコープを抜けドロップされる．そのメモリは消される．つまり，参照はあるけど，その先の実体が存在しない状態．これをコンパイルエラーにする

fn no_dangle() -> String { // no_dangle は String の実体を'ムーブして'返す
    let s = String::from("hello");
    s
} // s の所有権がムーブされるので，s はスコープを抜けても破棄されない