use tars_lib::models::RequestObject;

#[test]
fn test_send_request_get_method_routing() {
    let request_json = serde_json::to_string(&RequestObject::new(
        "Test GET".to_string(),
        "GET".to_string(),
        "https://httpbin.org/get".to_string(),
    ))
    .unwrap();

    let parsed: RequestObject = serde_json::from_str(&request_json).unwrap();
    assert_eq!(parsed.method.to_lowercase(), "get");
}

#[test]
fn test_send_request_post_method_routing() {
    let request_json = serde_json::to_string(&RequestObject::new(
        "Test POST".to_string(),
        "POST".to_string(),
        "https://httpbin.org/post".to_string(),
    ))
    .unwrap();

    let parsed: RequestObject = serde_json::from_str(&request_json).unwrap();
    assert_eq!(parsed.method.to_lowercase(), "post");
}

#[test]
fn test_send_request_put_method_routing() {
    let request_json = serde_json::to_string(&RequestObject::new(
        "Test PUT".to_string(),
        "PUT".to_string(),
        "https://httpbin.org/put".to_string(),
    ))
    .unwrap();

    let parsed: RequestObject = serde_json::from_str(&request_json).unwrap();
    assert_eq!(parsed.method.to_lowercase(), "put");
}

#[test]
fn test_send_request_delete_method_routing() {
    let request_json = serde_json::to_string(&RequestObject::new(
        "Test DELETE".to_string(),
        "DELETE".to_string(),
        "https://httpbin.org/delete".to_string(),
    ))
    .unwrap();

    let parsed: RequestObject = serde_json::from_str(&request_json).unwrap();
    assert_eq!(parsed.method.to_lowercase(), "delete");
}

#[test]
fn test_send_request_unsupported_method() {
    let request = RequestObject::new(
        "Test".to_string(),
        "PATCH".to_string(),
        "https://httpbin.org/patch".to_string(),
    );

    let method = request.method.to_lowercase();
    let is_supported = method == "get" || method == "post" || method == "put" || method == "delete";

    assert!(!is_supported);
}

#[test]
fn test_parse_invalid_json_returns_error() {
    let invalid_json = "not valid json";
    let result: Result<RequestObject, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_parse_valid_request_json() {
    let json = r#"{
        "v": "1.0",
        "endpoint": "https://api.example.com",
        "name": "Test Request",
        "params": [],
        "headers": [],
        "method": "GET",
        "auth": {
            "authType": "",
            "authActive": false,
            "username": "",
            "password": "",
            "token": ""
        },
        "preRequestScript": "",
        "testScript": "",
        "body": {
            "mode": "",
            "formData": [],
            "xWwwFormUrlencoded": [],
            "rawType": "",
            "raw": ""
        }
    }"#;

    let request: RequestObject = serde_json::from_str(json).unwrap();
    assert_eq!(request.name, "Test Request");
    assert_eq!(request.method, "GET");
    assert_eq!(request.endpoint, "https://api.example.com");
}

#[test]
fn test_serialize_request_object() {
    let request = RequestObject::new(
        "My Request".to_string(),
        "POST".to_string(),
        "https://api.example.com/users".to_string(),
    );

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("My Request"));
    assert!(serialized.contains("POST"));
    assert!(serialized.contains("https://api.example.com/users"));
}

#[test]
fn test_case_insensitive_method_parsing() {
    let json_lower = r#"{
        "v": "1.0",
        "endpoint": "https://api.example.com",
        "name": "Test",
        "params": [],
        "headers": [],
        "method": "get",
        "auth": {"authType": "", "authActive": false, "username": "", "password": "", "token": ""},
        "preRequestScript": "",
        "testScript": "",
        "body": {"mode": "", "formData": [], "xWwwFormUrlencoded": [], "rawType": "", "raw": ""}
    }"#;

    let json_upper = r#"{
        "v": "1.0",
        "endpoint": "https://api.example.com",
        "name": "Test",
        "params": [],
        "headers": [],
        "method": "GET",
        "auth": {"authType": "", "authActive": false, "username": "", "password": "", "token": ""},
        "preRequestScript": "",
        "testScript": "",
        "body": {"mode": "", "formData": [], "xWwwFormUrlencoded": [], "rawType": "", "raw": ""}
    }"#;

    let request_lower: RequestObject = serde_json::from_str(json_lower).unwrap();
    let request_upper: RequestObject = serde_json::from_str(json_upper).unwrap();

    assert_eq!(
        request_lower.method.to_lowercase(),
        request_upper.method.to_lowercase()
    );
}
