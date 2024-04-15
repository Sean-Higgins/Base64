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

    while () {
	// Update byte count, mod 3
	byte_count += 1;
	if byte_count == 3 {
	    byte_count = 0;
	}

	// TODO: Find the Rust equivalent of getchar() in C, as well as bit
	//	 shifting and bit operations in Rust.

	let byte: u8 = getchar();   // Gather the next 8 bits to put into the
				    // accumulator.
	ac <<= 8;		    // Shift the current contents of ac over to the
	ac |= byte;		    // right by 8 bits to allow for the byte to be
				    // bitwise-ORed into ac.
	ac_bits += 8;		    // Increment the count of bits to be encoded

	while ac_bits >= 6 {
	    ac_bits -= 6;   // Subtract 6 bits that we will be reading
	    let index = (ac >> ac_bits) & 0x3F;	// Shift to the next 6 bit to read,
						// cutting off the top 2 bits that we
						// don't need.
	    let c = B64ALPHABET[index];	// Convert the index to the Base64 character.
	    putchar(c);			// Print the converted Base64 to stdout.
	    wrap_counter += 1;

	    // Now to handle line wrapping. If the number of characters printed out
	    // reaches MAX_COLS, we print a newline character to move onto a new
	    // line.
	    if wrap_counter >= MAX_COLS {
		putchar('\n');
		wrap_counter = 0;   // Reset the wrap_counter for the next line.
	    }
	}
    }    
}

/*
fn b64decode() {
    let mut ac: u16 = 0;    // Accumulator to store the bits to decode from Base64.
    let mut ac_bits = 0;

    for (;;) {
	
    }
}
*/


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
