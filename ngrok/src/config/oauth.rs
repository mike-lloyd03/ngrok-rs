use std::fmt::Display;

use crate::internals::proto::{
    Oauth,
    SecretString,
};

/// Oauth Options configuration
#[derive(Clone)]
pub struct OauthOptions {
    /// The OAuth provider to use
    provider: OauthProvider,

    /// The client ID, if a custom one is being used
    client_id: String,
    /// The client secret, if a custom one is being used
    client_secret: SecretString,

    /// Email addresses of users to authorize.
    allow_emails: Vec<String>,
    /// Email domains of users to authorize.
    allow_domains: Vec<String>,
    /// OAuth scopes to request from the provider.
    scopes: Vec<String>,
}

impl OauthOptions {
    /// Create a new [OauthOptions] for the given provider.
    pub fn new(provider: OauthProvider) -> Self {
        OauthOptions {
            provider,
            client_id: String::default(),
            client_secret: SecretString::default(),
            allow_emails: Vec::default(),
            allow_domains: Vec::default(),
            scopes: Vec::default(),
        }
    }

    /// Provide an OAuth client ID for custom apps.
    pub fn client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = id.into();
        self
    }

    /// Provide an OAuth client secret for custom apps.
    pub fn client_secret(mut self, secret: impl Into<String>) -> Self {
        self.client_secret = SecretString::from(secret.into());
        self
    }

    /// Append an email address to the list of allowed emails.
    pub fn allow_email(mut self, email: impl Into<String>) -> Self {
        self.allow_emails.push(email.into());
        self
    }
    /// Append an email domain to the list of allowed domains.
    pub fn allow_domain(mut self, domain: impl Into<String>) -> Self {
        self.allow_domains.push(domain.into());
        self
    }
    /// Append a scope to the list of scopes to request.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scopes.push(scope.into());
        self
    }
}

// transform into the wire protocol format
impl From<OauthOptions> for Oauth {
    fn from(o: OauthOptions) -> Self {
        Oauth {
            provider: o.provider.to_string(),
            client_id: o.client_id,
            client_secret: o.client_secret,
            sealed_client_secret: Default::default(), // unused in this context
            allow_emails: o.allow_emails,
            allow_domains: o.allow_domains,
            scopes: o.scopes,
        }
    }
}

/// OAuth provider to be used with OauthOptions
#[derive(Clone)]
pub enum OauthProvider {
    Amazon,
    Facebook,
    GitHub,
    GitLab,
    Google,
    LinkedIn,
    Microsoft,
    Twitch,
}

impl Display for OauthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OauthProvider::Amazon => "amazon",
                OauthProvider::Facebook => "facebook",
                OauthProvider::GitHub => "github",
                OauthProvider::GitLab => "gitlab",
                OauthProvider::Google => "google",
                OauthProvider::LinkedIn => "linkedin",
                OauthProvider::Microsoft => "microsoft",
                OauthProvider::Twitch => "twitch",
            }
        )
    }
}
