use rpassword::prompt_password;
use std::fs;

fn main() {
    let key = String::from("LEMON");
    let text: String = String::from("THISISTEXT");

    let mut key_as_i32 = key
        .chars()
        .map(|c| c.to_ascii_uppercase() as i32 - 'A' as i32)
        .cycle();

    let text_encrypted: String = text
        .chars()
        .map(|c| shift_letter(c, key_as_i32.next().unwrap(), false))
        .collect();
    println!("{:?}", text_encrypted);
}

fn shift_letter(c: char, amount: i32, unshift: bool) -> char {
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
