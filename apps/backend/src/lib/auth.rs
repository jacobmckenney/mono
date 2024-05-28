use actix_web::{cookie::Cookie, dev::ServiceRequest, HttpMessage, HttpRequest};
use db::entities::user;
use log::warn;
use reqwest::{header::AUTHORIZATION, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthLink {
    url: String,
}
#[derive(Clone)]
pub struct AuthClient {
    pub google: OAuthConfig,
    pub microsoft: OAuthConfig,
}

#[derive(Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokens {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleProfile {
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub picture: String,
}

const GOOGLE_OAUTH_BASE: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_BASE: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug, Serialize, Deserialize)]
pub struct MicrosoftTokens {
    pub access_token: String,
    pub id_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MicrosoftTokenResponse {
    pub access_token: String,
    pub id_token: String,
}

const MICROSOFT_OAUTH_BASE: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";
const MICROSOFT_TOKEN_BASE: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

const AUTH_COOKIE_KEY: &str = "ekklesia";

pub trait AuthCookieExtractor {
    fn extract_auth_cookie(&self) -> Result<String, String>;
}

impl AuthCookieExtractor for ServiceRequest {
    fn extract_auth_cookie(&self) -> Result<String, String> {
        return match self
            .cookie(AUTH_COOKIE_KEY)
            .map(|cookie| cookie.value().to_string())
        {
            Some(value) => Ok(value),
            None => Err("Auth cookie not found".to_string()),
        };
    }
}

impl AuthCookieExtractor for HttpRequest {
    fn extract_auth_cookie(&self) -> Result<String, String> {
        return match self
            .cookie(AUTH_COOKIE_KEY)
            .map(|cookie| cookie.value().to_string())
        {
            Some(value) => Ok(value),
            None => Err("Auth cookie not found".to_string()),
        };
    }
}

pub fn extract_auth_cookie(req: &ServiceRequest) -> Result<String, String> {
    let cookie = req
        .cookie(AUTH_COOKIE_KEY)
        .map(|cookie| cookie.value().to_string());
    return match cookie {
        Some(value) => Ok(value),
        None => Err("Auth cookie not found".to_string()),
    };
}

pub fn build_auth_cookie(user: &user::Model) -> Cookie {
    // TODO: encode user id and email in cookie
    return Cookie::build(AUTH_COOKIE_KEY, user.email.clone())
        .domain("localhost")
        .path("/")
        .http_only(true)
        .finish();
}

pub fn get_user(req: HttpRequest) -> Result<user::Model, String> {
    return req
        .extensions()
        .get::<user::Model>()
        .map(|user| user.clone())
        .ok_or("User not found".to_string());
}

impl AuthClient {
    pub fn google_get_sign_in_link(&self) -> AuthLink {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("client_id", self.google.client_id.as_str()));
        params.push(("redirect_uri", self.google.redirect_uri.as_str()));
        params.push(("scope", "openid email profile"));
        params.push(("response_type", "code"));

        let url = reqwest::Url::parse_with_params(GOOGLE_OAUTH_BASE, params)
            .unwrap()
            .to_string();
        return AuthLink { url };
    }

    pub async fn google_get_tokens(&self, code: &str) -> Result<GoogleTokens, String> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("client_id", self.google.client_id.as_str()));
        params.push(("client_secret", self.google.client_secret.as_str()));
        params.push(("redirect_uri", self.google.redirect_uri.as_str()));
        params.push(("code", code));
        params.push(("grant_type", "authorization_code"));

        let res: Response = reqwest::Client::new()
            .post(GOOGLE_TOKEN_BASE)
            .form(&params)
            .send()
            .await
            .unwrap();
        if res.status().is_success() {
            let json: GoogleTokens = res.json().await.unwrap();
            return Ok(json);
        }

        warn!("Failed to get tokens {:?}", res.text().await.unwrap());
        return Err("Failed to get access token".to_string());
    }

    pub async fn google_get_profile(&self, access_token: &str) -> Result<GoogleProfile, String> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .header(AUTHORIZATION, format!("Bearer {:?}", access_token))
            .send()
            .await
            .unwrap();
        if res.status().is_success() {
            let json: GoogleProfile = res.json().await.unwrap();
            return Ok(json);
        }

        warn!("Failed to get profile {:?}", res.text().await.unwrap());
        return Err(String::from("Failed to get profile"));
    }

    pub fn microsoft_get_sign_in_link(&self) -> AuthLink {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("client_id", self.microsoft.client_id.as_str()));
        params.push(("redirect_uri", self.microsoft.redirect_uri.as_str()));
        params.push(("scope", "openid email profile"));
        params.push(("response_type", "code"));

        let url = reqwest::Url::parse_with_params(MICROSOFT_OAUTH_BASE, params)
            .unwrap()
            .to_string();
        return AuthLink { url };
    }

    pub async fn microsoft_get_tokens(
        &self,
        code: &str,
    ) -> Result<microsoft_jwt::IdTokenClaims, String> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("client_id", self.microsoft.client_id.as_str()));
        params.push(("client_secret", self.microsoft.client_secret.as_str()));
        params.push(("redirect_uri", self.microsoft.redirect_uri.as_str()));
        params.push(("code", code));
        params.push(("grant_type", "authorization_code"));
        let res: Response = reqwest::Client::new()
            .post(MICROSOFT_TOKEN_BASE)
            .form(&params)
            .send()
            .await
            .unwrap();
        if res.status().is_success() {
            let json: MicrosoftTokens = res.json().await.unwrap();
            let verified = microsoft_jwt::verify_jwt(&json.id_token, &self.microsoft.client_id)
                .await
                .unwrap();
            return Ok(verified.claims);
        }

        warn!("Failed to get profile {:?}", res.text().await.unwrap());
        return Err(String::from("Failed to get profile"));
    }
}

mod microsoft_jwt {
    use base64::prelude::*;
    use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IdTokenClaims {
        // Add known fields here based on your documentation
        pub sub: Option<String>,
        pub aud: Option<String>,
        pub exp: Option<usize>,
        pub iss: Option<String>,
        pub name: String,
        pub email: String,
        pub preferred_username: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Jwk {
        kty: String,
        kid: String,
        n: String,
        e: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Jwks {
        keys: Vec<Jwk>,
    }

    async fn fetch_jwks(jwks_url: &str) -> Result<Jwks, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(jwks_url).send().await?.json::<Jwks>().await?;
        Ok(response)
    }

    fn get_decoding_key(jwks: &Jwks, kid: &str) -> Option<DecodingKey> {
        for jwk in &jwks.keys {
            if jwk.kid == kid {
                let n = BASE64_URL_SAFE_NO_PAD.decode(&jwk.n).ok()?;
                let e = BASE64_URL_SAFE_NO_PAD.decode(&jwk.e).ok()?;
                return Some(DecodingKey::from_rsa_raw_components(&n, &e));
            }
        }
        return None;
    }

    pub async fn verify_jwt(
        id_token: &str,
        client_id: &str,
    ) -> Result<TokenData<IdTokenClaims>, Box<dyn std::error::Error>> {
        let jwks_url = "https://login.microsoftonline.com/common/discovery/v2.0/keys";

        // Fetch the JWKS
        let jwks = fetch_jwks(jwks_url).await?;

        // Decode the token header to get the key ID (kid)
        let header = decode_header(id_token)?;
        let kid = match header.kid {
            Some(k) => k,
            None => return Err("Token has no 'kid' field".into()),
        };

        // Get the decoding key
        let decoding_key = get_decoding_key(&jwks, &kid).ok_or("Decoding key not found")?;

        // Decode and verify the token
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);
        let token_data = decode::<IdTokenClaims>(id_token, &decoding_key, &validation)?;

        Ok(token_data)
    }
}
