use crate::{
    build_request::BuildRequest,
    constants::*,
};
use failure::bail;
use reqwest::{
    header::HeaderValue,
    Request,
    header::CONTENT_TYPE,
};
use std::{
    default::Default,
    str::{
        FromStr,
    },
};
use url::{
    Url,
    percent_encoding::{
            utf8_percent_encode,
            USERINFO_ENCODE_SET
        }
};

/// A struct used to conncet with the build server, it stores
/// attributes necessary to make a connection and provides methods
/// to interact with the server, including the ability to request
/// a build.
pub struct BuildServer {
    host: String,
    port: u32,
    domain: String,
}


impl BuildServer {

    /// New up a BuildServer. The BuildServer holds information that
    /// allows us to connect to the actual build server.
    ///
    /// # Parameters
    ///
    /// * `host` - The name of the host, sans the domain.
    /// * `port` - The port number
    /// * `domain` - The domain name.
    ///
    /// # Returns
    ///
    /// A new instance of BuildServer
    pub fn new<I>(host:I, port: u32, domain: I)  -> Self
    where I: Into<String>
    {
        Self {
            host: host.into(),
            port,
            domain: domain.into(),
        }
    }

    /// Attempt to generate a url to make a build request, assembling the vrarious
    /// components necessary to build this Url.
    ///
    /// # Returns
    ///
    /// A Url instance that may be invoked to request a build on the server. This
    /// method is generally used by the request_build method and is exposed publicly
    /// for visualization purposes.
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
    pub fn request_build(
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

