pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_segments: Vec<&str> = prefix.split('/').collect();
    let request_segments: Vec<&str> = request_path.split('/').collect();

    if prefix_segments.len() > request_segments.len() {
        return false;
    }

    for (i, segment) in prefix_segments.iter().enumerate() {
        if segment != &"*" && segment != &request_segments[i] {
            return false;
        }
    }

    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}