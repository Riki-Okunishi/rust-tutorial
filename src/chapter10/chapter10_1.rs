
pub fn chapter10_1() {
    find_max_using_specific_functions();
    generic_struct();
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_max_using_specific_functions() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list); // 最大値を得る i32 用の関数を呼ぶ
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list); // 最大値を得る char 用の関数を呼ぶ(同じ「最大値」なのに使い回しはできない)
    println!("The largest char is {}", result);
}

// 10-2から戻ってきて，トレイト境界を適切に設定
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // 関数シグネチャで用いる型引数名を型名宣言 '<>' 内で宣言
    let mut largest = list[0]; // T は Copy トレイトを実装しているので．借用したスライスから所有権を求めることなく，複製を生成できる

    for item in list.iter() { // ここはムーブされる文脈で「ムーブしないで」の'&'？それともパターンマッチの文脈で「参照が返ってくるけど参照を取った部分(値)が'item'ね」という構文？わからない…
        if *item > largest {
            largest = *item;
        }
    }

    largest
}


// Rust では単相化(monomorphization) によりジェネリックは具体的なコードに変換され，非ジェネリックの場合より実行速度が遅くならないようにしている
struct Point<T> { // 構造体の定義にもジェネリクスは使用できる
    x: T,
    y: T,
}

// コンパイラの単相化の結果，上記の構造体に i32 や f64 を当てはめると以下のコードでコンパイルされる
// struct Point_i32 {
//     x: i32,
//     y: i32,
// }
//
// struct Point_f64 {
//     x: f64,
//     y: f64,
// }

impl<T> Point<T> { // impl ブロック内でジェネリックが利用されいていることをコンパイラに伝えるために，impl キーワードの直後にも型宣言が必須
    pub fn x(&self) -> &T {
        &self.x // '.' の方が '&' よりも結合優先度が高いので，`&(self.x)` と同値
    }
}

impl Point<f32> { // impl で指定する構造体のジェネリックを具体的な型で埋めると，「その型だったとき」の関数が定義できる
    fn distance_from_origin(&self) -> f32 { // この関数は，Point<f32> でないときは定義されない
        (self.x.powi(2) + self.y.powi(2)).sqrt() // powi は f32 に実装されているメソッド．x の型は f32 であると前提にして記述可能
    }
}

struct Point2T<T, U> { // 複数のジェネリックな型を使ってもよい
    x: T,
    y: U,
}

impl<T, U> Point2T<T, U> {
    // 自身と引数の所有権はムーブされこのメソッド終了時に Drop される
    fn mixup<V, W>(self, other: Point2T<V, W>) -> Point2T<T, W> { // 関数の型引数宣言はあくまで引数のジェネリックについて．impl ブロックの時点でPoint2T そのもののジェネリック型は解決している
        Point2T {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_struct() {
    // Point の型引数から，x も y も同じ型が要請される
    let p_i32 = Point{x: 5, y: 10};
    let p_f64 = Point{x: 5.0, y: 10.0};
    // let _p_i32_and_f64 = Point{x: 5, y: 10.0}; // これはコンパイルエラー
    println!("p_i32.x = {}, p_f64.x = {}", p_i32.x(), p_f64.x());

    let p_f32: Point<f32> = Point{ x: 4.0, y: 10.0};
    println!("distance from origin = {}", p_f32.distance_from_origin());

    // Point2T の型引数から，x と y には異なる型の代入を要請
    let _p2t = Point2T{x: 4, y: 2.5};

    let p1 = Point2T { x: 5, y: 10.4 };
    let p2 = Point2T { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2); // Point<i32, char> であることがコンパイル時に確定

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

