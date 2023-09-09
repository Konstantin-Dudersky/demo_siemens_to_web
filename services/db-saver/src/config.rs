use db_saver_lib::models::Row;
use messages::Messages;

/// Преобразование сообщений из Redis в строки для базы данных
pub fn prepare_msg_from_redis_to_db(msg: Messages) -> Option<Row> {
    let entity = msg.key();
    match &msg {
        // Command
        Messages::CommandStart(value) | Messages::CommandStop(value) => {
            Some(Row::new(value.ts, &entity, "", 1.0))
        }
        // SingleValue<i16>
        Messages::MotorState(value) => {
            Some(Row::new(value.ts, &entity, "", value.value as f64))
        }
        // SingleValue<f64>
        Messages::SetpointRead(value) | Messages::Temperature(value) => {
            Some(Row::new(value.ts, &entity, "", value.value))
        }
        // not archiving
        Messages::SetpointWrite(_) => None,
    }
}
