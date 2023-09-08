use opcua::types::Variant;

use crate::errors::Errors;

pub fn variant_to_i16(opc: &Option<Variant>) -> Result<i16, Errors> {
    let opc = match opc {
        Some(opc) => opc,
        None => {
            let msg = "Empty value".to_string();
            return Err(Errors::ConvertFromVariantError(msg));
        }
    };
    let value = match opc {
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
    };
    Ok(value)
}
