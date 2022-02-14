
#[derive(Debug)]
enum IpAddrKind { // IP アドレスを示す enum
    V4, // それぞれのバージョンを示す列挙子．
    V6,
}

#[derive(Debug)]
struct IpAddr {
    version: IpAddrKind, // enum を型として使える
    address: String,
}

#[derive(Debug)]
enum IpAddress { // IpAddr のような余計な構造体は不要
    V4((u8, u8, u8, u8)), // V4の列挙子として 8bit * 4 のタプルを用いることを明記
    V6(String),// V6の列挙子として String で代替することを明記．同じタプル内でもどの型を割り当ててもよい．
}

// ちなみに，Rust 標準ライブラリでは以下のような実装になっている
// struct Ipv4Addr {
//     // 省略
// }

// struct Ipv6Addr {
//     // 省略
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }


pub fn chapter6_1() {
    use_ipaddr();
    use_message();
    use_option();
}

fn use_ipaddr() {
    let home = IpAddr {
        version: IpAddrKind::V4, // enum を初期化に使える．呼び出し方は C++ っぽい
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        version: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home address (IP {:?}) is {}", home.version, home.address);
    println!("loopback is {:?}", loopback);

    route(IpAddrKind::V6); // 関数呼び出しの引数にもできる

    // enum の列挙子に直接データを格納する方法で宣言すると，より簡潔になる
    let home = IpAddress::V4((127, 0, 0, 1));
    let loopback = IpAddress::V6(String::from("::1"));

    println!("home is {:?}, loopback is {:?}", home, loopback);
}

fn route(_ip_version: IpAddrKind) {} // enum は引数にもできる

// 別の enum の例
#[derive(Debug)]
enum Message {
    _Quit, // 紐付けられたデータなし
    _Move { x: i32, y: i32 }, // 構造体を紐付け
    Write(String), // String を紐付け
    _ChangeColor(i32, i32, i32), // タプルを紐付け
}

impl Message {
    fn call(&self) { // 構造体と同様に enum にもメソッドを定義できる
        println!("Hey! {:?}", self);
    }
}

// 上の enum は以下のように構造体でも実装できるが，これらすべてを同一視できるのが enum の強み
// struct QuitMessage; // ユニット構造体
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // タプル構造体
// struct ChangeColorMessage(i32, i32, i32); // タプル構造体

fn use_message() {
    let m = Message::Write(String::from("hello"));
    m.call(); // call メソッドには 'Write("hello")' が渡される
}

// 標準ライブラリの Option トレイト
// enum Option<T> {
//     Some(T),
//     None,
// }

fn use_option() {
    let _some_number = Some(5); // Option<i32> と推論
    let _some_string = Some("a string"); // Option<String> と推論

    let _absent_number: Option<i32> = None; // i32 だけど空っぽを示す．'None' では型推論できないので，型注釈が必須

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = x + y; // ここでエラー．_y が null でないか保証できないのでコンパイラが止める
}