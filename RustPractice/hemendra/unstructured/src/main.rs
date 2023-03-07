
use std::collections::HashMap;

fn main() {
    let mut db = HashMap::new();
    let alice_data = r#"{"name": "Alice", "age": 30, "address": "123 Main St"}"#;
    db.insert("alice".to_string(), alice_data.to_string());

    let andy_data= r#"{"name": "Andy", "job": "Junior Electrical Engineer", "luckynumber":1}"#;
    db.insert("andy".to_string(),andy_data.to_string());

    let alice_data = db.get("alice").unwrap();
    let alice: HashMap<String, serde_json::Value> = serde_json::from_str(&alice_data).unwrap();
    println!("{:?}", alice);

    let andy_query= db.get("andy").unwrap();
    let andyresult:HashMap<String, serde_json::Value> = serde_json::from_str(&andy_query).unwrap();
    println!("{:?}",andyresult);
}
