use opcua::types::Variant;

pub fn f64_to_variant(value: f64) -> Option<Variant> {
    let value = Variant::Double(value);
    Some(value)
}
