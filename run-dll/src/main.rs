//use std::error::Error;
//use csv::Reader;
//use std::process;

extern crate libloading as lib;


fn main() {
    let lib = lib::Library::new("SumAll.dll").unwrap();
    let func: lib::Symbol<unsafe extern "C" fn(i32, *const f64) -> f64>;
    unsafe {
        //let func: lib::Symbol<unsafe extern "C" fn(i32, *const f64) -> f64> = lib.get(b"sum_all").unwrap();
        func = lib.get(b"sum_all").unwrap();
        //println!("Call DLL function!");
        //let arr = [0.1, 0.2, 0.3, 0.4];
        //let r = &arr as *const f64;
        //println!("{}", func(4, r));
    }

    let mut rdr = csv::Reader::from_path("foo.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        //println!("{:?}", record);
        let val_num: i32 = record.get(0).unwrap().parse().unwrap();
        //println!("{}", lValNum);
        let vals_str: Vec<&str> = record.get(1).unwrap().split(',').collect();
        let mut vals_vec: Vec<f64> = Vec::new();
        for val in vals_str {
            vals_vec.push(val.parse().unwrap());
        }
        //println!("{:?}", lValsVec);
        let vals = vals_vec.as_ptr();
        unsafe {
            println!("DLL's return is {}", func(val_num, vals));
        }
    }
}