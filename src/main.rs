use axum::{extract::TypedHeader, routing::get, Router};
use headers::{Header, HeaderName, HeaderValue};

struct XForwardedFor(String);

static XFORWARDEDFOR: HeaderName = HeaderName::from_static("x-forwarded-for");

impl Header for XForwardedFor {
    fn name() -> &'static HeaderName {
        &XFORWARDEDFOR
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        Ok(XForwardedFor(
            value
                .to_str()
                .map_err(|_| headers::Error::invalid())?
                .to_string(),
        ))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from_str(&self.0.to_string());

        values.extend(std::iter::once(value.unwrap()))
    }
}

async fn show_ip(TypedHeader(forwarded_for): TypedHeader<XForwardedFor>) -> String {
    format!("ip: {}", forwarded_for.0)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(show_ip));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
