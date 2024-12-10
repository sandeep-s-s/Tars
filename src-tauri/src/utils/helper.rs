use std::collections::HashMap;

use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::multipart;
use reqwest::Client;

use crate::models::FormDaum;
use crate::models::XWwwFormUrlencoded;
use crate::models::{JsonResponse, RequestObject};

pub async fn send_get_request(request: RequestObject) -> Result<JsonResponse, String> {
    let client = Client::new();

    let url = request.endpoint;
    let mut request_builder = client.get(&url);

    // Build query parameters
    let query_params: HashMap<_, _> = request
        .params
        .iter()
        .filter_map(|param| {
            if param.checked.unwrap_or(false) {
                Some((param.key.clone(), param.value.clone()))
            } else {
                None
            }
        })
        .collect();

    let req_headers = request
        .headers
        .iter()
        .filter_map(|header| {
            if header.checked.unwrap_or(false) {
                let key = reqwest::header::HeaderName::from_bytes(header.key.as_bytes()).ok()?;
                let value = reqwest::header::HeaderValue::from_str(&header.value).ok()?;
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<reqwest::header::HeaderMap>();

    request_builder = request_builder.query(&query_params).headers(req_headers);

    if request.auth.auth_active {
        match request.auth.auth_type.as_str() {
            "bearer" => request_builder = request_builder.bearer_auth(&request.auth.token),
            "basic" => {
                request_builder =
                    request_builder.basic_auth(request.auth.username, Some(request.auth.password))
            }
            "noauth" => {}
            _ => return Err("Unsupported authentication type".to_string()),
        }
    }

    match request_builder.send().await {
        Ok(response) => {
            let status_code = response.status().as_u16();
            let success = response.status().is_success();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<_, _>>();
            match response.text().await {
                Ok(json_body) => {
                    let message = json_body;

                    let json_response = JsonResponse {
                        success,
                        message,
                        status_code,
                        headers,
                    };
                    Ok(json_response)
                }
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_post_request(request: RequestObject) -> Result<JsonResponse, String> {
    let client = Client::new();

    let url = request.endpoint;

    let mut req_headers = request
        .headers
        .iter()
        .filter_map(|header| {
            if header.checked.unwrap_or(false) {
                let key = reqwest::header::HeaderName::from_bytes(header.key.as_bytes()).ok()?;
                let value = reqwest::header::HeaderValue::from_str(&header.value).ok()?;
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<reqwest::header::HeaderMap>();

    let mut request_builder = client.post(&url);

    if request.body.mode == "formData" {
        let multipart = get_form_data(request.body.form_data);
        request_builder = request_builder.multipart(multipart)
    }

    if request.body.mode == "x-www-form-urlencoded" {
        let x_www_form_data = get_x_www_form_urlencoded_data(request.body.x_www_form_urlencoded);
        request_builder = request_builder.form(&x_www_form_data);
    }

    if request.body.mode == "raw" {
        if request.body.raw_type.to_lowercase() == "json" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        if request.body.raw_type.to_lowercase() == "xml" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        }
        if request.body.raw_type.to_lowercase() == "text" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        }
        if request.body.raw_type.to_lowercase() == "html" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
        }
        let body = request.body.raw.clone();
        request_builder = request_builder.body(body);
    }

    // Build query parameters
    let query_params: HashMap<_, _> = request
        .params
        .iter()
        .filter_map(|param| {
            if param.checked.unwrap_or(false) {
                Some((param.key.clone(), param.value.clone()))
            } else {
                None
            }
        })
        .collect();

    if request.auth.auth_active {
        match request.auth.auth_type.as_str() {
            "bearer" => request_builder = request_builder.bearer_auth(&request.auth.token),
            "basic" => {
                request_builder =
                    request_builder.basic_auth(request.auth.username, Some(request.auth.password))
            }
            "noauth" => {}
            _ => return Err("Unsupported authentication type".to_string()),
        }
    }

    request_builder = request_builder.query(&query_params).headers(req_headers);

    match request_builder.send().await {
        Ok(response) => {
            let status_code = response.status().as_u16();
            let success = response.status().is_success();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<_, _>>();
            match response.text().await {
                Ok(json_body) => {
                    let message = json_body;

                    let json_response = JsonResponse {
                        success,
                        message,
                        status_code,
                        headers,
                    };
                    Ok(json_response)
                }
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_put_request(request: RequestObject) -> Result<JsonResponse, String> {
    let client = Client::new();

    let url = request.endpoint;

    let mut req_headers = request
        .headers
        .iter()
        .filter_map(|header| {
            if header.checked.unwrap_or(false) {
                let key = reqwest::header::HeaderName::from_bytes(header.key.as_bytes()).ok()?;
                let value = reqwest::header::HeaderValue::from_str(&header.value).ok()?;
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<reqwest::header::HeaderMap>();

    let mut request_builder = client.put(&url);

    if request.body.mode == "formData" {
        let multipart = get_form_data(request.body.form_data);
        request_builder = request_builder.multipart(multipart)
    }

    if request.body.mode == "x-www-form-urlencoded" {
        let x_www_form_data = get_x_www_form_urlencoded_data(request.body.x_www_form_urlencoded);
        request_builder = request_builder.form(&x_www_form_data);
    }

    if request.body.mode == "raw" {
        if request.body.raw_type.to_lowercase() == "json" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        if request.body.raw_type.to_lowercase() == "xml" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        }
        if request.body.raw_type.to_lowercase() == "text" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        }
        if request.body.raw_type.to_lowercase() == "html" {
            req_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
        }
        let body = request.body.raw.clone();
        request_builder = request_builder.body(body);
    }

    // Build query parameters
    let query_params: HashMap<_, _> = request
        .params
        .iter()
        .filter_map(|param| {
            if param.checked.unwrap_or(false) {
                Some((param.key.clone(), param.value.clone()))
            } else {
                None
            }
        })
        .collect();

    request_builder = request_builder.query(&query_params).headers(req_headers);

    if request.auth.auth_active {
        match request.auth.auth_type.as_str() {
            "bearer" => request_builder = request_builder.bearer_auth(&request.auth.token),
            "basic" => {
                request_builder =
                    request_builder.basic_auth(request.auth.username, Some(request.auth.password))
            }
            "noauth" => {}
            _ => return Err("Unsupported authentication type".to_string()),
        }
    }
    match request_builder.send().await {
        Ok(response) => {
            let status_code = response.status().as_u16();
            let success = response.status().is_success();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<_, _>>();
            match response.text().await {
                Ok(json_body) => {
                    let message = json_body;

                    let json_response = JsonResponse {
                        success,
                        message,
                        status_code,
                        headers,
                    };
                    Ok(json_response)
                }
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub fn get_form_data(form_data_request: Vec<FormDaum>) -> multipart::Form {
    let mut form = multipart::Form::new();

    for item in form_data_request {
        let file_path = item.src.clone();
        let file_name = file_path.split('/').last().unwrap_or("file").to_string();
        if item.checked.unwrap_or(false) {
            if item.type_field == "text" {
                form = form.text(item.key, item.value);
            } else if item.type_field == "file" {
                match std::fs::read(&file_path) {
                    Ok(content) => {
                        let part = multipart::Part::bytes(content).file_name(file_name);
                        form = form.part(item.key, part);
                    }
                    Err(e) => {
                        eprintln!("Error reading file {}: {}", file_path, e);
                    }
                }
            }
        }
    }
    form
}

pub fn get_x_www_form_urlencoded_data(
    x_www_form_urlencoded_request: Vec<XWwwFormUrlencoded>,
) -> HashMap<String, String> {
    let x_www_form_data: HashMap<_, _> = x_www_form_urlencoded_request
        .iter()
        .filter_map(|f| {
            if let Some(checked) = f.checked {
                if checked {
                    Some((f.key.clone(), f.value.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    x_www_form_data
}
