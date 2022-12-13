use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;


//takes vector input and turns it into an array (what we can use to loop through!)
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn serialize_to_bytes(data: u32) -> [u8; 4]
{
    let ans = data.to_be_bytes();  
    return ans;  
}


//serde2 functions
pub fn serialize_data_to_disk(data: Vec<i32>, filename: &str) -> Result<(), Error> 
{
    let mut file = File::create(filename)?;
    for index in &data
    {
        let ser = index.to_be_bytes();
        file.write_all(&ser);
    }
    Ok(())
}

pub fn deserialize_data_from_disk(filename: &str) -> Vec<i32> 
{
    //Refer to read_bytes_from_file code
    let f = File::open(filename).expect("could not open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector... also from read_bytes_from_file code
    reader.read_to_end(&mut buffer).expect("error while reading file");

    let mut ans = Vec::new();

    // Transform Vec into array... also from read_bytes_from_file code
    //let array = vec_to_array(buffer);


    let mut index = 0; 
    let mut index_end;
    let stopval = buffer.len();
    while index < (stopval)
    {
        index_end = index + 4;
        let push_to_arr = i32::from_be_bytes(vec_to_array(buffer[index..index_end].to_vec()));
        ans.push(push_to_arr);
        index = index + 4;
    }

    return ans;

}
