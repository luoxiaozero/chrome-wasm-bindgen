use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Devtools;

    #[wasm_bindgen(method, getter, js_name = inspectedWindow)]
    pub fn inspected_window(this: &Devtools) -> InspectedWindow;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Object)]
    #[doc = "Use the chrome.devtools.inspectedWindow API to interact with the inspected window: obtain the tab ID for the inspected page, evaluate the code in the context of the inspected window, reload the page, or obtain the list of resources within the page."]
    #[doc = ""]
    #[doc = "Availability: Since Chrome 18."]
    pub type InspectedWindow;

    #[wasm_bindgen(method, getter, js_name = tabId)]
    #[doc = "The ID of the tab being inspected. This ID may be used with chrome.tabs.* API."]
    pub fn tab_id(this: &InspectedWindow) -> u32;
}
