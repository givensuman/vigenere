use clap::Parser;
use rpassword::prompt_password;
use std::io::{Read, Write};
use std::{fs::File, io::stdin};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::new())]
    input: String,

    #[arg(short, long, default_value_t = String::new())]
    output: String,

    #[arg(short, long, default_value_t = false)]
    decrypt: bool,
}

//
// ,o.          8 8
// d  bzzzzzzzza8o8b
// `o'
fn main() {
    let args = Args::parse();

    let mut input = String::new();
    match args.input.is_empty() {
        true => {
            // No file input argument provided, read from stdin
            stdin().read_line(&mut input).unwrap();
        }
        false => {
            // File input argument provided, read from that
            let file = File::open(args.input);

            match file {
                Ok(mut f) => {
                    f.read_to_string(&mut input).unwrap();
                }
                Err(_) => {
                    println!("I could not read the file you provided.");
                    std::process::exit(1);
                }
            };
            // .unwrap()
            // .read_to_string(&mut input)
            // .unwrap();
        }
    };

    let key = prompt_password("🔑: ").unwrap();

    // Convert cipher key to iterator of alphabetic positions
    // where A = 0, B = 1,... Z = 25
    let mut key_as_i32 = key
        .trim()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase() as i32 - 'A' as i32)
        .cycle();

    let text_encrypted: String = input
        .trim()
        .chars()
        .map(|c| shift_letter(c, key_as_i32.next().unwrap(), args.decrypt))
        .collect();

    if args.output.is_empty() {
        // No file output argument provided, write to stdout
        println!("{}", text_encrypted);
    } else {
        // File output argument provided, write to that
        let mut output = File::create(args.output).unwrap();
        write!(output, "{}", text_encrypted).unwrap();
    }
}

/**
* Shift a character by a value equal to an alphabetic position
* where A + 1 = B, B + 2 = E, etc.
*
* Note values overflow, so Z + 1 wraps back to A
*
* @param c char - Character to shift
* @param amount i32 - Amount to shift by, should be in range of 0 to 25
* @param unshift bool - Whether to shift or unshift, e.g. encrypt or decrypt
*
* @return char
*/
fn shift_letter(c: char, amount: i32, unshift: bool) -> char {
    if !c.is_alphabetic() {
        return c;
    }

    let c_as_i32 = c.to_ascii_uppercase() as i32 - 'A' as i32;
    let mut shifted_c_as_i32: i32;

    match unshift {
        true => {
            shifted_c_as_i32 = c_as_i32 + amount;
            if shifted_c_as_i32 > 25 {
                shifted_c_as_i32 -= 26
            }
        }
        false => {
            shifted_c_as_i32 = c_as_i32 - amount;
            if shifted_c_as_i32 < 0 {
                shifted_c_as_i32 += 26
            }
        }
    };

    std::char::from_u32(u32::try_from(shifted_c_as_i32).unwrap() + 'A' as u32).unwrap()
}
