use bson::{Bson, StreamDeserializer, StreamSerializer, bson};
use std::fs::File;
use std::io::{
    BufWriter,
    BufReader
};
fn main() {
    {
        let file = File::create("test.bson").unwrap();
        let out = BufWriter::new(file);
        let mut ser = StreamSerializer::new(out);
        for i in 0..100 {
            ser.serialize_element(&(bson!({"test": i}))).unwrap();
        }
        ser.end().unwrap();
    }
    let mut ins = BufReader::new(File::open("test.bson").unwrap());
    let des = StreamDeserializer::new(&mut ins);
    let data:Vec<Bson> = des.collect();
    println!("{:#?}", data);
}