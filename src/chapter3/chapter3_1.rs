
// see https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html

pub fn chapter3_1() {
    /* 3.1 変数と可変性 */

    // (1) 不変な値
    let x = 5; // immutable で宣言し初期化．
    println!("変数 x を immutable で宣言し同時に代入：x = {}", x); // 不変な値は参照できる．
    // x = 10; // immutable な変数には1度しか代入はできない．

    let x: i32; // immutable で宣言．変数の型を明示すれば宣言だけ可能．
    // println!("宣言時点での x の値は {}", x); // 宣言だけで初期化せずに参照しようとすると，"use of possibly-uninitialized `x`" で怒られる．
    x = 6; // 1度だけ代入が可能．
    println!("変数 x を immutable で宣言．その後で代入：x = {}", x);
    // x = 15; // もちろん再代入は不可．

    // (2) 可変な値
    let mut y = 3; // mutable で宣言し初期化．
    println!("変数 y を mutable で宣言し代入：y = {}", y); // 可変な値も参照できる．
    y = 13; // mutableな変数には代入ができる．
    println!("変数 y に値を再代入：y = {}", y);

    let mut y: i32; // mutable で宣言．
    // println!("宣言時点での y の値は {}", y); // "use of possibly-uninitialized `y`" で怒られるのは immutable のときと同じ．
    y = 7;
    println!("変数 y を mutable で宣言．その後で代入：y = {}", y);
    y = 15;
    println!("変数 y に値を再代入：y = {}", y);

    /* 定数 */
    const MAX_HP: i32 = 100; // 定数は 'const' で始め，型注釈は必須．常に immutable で，評価されない値(リテラル？)しか設定できない．
    const AUTHOR: &str = "Riki"; // 文字列定数なら &str 型？
    // const MAX_HP: i32 = 200; // 2回以上宣言できない．
    // AUTHOR = "Okunishi"; // const に対する代入は構文としてそもそも成立しない．
    println!("定数も参照は他の変数と一緒：MAX_HP = {}, AUTHOR = {}", MAX_HP, AUTHOR);
    // const mut MAX_ATK: i32 = 1500; // もちろん const は mutable にはできない

    /* シャドーイング */
    // (1) 基本型のシャドーイング
    let a = 10; // 基本型を immutable で宣言し初期化．
    println!("シャドーイング前；a = {}, &a = {:p}", a, &a);
    let a = 15; // immutable としてシャドーイング
    println!("シャドーイング後：a = {}, &a = {:p}", a, &a); // シャドーイングすると，新しいメモリ領域が割り当てられ，そちらが使用されるようになる．
    let mut a = 20; // mutable としてシャドーイング
    println!("mutable 変数 a としてシャドーイング後：a ={}, &a = {:p}", a, &a); // mutable への変更も可能．メモリの再割当ても同様．
    a = 30; // mutable なので変更可能．
    println!("mutable 変数 a として変更後：a = {}, &a = {:p}", a, &a);

    // (2) ヒープ格納型のシャドーイング
    let s = String::from("hello"); // ヒープ格納型を immutable で宣言し初期化．
    println!("シャドーイング前；s = {}, &s = {:p}", s, &s);
    let s = String::from("world"); // immutable としてシャドーイング．
    println!("シャドーイング後：s = {}, &s ~ {:p}", s, &s); // シャドーイングすると，新しいメモリ領域が割り当てられ，そちらが使用されるようになる．
    let mut s = String::from("hello"); // mutable としてシャドーイング
    println!("mutable 変数 s としてシャドーイング後：s ={}, &s = {:p}", s, &s); // mutable への変更も可能．メモリの再割当ても同様．
    s.push_str(", world!"); // mutable なので変更可能．
    println!("mutable 変数 s として変更後：s = {}, &s = {:p}", s, &s);
    
}