use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress6([u8; 6]);

impl Into<[u8; 6]> for MacAddress6 {
    fn into(self) -> [u8; 6] {
        self.0
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
        self.0.into_iter()
    }
}

impl Display for MacAddress6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
               self.0[0],
               self.0[1],
               self.0[2],
               self.0[3],
               self.0[4],
               self.0[5]
        )
    }
}

pub fn parse_multiple_strings(mac_addresses: Vec<String>) -> Result<Vec<MacAddress6>, String> {
    let mut mac_addresses_parsed: Vec<MacAddress6> = Vec::new();
    for mac_str in mac_addresses {
        mac_addresses_parsed.push(parse_from_string(mac_str)?)
    }
    Ok(mac_addresses_parsed)
}

fn parse_from_string(mac_str: String) -> Result<MacAddress6, String> {
    let split = mac_str.split(":");
    let mut result: [u8; 6] = [0; 6];
    if split.clone().count() != 6 {
        return Err("Invalid block count!")?;
    }
    for enumeration in split.enumerate() {
        let (index, block) = enumeration;
        if block.len() != 2 {
            return Err(format!("Wrong block size, in block: [{}]. Length needs to be 2!", block))?;
        }
        result[index] = u8::from_str_radix(block, 16)
            .map_err(|err | err.to_string())?
    }

    Ok(MacAddress6(result))
}