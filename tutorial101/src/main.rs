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
    // println!("result3 : {:?}", result3);

    println!("after type conversion of x");
    utils::print::print_type_of(&y);
    // println!("result3 : {:?}", result4);
}
