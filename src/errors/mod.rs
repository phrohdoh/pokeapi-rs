use reqwest;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidArgument(msg: String) {
            description("Received an invalid argument"),
            display("Received an invalid argument: {}", msg),
        }
        RequestFailed(method: reqwest::Method, url: reqwest::Url) {
            description("A request failed"),
            display("Failed to {} <{}>", method, url),
        }
        UnexpectedResponseBody {
            description("The remote service returned an unexpected response body"),
            display("The remote service returned an unexpected response body"),
        }
        FailedToParse {
            description("Failed to parse response JSON"),
            display("Failed to parse response JSON"),
        }
        NotFound {
            description("The requested Pokemon could not be found"),
            display("The requested Pokemon could not be found"),
        }
    }
}