#![allow(unused)]
use std::fs::File;
use std::io::*;
use std::path::Path;

mod data;
mod lang;
mod rsa;
mod vigenre;

fn main() -> Result<()> {
    println!("Run challenge: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match buffer.trim().parse::<usize>().expect("Expects a number") {
        0 => ch0()?,
        1 => ch1()?,
        2 => ch2()?,
        3 => ch3()?,
        4 => ch4()?,
        5 => ch5()?,
        6 => ch6()?,
        7 => ch7()?,
        _ => println!("Not a challenge"),
    }
    
    Ok(())
}

fn ch0() -> Result<()> {
    let encrypted = read_file("0-Zatanna.txt")?;
    encrypted
        .split(" ")
        .map(|word| word.chars().rev().collect::<String>())
        .for_each(|s| print!("{} ", s));
    Ok(())
}

fn ch1() -> Result<()> {
    let encrypted = read_file("1-YouWillEitherSolveThisOrYouWillNot.txt")?;
    for ch in encrypted.chars() {
        print!("{}", ch);
    }
    let res = encrypted
        .chars()
        .scan(String::new(), |state, ch| {
            if state.len() >= 8 {
                state.push(' ');
                let tmp = state.clone();
                *state = ch.to_string();
                Some(bin_to_num(tmp))
            } else {
                state.push(ch);
                Some(0u8)
            }
        })
        .filter(|s| *s > 0u8)
        .map(|s| s as char)
        .collect::<String>();
    println!("{:?}", res);
    Ok(())
}

fn ch2() -> Result<()> {
    let encrypted = read_file("2-0123456789ABCDEF.txt")?;
    let data = data::Bytes::read_hex(&encrypted);
    println!("{}", data);
    Ok(())
}

fn ch3() -> Result<()> {
    let encrypted = read_file("3-VediVidiVici.txt")?;
    println!(
        "{:?}",
        vigenre::decrypt_single_letter(
            &encrypted
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<String>(),
            &lang::histogram_score
        )
    );
    Ok(())
}

fn ch4() -> Result<()> {
    let encrypted = read_file("4-NovumOrganum.txt")?;
    println!(
        "{}",
        encrypted
            .split(' ')
            .map(|s| baconian_cipher(s.trim()))
            .collect::<String>()
    );
    Ok(())
}

fn ch5() -> Result<()> {
    let encrypted = read_file("5-Cryptoquote.txt")?;
    println!("{}", encrypted);
    println!("{}", lang::mono(&encrypted, "VCXMLYANKWFXDUBTOJHIRPGESZ"));
    Ok(())
}

fn ch6() -> Result<()> {
    //let encrypted = "WKHVTOHYEUARURRVGRSEGVTISIBHOPRZNTZLSXEESEBWOEZSBKHRHSFUEECMHYFEGRQVCNTFSSRVHIRJEAJEHYOHYEBUPBARRJ";
    let encrypted = read_file("6-Vigenere.txt")?;
    //let key = "ORANGE";
    //let key = "BEEMOVIE";

    //println!(
        //"`{}`",
        //&encrypted.chars().skip(0).step_by(8).collect::<String>()[0..10]
    //);
    //println!("{}", vigenre::decrypt(key, &encrypted));
    let key = vigenre::break_cipher(8, &encrypted);
    println!("{}", key);
    println!("{}", vigenre::decrypt(&key, &encrypted));
    //for (i, part) in parts.into_iter().enumerate() {
        //let score = lang::histogram_score(
            //&encrypted
                //.chars()
                //.skip(i)
                //.step_by(8).filter(char::is_ascii_alphabetic)
                //.map(|c| {
                    //vigenre::rot(c, part[0] as u8 - 'A' as u8)
                //})
                //.collect::<String>(),
        //);
        //print!("{:?} ", part);
        //print!(
            //"`{}` => ",
            //&encrypted.chars().skip(i).step_by(8).collect::<String>()[0..10]
        //);
        //print!(
            //"`{}`, {:05.2}",
            //&encrypted
                //.chars()
                //.skip(i)
                //.step_by(8).filter(char::is_ascii_alphabetic)
                //.map(|c| vigenre::rot(c, part[0] as u8 - 'A' as u8))
                //.collect::<String>()[0..10], score
        //);
    //}
    //for i in 3..20 {
    //println!("{}: {}", i, vigenre::ind_coin(&encrypted, i));
    //}
    //let raw = data::Bytes::from_bytes(encrypted.as_bytes());
    //println!("size: {}", raw.len());
    //for block in raw.split(8) {
    ////println!("{}", block);
    //}
    Ok(())
}

fn ch7() -> Result<()> {
    let rsa = rsa::RSAKey::private_key(89711, 0, 1025);
    println!("{}", rsa.decrypt_secret(66832));
    let rsa = rsa::RSAKey::key_gen(283, 317);
    println!("{:?}", rsa);
    Ok(())
}

fn baconian_cipher(letter: &str) -> char {
    match letter {
        "AAAAA" => 'A',
        "ABAAA" => 'I',
        "BAAAA" => 'R',
        "AAAAB" => 'B',
        "ABAAB" => 'K',
        "BAAAB" => 'S',
        "AAABA" => 'C',
        "ABABA" => 'L',
        "BAABA" => 'T',
        "AAABB" => 'D',
        "ABABB" => 'M',
        "BAABB" => 'U',
        "AABAA" => 'E',
        "ABBAA" => 'N',
        "BABAA" => 'W',
        "AABAB" => 'F',
        "ABBAB" => 'O',
        "BABAB" => 'X',
        "AABBA" => 'G',
        "ABBBA" => 'P',
        "BABBA" => 'Y',
        "AABBB" => 'H',
        "ABBBB" => 'Q',
        "BABBB" => 'Z',
        _ => panic!("{} is not a valid baconion cipher", letter),
    }
}

fn bin_to_num(s: String) -> u8 {
    let mut n = 0u8;
    for ch in s.chars() {
        n = n << 1;
        n = n | if ch == '1' { 1 } else { 0 };
    }
    n >> 1
}

fn read_file(f: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(f)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}
