use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut map: HashMap<String,u32> = HashMap::new();
    map.insert(address,amount);
    return map;
}
