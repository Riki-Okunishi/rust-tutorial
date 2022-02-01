
// see https://doc.rust-jp.rs/book-ja/ch03-05-control-flow.html

pub fn chapter3_5() {
    loop_fn();
    while_fn();
    for_fn();
}

fn loop_fn() {
    // 'loop'は無限ループ専用
    let mut n = 3;
    loop {
        println!("in loop!");
        if n == 0 {
            // if-else-break などで脱出するしかない．
            break;
        }
        n -= 1;
    }
}

fn while_fn() {
    // 'while'は条件式でループを制御する場合専用(？)
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 { // コンパイラがループの各回ごとに境界値チェックのコードを挿入するので遅い
        println!("in while! a[{}] = {}", i, a[i]);
        i += 1;
        // if-else-breakなしでも，whileの条件式が満たされれば脱出できる．
    }
}

fn for_fn() {
    // 'for'はコレクションの全要素を参照するときに使う．コレクションの長さを意識しなくてよい．いわゆるforeach．
    let a = [10, 20, 30, 40, 50];
    // let mut i = 0; // 添字管理は不要なので，配列外参照によるパニックや要素の列挙漏れが防げる．
    for i in a { // コンパイラが境界値チェックのコードを挿入しないので，whileよりも高速．
        println!("in for! {}", i);
        // println!("in while! a[{}] = {}", i, a[i]); // 順序の添字はこれだけではもってこれない．
        // 条件式もなく，コレクションの全てを参照したら自動で終了．
    }
    // コレクションでなくても，指定回数を'for'でループしたければ，Range型を使う．
    for i in 1..4 { // '始端..終端' と書けばRange型．単体ならカッコは不要．シェルっぽい．
        println!("in for! by Range: {}", i);
    }
    for i in (1..4).rev() {
        println!("in for! by reverced Range: {}", i);
    }
}