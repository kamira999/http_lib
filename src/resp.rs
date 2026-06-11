use std::collections::HashMap;




struct Response{
    status: Status,
    headers: HashMap<String, String>,
    body: String
}

impl Response{
    fn new(status: Status, headers: HashMap<String, String>, body: String) -> Self{
        Self { status, headers, body }
    }
}

struct Status{
    http_version: String,
    status_code: ResponseStatusCode,
    reason_phrase: Option<String>
}

impl Status{
    fn new(http_version: &str, status_code: ResponseStatusCode, reason_phrase: Option<&str>) -> Self{
        Self { http_version: http_version.to_string(), status_code, reason_phrase: {
            if reason_phrase.is_none() {
                None
            }else{
                Some(reason_phrase.unwrap().to_string())
            }
        } }
    }

    fn to_string(&mut self) -> String{
        format!("{} {:?} {}", self.http_version, self.status_code as isize, self.reason_phrase.clone().unwrap_or("".to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseStatusCode{
    //Informational responses
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    //Successful responses
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthInfo = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    //Redirection messages
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOthers = 303,
    NotModified = 304,
    UseProxy = 305,
    Unused = 306,
    TempRedirect = 307,
    PermRedirect = 308,
    
    //Client error responses
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    //Server error responses
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}