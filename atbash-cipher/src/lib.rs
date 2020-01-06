/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut cipher:String = String::new();
    let mut count:u16 = 0;
    for ch in plain.to_lowercase().chars(){
        if ch >= 'a' && ch <= 'z'
        {
            if count == 5{
                cipher.push(' ');
                count=0;
            }
            cipher.push((('z' as u8)- (ch as u8 - 'a' as u8)) as char);
            count = count + 1;
        }
        else if ch >= '0' && ch <= '9'{
            cipher.push(ch);
            count = count+1;
            if count == 5{
                cipher.push(' ');
                count=0;
            }
        }
    }
    return cipher;
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut plain:String = String::new();
    for ch in cipher.chars(){
        if ch >= 'a' && ch <= 'z'
        {
            plain.push((('z' as u8)- (ch as u8 - 'a' as u8)) as char);
        }
        else if ch >= '0' && ch <= '9'{
            plain.push(ch);
        }
    }
    return plain;
}
