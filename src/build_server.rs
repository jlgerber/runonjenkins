use std::default::Default;
use url::Url;
use std::str::FromStr;
use failure::bail;
use std::collections::HashMap;
use reqwest::{header::HeaderValue, header::CONTENT_TYPE,};
use crate::build_request::BuildRequest;
use crate::constants::*;
use url::form_urlencoded::{byte_serialize, parse};

use url::percent_encoding::{utf8_percent_encode, percent_decode, USERINFO_ENCODE_SET};
use std::str::Utf8Error;

pub struct BuildServer {
    host: String,
    port: u32,
    domain: String,
}


impl BuildServer {
    pub fn new<I>(host:I, port: u32, domain: I)  -> Self
    where I: Into<String>
    {
        Self {
            host: host.into(),
            port,
            domain: domain.into(),
        }
    }

    // generate a request route
    pub fn request_route(&self) -> Option<Url> {
        match Url::from_str(format!("http://{}.{}:{}/{}",self.host, self.domain, self.port, BUILD_ROUTE).as_str()) {
            Ok(e) => Some(e),
            Err(_) => None
        }
    }

    /// Request a build from the build server
    pub fn request(&self, req: &BuildRequest) -> Result<reqwest::Response, failure::Error> {

        let client = reqwest::Client::new();

        let mut hmap = HashMap::new();
        // TODO fix errors
        let route = self.request_route();
        if route.is_none() {
            bail!("Unable to call.request_route");
        }

        let route = route.unwrap();
        println!("requesting on route {:?}", route);
        println!("build parameters");
        //let params = serde_urlencoded::to_string(req.to_build_urlencodeable())?;
        println!("build params");
        println!("{:#?}", req.to_build_params() );
        let json = serde_json::to_string(&req.to_build_params())?;

        let json: String = utf8_percent_encode(&json, USERINFO_ENCODE_SET).collect();

        //let j: String = byte_serialize(json.as_bytes()).collect();
        hmap.insert("json".to_string(), &json);

        println!("hmap");
        println!("{:?}", hmap);
        let res = client.post(route)
        // .header(
        //     CONTENT_TYPE,
        //     HeaderValue::from_static("application/x-www-form-urlencoded"),
        // )
        .form(&hmap)
        .send();

        match res {
            Ok(mut res) => {
                println!("Headers:\n{:#?}", res.headers());
                println!("Status: {}\n", res.status());
                // copy the response body directly to stdout
                std::io::copy(&mut res, &mut std::io::stdout())?;

                Ok(res)
            },
            Err(e) => bail!("{}", e)
        }
    }
}

impl Default for BuildServer {
    fn default() -> Self {
        Self::new(BUILD_SERVER, BUILD_SERVER_PORT, BUILD_DOMAIN)
    }
}

