use std::env;
use std::io::{prelude::*, BufReader, Error};
use std::fs::File;

/******************************************************************************
 * Function:    xor_buffer()
 * Parameters:  buffer: &mut Vec<u8> - Data read from the file stored in a u8
 *                      vector
 * Description: To take a byte array and xor each byte with 0xff  
 * Return Val:  () - None
 ******************************************************************************
 */
fn xor_buffer(buffer: &mut Vec<u8>) -> () {
    let ff: u8 = 0xff;

    for i in buffer.iter_mut() {
        *i = *i ^ ff;
    }
}

/******************************************************************************
 * Function:    write_to_file()
 * Parameters:  f: & String         - file name string
 *              buffer: & Vec<u8>   - u8 Vector
 * Description: write_to_file takes a file name as a string and the xor'd u8
 *              vector and creates a file to write the data to
 * Return Val:  outFile - xor'd File
 ******************************************************************************
 */
fn write_to_file(f: & String, buffer: & Vec<u8>) -> Result<(), Error> {

    let mut outFile = File::create(f)
        .expect("Error creating output file");

    outFile.write_all(&buffer)?;

    Ok(())
}

/******************************************************************************
 * Function:    read_input_file()
 * Parameters:  f: & String
 * Description: To read the file given and store the contents in u8 vector
 * Return Val:  buffer - u8 vector
 ******************************************************************************
 */
fn read_input_file(f: & String) -> Vec<u8> {
    
    let mut buffer: Vec<u8> = vec![];

    let mut outputFile = File::open(f)
        .expect("Failed to open input file");

    outputFile.read_to_end(&mut buffer)
        .expect("failed to read file contents");

    buffer
}

fn main() {

    let mut inputFileName           = String::new();           // input file
    let mut xorFileName             = String::from("output/"); // result file
    let mut buffer: Vec<u8>         = vec![];                  // u8 vector
    let args: Vec<String>           = env::args().collect();   // cmd line args

    inputFileName = args[1].clone();            // File that is being read
    xorFileName.push_str(& args[2].clone());    // Target output file

    buffer = read_input_file(&inputFileName);   // Buffer of read file contents

    xor_buffer(&mut buffer);                    // Xor the buffer directly

    write_to_file(&xorFileName, &buffer);       // Write the result to target file
}
