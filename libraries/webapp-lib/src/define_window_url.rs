//! Определить URL-адрес, введенный в браузере

use std::str::FromStr;

use gloo::console::info;
use url::Url;
use web_sys::window;

use crate::Errors;

pub fn define_window_url() -> Result<Url, Errors> {
    let window = match window() {
        Some(value) => value,
        None => {
            let msg = "Window is None".to_string();
            return Err(Errors::WindowLocationError(msg));
        }
    };
    let href = match window.location().href() {
        Ok(value) => value,
        Err(error) => {
            let msg = format!("{:?}", error);
            return Err(Errors::WindowLocationError(msg));
        }
    };
    let url = match Url::from_str(&href) {
        Ok(value) => value,
        Err(error) => {
            let msg = error.to_string();
            return Err(Errors::WindowLocationError(msg));
        }
    };
    let url_str = format!("{:?}", url);
    info!("Window location: ", url_str);
    Ok(url)
}
