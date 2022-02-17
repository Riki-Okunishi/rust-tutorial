
pub fn chapter8_1() {
    use_vecter();
}

enum SpreadsheetCell { // 複数の異なる型を enum でラップし，同一視することで Vec に格納できる．どんな型が格納されうるか事前に分からない場合は，トレイトオブジェクトを使う
    Int(i32),
    Float(f64),
    Text(String),
}

fn use_vecter() {
    let _v_empty: Vec<i32> = Vec::new(); // Vec の関連関数 new で Vec 型を生成
    
    let _v = vec![1, 2, 3]; // 型推論があるので，vec! マクロで初期値を与える場合は型注釈は不要
    
    let mut v = Vec::new(); // 次の行で i32 を push しているので，型推論が可能．型注釈は不要
    v.push(4); // 可変な Vec は要素の追加ができる

    {
        let _inner_scope_v = vec![1, 2, 3];
    } // スコープを抜けると，Vec も，その要素すべてもドロップされる

    let v = vec![1, 2, 3, 4, 5];

    let _third: &i32 = &v[2]; // 3 番目の要素への参照を，& と [] を使って得る．存在しない要素を指定するとクラッシュさせられる
    let _third: Option<&i32> = v.get(2); // 3 番目の要素への参照を，get メソッドで得る．存在しない要素を指定すると，None が得られる．

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6); // Vec が要素を追加するときにメモリ再割当てが起こる可能性があるので，参照されている限り可変な Vec であっても要素を追加できない．不変参照と可変参照が同時に存在できない，というルールが，要素レベルで適用

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v { // ただ要素を確認するだけなら，Vec の参照を渡す
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 参照への演算は，アドレス移動が要素への演算か分からないので(？)，明示的な参照外し * が必要．自動参照外しは使えない
    }

    let _row = vec![ // row はあくまで SpreadsheetCell 型の Vec
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}