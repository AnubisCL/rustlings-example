// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    
    // 导出宏
    pub(crate) use my_macro;
}

use macros::my_macro as mm;
fn main() {
    mm!();
}
