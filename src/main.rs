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
