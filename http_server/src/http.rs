enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod,
    headers: Option<String>,
}
