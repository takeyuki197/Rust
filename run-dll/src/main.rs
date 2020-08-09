extern crate libloading as lib;

fn main() {
    let lib = lib::Library::new("SumAll.dll").unwrap();
    // lib.get().unwrap()が返すDLL関数の型
    let func: lib::Symbol<unsafe extern "C" fn(i32, *const f64) -> f64>;
    // unsafeな関数はunsafe block内で呼び出す
    unsafe {
        // lib.getはoption型を返すのでunwrapする
        func = lib.get(b"sum_all").unwrap();
        //let arr = [0.1, 0.2, 0.3, 0.4];
        // *const f64には生ポインタを入れる
        //let r = &arr as *const f64;
        //println!("{}", func(4, r));
    }

    // optional型の返り値はunwrapして中身を取り出す
    let mut rdr = csv::Reader::from_path("foo.csv").unwrap();
    for result in rdr.records() {
        // recordはStringRecord型
        let record = result.unwrap();
        // recordの第0要素はstringでありparseしてlongに変換
        let val_num: i32 = record.get(0).unwrap().parse().unwrap();
        //println!("{:?}", record);
        //println!("{}", lValNum);
        // recordの第1要素もstringであり、カンマで分けてからそれぞれをdoubleに変換
        let vals_str: Vec<&str> = record.get(1).unwrap().split(',').collect();
        let mut vals_vec: Vec<f64> = Vec::new();
        for val in vals_str {
            vals_vec.push(val.parse().unwrap());
        }
        //println!("{:?}", lValsVec);
        // funcに入れるために生ポインタに変換
        let vals = vals_vec.as_ptr();
        unsafe {
            println!("DLL's return is {}", func(val_num, vals));
        }
    }
}