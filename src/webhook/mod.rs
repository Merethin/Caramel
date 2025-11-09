use serenity::{all::{
    ExecuteWebhook, Http, LightMethod, Request, Route, WebhookId
}, utils, json};
use std::error::Error;
use url::Url;

pub type Webhook = (WebhookId, String);

pub fn parse_webhook_from_url(hook: &str) -> Option<(WebhookId, String)> {
    Url::parse(hook).ok().and_then(|url| {
        utils::parse_webhook(&url).and_then(|hook| Some((hook.0, hook.1.to_owned())))
    })
}

pub async fn execute_webhook(
    http: &Http,
    webhook: &Webhook,
    message: ExecuteWebhook,
) -> Result<(), Box<dyn Error>> {
    let params: Vec<(&'static str, String)> = vec![
        ("wait", "false".into()), 
        ("with_components", "true".into())
    ];

    let request = Request::new(
        Route::WebhookWithToken { webhook_id: webhook.0, token: &webhook.1 },
        LightMethod::Post,
    ).params(Some(params)).body(
        Some(json::to_vec(&message)?)
    );

    let response = http.request(request).await?;
    response.error_for_status()?;

    Ok(())
}