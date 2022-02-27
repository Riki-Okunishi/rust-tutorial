use std::fmt::{Display, Debug};
use crate::chapter10::chapter10_1;

pub fn chapter10_2() {
    use_summmary_trait();
    use_trait_in_args();
    find_max_using_generic_function();
    use_trait_bound();
}

pub trait Summary { // このクレートに基づいた実装を外部でも行う場合は pub で公開する
    fn summarize(&self) -> String; // 実装はしないのでセミコロンで終了
    // fn summarize(&self) -> String { // 普通に実装したら「デフォルト実装」となってオーバライドしなければこの実装が利用される
    //     String::from("(Read more...)")
    // }
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String { // この場合は，summarize_author を実装すればよい
    //     format!("(Read more from {}...)", self.summarize_author())
    // }

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { // メソッド定義と異なるのは，for キーワーが必要なこと
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        String::from("foo")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String { // もしSummary トレイトでの summarize メソッドの実装がこの通りであったら，'summarize_authoer' を呼び出すという動作だけが規定される
        format!("{}: {}", self.summarize_author(), self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

}

fn use_summmary_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            // もちろん、ご存知かもしれませんがね、みなさん
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub fn notify(item: &impl Summary) { // あるトレイトを実装した型を引数とする場合は，実装されているトレイトを明示的に impl キーワードを使って宣言
// pub fn notify<T: Summary>(item: &T) { // '上の行'はこの行(「トレイト境界」)の糖衣構文．本来はジェネリックに型宣言を付加した形で記述する
    println!("Breaking news! {}", item.summarize());
}

// impl キーワードを使う場合は，「該当トレイトを実装した型」がマッチするため，item1 と item2 の型が異なってもよい
pub fn join_summary(item1: &impl Summary, item2: &impl Summary) {
    println!("item1 summary = {}, item2 summary = {}", item1.summarize(), item2.summarize());
}

// ジェネリクスの型引数を使う場合は，単相化のため，2つの引数が同じ型であることを強制できる
pub fn join_tweets<T: Summary>(item1: &T, item2: &T) {
    println!("joined tweets:\n\t{}\n\t{}", item1.summarize(), item2.summarize());
}

fn use_trait_in_args() {
    let tweet1 = Tweet {
        username: String::from("Ricky"),
        content: String::from("Rust is very important!!!"),
        reply: false,
        retweet: false,
    };
    let tweet2 = Tweet {
        username: String::from("Ricky"),
        content: String::from("but it's too complex... :("),
        reply: true,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Does Rust make us craetive?"),
        location: String::from("Japan"),
        author: String::from("Ricky"),
        content: String::from("I'm only halfway through the Rust Tutorial, but I'm sure Rust will give us the creativity."),
    };
    
    notify(&article);
    join_tweets(&tweet1, &tweet2);
    join_summary(&article, &tweet1);
}

// 型引数には複数の型を指定できるし，複数のトレイトの実装を '+' で結合して指定できるが，シグネチャが長くなる
fn _some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    format!("{}{:?}", t, u);
    0
}

// where 句を使うと，トレイト境界を明確化できる
fn _some_function_use_where<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone, // 型注釈だけ後ろに回せる．Haskellっぽい
          U: Clone + Debug
{
    format!("{}{:?}", t, u);
    0
}

// トレイトを実装した型を返す．一種類の型を返す場合のみ利用可能
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


// `impl <トレイト名>`で指定するのは，'あるトレイトを実装するある型'という1つの型に過ぎない(「単相化」のため)ので，複数の型が返りうると型が特定できずコンパイルエラー(第17章で解決)
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn find_max_using_generic_function() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = chapter10_1::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_using_reference(&char_list);
    println!("The largest char is {}", result);
}

fn largest_using_reference<T: PartialOrd>(list: &[T]) -> &T { // copy トレイトを要請しないのであれば，参照を使う
    let mut largest = &list[0]; // list は借用しているので，その要素も借用(参照で扱う)しなければならない
    // let ref mut largest = list[0]; // ref mut を使うと，largestの型が「『T の可変変数』への不変参照」になるので，型が一致しない(欲しいのは「『Tの不変参照』の可変変数」)．C/C++ でいう「ポインタへの const ポインタ」みたいになってる
    // match のパターンマッチのとき『「参照型の変数」ではなく「変数の参照」が当てはまる』やつみたいな
    // 1. &some -> 変数 some にマッチした値を「借用して」入れる．← '&' は代入の仕方を指定する(借用するのかしないのか)
    // 2. ref some -> 変数 some に「マッチした値の参照」を入れる ← ref は代入するものの種類を指定する(値を代入するのか参照を代入するのか)(C/C+ でいう値なのかポインタなのか/ポインタなのかポインタのポインタなのか)

    for item in list.iter() {
        if item > largest { // 自動参照外しで `*item > *largest` として評価される
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 特定の型であるときの場合と同じように，トレイト境界をimpl に続ければ，条件付けで実装させるメソッドを定義できる
impl<T: Display + PartialOrd> Pair<T> { // Pair<T> の T が比較可能(PartialOrd)かつ出力可能(Display)であれば定義されるメソッド
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait Output {
    fn output_str(&self) -> String;
}

// 特定のトレイト境界を持つ型全てに対して Output トレイトを実装する「ブランケット実装」
impl<T: Display> Output for T {
    fn output_str(&self) -> String {
        format!("Output: {}", self)
    }
}

fn use_trait_bound() {
    let p1 = Pair::new(String::from("left_x"), String::from("right_y"));
    p1.cmp_display();
    let _p2 = Pair::new(Tweet{username: String::from("user1"), content: String::from("content1"), reply: false, retweet: false}, Tweet{username: String::from("user2"), content: String::from("content2"), reply: false, retweet: false});
    // _p2.cmp_display(); // Tweet は PartialOrd と Display を実装していないのでコンパイルエラー

    let s = String::from("It's String");
    println!("{}", s.output_str()); // String は Display を実装するトレイトなので，Output トレイトも実装する
}