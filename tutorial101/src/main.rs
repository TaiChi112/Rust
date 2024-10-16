#![allow(dead_code)]
struct GFG {
    a: i32,
    b: i32,
    c: i32,
}
mod casting;

mod utils {
    pub mod print;
    pub mod sub;
}

fn main() {
    let result = casting::add(1, 2);
    println!(
        "my result testing import fn add from other file : {}",
        result
    );
    let result2 = utils::sub::sub(27, 8);
    println!(
        "my result testing import fn sub from other folder file : {}",
        result2
    );

    let x: i32 = 112;
    let y: u32 = x as u32;

    println!("before type conversion of x");
    utils::print::print_type_of(&x);

    println!("after type conversion of x");
    utils::print::print_type_of(&y);

    let decimal_value = 65.4321_f32;
    let integer_value = decimal_value as u8;
    let character_value = integer_value as char;

    println!(
        "the value of decinal_value is : {} and its datatype is ",
        decimal_value
    );
    utils::print::print_type_of(&decimal_value);

    println!(
        "the value of integer_value is : {} and its datatype is ",
        integer_value
    );
    utils::print::print_type_of(&integer_value);

    println!(
        "the value of character_value is : {} and its datatype is ",
        character_value
    );
    utils::print::print_type_of(&character_value);

    println!(
        "casting from {} -> {} -> {}",
        decimal_value, integer_value, character_value
    );

    let g_var = GFG { a: 1, b: 2, c: 3 };
    let var1 = &g_var;
    let var2 = &g_var;

    println!("GEF has var : ({},{},{})", var1.a, var2.b, g_var.c);

    let mut geek_var = GFG { a: 1, b: 2, c: 3 };

    let vari1 = &geek_var;
    let vari2 = &geek_var;

    println!("GFG has vari : ({},{},{})", vari1.a, vari2.b, geek_var.c);

    let mutable_reborrow_variable = &mut geek_var;
    mutable_reborrow_variable.a = 10;
    mutable_reborrow_variable.b = 20;
    mutable_reborrow_variable.c = 30;

    println!(
        "GFE now has immutable variable : ({},{},{})",
        mutable_reborrow_variable.a, mutable_reborrow_variable.b, mutable_reborrow_variable.c
    );

    let new_reborrow = &geek_var;
    println!(
        "GFG now has reborrowed variable : ({},{},{})",
        new_reborrow.a, new_reborrow.b, new_reborrow.c
    );
}
