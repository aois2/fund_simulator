use yew::prelude::*;
use crate::components::simulation_form::SimulationParams;

#[derive(Properties, PartialEq)]
pub struct ResultsSummaryProps {
    pub params: SimulationParams,
    pub timeline: Vec<(u32, f64)>,
}

#[function_component(ResultsSummary)]
pub fn results_summary(props: &ResultsSummaryProps) -> Html {
    let total_invested = props.params.initial_amount + 
        props.params.monthly_contribution * (props.params.years * 12) as f64;
    
    let final_value = props.timeline.last().unwrap().1;
    let gain_percentage = (final_value / total_invested - 1.0) * 100.0;
    let total_gain = final_value - total_invested;

    html! {
        <div style="max-width: 800px; margin: 20px auto; padding: 20px; background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
            <h3 style="margin-bottom: 20px; color: #333; text-align: center;">{"Investment Summary"}</h3>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px;">
                // Total Invested Card
                <div style="padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border-radius: 8px; text-align: center;">
                    <div style="font-size: 14px; opacity: 0.9; margin-bottom: 5px;">{"Total Invested"}</div>
                    <div style="font-size: 24px; font-weight: bold;">{format!("${:.2}", total_invested)}</div>
                </div>

                // Final Value Card
                <div style="padding: 20px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); color: white; border-radius: 8px; text-align: center;">
                    <div style="font-size: 14px; opacity: 0.9; margin-bottom: 5px;">{"Final Value"}</div>
                    <div style="font-size: 24px; font-weight: bold;">{format!("${:.2}", final_value)}</div>
                </div>

                // Total Gain Card
                <div style={format!("padding: 20px; background: linear-gradient(135deg, {} 0%, {} 100%); color: white; border-radius: 8px; text-align: center;", 
                    if total_gain >= 0.0 { "#4facfe" } else { "#f093fb" },
                    if total_gain >= 0.0 { "#00f2fe" } else { "#f5576c" }
                )}>
                    <div style="font-size: 14px; opacity: 0.9; margin-bottom: 5px;">{"Total Gain"}</div>
                    <div style="font-size: 24px; font-weight: bold;">{format!("${:.2}", total_gain)}</div>
                </div>

                // Gain Percentage Card
                <div style={format!("padding: 20px; background: linear-gradient(135deg, {} 0%, {} 100%); color: white; border-radius: 8px; text-align: center;", 
                    if gain_percentage >= 0.0 { "#43e97b" } else { "#fa709a" },
                    if gain_percentage >= 0.0 { "#38f9d7" } else { "#fee140" }
                )}>
                    <div style="font-size: 14px; opacity: 0.9; margin-bottom: 5px;">{"Gain Percentage"}</div>
                    <div style="font-size: 24px; font-weight: bold;">{format!("{:.1}%", gain_percentage)}</div>
                </div>
            </div>

            // Additional Summary Information
            <div style="margin-top: 20px; padding: 15px; background: #f8f9fa; border-radius: 6px; border-left: 4px solid #007bff;">
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; font-size: 14px;">
                    <div><strong>{"Investment Period:"}</strong> {format!("{} years ({} months)", props.params.years, props.params.years * 12)}</div>
                    <div><strong>{"Monthly Contribution:"}</strong> {format!("${:.2}", props.params.monthly_contribution)}</div>
                    <div><strong>{"Annual Return Rate:"}</strong> {format!("{:.1}%", props.params.annual_return_rate)}</div>
                    <div><strong>{"Average Monthly Growth:"}</strong> {format!("${:.2}", total_gain / (props.params.years * 12) as f64)}</div>
                </div>
            </div>
        </div>
    }
}