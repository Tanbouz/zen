use serde_json::Value;
use wasm_bindgen::prelude::*;
use zen_engine::model::DecisionContent;
use zen_engine::Decision;
use zen_engine::{EvaluationSerializedOptions, EvaluationTraceKind};

#[wasm_bindgen]
pub struct ZenEngine {
    decision: Decision,
}

#[wasm_bindgen]
impl ZenEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(content: JsValue) -> Result<ZenEngine, JsValue> {
        console_error_panic_hook::set_once();

        // Deserialize content from JS object to serde_json::Value first
        let val: Value = serde_wasm_bindgen::from_value(content)
            .map_err(|e| JsValue::from_str(&format!("JS Conversion Error: {}", e)))?;

        // Then deserialize to DecisionContent
        let content: DecisionContent = serde_json::from_value(val)
            .map_err(|e| JsValue::from_str(&format!("Decision Content Error: {}", e)))?;

        let decision = Decision::from(content);

        Ok(ZenEngine { decision })
    }

    pub async fn evaluate(&self, context: JsValue) -> Result<JsValue, JsValue> {
        let context_val: Value = serde_wasm_bindgen::from_value(context)?;

        // Extract trace option if present
        let trace = context_val
            .get("trace")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let options = EvaluationSerializedOptions {
            trace: if trace {
                EvaluationTraceKind::Default
            } else {
                EvaluationTraceKind::None
            },
            max_depth: 5,
        };

        // Use evaluate_serialized which properly converts Variable to Value
        let result = self
            .decision
            .evaluate_serialized(context_val.into(), options)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert serde_json::Value to string then parse as JS object
        // This is more reliable than serde_wasm_bindgen for complex nested structures
        let json_string = result.to_string();
        js_sys::JSON::parse(&json_string)
            .map_err(|e| JsValue::from_str(&format!("JSON parse error: {:?}", e)))
    }
}
