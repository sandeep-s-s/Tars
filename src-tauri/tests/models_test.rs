use std::collections::HashMap;
use tars_lib::models::{
    Auth, Body, FormDaum, Header, JsonResponse, Param, RequestObject, XWwwFormUrlencoded,
};

#[test]
fn test_request_object_new() {
    let request = RequestObject::new(
        "Test Request".to_string(),
        "GET".to_string(),
        "https://api.example.com".to_string(),
    );

    assert_eq!(request.name, "Test Request");
    assert_eq!(request.method, "GET");
    assert_eq!(request.endpoint, "https://api.example.com");
    assert_eq!(request.v, "1.0");
    assert!(request.params.is_empty());
    assert!(request.headers.is_empty());
    assert_eq!(request.auth.auth_type, "");
    assert!(!request.auth.auth_active);
}

#[test]
fn test_request_object_with_params() {
    let mut request = RequestObject::new(
        "Test".to_string(),
        "GET".to_string(),
        "https://api.example.com".to_string(),
    );

    request.params.push(Param {
        key: "page".to_string(),
        value: "1".to_string(),
        checked: Some(true),
    });
    request.params.push(Param {
        key: "limit".to_string(),
        value: "10".to_string(),
        checked: Some(false),
    });

    assert_eq!(request.params.len(), 2);

    let active_params: Vec<_> = request
        .params
        .iter()
        .filter(|p| p.checked.unwrap_or(false))
        .collect();
    assert_eq!(active_params.len(), 1);
}

#[test]
fn test_request_object_with_headers() {
    let mut request = RequestObject::new(
        "Test".to_string(),
        "GET".to_string(),
        "https://api.example.com".to_string(),
    );

    request.headers.push(Header {
        key: "Authorization".to_string(),
        value: "Bearer token".to_string(),
        checked: Some(true),
        url: None,
        name: None,
    });
    request.headers.push(Header {
        key: "X-Custom".to_string(),
        value: "value".to_string(),
        checked: Some(false),
        url: None,
        name: None,
    });

    assert_eq!(request.headers.len(), 2);

    let active_headers: Vec<_> = request
        .headers
        .iter()
        .filter(|h| h.checked.unwrap_or(false))
        .collect();
    assert_eq!(active_headers.len(), 1);
}

#[test]
fn test_request_object_with_bearer_auth() {
    let mut request = RequestObject::new(
        "Test".to_string(),
        "GET".to_string(),
        "https://api.example.com".to_string(),
    );

    request.auth = Auth {
        auth_type: "bearer".to_string(),
        auth_active: true,
        username: String::new(),
        password: String::new(),
        token: "my-token".to_string(),
    };

    assert!(request.auth.auth_active);
    assert_eq!(request.auth.auth_type, "bearer");
    assert_eq!(request.auth.token, "my-token");
}

#[test]
fn test_request_object_with_basic_auth() {
    let mut request = RequestObject::new(
        "Test".to_string(),
        "GET".to_string(),
        "https://api.example.com".to_string(),
    );

    request.auth = Auth {
        auth_type: "basic".to_string(),
        auth_active: true,
        username: "admin".to_string(),
        password: "secret".to_string(),
        token: String::new(),
    };

    assert!(request.auth.auth_active);
    assert_eq!(request.auth.auth_type, "basic");
    assert_eq!(request.auth.username, "admin");
    assert_eq!(request.auth.password, "secret");
}

#[test]
fn test_body_default() {
    let body = Body::default();

    assert_eq!(body.mode, "");
    assert!(body.form_data.is_empty());
    assert!(body.x_www_form_urlencoded.is_empty());
    assert_eq!(body.raw_type, "");
    assert_eq!(body.raw, "");
}

#[test]
fn test_body_with_raw_json() {
    let body = Body {
        mode: "raw".to_string(),
        form_data: vec![],
        x_www_form_urlencoded: vec![],
        raw_type: "json".to_string(),
        raw: r#"{"key": "value"}"#.to_string(),
    };

    assert_eq!(body.mode, "raw");
    assert_eq!(body.raw_type, "json");
    assert_eq!(body.raw, r#"{"key": "value"}"#);
}

#[test]
fn test_body_with_form_data() {
    let body = Body {
        mode: "formData".to_string(),
        form_data: vec![FormDaum {
            key: "username".to_string(),
            value: "admin".to_string(),
            checked: Some(true),
            type_field: "text".to_string(),
            src: String::new(),
        }],
        x_www_form_urlencoded: vec![],
        raw_type: String::new(),
        raw: String::new(),
    };

    assert_eq!(body.mode, "formData");
    assert_eq!(body.form_data.len(), 1);
    assert_eq!(body.form_data[0].key, "username");
}

#[test]
fn test_auth_default() {
    let auth = Auth::default();

    assert_eq!(auth.auth_type, "");
    assert!(!auth.auth_active);
    assert_eq!(auth.username, "");
    assert_eq!(auth.password, "");
    assert_eq!(auth.token, "");
}

#[test]
fn test_form_daum_default() {
    let form_daum = FormDaum::default();

    assert_eq!(form_daum.key, "");
    assert_eq!(form_daum.value, "");
    assert_eq!(form_daum.checked, None);
    assert_eq!(form_daum.type_field, "");
    assert_eq!(form_daum.src, "");
}

#[test]
fn test_form_daum_file_type() {
    let form_daum = FormDaum {
        key: "file".to_string(),
        value: String::new(),
        checked: Some(true),
        type_field: "file".to_string(),
        src: "/path/to/file.txt".to_string(),
    };

    assert_eq!(form_daum.type_field, "file");
    assert_eq!(form_daum.src, "/path/to/file.txt");
}

#[test]
fn test_x_www_form_urlencoded_default() {
    let x_www = XWwwFormUrlencoded::default();

    assert_eq!(x_www.key, "");
    assert_eq!(x_www.value, "");
    assert_eq!(x_www.checked, None);
    assert_eq!(x_www.type_field, "");
}

#[test]
fn test_json_response_success() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = JsonResponse {
        success: true,
        message: r#"{"status": "ok"}"#.to_string(),
        status_code: 200,
        headers,
    };

    assert!(response.success);
    assert_eq!(response.status_code, 200);
}

#[test]
fn test_json_response_error() {
    let response = JsonResponse {
        success: false,
        message: "Not Found".to_string(),
        status_code: 404,
        headers: HashMap::new(),
    };

    assert!(!response.success);
    assert_eq!(response.status_code, 404);
}

#[test]
fn test_json_response_with_multiple_headers() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("X-Request-Id".to_string(), "12345".to_string());

    let response = JsonResponse {
        success: true,
        message: "OK".to_string(),
        status_code: 201,
        headers,
    };

    assert_eq!(response.headers.len(), 2);
    assert_eq!(
        response.headers.get("X-Request-Id"),
        Some(&"12345".to_string())
    );
}
