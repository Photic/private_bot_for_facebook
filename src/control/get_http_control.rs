use reqwest::{Client, header::HeaderMap};


pub(crate) struct SessionControl {
    pub session: Client,
    pub headers: HeaderMap,
}