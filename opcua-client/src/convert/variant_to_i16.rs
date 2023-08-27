use opcua::types::Variant;

pub fn variant_to_i16(opc: &Variant) -> i16 {
    match opc {
        Variant::Empty => todo!(),
        Variant::Boolean(_) => todo!(),
        Variant::SByte(_) => todo!(),
        Variant::Byte(_) => todo!(),
        Variant::Int16(value) => *value,
        Variant::UInt16(value) => *value as i16,
        Variant::Int32(_) => todo!(),
        Variant::UInt32(_) => todo!(),
        Variant::Int64(_) => todo!(),
        Variant::UInt64(_) => todo!(),
        Variant::Float(_) => todo!(),
        Variant::Double(_) => todo!(),
        Variant::String(_) => todo!(),
        Variant::DateTime(_) => todo!(),
        Variant::Guid(_) => todo!(),
        Variant::StatusCode(_) => todo!(),
        Variant::ByteString(_) => todo!(),
        Variant::XmlElement(_) => todo!(),
        Variant::QualifiedName(_) => todo!(),
        Variant::LocalizedText(_) => todo!(),
        Variant::NodeId(_) => todo!(),
        Variant::ExpandedNodeId(_) => todo!(),
        Variant::ExtensionObject(_) => todo!(),
        Variant::Variant(_) => todo!(),
        Variant::DataValue(_) => todo!(),
        Variant::Diagnostics(_) => todo!(),
        Variant::Array(_) => todo!(),
    }
}
