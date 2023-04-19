use std::time::Duration;

use crate::{conversion as cnv, exception_names as EXN, *};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};
use serde::{de::DeserializeOwned, Serialize};

fn one_post<RecvT>(url: &str, body: &str) -> Outcome<RecvT>
where
    RecvT: DeserializeOwned,
{
    let client: Client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .catch(EXN::DummyException, "Failed to create http client")?;
    let headers = {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h
    };
    let body = body.to_string();
    let res: String = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .catch(
            EXN::DummyException,
            &format!("Failed to send POST request to url \"{}\".", url),
        )?
        .text()
        .catch(
            EXN::DummyException,
            &format!("Response from url \"{}\" is not text", url),
        )?;
    let res_obj = cnv::json_to_obj(&res)
    .catch(
        EXN::DummyException,
        &format!(
            "Response from url \"{}\"\ncannot be parsed into type `{}`.\nThe response is \"\"\"\n{}\n\"\"\"", 
            url,
            std::any::type_name::<RecvT>(),
            &res[..std::cmp::min(512, res.len())]
        )
    )?;
    Ok(res_obj)
}

pub fn http_post<SendT, RecvT>(url: &str, send_obj: &SendT) -> Outcome<RecvT>
where
    SendT: Serialize,
    RecvT: DeserializeOwned,
{
    let n_retry: usize = 3;
    let retry_delay = std::time::Duration::from_millis(50);
    let body: String =
        cnv::obj_to_json(send_obj).catch(EXN::DummyException, "Request body is not valid JSON")?;
    let mut outcome = Err(Exception::dummy());

    for i in 0..=n_retry {
        match one_post(&url, &body) {
            Ok(res) => {
                outcome = Ok(res);
                break;
            }
            Err(e) => {
                outcome = Err(e);
                if i != n_retry {
                    std::thread::sleep(retry_delay);
                }
            }
        }
    }
    return outcome;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde::{Deserialize, Serialize};

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
        let resp: Outcome<Response> = crate::http_post("http://localhost:50000/test", &req); // this is a result type
        eprintln!("{:#?}", resp);
    }
}
