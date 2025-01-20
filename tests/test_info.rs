use noob_builder::Builder;

#[derive(Debug, Clone, Default, PartialEq, Builder)]
pub struct Info {
    pub title: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub version: String,
}

#[test]
fn test_info() {
    let info = Info::builder()
        .title("My API".to_string())
        .description("This is my API documentation".to_string())
        .version("1.0.0".to_string())
        .terms_of_service("https://example.com/terms".to_string())
        .build()
        .unwrap();
    assert!(info.title == "My API");
    assert!(info.summary.is_none());
    assert!(info.description == Some("This is my API documentation".to_string()));
    assert!(info.terms_of_service == Some("https://example.com/terms".to_string()));
    assert!(info.version == "1.0.0");
}
