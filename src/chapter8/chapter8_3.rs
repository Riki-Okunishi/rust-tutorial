
use std::collections::HashMap; // 標準ライブラリから HashMap を use

pub fn chapter8_3() {
    use_hash_map();
}

fn use_hash_map() {
    let mut scores = HashMap::new(); // 後で追加するキーと値の組み合わせから，HashMap<String, i32> と型推論してくれる

    scores.insert(String::from("Blue"), 10); // 要素の追加は insert メソッド
    println!("Blue is {}", scores["Blue"]); // 添字アクセスはリテラル(&str)で可能．存在しなければパニック
    let c = String::from("Yellow");
    scores.insert(c, 50);
    // println!("c = {}, v = {}", c, scores.get(&"Yellow")); // c の所有権は scores にムーブするので参照できなくなる
    /* 参照を HashMap に渡す場合，その参照が HashMap より長く有効でる必要があるので，ライフタイムへの理解が必要 */

    let teams  = vec![String::from("Blue"), String::from("Yellow")]; 
    let initial_scores = vec![10, 50];

    // collect メソッドの結果どの型が欲しいのかは型推論できないので，`HashMap<_, _>` は必須．しかし，HashMap だと確定されれば HashMap<String, i32> であることは推論できるので省略可．あんまり省略しない方がよい？
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // 2 つの Vec の IntoIterator を zip してタプルの Vec を作り，collectで HashMap を生成

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name); // 値へのアクセスは，get メソッド．Option 型を得る．参照を取るのでキーを指定する変数の所有権は奪われない

    for (key, value) in &scores { // for 終了後 move されてしまうので，&score としている．デフォルトのハッシュ関数がランダムにキーを割り振るので，実行のたびに表示順が変わる
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 同じキーに insert されれば，値が更新される

    println!("{:?}", scores); // {"Blue": 25}

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // entry メソッドでは，指定したキーが存在するかを Entry という Enum を返す．さらに or_insert メソッドで存在しない場合は 50 設定し，可変参照を返す
    scores.entry(String::from("Blue")).or_insert(50); // キー 'Blue' は存在するので更新しない．値は 10 のまま

    println!("{:?}", scores);


    let text = "hello world wonderful world";
    let mut map = HashMap::new(); // 各単語の登場回数の HashMap

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 単語が HashMap に存在しなければ，or_insert で 0 に初期化して可変参照を得る．存在すれば Entry が可変参照を返す？
        *count += 1; // 参照なので参照外し * が必要．for の終端でこの可変参照はスコープを抜けるので，借用規則には違反しない
    }

    println!("{:?}", map);

}