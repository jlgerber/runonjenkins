use std::default::Default;
use url::Url;
use std::str::FromStr;
use failure::bail;
use std::collections::HashMap;
use reqwest::{header::HeaderValue, Request, Response, header::CONTENT_TYPE,};
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
    /// New up a BuildServer. The BuildServer holds information that
    /// allows us to connect to the actual build server.
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
        match Url::from_str(format!("http://{}.{}:{}/{}", self.host, self.domain, self.port, BUILD_ROUTE).as_str()) {
            Ok(e) => Some(e),
            Err(_) => None
        }
    }

    // report on the request object's composition
    fn request_report(request: &Request) {
        println!("Request Information");
        println!("Request Headers");
        println!("{:#?}", request.headers());
        println!("Request URL");
        println!("{:?}", request.url());
        println!("");
        println!("Request Body");
        println!("{:#?}", request.body());
        println!("");
    }

    /// Request a build from the build server, providing information per the
    /// req
    ///
    /// # Parameters
    ///
    /// * `req` - an instance of BuildRequest which stores the user's job's information
    /// * `verbose` - should we print out info to stdout about the query
    /// * `dry_run` - are we simply fooling around or do we want to get stuff done?
    pub fn request(
        &self,
        req: &BuildRequest,
        verbose: bool,
        dry_run: bool
    ) -> Result<Option<reqwest::Response>, failure::Error> {

        let client = reqwest::Client::new();

        let route = self.request_route();
        if route.is_none() {
            bail!("Unable to call.request_route");
        }

        let route = route.unwrap();
        // convert the request to a json string
        let json = serde_json::to_string(&req.to_build_params())?;
        // url encode the string
        let json: String = utf8_percent_encode(&json, USERINFO_ENCODE_SET).collect();

        // why am i doing this instead of using client.post().json.send()?
        // because F*&ing Jenkins doesnt understand posted json data. it wants
        // x-www-form-urlencoded data. So we set the header manually, as well as
        // the body. fun
        let request = client.post(route)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(format!("json={}",json))
        .build().unwrap();

        if verbose || dry_run { Self::request_report(&request);}

        // execute the actual query
        if !dry_run {
            let res = client.execute(request);

            match res {
                Ok(mut res) => {
                    if verbose {
                        println!("Return Headers:\n{:#?}", res.headers());
                    }

                    println!("Return Status: {}\n", res.status());

                    if verbose {
                        // copy the response body directly to stdout
                        std::io::copy(&mut res, &mut std::io::stdout())?;
                    }

                    Ok(Some(res))
                },
                Err(e) => bail!("{}", e)
            }
        } else {
            println!("END DRY-RUN MODE");
            Ok(None)
        }
    }
}

impl Default for BuildServer {
    fn default() -> Self {
        Self::new(BUILD_SERVER, BUILD_SERVER_PORT, BUILD_DOMAIN)
    }
}

