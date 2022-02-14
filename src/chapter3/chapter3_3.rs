
// see https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html

pub fn chapter3_3() {
    one_function();
    what_is_function();
}

// Rustでは変数や関数の命名はスネークケース．関数の宣言順序は無関係．
fn one_function() {
    println!("Hello, world!");

    another_function(5);
}

// 引数は「仮引数: 型」の形．型は宣言しなければならないようになっている．Python の型ヒントと同じ．
fn another_function(x: i32) {
    println!("The value of x is {}.", x);
}

// 関数は「全部文」または「文が続いた最後に式」で構成．Rustは式が文の一部になる違いがある「式指向言語」．
// 文は，何らかの動作をして値を返さない．
// 式は．結果値として評価される．
fn what_is_function() {
    let _y = 6; // let文．文なので終端はセミコロン(;)
    // let x = (let y = 6); // 文は値を返さないので，これはコンパイルエラー．C言語系統のようなことはできない．
    // let x = y = 8; // なのでこれもダメ．
    let z = 5 + 6; // 「5 + 6」は式．
    another_function(z); // 関数呼び出しは式．
    println!("Calling macro is equation."); // マクロ呼び出しも式．
    let _y = {
        let x = 6; // ココは文だが，
        x + 5 // ココが式なので，ブロックが式を返し，全体として式．
    }; // 新しいスコープを作る {} も式．

}

// 関数の戻り値は '->' に続いて型を明記する．Python の型ヒントと同じ．
fn _five() -> i32 {
    5
}

// 関数が戻り値を返すにも関わらず戻り値の型が宣言されていないとコンパイルエラー．
/*fn one() {
    1
}*/

fn _ten() -> i32 {
    return 10 // return を使えば明示的．文末にセミコロンをつけてもOK．
}