use leptos::*;
use crate::api::shorten_url;

#[component]
pub fn Shortener(on_success: Action<String, ()>) -> impl IntoView {
    let (url, set_url) = create_signal(String::new());
    let (custom_alias, set_custom_alias) = create_signal(String::new());
    let (use_alias, set_use_alias) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (result, set_result) = create_signal(Option::<String>::None);

    let shorten_action = create_action(move |_| {
        let url = url.get();
        let alias = if use_alias.get() && !custom_alias.get().is_empty() {
            Some(custom_alias.get())
        } else {
            None
        };
        
        // TODO: Get real user ID
        let user_id = Some("anonymous".to_string()); 

        async move {
            set_loading.set(true);
            set_error.set(None);
            set_result.set(None);
            
            match shorten_url(url, alias, user_id).await {
                Ok(resp) => {
                    set_result.set(Some(resp.short_url));
                    set_loading.set(false);
                    on_success.dispatch("refresh".to_string());
                },
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        }
    });

    view! {
        <div class="shortener-card">
            <form on:submit:prevent=move |ev: web_sys::SubmitEvent| shorten_action.dispatch(())>
                <div class="input-group">
                    <input 
                        type="url" 
                        placeholder="Paste your long link here..." 
                        required
                        prop:value=url
                        on:input=move |ev| set_url.set(event_target_value(&ev))
                    />
                    <button type="submit" class="btn-primary" disabled=move || loading.get()>
                        <span class="btn-text" style=move || if loading.get() { "display: none" } else { "" }>
                            "Shorten"
                        </span>
                        <span class="btn-loading" style=move || if loading.get() { "" } else { "display: none" }>
                            "⏳"
                        </span>
                    </button>
                </div>

                <div class="advanced-options">
                    <label class="checkbox-label">
                        <input 
                            type="checkbox" 
                            prop:checked=use_alias
                            on:change=move |ev| set_use_alias.set(event_target_checked(&ev))
                        />
                        <span>"Custom alias"</span>
                    </label>
                    <input 
                        type="text" 
                        placeholder="my-custom-link" 
                        pattern="[a-zA-Z0-9_-]{3,20}"
                        style=move || if use_alias.get() { "display: block" } else { "display: none" }
                        prop:value=custom_alias
                        on:input=move |ev| set_custom_alias.set(event_target_value(&ev))
                    />
                </div>
            </form>

            {move || error.get().map(|e| view! {
                <div class="error-message" style="color: red; margin-top: 10px;">
                    {e}
                </div>
            })}

            {move || result.get().map(|short_url| {
                let short_url_for_copy = short_url.clone();
                view! {
                <div class="result" style="display: block;">
                    <div class="result-content">
                        <label>"Your short link:"</label>
                        <div class="short-url-display">
                            <input type="text" readonly value=short_url.clone() />
                            <button class="btn-copy" on:click=move |_| {
                                let _ = window().navigator().clipboard().write_text(&short_url_for_copy);
                            }>
                                "📋 Copy"
                            </button>
                        </div>
                        <div class="result-stats">
                            <span>"✅ Created successfully!"</span>
                        </div>
                    </div>
                </div>
            }})}
        </div>
    }
}
