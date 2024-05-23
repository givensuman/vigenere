use rpassword::prompt_password;
use std::fs;

/**
                            :
                           :::
    '::                   ::::
    '::::.     .....:::.:::::::
    '::::::::::::::::::::::::::::
    ::::::XUWWWWWU:::::XW$$$$$$WX:
    ::::X$$$$$$$$$$W::X$$$$$$$$$$Wh
   ::::t$$$$$$$$$$$$W:$$$$$$P*$$$$M::
   :::X$$$$$$""""$$$$X$$$$$   ^$$$$X:::
  ::::M$$$$$$    ^$$$RM$$$L    <$$$X::::
.:::::M$$$$$$     $$$R:$$$$.   d$$R:::`
'~::::::?$$$$$$...d$$$X$6R$$$$$$$$RXW$X:'`
 '~:WNWUXT#$$$$$$$$TU$$$$W6IBBIW@$$RX:
*/
fn main() {
    let password = prompt_password("Your cipher key: ").unwrap();
    println!("Your cipher key: {}", password);

    let filename = std::env::args().nth(1).unwrap();

    let contents = fs::read_to_string(filename).unwrap();

    let mut password_by_alphabetic_position = password
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase() as u32 - 'a' as u32)
        .cycle();

    let shifted_contents: String = contents
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let num = c.to_ascii_lowercase() as u32 - 'a' as u32;
            let caesar_shift = (num + password_by_alphabetic_position.next().unwrap() % 26) % 26;

            std::char::from_u32(caesar_shift + 'a' as u32).unwrap()
        })
        .collect();

    println!("{:?}", shifted_contents);
}

/**
fn main() {
    let key = String::from("LEMON");
    let text: String = String::from("THISISTEXT");

    let mut key_as_u32 = key.chars().map(|c| c.to_ascii_uppercase() as u32 - 'A' as u32).cycle();
    
    let text_encrypted: String = text.chars().map(|c| shift_letter(c, key_as_u32.next().unwrap())).collect();
    println!("{:?}", text_encrypted);

    let mut unkey_as_u32 = key.chars().map(|c| c.to_ascii_uppercase() as u32 - 'A' as u32).cycle();

    let text_unencrypted: String = text_encrypted.chars().map(|c| unshift_letter(c, unkey_as_u32.next().unwrap())).collect();
    println!("{:?}", text_unencrypted);
}

fn shift_letter(c: char, amount: u32) -> char {
   let c_as_u32 = c.to_ascii_uppercase() as u32 - 'A' as u32;
   let mut shifted_c_as_u32: u32 = c_as_u32 + amount;
   if shifted_c_as_u32 > 25 {
    shifted_c_as_u32 -= 26
   }

   return std::char::from_u32(shifted_c_as_u32 + 'A' as u32).unwrap();
}

fn unshift_letter(c: char, amount: u32) -> char {
    let c_as_i32: i32 = c.to_ascii_uppercase() as i32 - 'A' as i32;
    let mut shifted_c_as_i32: i32 = c_as_i32 - i32::try_from(amount).unwrap();
    if shifted_c_as_i32 < 0 {
        shifted_c_as_i32 += 26
    }

    return std::char::from_u32(u32::try_from(shifted_c_as_i32).unwrap() + 'A' as u32).unwrap();
}
*/
