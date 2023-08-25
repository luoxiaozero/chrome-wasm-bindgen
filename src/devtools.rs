use serde::{Deserialize, Serialize};
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

    #[wasm_bindgen(method)]
    #[doc = "Reloads the inspected page."]
    pub fn reload(this: &InspectedWindow);

    #[wasm_bindgen(method, js_name = reload)]
    #[doc = "Reloads the inspected page."]
    pub fn reload_with_options(this: &InspectedWindow, options: JsValue);

    #[wasm_bindgen(method, js_name = eval)]
    #[doc = "Evaluates a JavaScript expression in the context of the main frame of the inspected page. The expression must evaluate to a JSON-compliant object, otherwise an exception is thrown. The eval function can report either a DevTools-side error or a JavaScript exception that occurs during evaluation. In either case, the result parameter of the callback is undefined. In the case of a DevTools-side error, the isException parameter is non-null and has isError set to true and code set to an error code. In the case of a JavaScript error, isException is set to true and value is set to the string value of thrown object."]
    #[doc = ""]
    #[doc = "@param expression An expression to evaluate."]
    pub fn eval_with_expression(this: &InspectedWindow, expression: &str);

    #[wasm_bindgen(method, js_name = eval)]
    #[doc = "Evaluates a JavaScript expression in the context of the main frame of the inspected page. The expression must evaluate to a JSON-compliant object, otherwise an exception is thrown. The eval function can report either a DevTools-side error or a JavaScript exception that occurs during evaluation. In either case, the result parameter of the callback is undefined. In the case of a DevTools-side error, the isException parameter is non-null and has isError set to true and code set to an error code. In the case of a JavaScript error, isException is set to true and value is set to the string value of thrown object."]
    #[doc = ""]
    #[doc = "@param expression An expression to evaluate."]
    #[doc = ""]
    #[doc = "@param callback A function called when evaluation completes."]
    #[doc = ""]
    #[doc = "Parameter result: The result of evaluation."]
    #[doc = ""]
    #[doc = "Parameter exceptionInfo: An object providing details if an exception occurred while evaluating the expression."]
    pub fn eval_with_expression_and_callback(this: &InspectedWindow, callback: js_sys::Function);

    #[wasm_bindgen(method, js_name = eval)]
    #[doc = "Evaluates a JavaScript expression in the context of the main frame of the inspected page. The expression must evaluate to a JSON-compliant object, otherwise an exception is thrown. The eval function can report either a DevTools-side error or a JavaScript exception that occurs during evaluation. In either case, the result parameter of the callback is undefined. In the case of a DevTools-side error, the isException parameter is non-null and has isError set to true and code set to an error code. In the case of a JavaScript error, isException is set to true and value is set to the string value of thrown object."]
    #[doc = ""]
    #[doc = "@param expression An expression to evaluate."]
    #[doc = ""]
    #[doc = "@param options The options parameter can contain one or more options."]
    pub fn eval_with_expression_and_options(this: &InspectedWindow, options: JsValue);

    #[wasm_bindgen(method, js_name = eval)]
    #[doc = "Evaluates a JavaScript expression in the context of the main frame of the inspected page. The expression must evaluate to a JSON-compliant object, otherwise an exception is thrown. The eval function can report either a DevTools-side error or a JavaScript exception that occurs during evaluation. In either case, the result parameter of the callback is undefined. In the case of a DevTools-side error, the isException parameter is non-null and has isError set to true and code set to an error code. In the case of a JavaScript error, isException is set to true and value is set to the string value of thrown object."]
    #[doc = ""]
    #[doc = "@param expression An expression to evaluate."]
    #[doc = ""]
    #[doc = "@param options The options parameter can contain one or more options."]
    #[doc = ""]
    #[doc = "@param callback A function called when evaluation completes."]
    #[doc = ""]
    #[doc = "Parameter result: The result of evaluation."]
    #[doc = ""]
    #[doc = "Parameter exceptionInfo: An object providing details if an exception occurred while evaluating the expression."]
    pub fn eval_with_expression_and_options_and_callback(this: &InspectedWindow, options: JsValue, callback: js_sys::Function);

    #[wasm_bindgen(method, js_name = getResources)]
    #[doc = "Retrieves the list of resources from the inspected page."]
    #[doc = ""]
    #[doc = "@param callback A function that receives the list of resources when the request completes."]
    pub fn get_resources(this: &InspectedWindow, callback: js_sys::Function);

    #[wasm_bindgen(method, setter, js_name = onResourceAdded)]
    #[doc = "Fired when a new resource is added to the inspected page."]
    pub fn on_resource_added(this: &InspectedWindow, value: js_sys::Function);

    #[wasm_bindgen(method, setter, js_name = onResourceContentCommitted)]
    #[doc = "Fired when a new revision of the resource is committed (e.g. user saves an edited version of the resource in the Developer Tools)."]
    pub fn on_resource_content_committed(this: &InspectedWindow, value: js_sys::Function);
}

#[derive(Serialize, Deserialize)]
struct ReloadOptions<'a> {
    #[serde(alias = "userAgent")]
    /// Optional. If specified, the string will override the value of the User-Agent HTTP header that's sent while loading the resources of the inspected page. The string will also override the value of the navigator.userAgent property that's returned to any scripts that are running within the inspected page.
    user_agent: Option<&'a str>,
    #[serde(alias = "ignoreCache")]
    /// Optional. When true, the loader will ignore the cache for all inspected page resources loaded before the load event is fired. The effect is similar to pressing Ctrl+Shift+R in the inspected window or within the Developer Tools window.
    ignore_cache: Option<bool>,
    #[serde(alias = "injectedScript")]
    /// Optional. If specified, the script will be injected into every frame of the inspected page immediately upon load, before any of the frame's scripts. The script will not be injected after subsequent reloads—for example, if the user presses Ctrl+R.
    injected_script: Option<&'a str>,
    #[serde(alias = "preprocessorScript")]
    /**
     * Optional.
     * If specified, this script evaluates into a function that accepts three string arguments: the source to preprocess, the URL of the source, and a function name if the source is an DOM event handler. The preprocessorerScript function should return a string to be compiled by Chrome in place of the input source. In the case that the source is a DOM event handler, the returned source must compile to a single JS function.
     * @deprecated Deprecated since Chrome 41. Please avoid using this parameter, it will be removed soon.
     */
    preprocessor_script: Option<&'a str>,
}

impl From<ReloadOptions<'_>> for JsValue {
    fn from(value: ReloadOptions) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct EvalOptions<'a> {
    #[serde(alias = "frameURL")]
    /// If specified, the expression is evaluated on the iframe whose URL matches the one specified. By default, the expression is evaluated in the top frame of the inspected page.
    frame_url: Option<&'a str>,
    #[serde(alias = "useContentScriptContext")]
    /// Evaluate the expression in the context of the content script of the calling extension, provided that the content script is already injected into the inspected page. If not, the expression is not evaluated and the callback is invoked with the exception parameter set to an object that has the isError field set to true and the code field set to E_NOTFOUND.
    use_content_script_context: Option<bool>,
    #[serde(alias = "contextSecurityOrigin")]
    /// Evaluate the expression in the context of a content script of an extension that matches the specified origin. If given, contextSecurityOrigin overrides the 'true' setting on userContentScriptContext.
    context_security_origin: Option<&'a str>,
}

impl From<EvalOptions<'_>> for JsValue {
    fn from(value: EvalOptions) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}