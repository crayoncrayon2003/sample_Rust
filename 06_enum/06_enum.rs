enum Color {
    Red,
    Green,
    Blue,
}
enum GoodsId {
    Cup      = 10000,
    Mouse    = 218,
    Bolt     = 508
}

fn main() {
    let color = Color::Red;

    // match式で比較する
    // colorの値を評価して、結果をresultに入れる
    let result = match color {
        Color::Red => 1,
        Color::Green => 2,
        Color::Blue => 3,
    };
    println!("{}",result);

    // if let式で比較する
    // idの値を評価して、結果をresultに入れる
    let id = GoodsId::Cup;
    let result = if let GoodsId::Mouse = id {
        true
    } else {
        false
    };
    println!("{}",result);
}
