/// "Encipher" with the Atbash cipher.
pub fn convert(text: &str, gap:usize) -> String{
    let mut cnvrt:String = String::new();
    text.to_lowercase().chars()
        .filter(|x| x.is_alphanumeric())
        .for_each(|ch| if ch.is_alphabetic(){ cnvrt.push((('z' as u8)- (ch as u8 - 'a' as u8)) as char)} else { cnvrt.push(ch)});
    if gap > 0{
        let length = cnvrt.len();
    for i in (gap..(length -1 + (length/gap) )).step_by(gap+1){
        cnvrt.insert(i, ' ');
    }
    }
    return cnvrt;
}


pub fn encode(plain: &str) -> String {
    convert(plain,5)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    convert(cipher,0)
}
