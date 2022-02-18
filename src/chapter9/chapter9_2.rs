
use std::io::{self, Read, ErrorKind, Write};
use std::fs::File;

pub fn chapter9_2() {
    use_result();
    use_unwrap_and_expect();
    transfer_error();
}

fn use_result() {
    let _f = File::open("hello.txt"); // 戻り地は Result<T, E> 型．具体的には，Result<std::fs:File, std::io::Err>．f は成功すると Ok インスタンス，失敗すると Err インスタンス

    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => { // マッチガード．アームの条件を if 式でさらに補強できる．kind メソッドは参照に対して呼べる．ref は自動で解決されるので不要？
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    
    f.write_all(b"ricky").unwrap();
}

fn use_unwrap_and_expect() {
    // let _f = File::open("hello.txt").unwrap(); // unwrap は Ok を返すが，返せず失敗すれば panic!
    // let _f = File::open("hello.txt").expect("Failed to open hello.txt"); // expect は panic! 時のエラーメッセージを指定
}

fn transfer_error() {
    let res = read_username_from_file();
    match res {
        Ok(s) => println!("username is {}", s),
        Err(e) => println!("error occured at read_username_from_file(): {:?}", e)
    }
}

fn _read_username_from_file_naive() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file_using_question() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? を Result<T, E> を返す関数の後につけると，Err が返ると自動でそれを return．From トレイト経由してどんな型のエラーも戻り値の型へ変換して返せる
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // メソッドチェーンで簡潔に書ける．Err が返った時点で return

    Ok(s)
}