
#[derive(Debug)] // println! マクロでフォーマット指定子 '{:?}' を指定して表示できるようにするための注釈．Debug トレイトから派生させる
struct Rectangle {
    width: u64,
    height: u64,
}

pub fn chapter5_2() {
    calc_rectangle();
}

fn calc_rectangle() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect 1 is {:?}", rect1); // Debug トレイトを持つ構造体の中身を表示する
    println!("The area of rect1 is {}", area(&rect1));
}

// 長方形の面積を不変参照を借用して計算
fn area(rect: &Rectangle) -> u64{
    rect.width * rect.height
}