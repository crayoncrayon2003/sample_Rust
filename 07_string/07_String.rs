fn main() {
    // 文字列を初期化する
    let mut name: &str = "Yamada";
    println!("{}", name);	// => Yamada
    name = "Tanaka";
    println!("{}", name);	// => Tanaka

    // 文字列を初期化する
    let mut name = String::from("Yamada");
    // 別の文字列を設定する
    name = "Tanaka".to_string();
    // 文字列に追加する
    name.push_str(" Taro");
    println!("{}", name);	// => Tanaka Taro

    //スライス
    let s = String::from("ABCDEFGH");
    let s1 = &s[0..3];          // 0番目から3番目の手前までのスライス("ABC")
    let s2 = &s[3..6];          // 3番目から6番目の手前までのスライス("DEF")
    println!("{} {}", s1, s2);	// => ABC DEF

    let a = [10, 20, 30, 40, 50, 60, 70, 80];
    let a1 = &a[0..3];              // 0番目から3番目の手前までのスライス[10, 20, 30]
    let a2 = &a[3..6];              // 0番目から3番目の手前までのスライス[40, 50, 60]
    println!("{:?} {:?}", a1, a2);  // => [10, 20, 30] [40, 50, 60]
}