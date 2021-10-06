use hyper::{HeaderMap, header::HeaderValue};
use juniper::GraphQLObject;

use crate::http::auth::AuthConfig;


/// Data about a logged-in user.
#[derive(Debug, GraphQLObject)]
pub(crate) struct User {
    pub(crate) username: String,
    pub(crate) display_name: String,
    pub(crate) roles: Vec<String>,
}

impl User {
    pub(crate) fn from_headers(headers: &HeaderMap, auth_config: &AuthConfig) -> Option<Self> {
        // We only read these header values if the auth proxy is enabled.
        if !auth_config.proxy.enabled {
            return None;
        }

        let as_utf8 = |v: &HeaderValue| String::from_utf8_lossy(v.as_bytes()).trim().to_owned();
        let username = as_utf8(headers.get(&auth_config.username_header)?);
        let display_name = as_utf8(headers.get(&auth_config.display_name_header)?);

        let roles = match headers.get(&auth_config.roles_header) {
            None => vec![],
            Some(roles_raw) => {
                String::from_utf8_lossy(roles_raw.as_bytes())
                    .split(',')
                    .map(|role| role.trim().to_owned())
                    .collect()
            },
        };

        Some(Self { username, display_name, roles })
    }
}
