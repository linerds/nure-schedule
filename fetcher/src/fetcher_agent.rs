use crate::ProxyError;

use std::time::Duration;

use ureq::{Agent, BodyReader, Proxy, config::ConfigBuilder, typestate::AgentScope};

#[derive(Clone, Debug)]
pub struct FetcherAgent(Agent);

impl Default for FetcherAgent {
    fn default() -> Self {
        Self(Self::config().build().into())
    }
}

impl FetcherAgent {
    fn config() -> ConfigBuilder<AgentScope> {
        Agent::config_builder()
            .https_only(true)
            .accept("application/json")
            .http_status_as_error(true)
            .max_idle_connections(5)
            .timeout_global(None) // TODO
            .timeout_connect(Some(Duration::from_secs(10))) // TODO
            .user_agent(concat!("LinerdsTimetable/", env!("CARGO_PKG_VERSION")))
    }

    /// The uri for `proxy` must be in the format of `<protocol>://<user>:<password>@<host>:port`. All parts except host are optional.
    /// Environment variables that will take precedence: `ALL_PROXY`, `all_proxy`, `HTTPS_PROXY`, `https_proxy`, `HTTP_PROXY`, `http_proxy`.
    pub fn new<'a>(proxy: impl Into<Option<&'a str>>) -> Result<Self, ProxyError> {
        let p = proxy.into().map_or_else(
            || Ok(Proxy::try_from_env()),
            |str| Proxy::new(str).map(Some),
        )?;
        Ok(Self(Self::config().proxy(p).build().into()))
    }

    pub fn request(&self, url: &str) -> Result<BodyReader<'static>, ureq::Error> {
        Ok(self
            .0
            .get(url)
            .call()?
            .into_body()
            .into_with_config()
            .limit(42 * 1024 * 1024) // bytes
            .reader())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{env, io::Read};

    fn real_ip() -> Result<String, Box<dyn std::error::Error>> {
        let mut real_ip = String::new();
        FetcherAgent(FetcherAgent::config().proxy(None).build().into())
            .request("https://dev.linerds.us/ip")?
            .read_to_string(&mut real_ip)?;
        Ok(real_ip)
    }

    #[test]
    #[should_panic = ""]
    fn invalid_proxy() {
        println!(
            "{:#?}",
            FetcherAgent::new("sock:/127.0.0.1:9050")
                .unwrap()
                .0
                .config()
                .proxy()
        );
    }

    #[test]
    #[ignore = "needs Tor, Linerds VPS sees real ip"]
    fn proxy() -> Result<(), Box<dyn std::error::Error>> {
        let mut proxied_ip = String::new();
        FetcherAgent::new("socks5://127.0.0.1:9050")?
            .request("https://dev.linerds.us/ip")?
            .read_to_string(&mut proxied_ip)?;

        println!("{proxied_ip}");

        assert_ne!(proxied_ip, real_ip()?);

        Ok(())
    }

    #[test]
    #[ignore = "needs Tor, Linerds VPS sees your ip"]
    fn env_proxy() -> Result<(), Box<dyn std::error::Error>> {
        // build.rs currently sets them, just in case
        assert!(
            env::var("ALL_PROXY").is_ok()
                || env::var("HTTP_PROXY").is_ok()
                || env::var("HTTPS_PROXY").is_ok(),
            "Set one of ALL_PROXY, HTTP_PROXY or HTTPS_PROXY environment variables!"
        );

        let mut proxied_ip = String::new();
        FetcherAgent::default()
            .request("https://dev.linerds.us/ip")?
            .read_to_string(&mut proxied_ip)?;

        println!("{proxied_ip}");

        assert_ne!(proxied_ip, real_ip()?);

        Ok(())
    }
}
