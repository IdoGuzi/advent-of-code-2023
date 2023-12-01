use calibration_sum::read_file;

mod calibration_sum;

fn main() {
    let file_name: String = "../inputs".to_owned();
    let result = read_file(file_name);
    match result {
        Ok(v) => println!("calibration value: {0}", v),
        Err(e) => println!("failed to get calibration value, what: {0}", e)
    }
}
