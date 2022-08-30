use std::collections::HashMap;

struct Test<K, V> {
    map: std::collections::HashMap<K, V>,
}

impl Test<u32, u32> {
    fn new() -> Test<u32, u32> {
        Test {
            map: HashMap::new(),
        }
    }
}

fn main() {
    let mut mapping = Test::new();
    mapping.map.insert(3u32, 6u32);

    println!("Key is {}, and Value is {:?}", 3u32, mapping.map.get(&3u32));
}
