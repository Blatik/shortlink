use leptos::*;
use leptos::window;
use crate::components::shortener::Shortener;
use crate::components::dashboard::Dashboard;

#[component]
pub fn Home() -> impl IntoView {
    let (refresh_trigger, set_refresh_trigger) = create_signal(0);

    let on_shorten_success = create_action(move |_| async move {
        set_refresh_trigger.update(|n| *n += 1);
    });

    view! {
        <div class="container">
            <header class="hero">
                <h1>"🔗 URL Shortener"</h1>
                <p class="subtitle">"Shorten links. Track clicks. Free forever."</p>
                
                // Auth Section
                <div id="authSection" class="auth-section">
                    {move || {
                        use crate::auth::{get_stored_token, decode_jwt};
                        let token = get_stored_token();
                        let user = token.as_deref().and_then(decode_jwt);
                        
                        match user {
                            Some(u) => view! {
                                <div id="userProfile" class="user-profile" style="display: flex;">
                                    <img id="userAvatar" src=u.picture alt="Avatar" class="user-avatar" />
                                    <span id="userName">{u.name}</span>
                                    <button id="signOutBtn" class="btn-outline-sm" on:click=move |_| {
                                        crate::auth::set_stored_token(None);
                                        let _ = window().location().reload();
                                    }>"Sign Out"</button>
                                </div>
                            }.into_view(),
                            None => view! {
                                <div>
                                    <div id="g_id_onload" 
                                        data-client_id="YOUR_GOOGLE_CLIENT_ID_HERE"
                                        data-callback="handleCredentialResponse" 
                                        data-auto_prompt="false">
                                    </div>
                                    <div class="g_id_signin" 
                                        data-type="standard" 
                                        data-size="large" 
                                        data-theme="outline"
                                        data-text="sign_in_with" 
                                        data-shape="rectangular" 
                                        data-logo_alignment="left">
                                    </div>
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </header>

            <Shortener on_success=on_shorten_success/>
            
            <Dashboard refresh_signal=refresh_trigger/>

            <div class="features">
                <div class="feature-card">
                    <div class="feature-icon">"⚡"</div>
                    <h3>"Lightning Fast"</h3>
                    <p>"Redirects in <50ms thanks to Cloudflare Workers"</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">"📊"</div>
                    <h3>"Detailed Stats"</h3>
                    <p>"Track click counts for every link you create"</p>
                </div>
                <div class="feature-card">
                    <div class="feature-icon">"💰"</div>
                    <h3>"100% Free"</h3>
                    <p>"No subscriptions. Use without limits."</p>
                </div>
            </div>

            <footer>
                <p>"Built with 🦀 Rust + Cloudflare Workers"</p>
                <div class="footer-links">
                    <a href="#api">"API Docs"</a>
                    <a href="#privacy">"Privacy"</a>
                    <a href="#terms">"Terms"</a>
                </div>
            </footer>
        </div>
    }
}
