
pub fn chapter8_2() {
    use_string();
}

fn use_string() {
    let mut _s = String::new(); // 空の String は関連関数 new で生成

    let data = "initial contents";
    let _s = data.to_string(); // Display トレイトを実装する型なら，String 型として変換できる

    let _s = "initial contents".to_string(); // どちらも
    let _s = String::from("initial contents"); // 同じ

    /* UTF-8 エンコードされていれば，どんな文字でも受け入れる */
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str で String 型に 文字列を借用して追加．s2 はそもそも参照なので，& は不要
    println!("s2 is {}", s2); // s2 の所有権は奪われていないので，まだ使える

    let mut s = String::from("hello");
    let l = String::from(", world");
    s.push_str(&l); // &str ではなく String でも可能．&String である l は，`&l[..]` とみなされ，スライス &str として「型強制」される

    let mut s = String::from("lo");
    let l = 'l'; // シングルクォーテーション(')は char，ダブルクォーテーション(")は&str．C 言語と同じ
    s.push(l); // push では char を追加する

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s2 は借用することしかできないため，まだ使用可能．s2 は String だが，「参照型外し型強制」によって変換されている．&s2[..] と暗黙に変換
    // この時点で s1 はムーブされているので使用不可．add メソッドは self を取るため
    // s3 は s1 の所有権を奪うので，低コスト

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // この書き方では，s1 の所有権が奪われる
    let _s = format!("{}-{}-{}", s1, s2, s3); // s1 から s3 の所有権はどれも奪われない．format! マクロは，C++ の sprintf，Golang の fmt.Sprintf に相当

    let _s1 = String::from("hello");
    // let h = s1[0]; // Rust では String の添字アクセスは不可．UTF-8では文字によって 1 文字のバイト数が異なるため．そしてそのため，添字アクセスが O(1) にならないため
    let len = String::from("Здравствуйте").len();
    println!("length of 'Здравствуйте' is {}", len); // 24 を返す．2 バイト * 12 文字のため

    let _s = "नमस्ते"; // デーヴァナーガリー(ヒンディー語)
    // 1. これを Vec<u8> (コンピュータが保持する形式)で表現：[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // 2. これを Unicode スカラ値(Rustでの表現．おそらくVec<char>)で表現：['न', 'म', 'स', '्', 'त', 'े'] // 4番目と6番目は文字ではなく，単独では意味をなさないダイアクリティック
    // 3. これを 書記素クラスタ(ヒンディー語話者が捉える単位)として表現：["न", "म", "स्", "ते"]

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 文字列スライスが欲しいと明確に表現：バイトとして前から4個分
    // let s = &hello[0..1]; // 先頭 0 バイト目だけアクセスしようとすると，文字として無意味になるので実行時エラー：' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'

    for c in "नमस्ते".chars() { // chars メソッドで char を 1 つずつ得られる(2.)
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // bytes メソッドで u8 を 1 つずつ得られる(1.)
        println!("{}", b);
    }

    // 書記素クラスタ(3.) を得るのは複雑なので標準では非対応

    
}