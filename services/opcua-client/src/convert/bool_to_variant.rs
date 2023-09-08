use opcua::types::Variant;

pub fn bool_to_variant(value: bool) -> Option<Variant> {
    let value = Variant::Boolean(value);
    Some(value)
}
