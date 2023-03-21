use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;
use super::*;
use super::conversion as cnv;
use super::ExceptionNames as ex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, CONTENT_TYPE, HeaderValue},
};

fn one_post<RecvT>(
    url: &str,
    body: &str
) -> Outcome<RecvT> 
where RecvT: DeserializeOwned {
    let client: Client = Client::new();
    let headers = {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h
    };
    let res = client.post(url).headers(headers.clone()).body(body.clone()).send();
    if res.is_err() {
        let err = Exception::new_auto(ex::HttpPostException, res.err().unwrap());
        err.context("Failed to send");
        return Err(err);
    }
    let res = res.unwrap().text();
    if res.is_err() {
        let err = Exception::new_auto(ex::HttpPostException, res.err().unwrap());
        err.context("Response is not text");
        return Err(err);
    }
    let res = cnv::json_to_obj(&res.unwrap());
    if res.is_err() {
        let err = Exception::new_auto(ex::DeserializationException, res.err().unwrap());
        err.context(&format!("Response cannot be parsed into type `{}`", typename(res)));
        return Err(err);
    }
    Ok(res.unwrap())
}


pub fn http_post<SendT, RecvT>(
    url: &str,
    send_obj: &SendT
) -> Outcome<RecvT>
where SendT: Serialize, RecvT: DeserializeOwned
{
    let n_retry: usize = 3;
    let retry_delay = std::time::Duration::from_millis(250);
    let body: String = cnv::obj_to_json(send_obj)?;

    for i in 0..=n_retry {
        match one_post(&url, &body) {
            Ok(res) => { return Ok(res); },
            Err(__) => {
                if i == n_retry {
                    return Err(__);
                } else {
                    std::thread::sleep(retry_delay);
                    continue;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize, de::DeserializeOwned};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Request {
        uname: String,
        email: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Response {
        a: String,
        b: String,
    }

    #[test]
    /// You can start a sample server using "luban_util/tests/sample_server.py"
    fn test_http_post() {
        let req = Request {
            uname: "luban".to_string(),
            email: "luban@example.com".to_string(),
        };
        let resp = crate::http_post("http://localhost:50000/test", &req); // this is a result type
        use std::io::Write;
        let mut cout = std::io::stdout();
        writeln!(cout, "{:#?}", resp);
    }
}