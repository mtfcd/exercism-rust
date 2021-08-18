use std::collections::BTreeMap;

pub fn raindrops(n: u32) -> String {
    let mut map = BTreeMap::new();
    map.insert(3, "Pling");
    map.insert(5, "Plang");
    map.insert(7, "Plong");

    let mut s = Vec::<String>::new();
    for (k, v) in map {
        if n % k == 0 {
            s.push(v.into())
        }
    }
    if s.len() == 0 {
        return n.to_string();
    }

    return s.join("");
}
