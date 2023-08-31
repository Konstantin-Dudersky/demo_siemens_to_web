use opcua::types::Variant;

pub fn i16_to_variant(value: i16) -> Option<Variant> {
    let value = Variant::Int16(value);
    Some(value)
}
