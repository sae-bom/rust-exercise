struct Item {
    type_: String,
    color: String,
    name: String,
}

impl Item {
    fn new(type_: String, color: String, name: String) -> Self {
        Self { type_, color, name }
    }
}

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    items
        .into_iter()
        .map(|i| {
            let [type_, color, name] = i.try_into().expect("Each item should have exactly 3 fields");
            Item::new(type_, color, name)
        })
        .filter(|i| match rule_key.as_str() {
            "type" => &i.type_,
            "color" => &i.color,
            "name" => &i.name,
            _ => panic!("rule_key should be one of three above")
        } == rule_value.as_str())
        .count()
        .try_into().expect("The number of items is within i32 range")
}
