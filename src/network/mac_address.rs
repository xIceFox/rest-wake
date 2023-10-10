#[derive(Debug, Clone)]
pub struct MacAddress6 {
    pub mac: [u8; 6],
}

impl Into<[u8; 6]> for MacAddress6 {
    fn into(self) -> [u8; 6] {
        self.mac
    }
}

impl TryFrom<String> for MacAddress6 {
    type Error = String;

    fn try_from(mac_str: String) -> Result<MacAddress6, Self::Error> {
        parse_from_string(mac_str.clone()).map_err(|err| format!("There was an error during parsing of provided mac address [{}], following error message was thrown:\n\t{}", mac_str, err))
    }
}

impl IntoIterator for MacAddress6 {
    type Item = u8;
    type IntoIter = std::array::IntoIter<Self::Item, 6>;

    fn into_iter(self) -> Self::IntoIter {
        self.mac.into_iter()
    }
}

fn parse_from_string(mac_str: String) -> Result<MacAddress6, String> {
    let split = mac_str.split(":");
    let mut result: [u8; 6] = [0; 6];
    if split.clone().count() != 6 {
        return Err("Invalid block count!")?;
    }
    for enumeration in split.enumerate() {
        result[enumeration.0] = hex_pair_to_byte(enumeration.1)?;
    }

    Ok(MacAddress6 { mac: result })
}

fn hex_pair_to_byte(hex_repr: &str) -> Result<u8, String> {
    if hex_repr.len() != 2 {
        return Err(format!("Wrong block size, in block: [{}]. Length needs to be 2!", hex_repr))?;
    }
    let mut chars = hex_repr.chars();

    fn print_parse_error(ch: char) -> String {
        return format!("Found invalid hex value during parse: '{}'", ch);
    }

    let ch1 = chars.next().unwrap();
    let v1 = ch1
        .clone()
        .to_digit(16)
        .ok_or(print_parse_error(ch1))?;

    let ch2 = chars.next().unwrap();
    let v2 = ch2
        .clone()
        .to_digit(16)
        .ok_or(print_parse_error(ch2))?;

    Ok(((v1 << 4) + v2) as u8)
}