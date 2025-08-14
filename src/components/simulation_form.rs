use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
pub struct SimulationParams {
    pub initial_amount: f64,
    pub monthly_contribution: f64,
    pub annual_return_rate: f64,
    pub years: u32,
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            initial_amount: 10000.0,
            monthly_contribution: 500.0,
            annual_return_rate: 7.0,
            years: 10,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SimulationFormProps {
    pub on_submit: Callback<SimulationParams>,
}

#[function_component(SimulationForm)]
pub fn simulation_form(props: &SimulationFormProps) -> Html {
    let params = use_state(|| SimulationParams::default());

    let on_initial_change = {
        let params = params.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<f64>().unwrap_or(0.0);
            let mut new_params = (*params).clone();
            new_params.initial_amount = value;
            params.set(new_params);
        })
    };

    let on_monthly_change = {
        let params = params.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<f64>().unwrap_or(0.0);
            let mut new_params = (*params).clone();
            new_params.monthly_contribution = value;
            params.set(new_params);
        })
    };

    let on_rate_change = {
        let params = params.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<f64>().unwrap_or(0.0);
            let mut new_params = (*params).clone();
            new_params.annual_return_rate = value;
            params.set(new_params);
        })
    };

    let on_years_change = {
        let params = params.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<u32>().unwrap_or(0);
            let mut new_params = (*params).clone();
            new_params.years = value;
            params.set(new_params);
        })
    };

    let on_submit = {
        let params = params.clone();
        let callback = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            callback.emit((*params).clone());
        })
    };

    html! {
        <div style="max-width: 400px; margin: 20px auto; padding: 20px; border: 1px solid #ccc; border-radius: 8px;">
            <h2>{"Investment Simulation"}</h2>
            <form onsubmit={on_submit}>
                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                        {"Initial Amount ($)"}
                    </label>
                    <input
                        type="number"
                        step="0.01"
                        value={params.initial_amount.to_string()}
                        onchange={on_initial_change}
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>

                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                        {"Monthly Contribution ($)"}
                    </label>
                    <input
                        type="number"
                        step="0.01"
                        value={params.monthly_contribution.to_string()}
                        onchange={on_monthly_change}
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>

                <div style="margin-bottom: 15px;">
                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                        {"Annual Return Rate (%)"}
                    </label>
                    <input
                        type="number"
                        step="0.1"
                        value={params.annual_return_rate.to_string()}
                        onchange={on_rate_change}
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>

                <div style="margin-bottom: 20px;">
                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                        {"Investment Period (Years)"}
                    </label>
                    <input
                        type="number"
                        value={params.years.to_string()}
                        onchange={on_years_change}
                        style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>

                <button
                    type="submit"
                    style="width: 100%; padding: 12px; background-color: #007bff; color: white; border: none; border-radius: 4px; font-size: 16px; cursor: pointer;"
                >
                    {"Simulate"}
                </button>
            </form>
        </div>
    }
}