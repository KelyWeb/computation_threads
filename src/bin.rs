extern crate lib_m;
use lib_m::parallel_computing;


fn main() {

    let test_data = vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
    println!("{:?}", parallel_computing(test_data, |x| x * x));
}