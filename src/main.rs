/*
 * Program: Base64
 * Summary: A base64 encoding and decoding program written in the Rust language.
 *          This encoding and decoding method is specified in the following IETF
 *          documentation: https://datatracker.ietf.org/doc/html/rfc4648
 * Programmer: Sean B. Higgins
 * Date: April 14, 2024
 */

static B64ALPHABET: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
                                  'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                                  'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
                                  'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
                                  'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                                  'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                  'w', 'x', 'y', 'z', '0', '1', '2', '3',
                                  '4', '5' ,'6', '7', '8', '9', '+', '/'];

const MAX_COLS: u8 = 76;

const PAD_CHAR: char = '=';

fn b64encode() {
    let mut ac: u16 = 0;    // Accumulator to store the bits to encode into base64
    let mut ac_bits = 0;
    let mut byte_counter = 0;
    let mut wrap_counter = 0;

    
}
fn main() {
    // TODO: Redirect stdin to come from a file provided to argv[1]
    print!("B64ALPHABET =");

    for i in 0..B64ALPHABET.len()-1 {
        print!(" {}", B64ALPHABET[i]);
    }

    print!("\n");

    println!("MAX_COLS = {MAX_COLS}");
    println!("PAD_CHAR = {PAD_CHAR}");
}
