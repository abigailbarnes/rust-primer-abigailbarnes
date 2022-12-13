use serde::{Serialize, Deserialize};
use std::fs::File;


#[derive(Debug, Serialize, Deserialize)]
pub struct University 
{
    pub name: String,
    pub undergraduate_enrollment: u64,
    pub graduate_enrollment: u64,
    pub schools: Vec<String>,
    pub acceptance_rate: f64,
}
//string -> json
pub fn serialize_string_to_json(string_data: &str) -> University 
{
    let serialized: University = serde_json::from_str(&string_data).unwrap();
    return serialized;
}

pub fn deserialize_string_from_json(json_data: &University) -> String 
{
    let deserialized = serde_json::to_string(&json_data).unwrap();
    return deserialized;
}

//https://docs.rs/serde_cbor/0.9.0/serde_cbor/fn.to_writer.html
pub fn serialize_json_to_cbor(json_data: &University, filename: &str) 
{
    let file = File::create(&filename).unwrap();
    serde_cbor::to_writer(file, &json_data).unwrap();
}

//https://docs.rs/serde_cbor/0.9.0/serde_cbor/fn.from_reader.html
pub fn deserialize_json_from_cbor(filename: &str) -> University 
{
    let file = File::open(&filename).unwrap();
    let ans: University = serde_cbor::from_reader(file).unwrap();
    return ans;
}

