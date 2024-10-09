struct Rect { width: u32, height: u32 }

// traitは、構造体が実装すべきメソッドを定義する
// 他言語の インタフェース(interface) に似ている
//
// 例えば、std::fmt::Display トレイトを実装した構造体は println!() の "{}" で、
// std::fmt::Debug トレイトを実装した構造体は "{:?}" で書き出すことが可能です

// 構造体Printableにメソッドprintを定義するためのインターフェース定義
trait Printable { fn print(&self); }
impl Printable for Rect {
    fn print(&self) {
        println!("width:{}, height:{}", self.width, self.height)
    }
}

fn main() {
    let r = Rect { width: 200, height: 300 };
    r.print();
}