use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Response{
    status: Status,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl Response{
    pub fn new(status: Status, headers: HashMap<String, String>, body: Option<String>) -> Self{
        Self { status, headers, body }
    }


    pub fn parse(data: String) -> anyhow::Result<Self, anyhow::Error> {

        //Split into two: (Status, headers) and body
        let mut parts = data.split("\r\n\r\n");

        let mut status_and_headers = parts
            .next()
            .unwrap()
            .split("\r\n");

        let body = parts.next().unwrap_or_default().to_string();

        let status: Status = status_and_headers.next().unwrap().try_into()?;


        let mut headers: HashMap<String, String> = HashMap::new();

        loop {
            let (k, v): (String, String) = match status_and_headers.next(){
                Some(str) => {
                    let mut str = str.split(":");
                    (str.next().unwrap().to_string(),  str.next().unwrap().to_string())
                },
                None => break,
            };

            headers.insert(k, v);
        }


        let body = if body.is_empty() { None } else { Some(body) };

        let response = Self::new(status, headers, body);

        Ok(response)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Status{
    http_version: String,
    status_code: ResponseStatusCode,
    reason_phrase: Option<String>
}

// TODO: impl proper default Status
impl Status{
    pub fn new(http_version: String, status_code: ResponseStatusCode, reason_phrase: Option<String>) -> Self{
        Self { http_version, status_code, reason_phrase: {
            if reason_phrase.is_none() {
                None
            }else{
                Some(reason_phrase.unwrap().to_string())
            }
        } }
    }

    pub fn to_string(&mut self) -> String{
        format!("{} {:?} {}", self.http_version, self.status_code as isize, self.reason_phrase.clone().unwrap_or("".to_string()))
    }
}

impl TryFrom<&str> for Status{
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        let http_version = parts
            .next()
            .ok_or(anyhow::Error::msg("HTTP version missing\n request:{value}"))?
            .to_string();

        let status_code: ResponseStatusCode = parts
            .next()
            .ok_or(anyhow::Error::msg("HTTP Status Code missing\n request:{value}"))?
            .try_into()?;

        let reason_phrase = parts
            .next()
            // .ok_or(anyhow::Error::msg("HTTP Reason missing\n request:{value}"))?
            .unwrap_or_default()
            .to_string();

        Ok(Self{http_version, status_code, reason_phrase: Some(reason_phrase)})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ResponseStatusCode{
    //Informational responses
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    //Successful responses
    #[default]
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

impl TryFrom<&str> for ResponseStatusCode{
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let n = i32::from_str(value)?;

        return n.try_into();
    }
}

impl TryFrom<i32> for ResponseStatusCode{
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Continue),
            101 => Ok(Self::SwitchingProtocols),
            102 => Ok(Self::Processing),
            103 => Ok(Self::EarlyHints),

            200 => Ok(Self::OK),
            201 => Ok(Self::Created),
            202 => Ok(Self::Accepted),
            203 => Ok(Self::NonAuthInfo),
            204 => Ok(Self::NoContent),
            205 => Ok(Self::ResetContent),
            206 => Ok(Self::PartialContent),
            207 => Ok(Self::MultiStatus),
            208 => Ok(Self::AlreadyReported),
            226 => Ok(Self::IMUsed),

            300 => Ok(Self::MultipleChoices),
            301 => Ok(Self::MovedPermanently),
            302 => Ok(Self::Found),
            303 => Ok(Self::SeeOthers),
            304 => Ok(Self::NotModified),
            305 => Ok(Self::UseProxy),
            306 => Ok(Self::Unused),
            307 => Ok(Self::TempRedirect),
            308 => Ok(Self::PermRedirect),

            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            402 => Ok(Self::PaymentRequired),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::MethodNotAllowed),
            406 => Ok(Self::NotAcceptable),
            407 => Ok(Self::ProxyAuthRequired),
            408 => Ok(Self::RequestTimeout),
            409 => Ok(Self::Conflict),
            410 => Ok(Self::Gone),
            411 => Ok(Self::LengthRequired),
            412 => Ok(Self::PreconditionFailed),
            413 => Ok(Self::ContentTooLarge),
            414 => Ok(Self::URITooLong),
            415 => Ok(Self::UnsupportedMediaType),
            416 => Ok(Self::RangeNotSatisfiable),
            417 => Ok(Self::ExpectationFailed),
            418 => Ok(Self::ImATeapot),
            421 => Ok(Self::MisdirectedRequest),
            422 => Ok(Self::UnprocessableContent),
            423 => Ok(Self::Locked),
            424 => Ok(Self::FailedDependency),
            425 => Ok(Self::TooEarly),
            426 => Ok(Self::UpgradeRequired),
            428 => Ok(Self::PreconditionRequired),
            429 => Ok(Self::TooManyRequests),
            430 => Ok(Self::RequestHeaderFieldsTooLarge),
            431 => Ok(Self::UnavailableForLegalReasons),

            500 => Ok(Self::InternalServerError),
            501 => Ok(Self::NotImplemented),
            502 => Ok(Self::BadGateway),
            503 => Ok(Self::ServiceUnavailable),
            504 => Ok(Self::GatewayTimeout),
            505 => Ok(Self::HTTPVersionNotSupported),
            506 => Ok(Self::VariantAlsoNegotiates),
            507 => Ok(Self::InsufficientStorage),
            508 => Ok(Self::LoopDetected),
            510 => Ok(Self::NotExtended),
            511 => Ok(Self::NetworkAuthenticationRequired),

            _ => Err(anyhow::Error::msg("Unknown HTTP Status code: {value}")),
        }
    }
}

#[derive(Debug, Default)]
struct ResponseBuilder {
    http_version: String,
    status_code: ResponseStatusCode,
    reason_phrase: Option<String>,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl ResponseBuilder {

    pub fn new() -> Self{
        Self::default()
    }

    pub fn http_version(mut self, version: &str) -> Self {
        self.http_version = version.to_string();

        self
    }

    pub fn status_code(mut self, code: i32) -> Self {
        self.status_code = ResponseStatusCode::try_from(code).unwrap();

        self
    }

    pub fn reason_phrase(mut self, phrase: &str) -> Self {
        self.reason_phrase = Some(phrase.to_string());

        self
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());

        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());

        self
    }

    pub fn build(&self) -> anyhow::Result<Response> {
        if self.http_version == String::default() || self.reason_phrase == Option::default() {
            return Err(anyhow::Error::msg("Response builder data incomplete"))
        }

        Ok(Response{
            status: Status::new(self.http_version.to_owned(), self.status_code, self.reason_phrase.to_owned()),
            headers: self.headers.to_owned(),
            body: self.body.to_owned(),
        })
    }
}
