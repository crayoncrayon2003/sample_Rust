mod sub1;

fn main() {
    sub1::show_name();

    let var_const = sub1::get_var_const();
    println!("var_const {}", var_const);

    let mut var_static;
    var_static = sub1::get_var_static();
    println!("var_static {}", var_static);

    sub1::countup_var_static();

    var_static = sub1::get_var_static();
    println!("var_static {}", var_static);
}