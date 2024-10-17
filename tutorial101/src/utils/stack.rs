
pub fn _test1() {
    let n1 = 112;
    let n2 = &n1;
    let n3 = n2;
    let n4 = &n2;

    println!("            address                  value");
    println!("ads of n1 : {:p} value of n1 : {}", &n1, n1); //show ads & value n1
    println!("ads of n2 : {:p} ads of n1 : {:p}", &n2, &n1); //show ads n2 & n1
    println!("ads of n3 : {:p} ads of n2 : {:p}", &n3, n2); // show ads n3 & n2
    println!("ads of n4 : {:p} ads of n2 : {:p}", &n4, &n2); //show ads n4 & n2
}
