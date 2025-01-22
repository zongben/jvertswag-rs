use anyhow::{anyhow, Result};
use std::future::Future;
use tokio::runtime::Builder;

use crate::http;
use reqwest::header::{HeaderMap, HeaderName};

#[derive(Debug)]
pub struct Schema {
    pub root: String,
    pub path: String,
    pub method: String,
    pub body: Option<String>,
    pub header: Vec<String>,
    pub query: Option<Vec<String>>,
    pub param: Option<Vec<String>>,
    pub res: String,
}

pub struct SchemaParams {
    pub root: String,
    pub path: String,
    pub method: String,
    pub body: Option<String>,
    pub header: Vec<String>,
    pub query: Option<Vec<String>>,
    pub param: Option<Vec<String>>,
    pub res: Option<String>,
}

impl Schema {
    fn get_url(&self) -> String {
        let query = self
            .query
            .as_ref()
            .map(|q| format!("?{}", q.join("&")))
            .unwrap_or_default();

        let path = self.param.as_ref().map_or_else(
            || self.path.to_string(),
            |p| {
                p.iter().fold(self.path.to_string(), |acc, s| {
                    let splited = s.split_once('=').unwrap();
                    acc.replace(&format!("{{{}}}", splited.0), splited.1)
                })
            },
        );

        format!("{}{}{}", self.root, path, query)
    }

    pub fn get_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        for header in self.header.iter() {
            let (key, value) = match header.split_once(':') {
                Some((key, value)) => (key.trim(), value.trim()),
                None => (header.trim(), ""),
            };

            if let (Ok(key), Ok(value)) = (HeaderName::from_bytes(key.as_bytes()), value.parse()) {
                headers.insert(key, value);
            } else {
                return Err(anyhow!("Invalid header: {}", header));
            }
        }
        Ok(headers)
    }

    pub fn get_query_keys(&self) -> Vec<&str> {
        self.query.as_ref().map_or_else(Vec::new, |q| {
            q.iter().map(|s| s.split_once('=').unwrap().0).collect()
        })
    }

    pub fn get_param_keys(&self) -> Vec<&str> {
        self.param.as_ref().map_or_else(Vec::new, |p| {
            p.iter().map(|s| s.split_once('=').unwrap().0).collect()
        })
    }
}

async fn handle_req<F>(req_fn: F) -> Result<String>
where
    F: Future<Output = Result<String, reqwest::Error>>,
{
    match req_fn.await {
        Ok(res) => Ok(res),
        Err(e) => {
            return Err(anyhow!("Error: {}", e));
        }
    }
}

async fn fetch_response(schema: &Schema) -> Result<String> {
    let url = schema.get_url();
    let headers = schema.get_headers()?;
    match schema.method.as_str() {
        "GET" => handle_req(http::get(&url, headers)).await,
        "POST" => handle_req(http::post(&url, headers, schema.body.as_deref())).await,
        "PUT" => handle_req(http::put(&url, headers, schema.body.as_deref())).await,
        "DELETE" => handle_req(http::delete(&url, headers)).await,
        "PATCH" => handle_req(http::patch(&url, headers, schema.body.as_deref())).await,
        _ => {
            return Err(anyhow!("Invalid method: {}", schema.method));
        }
    }
}

pub fn create_schema(params: SchemaParams) -> Result<Schema> {
    let mut schema = Schema {
        root: params.root,
        path: params.path,
        method: params.method,
        body: params.body,
        header: params.header,
        query: params.query,
        param: params.param,
        res: match params.res {
            Some(res) => res,
            None => String::new(),
        },
    };

    if schema.res.is_empty() {
        schema.res = Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap()
            .block_on(fetch_response(&schema))?;
    }

    Ok(schema)
}
