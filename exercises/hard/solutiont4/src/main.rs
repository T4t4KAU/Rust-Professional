//I AM NOT DONE 
//Calculated according to ISO8061 standard

mod calc_time;

fn main() {
    let res = calc_time::time_info("2025-01-18");
    println!("{:?}", res);
}
