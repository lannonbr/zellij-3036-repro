use std::{collections::BTreeMap, error};

use async_std::task;
use surf::{
    http::{Method, Url},
    RequestBuilder,
};

enum HttpVerb {
    Get,
    Post,
    Put,
    Delete,
}

async fn web_req(url: String, verb: HttpVerb, headers: BTreeMap<String, String>, body: Vec<u8>) {
    task::spawn({
        // let senders = bus.senders.clone();
        async move {
            async fn web_request(
                url: String,
                verb: HttpVerb,
                headers: BTreeMap<String, String>,
                body: Vec<u8>,
            ) -> Result<
                (u16, BTreeMap<String, String>, Vec<u8>), // status_code, headers, body
                surf::Error,
            > {
                let url = Url::parse(&url)?;
                let http_method = match verb {
                    HttpVerb::Get => Method::Get,
                    HttpVerb::Post => Method::Post,
                    HttpVerb::Put => Method::Put,
                    HttpVerb::Delete => Method::Delete,
                };
                let mut req = RequestBuilder::new(http_method, url);
                if !body.is_empty() {
                    req = req.body(body);
                }
                for (header, value) in headers {
                    req = req.header(header.as_str(), value);
                }
                let mut res = req.await?;
                let status_code = res.status();
                let headers: BTreeMap<String, String> = res
                    .iter()
                    .map(|(name, value)| (name.to_string(), value.to_string()))
                    .collect();
                let body = res.take_body().into_bytes().await?;
                Ok((status_code as u16, headers, body))
            }

            match web_request(url, verb, headers, body).await {
                Ok((status, headers, body)) => {
                    // let _ = senders.send_to_plugin(PluginInstruction::Update(vec![(
                    //     Some(plugin_id),
                    //     Some(client_id),
                    //     Event::WebRequestResult(status, headers, body, context),
                    // )]));
                    println!("{:?}", String::from_utf8(body).unwrap());
                }
                Err(e) => {
                    eprintln!("Failed to send web request: {}", e);
                    // let error_body = e.to_string().as_bytes().to_vec();
                    // let _ = senders.send_to_plugin(PluginInstruction::Update(vec![(
                    //     Some(plugin_id),
                    //     Some(client_id),
                    //     Event::WebRequestResult(400, BTreeMap::new(), error_body, context),
                    // )]));
                }
            }
        }
    })
    .await
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");

    web_req(
        String::from("https://lannonbr.com/posts.json"),
        HttpVerb::Get,
        BTreeMap::new(),
        vec![],
    )
    .await;

    Ok(())
}
