use leptos::*;
use crate::api::{get_user_urls, UrlInfo};

#[component]
pub fn Dashboard(refresh_signal: ReadSignal<i32>) -> impl IntoView {
    // TODO: Get real user ID
    let user_id = "anonymous".to_string(); 
    
    let urls_resource = create_resource(
        move || refresh_signal.get(),
        move |_| {
            let uid = user_id.clone();
            async move { get_user_urls(uid).await }
        }
    );

    view! {
        <div id="dashboard" class="dashboard-section" style="display: block;">
            <div class="dashboard-header">
                <h2>"📊 Your Links"</h2>
                // Refresh button logic handled by parent or auto-refresh
            </div>

            <div class="links-table-container">
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    {move || match urls_resource.get() {
                        Some(Ok(urls)) => {
                            if urls.is_empty() {
                                view! {
                                    <div class="empty-state" style="display: block;">
                                        <p>"You haven't created any links yet. Create your first one above! 👆"</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <table class="links-table">
                                        <thead>
                                            <tr>
                                                <th>"Short"</th>
                                                <th>"Original"</th>
                                                <th>"Clicks"</th>
                                                <th>"Date"</th>
                                                <th>"Actions"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {urls.into_iter().map(|url| {
                                                let short_code = url.short_code.clone();
                                                let short_code_for_analytics = url.short_code.clone();
                                                let short_link = format!("s.blatik-short.workers.dev/{}", url.short_code); 
                                                let short_link_for_copy = short_link.clone();
                                                
                                                view! {
                                                    <tr>
                                                        <td><a href=format!("https://{}", short_link) target="_blank" class="short-link">{short_code}</a></td>
                                                        <td><span class="original-link" title=url.original_url.clone()>{url.original_url}</span></td>
                                                        <td>{url.clicks}</td>
                                                        <td>{url.created_at}</td> 
                                                        <td>
                                                            <button class="action-btn" on:click=move |_| {
                                                                let _ = window().navigator().clipboard().write_text(&short_link_for_copy);
                                                            } title="Copy">"📋"</button>
                                                            <a href=format!("/analytics/{}", short_code_for_analytics) class="action-btn" title="Analytics">"📊"</a>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                }.into_view()
                            }
                        },
                        Some(Err(e)) => view! { <p class="error">{format!("Error loading links: {}", e)}</p> }.into_view(),
                        None => view! { <p>"Loading..."</p> }.into_view()
                    }}
                </Suspense>
            </div>
        </div>
    }
}
