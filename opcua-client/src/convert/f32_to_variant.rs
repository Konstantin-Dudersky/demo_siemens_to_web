use opcua::types::Variant;

pub fn f32_to_variant(value: f32) -> Option<Variant> {
    let value = Variant::Float(value);
    Some(value)
}
