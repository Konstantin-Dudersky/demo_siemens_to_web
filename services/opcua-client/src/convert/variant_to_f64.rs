use opcua::types::Variant;

use crate::errors::Errors;

pub fn variant_to_f64(opc: &Option<Variant>) -> Result<f64, Errors> {
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
        Variant::Int16(_) => todo!(),
        Variant::UInt16(_) => todo!(),
        Variant::Int32(_) => todo!(),
        Variant::UInt32(_) => todo!(),
        Variant::Int64(_) => todo!(),
        Variant::UInt64(_) => todo!(),
        Variant::Float(value) => *value as f64,
        Variant::Double(value) => *value,
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
