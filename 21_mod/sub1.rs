const MY_VAR_CONST: u32 = 100;

use std::sync::atomic::{self, AtomicU32, Ordering};
static MY_VAR_STATIC: AtomicU32 = AtomicU32::new(0);

// pubで関数を外部公開
pub fn show_name() {
    println!("this is foo_func");
}

pub fn get_var_const() -> u32{
    return MY_VAR_CONST;
}

pub fn get_var_static() -> u32{
    return MY_VAR_STATIC.load(Ordering::SeqCst);
}

pub fn countup_var_static(){
    MY_VAR_STATIC.fetch_add(1, atomic::Ordering::SeqCst);
}