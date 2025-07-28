use error_set::error_set;

error_set! {
    FetcherError = ResponseError || {
        #[display("Request failed: {}")]
        Request(ureq::Error),
    };

    ResponseError = {
        #[display("Could not parse response: {}")]
        Deserialization(serde_json::Error),
        #[display("Bad response: {}")]
        BadResponse(Box<dyn std::error::Error>),
    };


    ProxyError = {
        #[display("Invalid proxy URI")]
        InvalidProxy(ureq::Error), // ureq here just for From impl
    };

}
