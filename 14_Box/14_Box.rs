struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //関数内の変数は通常スタック領域にメモリを確保される
    //String や Vec などの型はヒープ領域にメモリを確保する
    //ヒープ領域は関数が終わっても存在できる
    //ヒープ領域のメモリを確保する汎用的な型に Box<T> がある
    let p: Box<Point> = Box::new(Point { x: 100, y: 200 });
    println!("{} {}", p.x, p.y);
}

//Dropトレイトを実装すると、メモリ解放時に後処理関数を呼び出すことができる
impl Drop for Point {
    fn drop(&mut self) {    // Pointが解放される際に呼び出される
        println!("Bye!");
    }
}