use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Array;

#[wasm_bindgen(module = "/src/chart.js")]
extern "C" {
    pub fn draw_chart(labels: JsValue, data: JsValue);
}

pub fn render_chart(timeline: &Vec<(u32, f64)>) {
    let labels = Array::new();
    let data = Array::new();

    for (month, value) in timeline {
        labels.push(&JsValue::from_str(&format!("Month {}", month)));
        data.push(&JsValue::from_f64(*value));
    }

    draw_chart(labels.into(), data.into());
}