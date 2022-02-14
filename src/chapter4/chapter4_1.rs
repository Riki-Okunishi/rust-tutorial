
pub fn chapter4_1() {
    scope_literal();
    scope_heap();
    move_of_const();
    move_of_heap();
    clone_heap();
    move_scope();
}

fn scope_literal() {
    {// sはまだ有効ではない．
        let s = "hello"; // 文字列リテラルとしてハードコード．'hello'の所有者は変数's'ここからsは有効になる．
        // sで作業
    }// ここで'hello'の所有者's'のスコープが切れるため，値'hello'は破棄される．
}

fn scope_heap() {
    {
        let s = String::from("hello"); // sはここから有効．ヒープにメモリを確保．
    } // ここで s がスコープを抜け，確保したメモリが開放される．内部的に drop という関数を呼んでいる．C++のデストラクタ的なもの．
}

fn move_of_const() {
    {
        let x = 5;
        let y = x;
        println!("x = {}, &x = {:p}, y = {}, &y = {:p}", x, &x, y, &y) // x も y も両方5．yはxとは異なるメモリ上の領域に置かれる．
    }
}

fn move_of_heap() {
    {
        let s1 = String::from("hello");
        println!("s1 = '{}', address of s1 = {:p}", s1, &s1);
        let s2 = s1; // どちらも参照すると'hello'が見える．実体は同じでメモリ上でも同一領域．でも参照するためのポインタのアドレスは別領域．s1がs2に'ムーブ'された => Rustでは暗黙的にDeep Copyは行われない！
        println!("s2 = '{}', address of s2 = {:p}", s2, &s2); // 所有権がs1からs2に移っているので，s1からはもう参照できなくなる('borrow of moved value: `s1`'や'value borrowed here after move'とエラーが出てs1は使えない)
    }
}

fn clone_heap() {
    {
        let s1 = String::from("hello");
        println!("s1 = '{}', address of s1 = {:p}", s1, &s1);
        let s2 = s1.clone(); // Stringならclone メソッドでDeep Copyが可能
        println!("s1 = '{}', address of s1 = {:p}, s2 = '{}', address of s2 = {:p}", s1, &s1, s2, &s2); // Deep Copyなら両方の変数が所有権を持ったまま複製される．
    }
}

fn move_scope(){
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
    //println!("s = {}", s);        // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが，
    println!("x = {}", x);          // i32はCopyなので，この後にxを使っても大丈夫

} // ここでxがスコープを抜け，sもスコープを抜ける．ただし，sの値はムーブされているので，何も特別なことは起こらない

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け，`drop`が呼ばれる．後ろ盾していたメモリが解放される

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける．何も特別なことはない