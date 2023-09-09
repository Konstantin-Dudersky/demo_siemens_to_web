use messages::Messages;

use crate::models;

/// Преобразование сообщений из Redis в строки для базы данных
pub fn prepare_msg_from_redis_to_db(msg: Messages) -> Option<models::Row> {
    let entity = msg.key();
    match &msg {
        // Command
        Messages::CommandStart(value) | Messages::CommandStop(value) => {
            Some(models::Row::new(value.ts, &entity, "", 1.0))
        }
        // SingleValue<i16>
        Messages::MotorState(value) => {
            Some(models::Row::new(value.ts, &entity, "", value.value as f64))
        }
        // SingleValue<f64>
        Messages::SetpointRead(value) => {
            Some(models::Row::new(value.ts, &entity, "", value.value))
        }
        // not archiving
        Messages::SetpointChange(_) => None,
    }
}
