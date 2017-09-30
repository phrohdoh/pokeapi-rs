pub extern crate reqwest;
pub extern crate serde_json;

#[macro_use]
pub extern crate serde_derive;

#[macro_use]
pub extern crate error_chain;

pub mod errors;
use errors::{ErrorKind, ResultExt};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResult {
    Success(Pokemon),
    Error(ApiError)
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub base_experience: i64,
    pub height: i64,
    pub is_default: bool,
    pub order: i64,
    pub weight: i64,
}

pub enum QueryType<'q> {
    Id(u32),
    Name(&'q str),
}

impl Pokemon {
    const ENDPOINT: &'static str = "http://pokeapi.co/api/v2/pokemon";

    pub fn get(by: QueryType) -> errors::Result<Self> {
        match by {
            QueryType::Id(id) => Self::impl_get(&id.to_string()),
            QueryType::Name(name) => Self::impl_get(&name),
        }
    }

    fn impl_get(name_or_id: &str) -> errors::Result<Self> {
        let url = reqwest::Url::parse(&format!("{}/{}", Self::ENDPOINT, name_or_id)).chain_err(|| "Name given results in an invalid URL")?;
        let mut resp = reqwest::get(url.as_str()).chain_err(|| ErrorKind::RequestFailed(reqwest::Method::Get, url))?;

        // Sometimes pokeapi returns HTML
        let res = resp.json::<ApiResult>().chain_err(|| ErrorKind::UnexpectedResponseBody)?;

        match res {
            ApiResult::Success(pokemon) => Ok(pokemon),
            ApiResult::Error(e) => match resp.status() {
                reqwest::StatusCode::NotFound => Err(ErrorKind::NotFound.into()),
                _ => Err(ErrorKind::Msg(e.detail + " (message from remote service)").into())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
