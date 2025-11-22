use leptos::*;

#[component]
pub fn BarChart(
    title: String,
    data: Vec<(String, f64)>,
    color: String,
) -> impl IntoView {
    let max_val = data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    
    view! {
        <div class="chart-card">
            <h3>{title}</h3>
            <div class="bar-chart-container">
                {data.into_iter().map(|(label, value)| {
                    let width_percent = if max_val > 0.0 { (value / max_val) * 100.0 } else { 0.0 };
                    view! {
                        <div class="bar-row">
                            <div class="bar-label">{label}</div>
                            <div class="bar-track">
                                <div 
                                    class="bar-fill" 
                                    style=format!("width: {}%; background-color: {};", width_percent, color)
                                >
                                    <span class="bar-value">{value}</span>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
