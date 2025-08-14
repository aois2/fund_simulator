use yew::prelude::*;
use crate::utils::chart::render_chart;

#[derive(Properties, PartialEq)]
pub struct GrowthChartProps {
    pub timeline: Vec<(u32, f64)>,
}

#[function_component(GrowthChart)]
pub fn growth_chart(props: &GrowthChartProps) -> Html {
    let timeline = props.timeline.clone();
    
    use_effect_with(timeline.clone(), move |timeline| {
        if !timeline.is_empty() {
            // Small delay to ensure canvas is rendered
            let timeline_clone = timeline.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(100).await;
                render_chart(&timeline_clone);
            });
        }
        || ()
    });

    html! {
        <div style="max-width: 800px; margin: 20px auto; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
            <h3 style="margin-bottom: 20px; color: #333; text-align: center;">{"Portfolio Growth Over Time"}</h3>
            <div style="position: relative; height: 400px; width: 100%;">
                <canvas id="growthChart" style="width: 100%; height: 100%;"></canvas>
            </div>
        </div>
    }
}