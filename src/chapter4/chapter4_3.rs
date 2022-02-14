
pub fn chapter4_3() {
    use_slice();
}

fn use_slice() {
    let mut s = String::from("hello, world");
    
    let hello = &s[..5]; // `&s[0..5]` と等価
    let world = &s[6..]; // `s[6..11]` と等価

    let word = first_word(&s); // ここで得た変数 s の「最初の1単語」は，意味を失わない．不変借用を使うことで，以降は可変変数(変数 s)にも不変性が要求されるから．
    // s.clear(); // Chap. 4.2で扱った「同一スコープ内に可変参照と不変参照が共存できない」に反するためエラー．'cannot borrow `s` as mutable because it is also borrowed as immutable'でエラー
    println!("the first word is {}", word); // 不変な借用の結果得た値が，使用するまでに意味をなさなくなる可能性を排除できる

    let s = "hello, world!"; // 文字列リレラルを格納する変数の型は，'&str'．つまり，バイナリの特定の位置を指すスライス str の，不変な参照 &str．
    let first = first_word_by_str(&s);
    println!("first word is {}", first);
}

fn first_word(s: &String) -> &str { // String を受け取って String のスライスを返す
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String からスライスは作れるので，そもそもスライスを受け取りスライスを返す関数でいい
fn first_word_by_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}