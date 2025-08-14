use yew::prelude::*;
use web_sys::console;
use wasm_bindgen::JsValue;

mod components;
mod utils;

use components::simulation_form::{SimulationForm, SimulationParams};
use components::results_summary::ResultsSummary;
use components::growth_chart::GrowthChart;
use utils::calculator::run_simulation;

#[function_component(App)]
fn app() -> Html {
    let simulation_params = use_state(|| Option::<SimulationParams>::None);
    let simulation_results = use_state(|| Option::<Vec<(u32, f64)>>::None);

    let on_form_submit = {
        let simulation_params = simulation_params.clone();
        let simulation_results = simulation_results.clone();
        Callback::from(move |params: SimulationParams| {
            // Log the parameters to browser console
            console::log_1(&JsValue::from_str(&format!(
                "Simulation Parameters: Initial: ${:.2}, Monthly: ${:.2}, Rate: {:.1}%, Years: {}",
                params.initial_amount,
                params.monthly_contribution,
                params.annual_return_rate,
                params.years
            )));
            
            // Run the simulation
            let results = run_simulation(
                params.initial_amount,
                params.monthly_contribution,
                params.annual_return_rate,
                params.years
            );
            
            simulation_params.set(Some(params));
            simulation_results.set(Some(results));
        })
    };

    html! {
        <div style="min-height: 100vh; background-color: #f5f5f5; padding: 20px;">
            <div style="max-width: 1200px; margin: 0 auto;">
                <h1 style="text-align: center; color: #333; margin-bottom: 30px;">
                    {"Fund Investment Simulator"}
                </h1>
                
                <SimulationForm on_submit={on_form_submit} />
                
                {
                    if let (Some(params), Some(results)) = (simulation_params.as_ref(), simulation_results.as_ref()) {
                        html! {
                            <>
                                <ResultsSummary params={params.clone()} timeline={results.clone()} />
                                
                                <GrowthChart timeline={results.clone()} />
                                
                                <div style="max-width: 800px; margin: 20px auto; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                                    <h3>{"Monthly Portfolio Values:"}</h3>
                                    <div style="max-height: 400px; overflow-y: auto; border: 1px solid #eee; border-radius: 4px;">
                                        <ul style="list-style: none; padding: 0; margin: 0;">
                                            {
                                                results.iter().enumerate().map(|(i, (month, value))| {
                                                    let bg_color = if i % 2 == 0 { "#f9f9f9" } else { "white" };
                                                    html! {
                                                        <li key={month.to_string()} style={format!("padding: 8px 16px; background-color: {}; border-bottom: 1px solid #eee;", bg_color)}>
                                                            {format!("Month {}: ${:.2}", month, value)}
                                                        </li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! { <></> }
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}