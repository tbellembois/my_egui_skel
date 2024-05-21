use poll_promise::Promise;

pub fn retrieve_feeds(ctx: &egui::Context) -> Promise<Result<String, String>> {
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get("https://hacker-news.firebaseio.com/v0/topstories.json");

    ehttp::fetch(request, move |response| {
        let feeds = response.and_then(parse_retrieve_feeds_response);
        sender.send(feeds);
        ctx.request_repaint();
    });

    promise
}

fn parse_retrieve_feeds_response(response: ehttp::Response) -> Result<String, String> {
    match response.status {
        200 => Ok(response.text().unwrap().to_string()),
        _ => match response.text() {
            Some(text_response) => Err(text_response.to_string()),
            None => Err(response.status.to_string()),
        },
    }
}
