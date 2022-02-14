
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // Rectangle 構造体のメソッドをこの中で定義
    // self が不変参照なので，area メソッドはインスタンスを書き換えないことを保証
    fn area(&self) -> u32 { // インスタンスメソッドの第一引数は必ず '&self'．Pythonと同じ．
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // impl ブロックは分割してもよい
    fn square(size: u32) -> Rectangle { // self を第一引数にとらない関連関数．Java のクラスメソッド，C++ のスタティックメソッド．
        Rectangle { width: size, height: size }
    }
}


pub fn chapter5_3() {
    method_for_rectangle();
}

fn method_for_rectangle() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);

    println!("The area of rect1 is {}", rect1.area()); // area メソッドの第一引数が '&self' なので，Rust は rect1 を自動で参照だとみなしてくれる．なので C/C++ のアロー演算子(->)は不要
    println!("The area of rect1 is {}", (&rect1).area()); // 上は実質コレと同じ？(間違っている，例えば'&mut rect1' でも，Rust が「自動参照および参照外し」してくれるのでうまく実行できてしまう)
    
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle::square(60);

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
}