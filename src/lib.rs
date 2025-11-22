mod models;
mod utils;

use worker::*;
use models::{ShortenRequest, ShortenResponse, ErrorResponse, Url};
use utils::{generate_short_code, generate_uuid, is_valid_url, is_valid_alias, current_timestamp};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    
    // CORS configuration
    let cors = Cors::new()
        .with_origins(vec!["*"])
        .with_methods(Method::all())
        .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);

    router
        // API endpoints
        .post_async("/api/shorten", |mut req, ctx| async move {
            let cors = Cors::new()
                .with_origins(vec!["*"])
                .with_methods(Method::all())
                .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);
            
            match handle_shorten(req, ctx).await {
                Ok(resp) => resp.with_cors(&cors),
                Err(e) => Err(e),
            }
        })
        // Preflight for shorten
        .options("/api/shorten", |_, _| {
            let cors = Cors::new()
                .with_origins(vec!["*"])
                .with_methods(Method::all())
                .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);
            Response::empty()?.with_cors(&cors)
        })
        .get_async("/api/urls", |req, ctx| async move {
            let cors = Cors::new()
                .with_origins(vec!["*"])
                .with_methods(Method::all())
                .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);
            
            match handle_list_urls(req, ctx).await {
                Ok(resp) => resp.with_cors(&cors),
                Err(e) => Err(e),
            }
        })
        .options("/api/urls", |_, _| {
             let cors = Cors::new()
                .with_origins(vec!["*"])
                .with_methods(Method::all())
                .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);
            Response::empty()?.with_cors(&cors)
        })
        .get_async("/api/analytics/:code", |req, ctx| async move {
            let cors = Cors::new()
                .with_origins(vec!["*"])
                .with_methods(Method::all())
                .with_allowed_headers(vec!["Content-Type", "X-User-ID"]);
            
            match handle_analytics(req, ctx).await {
                Ok(resp) => resp.with_cors(&cors),
                Err(e) => Err(e),
            }
        })
        .get_async("/:code", |req, ctx| async move {
            handle_redirect(req, ctx).await
        })
        .get("/", |_, _| {
            Response::redirect(url::Url::parse("https://blatik.github.io/shortlink")?)
        })
        .run(req, env)
        .await
}

async fn handle_shorten(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Parse request body
    let body: ShortenRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => {
            return Response::from_json(&ErrorResponse {
                error: "Invalid request body".to_string(),
            })
        }
    };

    // Validate URL
    if !is_valid_url(&body.url) {
        return Response::from_json(&ErrorResponse {
            error: "Invalid URL format. Must be http:// or https://".to_string(),
        });
    }

    // Get User ID from header
    let user_id = if let Some(auth_header) = req.headers().get("Authorization").ok().flatten() {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            // Verify token with Google
            let client = reqwest::Client::new();
            let res = client.get("https://oauth2.googleapis.com/tokeninfo")
                .query(&[("id_token", token)])
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: serde_json::Value = response.json().await.unwrap_or(serde_json::json!({}));
                        json["sub"].as_str().map(|s| s.to_string())
                    } else {
                        None
                    }
                },
                Err(_) => None,
            }
        } else {
            None
        }
    } else {
        req.headers().get("X-User-ID").ok().flatten()
    };
    
    // Default to anonymous if no valid ID found
    let user_id = user_id.or(Some("anonymous".to_string()));

    // Get KV namespace
    let kv = ctx.kv("URLS")?;
    
    // Determine short code
    let short_code = if let Some(alias) = body.custom_alias {
        // Validate custom alias
        if !is_valid_alias(&alias) {
            return Response::from_json(&ErrorResponse {
                error: "Invalid custom alias. Use 3-20 alphanumeric characters, hyphens, or underscores.".to_string(),
            });
        }

        // Check if alias already exists
        if kv.get(&alias).text().await?.is_some() {
            return Response::from_json(&ErrorResponse {
                error: "Custom alias already taken".to_string(),
            });
        }

        alias
    } else {
        // Generate unique short code (Start with 4 chars for shorter links)
        let mut code = generate_short_code(4);
        let mut attempts = 0;

        while kv.get(&code).text().await?.is_some() {
            attempts += 1;
            code = if attempts > 5 {
                generate_short_code(5) // Increase length if collision
            } else {
                generate_short_code(4)
            };
        }

        code
    };

    // Create URL object
    let url = Url {
        id: generate_uuid(),
        short_code: short_code.clone(),
        original_url: body.url.clone(),
        user_id: user_id.clone(),
        created_at: current_timestamp(),
        expires_at: None,
        clicks: 0,
    };

    // Store in KV
    let url_json = serde_json::to_string(&url)?;
    kv.put(&short_code, url_json)?.execute().await?;

    // Store in D1 database (for Dashboard)
    let db = ctx.env.d1("DB")?;
    let d1_result = db.prepare(
        "INSERT INTO urls (id, short_code, original_url, user_id, created_at, clicks) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&[
        url.id.clone().into(),
        url.short_code.clone().into(),
        url.original_url.clone().into(),
        user_id.clone().unwrap_or("anonymous".to_string()).into(),
        url.created_at.clone().into(),
        0.into(),
    ])?
    .run()
    .await;

    if let Err(e) = d1_result {
        console_log!("D1 Error: {}", e);
        // Fallback: if D1 fails, we still return the short URL (it's in KV)
        // But ideally we want to know. For now, let's return error to debug.
        return Response::from_json(&ErrorResponse {
            error: format!("Database error: {}", e),
        });
    }

    // Get base URL
    let base_url = ctx.var("BASE_URL")?.to_string();
    let short_url = format!("{}/{}", base_url, short_code);

    // Return response
    Response::from_json(&ShortenResponse {
        short_url,
        short_code: url.short_code,
        original_url: url.original_url,
    })
}

async fn handle_analytics(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let short_code = match ctx.param("code") {
        Some(code) => code,
        None => return Response::error("Short code required", 400),
    };

    let db = ctx.env.d1("DB")?;

    // Get total clicks
    let total_result = db.prepare("SELECT COUNT(*) as total FROM clicks WHERE short_code = ?")
        .bind(&[short_code.into()])?
        .first::<serde_json::Value>(None)
        .await?;
    
    let total_clicks = if let Some(v) = total_result {
        v.get("total").and_then(|t| t.as_i64()).unwrap_or(0)
    } else {
        0
    };

    // Get clicks by country
    let countries_result = db.prepare(
        "SELECT country, COUNT(*) as count FROM clicks WHERE short_code = ? GROUP BY country ORDER BY count DESC LIMIT 10"
    )
    .bind(&[short_code.into()])?
    .all()
    .await?;

    let countries: Vec<serde_json::Value> = countries_result.results()?;

    // Get clicks by device
    let devices_result = db.prepare(
        "SELECT device_type, COUNT(*) as count FROM clicks WHERE short_code = ? GROUP BY device_type"
    )
    .bind(&[short_code.into()])?
    .all()
    .await?;

    let devices: Vec<serde_json::Value> = devices_result.results()?;

    // Get clicks by browser
    let browsers_result = db.prepare(
        "SELECT browser, COUNT(*) as count FROM clicks WHERE short_code = ? GROUP BY browser ORDER BY count DESC LIMIT 10"
    )
    .bind(&[short_code.into()])?
    .all()
    .await?;

    let browsers: Vec<serde_json::Value> = browsers_result.results()?;

    // Get clicks over time (last 30 days)
    let timeline_result = db.prepare(
        "SELECT DATE(clicked_at) as date, COUNT(*) as count FROM clicks WHERE short_code = ? AND clicked_at >= datetime('now', '-30 days') GROUP BY DATE(clicked_at) ORDER BY date"
    )
    .bind(&[short_code.into()])?
    .all()
    .await?;

    let timeline: Vec<serde_json::Value> = timeline_result.results()?;

    // Get top referrers
    let referrers_result = db.prepare(
        "SELECT referrer, COUNT(*) as count FROM clicks WHERE short_code = ? GROUP BY referrer ORDER BY count DESC LIMIT 10"
    )
    .bind(&[short_code.into()])?
    .all()
    .await?;

    let referrers: Vec<serde_json::Value> = referrers_result.results()?;

    Response::from_json(&serde_json::json!({
        "total_clicks": total_clicks,
        "countries": countries,
        "devices": devices,
        "browsers": browsers,
        "timeline": timeline,
        "referrers": referrers
    }))
}

async fn handle_list_urls(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get User ID from header
    let user_id = if let Some(auth_header) = req.headers().get("Authorization").ok().flatten() {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            // Verify token with Google
            let client = reqwest::Client::new();
            let res = client.get("https://oauth2.googleapis.com/tokeninfo")
                .query(&[("id_token", token)])
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: serde_json::Value = response.json().await.unwrap_or(serde_json::json!({}));
                        json["sub"].as_str().map(|s| s.to_string())
                    } else {
                        None
                    }
                },
                Err(_) => None,
            }
        } else {
            None
        }
    } else {
        req.headers().get("X-User-ID").ok().flatten()
    };

    let user_id = match user_id {
        Some(id) => id,
        None => return Response::error("User ID required", 400),
    };

    let db = ctx.env.d1("DB")?;
    let result = db
        .prepare("SELECT * FROM urls WHERE user_id = ? ORDER BY created_at DESC LIMIT 50")
        .bind(&[user_id.into()])?
        .all()
        .await?;

    Response::from_json(&result.results::<Url>()?)
}

async fn handle_redirect(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let short_code = match ctx.param("code") {
        Some(code) => code,
        None => return Response::error("Not found", 404),
    };

    // Get URL from KV
    let kv = ctx.kv("URLS")?;
    let url_data = match kv.get(short_code).text().await? {
        Some(data) => data,
        None => return Response::error("URL not found", 404),
    };

    let url: Url = serde_json::from_str(&url_data)?;

    // Check if expired
    if let Some(expires_at) = &url.expires_at {
        if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at) {
            if expires < chrono::Utc::now() {
                return Response::error("URL has expired", 410);
            }
        }
    }

    // Extract analytics data from headers
    let country = req.headers().get("CF-IPCountry").ok().flatten().unwrap_or("Unknown".to_string());
    let city = req.headers().get("CF-IPCity").ok().flatten().unwrap_or("Unknown".to_string());
    let user_agent = req.headers().get("User-Agent").ok().flatten().unwrap_or("Unknown".to_string());
    let referrer = req.headers().get("Referer").ok().flatten().unwrap_or("Direct".to_string());
    
    // Parse User-Agent for device/browser/OS
    let (device_type, browser, os) = parse_user_agent(&user_agent);
    
    // Hash IP for privacy (Cloudflare provides CF-Connecting-IP)
    let ip = req.headers().get("CF-Connecting-IP").ok().flatten().unwrap_or("0.0.0.0".to_string());
    let ip_hash = format!("{:x}", md5::compute(ip.as_bytes()));

    // Store analytics asynchronously (don't block redirect)
    let db = ctx.env.d1("DB")?;
    let click_id = generate_uuid();
    let clicked_at = current_timestamp();
    
    // Fire and forget analytics insert
    let _ = db.prepare(
        "INSERT INTO clicks (id, short_code, clicked_at, country, city, device_type, browser, os, referrer, ip_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&[
        click_id.into(),
        short_code.to_string().into(),
        clicked_at.into(),
        country.into(),
        city.into(),
        device_type.into(),
        browser.into(),
        os.into(),
        referrer.into(),
        ip_hash.into(),
    ])?
    .run()
    .await;

    // Update click count in D1
    let _ = db.prepare("UPDATE urls SET clicks = clicks + 1 WHERE short_code = ?")
        .bind(&[short_code.into()])?
        .run()
        .await;

    // Perform redirect
    Response::redirect(url::Url::parse(&url.original_url)?)
}

// Helper function to parse User-Agent
fn parse_user_agent(ua: &str) -> (String, String, String) {
    let ua_lower = ua.to_lowercase();
    
    // Detect device type
    let device = if ua_lower.contains("mobile") || ua_lower.contains("android") || ua_lower.contains("iphone") {
        "Mobile"
    } else if ua_lower.contains("tablet") || ua_lower.contains("ipad") {
        "Tablet"
    } else {
        "Desktop"
    }.to_string();
    
    // Detect browser
    let browser = if ua_lower.contains("edg") {
        "Edge"
    } else if ua_lower.contains("chrome") {
        "Chrome"
    } else if ua_lower.contains("safari") && !ua_lower.contains("chrome") {
        "Safari"
    } else if ua_lower.contains("firefox") {
        "Firefox"
    } else if ua_lower.contains("opera") || ua_lower.contains("opr") {
        "Opera"
    } else {
        "Other"
    }.to_string();
    
    // Detect OS
    let os = if ua_lower.contains("windows") {
        "Windows"
    } else if ua_lower.contains("mac") {
        "macOS"
    } else if ua_lower.contains("linux") {
        "Linux"
    } else if ua_lower.contains("android") {
        "Android"
    } else if ua_lower.contains("ios") || ua_lower.contains("iphone") || ua_lower.contains("ipad") {
        "iOS"
    } else {
        "Other"
    }.to_string();
    
    (device, browser, os)
}

