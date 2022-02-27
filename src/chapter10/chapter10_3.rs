
use std::fmt::Display;

pub fn chapter10_3() {
    invalid_lifetime();
    valid_lifetime();
    generic_lifetime_in_function();
    generic_lifetime_in_different_scope();
    generic_lifetime_in_invalid_different_scope();
    lifetime_of_struct();
}


fn invalid_lifetime() {
    {                       // 「借用チェッカー」が認識する借用の有効期限
        let _r;             //--------+-- 'a
                            //        |
        {                   //-+-- 'b |
            let x = 5;      // |      |
            _r = &x;        // |      |
        }                   //-+      |
                            //        |
        // println!("r: {}", r);      |  <=- 参照 r のライフタイム 'a が，その参照の対象 x のライムタイム 'b よりも長いのでコンパイルエラー
    }                       // -------+
}

fn valid_lifetime() {
    {
        let x = 5;              // ----------+-- 'b
                                //           |
        let r = &x;             // --+-- 'a  |
                                //   |       |
        println!("r: {}", r);   //   |       |
                                // --+       |
    }                           // ----------+
} // x のライフタイム 'b が r のライフタイム 'a より長いのでエラーにならない

// コンパイルエラーになる実装．x と y のどちらが戻り値として返され，今後も使われるか決定できない
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// &<ライフタイム引数の名前> 型注釈 --<ライフタイム引数の名前>... 「'」で始まって名前はすべて小文字．通常「'a」を使う
// &i32        // ただの参照
// &'a i32     // 明示的なライフタイム付きの参照
// &'a mut i32 // 明示的なライフタイム付きの可変参照

// ライフタイム注釈を付与しコンパイル可能に．引数のライフタイムが決定されることで，借用チェッカーがコンパイル時にチェックできるようになるだけなのを注意
// ジェネリック型引数と同じようにライフタイム引数を<>内に宣言
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// コンパイルエラーになる実装．戻り値の参照は，ライフタイムが一致するいずれかの引数を参照していなければならない．戻り値の参照は引数の参照でなければ，関数の内外で存続できない．
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     // 本当に長い文字列
//     let result = String::from("really long string");
//     result.as_str() // 引数とは異なる値の参照を返すので，ライフタイムが一致しない．つまり，result はダングリング参照になる
// }

fn generic_lifetime_in_function() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); // 最長の文字列は、{}です
}

fn generic_lifetime_in_different_scope() {
    let string1 = String::from("long string is long"); // 'a

    {
        let string2 = String::from("xyz"); // 'b
        let result = longest(string1.as_str(), string2.as_str()); // longest には異なるライフタイム 'a と 'b が渡るが，短い方に合わせられて，'b としてチェックされる
        println!("The longest string is {}", result);
    } // result は 'b と判定され 'a よりも短いため，コンパイルエラーにならない
}

fn generic_lifetime_in_invalid_different_scope() {
    let string1 = String::from("long string is long"); // 'a
    let _result; // 'b
    {
        let string2 = String::from("xyz"); // 'c
        _result = longest(string1.as_str(), string2.as_str()); // result のライフタイムは，引数のうち短い方の 'c として評価される
    }
    // println!("The longest string is {}", result); // result は 'c と判定されるが，そのライフタイムを超えてここで使用されているためコンパイルエラー
}

// 構造体自体のライフタイムより，その構造体が保持する参照のフィールドのライフタイムの方が長くなければならない(構造体の存続期間中は，フィールドにアクセスされうる)
// <'a> というライフタイム引数により，この構造体自体よりも，参照 part の方が長生きするという前提が成り立つようになる
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_of_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i = ImportantExcerpt { part: first_sentence };
    
    println!("first_sentence is {}", i.part);
}

// ライフタイム省略規則
//      Rust コンパイラは以下の規則に従って，ライフタイム引数の無い関数やメソッドのライフタイム引数を補完していき，
//      違反もしくはライフタイムが決定しなければコンパイルエラーとする
// 1. 入力ライフタイム(関数やメソッドの引数のライフタイムに対する規則)
//      「引数の数だけ個別にライフタイム引数を割り当てる」
//      e.g.: 
//          a. fn first_word(s: &str) -> &str {  -->  fn first_word<'a>(s: &'a str) -> &str {
//          b. fn longest(x: &str, y: &str) -> &str { -->  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//
// 2.1 出力ライフタイム1(戻り値のライフタイム)
//      「入力ライフタイム引数が1つであれば，そのライフタイムがすべての出力ライフタイムに代入される」
//      e.g.:
//          a. fn first_word<'a>(s: &'a str) -> &str {  -->  fn first_word<'a>(s: &'a str) -> &'a str { --> 確定したので終了
//          b. ライフタイム引数が1つでないのでスキップ
//
// 2.2 出力ライフタイム2
//      「メソッドの第1引数が &self か &mut self であれば，self のライフタイムが出力ライフタイムになる」
//      e.g.:
//          b. メソッドではないため self 引数がないためスキップ  -->  引数と戻り値のライフタイムがすべて決定していないのでコンパイルエラー

// impl キーワードの後にライフタイム引数を付け，さらに型名の後にもライフタイム引数を付ける
impl<'a> ImportantExcerpt<'a> {
    // 3番目の省略規則が適用されない例
    fn _level(&self) -> i32 { // 1番目の省略規則「引数の数だけライフタイムを指定する」で確定する(戻り値は参照ではないのでライフタイム指定は不要)
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // 3番目の省略規則が適用される例
    fn _announce_and_return_part(&self, announcement: &str) -> &str { // 1番目の適用の後，3番目を適用し，戻り値のライフタイムが決定
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静的ライフタイム
// 'static ライフタイムは特別で，プログラムの全期間で存続する．Java や C の関数内変数に対する static と同一
//      e.g.:
//          let s: &'static str = "I have a static lifetime.";
//  ※'static ライフタイムの指定をコンパイラが提言する場合は，基本的にダングリング参照の発生か，ライフタイムの不一致が原因なので，なるべく従わない


// ジェンリック型引数，トレイト境界，ライフタイム引数すべてを用いた関数の例
// 同じライフタイムを持つ2つの参照引数 x と y と，std::fmt::Display トレイトを実装するある型 T の所有権を受け取り，参照引数と同じライフタイムの参照を返す関数
fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    // アナウンス！
    println!("Announcement! {}", ann); // T は Display を実装することが，トレイト境界により決定
    if x.len() > y.len() {
        x
    } else {
        y
    } // 実際に戻り値として返される参照のライフタイムが，ライフタイム引数により決定
}