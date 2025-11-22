use leptos::*;
use leptos_router::*;
use crate::api::{get_analytics, AnalyticsData};
use crate::components::charts::BarChart;

#[component]
pub fn Analytics() -> impl IntoView {
    let params = use_params_map();
    let code = move || params.with(|p| p.get("code").cloned().unwrap_or_default());

    let analytics_resource = create_resource(
        code,
        move |code| async move {
            if code.is_empty() {
                return Err("No code provided".to_string());
            }
            get_analytics(code).await
        }
    );

    view! {
        <div class="container">
            <header class="hero">
                <h1>"📊 Analytics"</h1>
                <p class="subtitle">"Stats for " <code>{code}</code></p>
                <a href="/" class="btn-outline-sm">"← Back to Home"</a>
            </header>

            <Suspense fallback=move || view! { <p>"Loading analytics..."</p> }>
                {move || match analytics_resource.get() {
                    Some(Ok(data)) => {
                        // Helper to convert JSON data to Vec<(String, f64)>
                        let to_chart_data = |json_vec: &Vec<serde_json::Value>, label_key: &str| {
                            json_vec.iter().map(|v| {
                                (
                                    v[label_key].as_str().unwrap_or("Unknown").to_string(),
                                    v["count"].as_f64().unwrap_or(0.0)
                                )
                            }).collect::<Vec<_>>()
                        };

                        let countries_data = to_chart_data(&data.countries, "country");
                        let devices_data = to_chart_data(&data.devices, "device_type");
                        let browsers_data = to_chart_data(&data.browsers, "browser");
                        let referrers_data = to_chart_data(&data.referrers, "referrer");

                        view! {
                            <div class="analytics-dashboard">
                                <div class="stat-card">
                                    <h3>"Total Clicks"</h3>
                                    <div class="big-number">{data.total_clicks}</div>
                                </div>
                                
                                <div class="charts-grid">
                                    <BarChart title="Top Countries".to_string() data=countries_data color="#3b82f6".to_string() />
                                    <BarChart title="Devices".to_string() data=devices_data color="#10b981".to_string() />
                                    <BarChart title="Browsers".to_string() data=browsers_data color="#f59e0b".to_string() />
                                    <BarChart title="Referrers".to_string() data=referrers_data color="#8b5cf6".to_string() />
                                </div>
                            </div>
                        }.into_view()
                    },
                    Some(Err(e)) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                    None => view! { <p>"Loading..."</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}
