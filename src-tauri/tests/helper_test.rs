use tars_lib::models::XWwwFormUrlencoded;
use tars_lib::utils::helper::get_x_www_form_urlencoded_data;

#[test]
fn test_get_x_www_form_urlencoded_data_with_checked_items() {
    let data = vec![
        XWwwFormUrlencoded {
            key: "username".to_string(),
            value: "admin".to_string(),
            checked: Some(true),
            type_field: "text".to_string(),
        },
        XWwwFormUrlencoded {
            key: "password".to_string(),
            value: "secret".to_string(),
            checked: Some(true),
            type_field: "text".to_string(),
        },
    ];

    let result = get_x_www_form_urlencoded_data(data);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("username"), Some(&"admin".to_string()));
    assert_eq!(result.get("password"), Some(&"secret".to_string()));
}

#[test]
fn test_get_x_www_form_urlencoded_data_with_unchecked_items() {
    let data = vec![
        XWwwFormUrlencoded {
            key: "username".to_string(),
            value: "admin".to_string(),
            checked: Some(false),
            type_field: "text".to_string(),
        },
        XWwwFormUrlencoded {
            key: "password".to_string(),
            value: "secret".to_string(),
            checked: None,
            type_field: "text".to_string(),
        },
    ];

    let result = get_x_www_form_urlencoded_data(data);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_x_www_form_urlencoded_data_empty_input() {
    let data: Vec<XWwwFormUrlencoded> = vec![];
    let result = get_x_www_form_urlencoded_data(data);
    assert_eq!(result.len(), 0);
}
