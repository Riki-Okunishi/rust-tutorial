
pub fn chapter9_1() {
    raise_panic();
}

fn raise_panic() {
    // panic!("crash and burn"); // panic! マクロを呼び出した行数が表示される．外部ライブラリの場合はそのソースコードの該当箇所になってしまう

    // let v = vec![1, 2, 3];
    // v[99]; // 環境変数 RUST_BACKTRACE を 0 以外に設定して carge run すると，panic! 呼び出し箇所だけでなく呼び出しのスタックトレースも表示する．Pwsh では $env:RUST_BACKTRACE=1 とする
}