use crate::events::Event;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[doc = "Use the chrome.runtime API to retrieve the background page, return details about the manifest, and listen for and respond to events in the app or extension lifecycle. You can also use this API to convert the relative path of URLs to fully-qualified URLs."]
    #[doc = ""]
    #[doc = "@since Chrome 22"]
    pub type Runtime;

    #[wasm_bindgen(method, js_name = connect)]
    #[doc = "Attempts to connect to connect listeners within an extension/app (such as the background page), or other extensions/apps. This is useful for content scripts connecting to their extension processes, inter-app/extension communication, and web messaging. Note that this does not connect to any listeners in a content script. Extensions may connect to content scripts embedded in tabs via tabs.connect."]
    #[doc = ""]
    #[doc = "@since Chrome 26."]
    pub fn connect_with_connect_info(this: &Runtime, connect_info: JsValue) -> Port;
}

#[derive(Serialize, Deserialize)]
pub struct ConnectInfo<'a> {
    pub name: Option<&'a str>,
}

impl From<ConnectInfo<'_>> for JsValue {
    fn from(value: ConnectInfo) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[doc = "An object which allows two way communication with other pages."]
    #[doc = ""]
    #[doc = "@since Chrome 26."]
    pub type Port;

    #[wasm_bindgen(method, js_name = postMessage)]
    pub fn post_message(this: &Port, message: JsValue);

    #[wasm_bindgen(method, getter, js_name = onMessage)]
    #[doc = "An object which allows the addition and removal of listeners for a Chrome event."]
    pub fn on_message(this: &Port) -> Event;
}
