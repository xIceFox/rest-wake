use std::error::Error;
#[derive(Debug)]
pub struct MacAddress6 {
    pub _mac: [u8; 6],
}

impl TryFrom<String> for MacAddress6 {
    type Error = Box<dyn std::error::Error>;

    fn try_from(mac_str: String) -> Result<MacAddress6, Self::Error> {
        let split = mac_str.split(":");
        let mut collector: [u8; 6] = [0; 6];
        if split.clone().count() != 6 {
            return Err("Invalid block count!")?;
        }
        for enumerator in split.enumerate() {
            collector[enumerator.0] = hex_string_to_byte(enumerator.1)?;
        }

        Ok(MacAddress6 { _mac: collector })
    }
}

fn hex_string_to_byte(hex_repr: &str) -> Result<u8, Box<dyn Error>> {
    if hex_repr.len() != 2 {
        return Err(format!("Wrong block size, in block: [{}]. Length needs to be 2!", hex_repr))?;
    }
    let mut chars = hex_repr.chars();
    let v1 = hex_char_to_decimal(chars.next().unwrap())? << 4;
    let v2 = hex_char_to_decimal(chars.next().unwrap())?;
    Ok(v1 + v2)
}

fn hex_char_to_decimal(ch: char) -> Result<u8, Box<dyn Error>> {
    let ch_upper = ch.to_ascii_uppercase();
    let utf32_value = ch_upper as u32;
    if utf32_value >= ('0' as u32) && utf32_value <= ('9' as u32) {
        return Ok((ch_upper as u8) - ('0' as u8));
    } else if utf32_value >= ('A' as u32) && utf32_value <= ('F' as u32) {
        return Ok((ch_upper as u8) - ('A' as u8) + 10);
    }
    Err(format!("Found invalid hex value during parse: '{}'!", ch))?
}