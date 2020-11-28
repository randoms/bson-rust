use std::fs::File;
use std::io::{
    BufWriter,
    BufReader
};

use bson::{deserialize_array, serialize_array, bson};
fn main() {
    let test_bson = bson!{[1,2,3,4]};
    let file = File::create("test.bson").unwrap();
    let mut out = BufWriter::new(file);
    serialize_array(out.get_mut(), test_bson.as_array().unwrap()).expect("write array");
    let in_file = File::open("test.bson").unwrap();
    let mut in_buf = BufReader::new(in_file);
    let data = deserialize_array(in_buf.get_mut(), false).unwrap();
    println!("{:#?}", data);
}
