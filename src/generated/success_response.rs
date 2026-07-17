#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`AgentInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent_status\","]
#[doc = "    \"focused\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"revision\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"terminal_id\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_session\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/AgentSessionInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_status\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"display_agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"foreground_cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"screen_detection_skipped\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"state_labels\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"terminal_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"terminal_title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal_title_stripped\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 32,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session: ::std::option::Option<AgentSessionInfo>,
    pub agent_status: AgentStatus,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display_agent: ::std::option::Option<::std::string::String>,
    pub focused: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub foreground_cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    pub revision: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub screen_detection_skipped: ::std::option::Option<bool>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub state_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub tab_id: ::std::string::String,
    pub terminal_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub terminal_title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub terminal_title_stripped: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub tokens: ::std::collections::HashMap<AgentInfoTokensKey, ::std::string::String>,
    pub workspace_id: ::std::string::String,
}
impl AgentInfo {
    pub fn builder() -> builder::AgentInfo {
        Default::default()
    }
}
#[doc = "`AgentInfoTokensKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AgentInfoTokensKey(::std::string::String);
impl ::std::ops::Deref for AgentInfoTokensKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgentInfoTokensKey> for ::std::string::String {
    fn from(value: AgentInfoTokensKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AgentInfoTokensKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9_-]{1,32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9_-]{1,32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AgentInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AgentInfoTokensKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`AgentManifestInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"local_override_shadowing_remote\","]
#[doc = "    \"source\","]
#[doc = "    \"source_kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active_version\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"cached_remote_version\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"local_override_shadowing_remote\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"remote_last_checked_unix\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"remote_update_error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"remote_update_result\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_kind\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"warning\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentManifestInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub active_version: ::std::option::Option<::std::string::String>,
    pub agent: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cached_remote_version: ::std::option::Option<::std::string::String>,
    pub local_override_shadowing_remote: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remote_last_checked_unix: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remote_update_error: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remote_update_result: ::std::option::Option<::std::string::String>,
    pub source: ::std::string::String,
    pub source_kind: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub warning: ::std::option::Option<::std::string::String>,
}
impl AgentManifestInfo {
    pub fn builder() -> builder::AgentManifestInfo {
        Default::default()
    }
}
#[doc = "`AgentSessionInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"kind\","]
#[doc = "    \"source\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentSessionRefKind\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentSessionInfo {
    pub agent: ::std::string::String,
    pub kind: AgentSessionRefKind,
    pub source: ::std::string::String,
    pub value: ::std::string::String,
}
impl AgentSessionInfo {
    pub fn builder() -> builder::AgentSessionInfo {
        Default::default()
    }
}
#[doc = "`AgentSessionRefKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"id\","]
#[doc = "    \"path\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AgentSessionRefKind {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "path")]
    Path,
}
impl ::std::fmt::Display for AgentSessionRefKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Id => f.write_str("id"),
            Self::Path => f.write_str("path"),
        }
    }
}
impl ::std::str::FromStr for AgentSessionRefKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "id" => Ok(Self::Id),
            "path" => Ok(Self::Path),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentSessionRefKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentSessionRefKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentSessionRefKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AgentStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"idle\","]
#[doc = "    \"working\","]
#[doc = "    \"blocked\","]
#[doc = "    \"done\","]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AgentStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("idle"),
            Self::Working => f.write_str("working"),
            Self::Blocked => f.write_str("blocked"),
            Self::Done => f.write_str("done"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for AgentStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "idle" => Ok(Self::Idle),
            "working" => Ok(Self::Working),
            "blocked" => Ok(Self::Blocked),
            "done" => Ok(Self::Done),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClientWindowTitleReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"set\","]
#[doc = "    \"cleared\","]
#[doc = "    \"no_foreground_client\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClientWindowTitleReason {
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "cleared")]
    Cleared,
    #[serde(rename = "no_foreground_client")]
    NoForegroundClient,
}
impl ::std::fmt::Display for ClientWindowTitleReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
            Self::Cleared => f.write_str("cleared"),
            Self::NoForegroundClient => f.write_str("no_foreground_client"),
        }
    }
}
impl ::std::str::FromStr for ClientWindowTitleReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            "cleared" => Ok(Self::Cleared),
            "no_foreground_client" => Ok(Self::NoForegroundClient),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientWindowTitleReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClientWindowTitleReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientWindowTitleReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ConfigReloadStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"applied\","]
#[doc = "    \"partial\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ConfigReloadStatus {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ConfigReloadStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Applied => f.write_str("applied"),
            Self::Partial => f.write_str("partial"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ConfigReloadStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "applied" => Ok(Self::Applied),
            "partial" => Ok(Self::Partial),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigReloadStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConfigReloadStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigReloadStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EventData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_created\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_updated\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_metadata_updated\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_closed\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"label\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_renamed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"insert_index\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\","]
#[doc = "        \"workspaces\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"insert_index\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_moved\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"workspaces\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_focused\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace\","]
#[doc = "        \"worktree\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_created\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        },"]
#[doc = "        \"worktree\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"already_open\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace\","]
#[doc = "        \"worktree\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"already_open\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_opened\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        },"]
#[doc = "        \"worktree\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"forced\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\","]
#[doc = "        \"worktree\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"forced\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_removed\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"worktree\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tab\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_created\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tab_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_closed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"label\","]
#[doc = "        \"tab_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_renamed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"insert_index\","]
#[doc = "        \"tab_id\","]
#[doc = "        \"tabs\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"insert_index\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"tabs\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_moved\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tab_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_focused\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_created\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_closed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_updated\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_focused\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane\","]
#[doc = "        \"previous_pane_id\","]
#[doc = "        \"previous_tab_id\","]
#[doc = "        \"previous_workspace_id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"closed_tab_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"closed_workspace_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"created_tab\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"created_workspace\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"previous_pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"previous_tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"previous_workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_moved\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"revision\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"revision\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint64\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_output_changed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_exited\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_agent_detected\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agent_status\","]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"agent_status\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "        },"]
#[doc = "        \"display_agent\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"state_labels\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"title\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_agent_status_changed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"layout\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"layout\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout_updated\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum EventData {
    #[serde(rename = "workspace_created")]
    WorkspaceCreated { workspace: WorkspaceInfo },
    #[serde(rename = "workspace_updated")]
    WorkspaceUpdated { workspace: WorkspaceInfo },
    #[serde(rename = "workspace_metadata_updated")]
    WorkspaceMetadataUpdated { workspace: WorkspaceInfo },
    #[serde(rename = "workspace_closed")]
    WorkspaceClosed {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace: ::std::option::Option<WorkspaceInfo>,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "workspace_renamed")]
    WorkspaceRenamed {
        label: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "workspace_moved")]
    WorkspaceMoved {
        insert_index: u32,
        workspace_id: ::std::string::String,
        workspaces: ::std::vec::Vec<WorkspaceInfo>,
    },
    #[serde(rename = "workspace_focused")]
    WorkspaceFocused { workspace_id: ::std::string::String },
    #[serde(rename = "worktree_created")]
    WorktreeCreated {
        workspace: WorkspaceInfo,
        worktree: WorktreeInfo,
    },
    #[serde(rename = "worktree_opened")]
    WorktreeOpened {
        already_open: bool,
        workspace: WorkspaceInfo,
        worktree: WorktreeInfo,
    },
    #[serde(rename = "worktree_removed")]
    WorktreeRemoved {
        forced: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace: ::std::option::Option<WorkspaceInfo>,
        workspace_id: ::std::string::String,
        worktree: WorktreeInfo,
    },
    #[serde(rename = "tab_created")]
    TabCreated { tab: TabInfo },
    #[serde(rename = "tab_closed")]
    TabClosed {
        tab_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "tab_renamed")]
    TabRenamed {
        label: ::std::string::String,
        tab_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "tab_moved")]
    TabMoved {
        insert_index: u32,
        tab_id: ::std::string::String,
        tabs: ::std::vec::Vec<TabInfo>,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "tab_focused")]
    TabFocused {
        tab_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_created")]
    PaneCreated { pane: PaneInfo },
    #[serde(rename = "pane_closed")]
    PaneClosed {
        pane_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_updated")]
    PaneUpdated { pane: PaneInfo },
    #[serde(rename = "pane_focused")]
    PaneFocused {
        pane_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_moved")]
    PaneMoved {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        closed_tab_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        closed_workspace_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        created_tab: ::std::option::Option<TabInfo>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        created_workspace: ::std::option::Option<WorkspaceInfo>,
        pane: PaneInfo,
        previous_pane_id: ::std::string::String,
        previous_tab_id: ::std::string::String,
        previous_workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_output_changed")]
    PaneOutputChanged {
        pane_id: ::std::string::String,
        revision: u64,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_exited")]
    PaneExited {
        pane_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_agent_detected")]
    PaneAgentDetected {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent: ::std::option::Option<::std::string::String>,
        pane_id: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "pane_agent_status_changed")]
    PaneAgentStatusChanged {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent: ::std::option::Option<::std::string::String>,
        agent_status: AgentStatus,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        display_agent: ::std::option::Option<::std::string::String>,
        pane_id: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        state_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        title: ::std::option::Option<::std::string::String>,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "layout_updated")]
    LayoutUpdated { layout: PaneLayoutSnapshot },
}
#[doc = "`EventEnvelope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"event\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventData\""]
#[doc = "    },"]
#[doc = "    \"event\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventKind\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EventEnvelope {
    pub data: EventData,
    pub event: EventKind,
}
impl EventEnvelope {
    pub fn builder() -> builder::EventEnvelope {
        Default::default()
    }
}
#[doc = "`EventKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"workspace_created\","]
#[doc = "    \"workspace_updated\","]
#[doc = "    \"workspace_metadata_updated\","]
#[doc = "    \"workspace_closed\","]
#[doc = "    \"workspace_renamed\","]
#[doc = "    \"workspace_moved\","]
#[doc = "    \"workspace_focused\","]
#[doc = "    \"worktree_created\","]
#[doc = "    \"worktree_opened\","]
#[doc = "    \"worktree_removed\","]
#[doc = "    \"tab_created\","]
#[doc = "    \"tab_closed\","]
#[doc = "    \"tab_renamed\","]
#[doc = "    \"tab_moved\","]
#[doc = "    \"tab_focused\","]
#[doc = "    \"pane_created\","]
#[doc = "    \"pane_closed\","]
#[doc = "    \"pane_updated\","]
#[doc = "    \"pane_focused\","]
#[doc = "    \"pane_moved\","]
#[doc = "    \"pane_output_changed\","]
#[doc = "    \"pane_exited\","]
#[doc = "    \"pane_agent_detected\","]
#[doc = "    \"pane_agent_status_changed\","]
#[doc = "    \"layout_updated\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventKind {
    #[serde(rename = "workspace_created")]
    WorkspaceCreated,
    #[serde(rename = "workspace_updated")]
    WorkspaceUpdated,
    #[serde(rename = "workspace_metadata_updated")]
    WorkspaceMetadataUpdated,
    #[serde(rename = "workspace_closed")]
    WorkspaceClosed,
    #[serde(rename = "workspace_renamed")]
    WorkspaceRenamed,
    #[serde(rename = "workspace_moved")]
    WorkspaceMoved,
    #[serde(rename = "workspace_focused")]
    WorkspaceFocused,
    #[serde(rename = "worktree_created")]
    WorktreeCreated,
    #[serde(rename = "worktree_opened")]
    WorktreeOpened,
    #[serde(rename = "worktree_removed")]
    WorktreeRemoved,
    #[serde(rename = "tab_created")]
    TabCreated,
    #[serde(rename = "tab_closed")]
    TabClosed,
    #[serde(rename = "tab_renamed")]
    TabRenamed,
    #[serde(rename = "tab_moved")]
    TabMoved,
    #[serde(rename = "tab_focused")]
    TabFocused,
    #[serde(rename = "pane_created")]
    PaneCreated,
    #[serde(rename = "pane_closed")]
    PaneClosed,
    #[serde(rename = "pane_updated")]
    PaneUpdated,
    #[serde(rename = "pane_focused")]
    PaneFocused,
    #[serde(rename = "pane_moved")]
    PaneMoved,
    #[serde(rename = "pane_output_changed")]
    PaneOutputChanged,
    #[serde(rename = "pane_exited")]
    PaneExited,
    #[serde(rename = "pane_agent_detected")]
    PaneAgentDetected,
    #[serde(rename = "pane_agent_status_changed")]
    PaneAgentStatusChanged,
    #[serde(rename = "layout_updated")]
    LayoutUpdated,
}
impl ::std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WorkspaceCreated => f.write_str("workspace_created"),
            Self::WorkspaceUpdated => f.write_str("workspace_updated"),
            Self::WorkspaceMetadataUpdated => f.write_str("workspace_metadata_updated"),
            Self::WorkspaceClosed => f.write_str("workspace_closed"),
            Self::WorkspaceRenamed => f.write_str("workspace_renamed"),
            Self::WorkspaceMoved => f.write_str("workspace_moved"),
            Self::WorkspaceFocused => f.write_str("workspace_focused"),
            Self::WorktreeCreated => f.write_str("worktree_created"),
            Self::WorktreeOpened => f.write_str("worktree_opened"),
            Self::WorktreeRemoved => f.write_str("worktree_removed"),
            Self::TabCreated => f.write_str("tab_created"),
            Self::TabClosed => f.write_str("tab_closed"),
            Self::TabRenamed => f.write_str("tab_renamed"),
            Self::TabMoved => f.write_str("tab_moved"),
            Self::TabFocused => f.write_str("tab_focused"),
            Self::PaneCreated => f.write_str("pane_created"),
            Self::PaneClosed => f.write_str("pane_closed"),
            Self::PaneUpdated => f.write_str("pane_updated"),
            Self::PaneFocused => f.write_str("pane_focused"),
            Self::PaneMoved => f.write_str("pane_moved"),
            Self::PaneOutputChanged => f.write_str("pane_output_changed"),
            Self::PaneExited => f.write_str("pane_exited"),
            Self::PaneAgentDetected => f.write_str("pane_agent_detected"),
            Self::PaneAgentStatusChanged => f.write_str("pane_agent_status_changed"),
            Self::LayoutUpdated => f.write_str("layout_updated"),
        }
    }
}
impl ::std::str::FromStr for EventKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "workspace_created" => Ok(Self::WorkspaceCreated),
            "workspace_updated" => Ok(Self::WorkspaceUpdated),
            "workspace_metadata_updated" => Ok(Self::WorkspaceMetadataUpdated),
            "workspace_closed" => Ok(Self::WorkspaceClosed),
            "workspace_renamed" => Ok(Self::WorkspaceRenamed),
            "workspace_moved" => Ok(Self::WorkspaceMoved),
            "workspace_focused" => Ok(Self::WorkspaceFocused),
            "worktree_created" => Ok(Self::WorktreeCreated),
            "worktree_opened" => Ok(Self::WorktreeOpened),
            "worktree_removed" => Ok(Self::WorktreeRemoved),
            "tab_created" => Ok(Self::TabCreated),
            "tab_closed" => Ok(Self::TabClosed),
            "tab_renamed" => Ok(Self::TabRenamed),
            "tab_moved" => Ok(Self::TabMoved),
            "tab_focused" => Ok(Self::TabFocused),
            "pane_created" => Ok(Self::PaneCreated),
            "pane_closed" => Ok(Self::PaneClosed),
            "pane_updated" => Ok(Self::PaneUpdated),
            "pane_focused" => Ok(Self::PaneFocused),
            "pane_moved" => Ok(Self::PaneMoved),
            "pane_output_changed" => Ok(Self::PaneOutputChanged),
            "pane_exited" => Ok(Self::PaneExited),
            "pane_agent_detected" => Ok(Self::PaneAgentDetected),
            "pane_agent_status_changed" => Ok(Self::PaneAgentStatusChanged),
            "layout_updated" => Ok(Self::LayoutUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InstalledPluginInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enabled\","]
#[doc = "    \"manifest_path\","]
#[doc = "    \"name\","]
#[doc = "    \"plugin_id\","]
#[doc = "    \"plugin_root\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginManifestAction\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"build\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginManifestBuild\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"events\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginManifestEventHook\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"link_handlers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginManifestLinkHandler\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"manifest_path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"min_herdr_version\": {"]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"panes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginManifestPane\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"plugin_root\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"kind\": \"local\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/$defs/PluginSourceInfo\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"warnings\": {"]
#[doc = "      \"description\": \"Warnings collected at link time or on registry load (e.g. unknown event names,\\nmissing manifest file). Non-fatal — the entry is kept and surfaced by plugin.list.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InstalledPluginInfo {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub actions: ::std::vec::Vec<PluginManifestAction>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub build: ::std::vec::Vec<PluginManifestBuild>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub events: ::std::vec::Vec<PluginManifestEventHook>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub link_handlers: ::std::vec::Vec<PluginManifestLinkHandler>,
    pub manifest_path: ::std::string::String,
    #[serde(default)]
    pub min_herdr_version: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub panes: ::std::vec::Vec<PluginManifestPane>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
    pub plugin_id: ::std::string::String,
    pub plugin_root: ::std::string::String,
    #[serde(default = "defaults::installed_plugin_info_source")]
    pub source: PluginSourceInfo,
    pub version: ::std::string::String,
    #[doc = "Warnings collected at link time or on registry load (e.g. unknown event names,\nmissing manifest file). Non-fatal — the entry is kept and surfaced by plugin.list."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub warnings: ::std::vec::Vec<::std::string::String>,
}
impl InstalledPluginInfo {
    pub fn builder() -> builder::InstalledPluginInfo {
        Default::default()
    }
}
#[doc = "`IntegrationInstallResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct IntegrationInstallResult {
    pub messages: ::std::vec::Vec<::std::string::String>,
}
impl IntegrationInstallResult {
    pub fn builder() -> builder::IntegrationInstallResult {
        Default::default()
    }
}
#[doc = "`IntegrationTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pi\","]
#[doc = "    \"omp\","]
#[doc = "    \"claude\","]
#[doc = "    \"codex\","]
#[doc = "    \"copilot\","]
#[doc = "    \"devin\","]
#[doc = "    \"droid\","]
#[doc = "    \"kimi\","]
#[doc = "    \"opencode\","]
#[doc = "    \"kilo\","]
#[doc = "    \"hermes\","]
#[doc = "    \"qodercli\","]
#[doc = "    \"cursor\","]
#[doc = "    \"mastracode\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum IntegrationTarget {
    #[serde(rename = "pi")]
    Pi,
    #[serde(rename = "omp")]
    Omp,
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "copilot")]
    Copilot,
    #[serde(rename = "devin")]
    Devin,
    #[serde(rename = "droid")]
    Droid,
    #[serde(rename = "kimi")]
    Kimi,
    #[serde(rename = "opencode")]
    Opencode,
    #[serde(rename = "kilo")]
    Kilo,
    #[serde(rename = "hermes")]
    Hermes,
    #[serde(rename = "qodercli")]
    Qodercli,
    #[serde(rename = "cursor")]
    Cursor,
    #[serde(rename = "mastracode")]
    Mastracode,
}
impl ::std::fmt::Display for IntegrationTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pi => f.write_str("pi"),
            Self::Omp => f.write_str("omp"),
            Self::Claude => f.write_str("claude"),
            Self::Codex => f.write_str("codex"),
            Self::Copilot => f.write_str("copilot"),
            Self::Devin => f.write_str("devin"),
            Self::Droid => f.write_str("droid"),
            Self::Kimi => f.write_str("kimi"),
            Self::Opencode => f.write_str("opencode"),
            Self::Kilo => f.write_str("kilo"),
            Self::Hermes => f.write_str("hermes"),
            Self::Qodercli => f.write_str("qodercli"),
            Self::Cursor => f.write_str("cursor"),
            Self::Mastracode => f.write_str("mastracode"),
        }
    }
}
impl ::std::str::FromStr for IntegrationTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pi" => Ok(Self::Pi),
            "omp" => Ok(Self::Omp),
            "claude" => Ok(Self::Claude),
            "codex" => Ok(Self::Codex),
            "copilot" => Ok(Self::Copilot),
            "devin" => Ok(Self::Devin),
            "droid" => Ok(Self::Droid),
            "kimi" => Ok(Self::Kimi),
            "opencode" => Ok(Self::Opencode),
            "kilo" => Ok(Self::Kilo),
            "hermes" => Ok(Self::Hermes),
            "qodercli" => Ok(Self::Qodercli),
            "cursor" => Ok(Self::Cursor),
            "mastracode" => Ok(Self::Mastracode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IntegrationTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`IntegrationUninstallResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct IntegrationUninstallResult {
    pub messages: ::std::vec::Vec<::std::string::String>,
}
impl IntegrationUninstallResult {
    pub fn builder() -> builder::IntegrationUninstallResult {
        Default::default()
    }
}
#[doc = "`LayoutDescription`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"root\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"workspace_id\","]
#[doc = "    \"zoomed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"root\": {"]
#[doc = "      \"$ref\": \"#/$defs/LayoutNode\""]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"zoomed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LayoutDescription {
    pub focused_pane_id: ::std::string::String,
    pub root: LayoutNode,
    pub tab_id: ::std::string::String,
    pub workspace_id: ::std::string::String,
    pub zoomed: bool,
}
impl LayoutDescription {
    pub fn builder() -> builder::LayoutDescription {
        Default::default()
    }
}
#[doc = "`LayoutNode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"command\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"cwd\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"env\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"direction\","]
#[doc = "        \"first\","]
#[doc = "        \"ratio\","]
#[doc = "        \"second\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"direction\": {"]
#[doc = "          \"$ref\": \"#/$defs/SplitDirection\""]
#[doc = "        },"]
#[doc = "        \"first\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutNode\""]
#[doc = "        },"]
#[doc = "        \"ratio\": {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\""]
#[doc = "        },"]
#[doc = "        \"second\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutNode\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"split\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum LayoutNode {
    #[serde(rename = "pane")]
    Pane {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        command: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        cwd: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pane_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "split")]
    Split {
        direction: SplitDirection,
        first: ::std::boxed::Box<LayoutNode>,
        ratio: f32,
        second: ::std::boxed::Box<LayoutNode>,
    },
}
#[doc = "`NotificationShowReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"shown\","]
#[doc = "    \"disabled\","]
#[doc = "    \"rate_limited\","]
#[doc = "    \"no_foreground_client\","]
#[doc = "    \"busy\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NotificationShowReason {
    #[serde(rename = "shown")]
    Shown,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "rate_limited")]
    RateLimited,
    #[serde(rename = "no_foreground_client")]
    NoForegroundClient,
    #[serde(rename = "busy")]
    Busy,
}
impl ::std::fmt::Display for NotificationShowReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shown => f.write_str("shown"),
            Self::Disabled => f.write_str("disabled"),
            Self::RateLimited => f.write_str("rate_limited"),
            Self::NoForegroundClient => f.write_str("no_foreground_client"),
            Self::Busy => f.write_str("busy"),
        }
    }
}
impl ::std::str::FromStr for NotificationShowReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shown" => Ok(Self::Shown),
            "disabled" => Ok(Self::Disabled),
            "rate_limited" => Ok(Self::RateLimited),
            "no_foreground_client" => Ok(Self::NoForegroundClient),
            "busy" => Ok(Self::Busy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NotificationShowReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NotificationShowReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NotificationShowReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneDirection`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"up\","]
#[doc = "    \"down\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneDirection {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::fmt::Display for PaneDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
        }
    }
}
impl ::std::str::FromStr for PaneDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneEdgesResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"down\","]
#[doc = "    \"layout\","]
#[doc = "    \"left\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"right\","]
#[doc = "    \"up\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"down\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"left\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"right\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"up\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneEdgesResult {
    pub down: bool,
    pub layout: PaneLayoutSnapshot,
    pub left: bool,
    pub pane_id: ::std::string::String,
    pub right: bool,
    pub up: bool,
}
impl PaneEdgesResult {
    pub fn builder() -> builder::PaneEdgesResult {
        Default::default()
    }
}
#[doc = "`PaneFocusDirectionReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"no_neighbor\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneFocusDirectionReason {
    #[serde(rename = "no_neighbor")]
    NoNeighbor,
}
impl ::std::fmt::Display for PaneFocusDirectionReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NoNeighbor => f.write_str("no_neighbor"),
        }
    }
}
impl ::std::str::FromStr for PaneFocusDirectionReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "no_neighbor" => Ok(Self::NoNeighbor),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneFocusDirectionReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneFocusDirectionReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneFocusDirectionReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneFocusDirectionResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changed\","]
#[doc = "    \"layout\","]
#[doc = "    \"source_pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneFocusDirectionReason\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneFocusDirectionResult {
    pub changed: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_id: ::std::option::Option<::std::string::String>,
    pub layout: PaneLayoutSnapshot,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<PaneFocusDirectionReason>,
    pub source_pane_id: ::std::string::String,
}
impl PaneFocusDirectionResult {
    pub fn builder() -> builder::PaneFocusDirectionResult {
        Default::default()
    }
}
#[doc = "`PaneInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent_status\","]
#[doc = "    \"focused\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"revision\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"terminal_id\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_session\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/AgentSessionInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_status\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"display_agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"foreground_cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"scroll\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneScrollInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"state_labels\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"terminal_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"terminal_title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal_title_stripped\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 32,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session: ::std::option::Option<AgentSessionInfo>,
    pub agent_status: AgentStatus,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display_agent: ::std::option::Option<::std::string::String>,
    pub focused: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub foreground_cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    pub revision: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scroll: ::std::option::Option<PaneScrollInfo>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub state_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub tab_id: ::std::string::String,
    pub terminal_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub terminal_title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub terminal_title_stripped: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub tokens: ::std::collections::HashMap<PaneInfoTokensKey, ::std::string::String>,
    pub workspace_id: ::std::string::String,
}
impl PaneInfo {
    pub fn builder() -> builder::PaneInfo {
        Default::default()
    }
}
#[doc = "`PaneInfoTokensKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PaneInfoTokensKey(::std::string::String);
impl ::std::ops::Deref for PaneInfoTokensKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PaneInfoTokensKey> for ::std::string::String {
    fn from(value: PaneInfoTokensKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PaneInfoTokensKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9_-]{1,32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9_-]{1,32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PaneInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PaneInfoTokensKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`PaneLayoutPane`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"focused\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"rect\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"focused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"rect\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutRect\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneLayoutPane {
    pub focused: bool,
    pub pane_id: ::std::string::String,
    pub rect: PaneLayoutRect,
}
impl PaneLayoutPane {
    pub fn builder() -> builder::PaneLayoutPane {
        Default::default()
    }
}
#[doc = "`PaneLayoutRect`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"height\","]
#[doc = "    \"width\","]
#[doc = "    \"x\","]
#[doc = "    \"y\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"maximum\": 65535.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"maximum\": 65535.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"maximum\": 65535.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"y\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"maximum\": 65535.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneLayoutRect {
    pub height: u16,
    pub width: u16,
    pub x: u16,
    pub y: u16,
}
impl PaneLayoutRect {
    pub fn builder() -> builder::PaneLayoutRect {
        Default::default()
    }
}
#[doc = "`PaneLayoutSnapshot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"panes\","]
#[doc = "    \"splits\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"workspace_id\","]
#[doc = "    \"zoomed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutRect\""]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"panes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PaneLayoutPane\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"splits\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PaneLayoutSplit\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"zoomed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneLayoutSnapshot {
    pub area: PaneLayoutRect,
    pub focused_pane_id: ::std::string::String,
    pub panes: ::std::vec::Vec<PaneLayoutPane>,
    pub splits: ::std::vec::Vec<PaneLayoutSplit>,
    pub tab_id: ::std::string::String,
    pub workspace_id: ::std::string::String,
    pub zoomed: bool,
}
impl PaneLayoutSnapshot {
    pub fn builder() -> builder::PaneLayoutSnapshot {
        Default::default()
    }
}
#[doc = "`PaneLayoutSplit`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\","]
#[doc = "    \"id\","]
#[doc = "    \"ratio\","]
#[doc = "    \"rect\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/SplitDirection\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ratio\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"rect\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutRect\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneLayoutSplit {
    pub direction: SplitDirection,
    pub id: ::std::string::String,
    pub ratio: f32,
    pub rect: PaneLayoutRect,
}
impl PaneLayoutSplit {
    pub fn builder() -> builder::PaneLayoutSplit {
        Default::default()
    }
}
#[doc = "`PaneMoveReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"same_tab\","]
#[doc = "    \"zoomed_tab\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneMoveReason {
    #[serde(rename = "same_tab")]
    SameTab,
    #[serde(rename = "zoomed_tab")]
    ZoomedTab,
}
impl ::std::fmt::Display for PaneMoveReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SameTab => f.write_str("same_tab"),
            Self::ZoomedTab => f.write_str("zoomed_tab"),
        }
    }
}
impl ::std::str::FromStr for PaneMoveReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "same_tab" => Ok(Self::SameTab),
            "zoomed_tab" => Ok(Self::ZoomedTab),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneMoveReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneMoveReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneMoveReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneMoveResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changed\","]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"pane\","]
#[doc = "    \"previous_pane_id\","]
#[doc = "    \"previous_tab_id\","]
#[doc = "    \"previous_workspace_id\","]
#[doc = "    \"target_layout\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"closed_tab_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"closed_workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created_tab\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created_workspace\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pane\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "    },"]
#[doc = "    \"previous_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"previous_tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"previous_workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneMoveReason\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_layout\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"target_layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneMoveResult {
    pub changed: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub closed_tab_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub closed_workspace_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created_tab: ::std::option::Option<TabInfo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created_workspace: ::std::option::Option<WorkspaceInfo>,
    pub focused_pane_id: ::std::string::String,
    pub pane: PaneInfo,
    pub previous_pane_id: ::std::string::String,
    pub previous_tab_id: ::std::string::String,
    pub previous_workspace_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<PaneMoveReason>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source_layout: ::std::option::Option<PaneLayoutSnapshot>,
    pub target_layout: PaneLayoutSnapshot,
}
impl PaneMoveResult {
    pub fn builder() -> builder::PaneMoveResult {
        Default::default()
    }
}
#[doc = "`PaneNeighborResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\","]
#[doc = "    \"layout\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneDirection\""]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"neighbor_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneNeighborResult {
    pub direction: PaneDirection,
    pub layout: PaneLayoutSnapshot,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub neighbor_pane_id: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
}
impl PaneNeighborResult {
    pub fn builder() -> builder::PaneNeighborResult {
        Default::default()
    }
}
#[doc = "`PaneProcessInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"foreground_process_group_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"foreground_processes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PaneProcessInfoProcess\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"shell_pid\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tty\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneProcessInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub foreground_process_group_id: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub foreground_processes: ::std::vec::Vec<PaneProcessInfoProcess>,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shell_pid: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tty: ::std::option::Option<::std::string::String>,
}
impl PaneProcessInfo {
    pub fn builder() -> builder::PaneProcessInfo {
        Default::default()
    }
}
#[doc = "`PaneProcessInfoProcess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"pid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"argv\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"argv0\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cmdline\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pid\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneProcessInfoProcess {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub argv: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub argv0: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cmdline: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub pid: u32,
}
impl PaneProcessInfoProcess {
    pub fn builder() -> builder::PaneProcessInfoProcess {
        Default::default()
    }
}
#[doc = "`PaneReadResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"format\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"revision\","]
#[doc = "    \"source\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"text\","]
#[doc = "    \"truncated\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReadFormat\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"revision\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReadSource\""]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"truncated\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReadResult {
    pub format: ReadFormat,
    pub pane_id: ::std::string::String,
    pub revision: u64,
    pub source: ReadSource,
    pub tab_id: ::std::string::String,
    pub text: ::std::string::String,
    pub truncated: bool,
    pub workspace_id: ::std::string::String,
}
impl PaneReadResult {
    pub fn builder() -> builder::PaneReadResult {
        Default::default()
    }
}
#[doc = "`PaneResizeReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unchanged\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneResizeReason {
    #[serde(rename = "unchanged")]
    Unchanged,
}
impl ::std::fmt::Display for PaneResizeReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unchanged => f.write_str("unchanged"),
        }
    }
}
impl ::std::str::FromStr for PaneResizeReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unchanged" => Ok(Self::Unchanged),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneResizeReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneResizeReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneResizeReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneResizeResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changed\","]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"layout\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneResizeReason\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneResizeResult {
    pub changed: bool,
    pub focused_pane_id: ::std::string::String,
    pub layout: PaneLayoutSnapshot,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<PaneResizeReason>,
}
impl PaneResizeResult {
    pub fn builder() -> builder::PaneResizeResult {
        Default::default()
    }
}
#[doc = "`PaneScrollInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"max_offset_from_bottom\","]
#[doc = "    \"offset_from_bottom\","]
#[doc = "    \"viewport_rows\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"max_offset_from_bottom\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"offset_from_bottom\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"viewport_rows\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneScrollInfo {
    pub max_offset_from_bottom: u64,
    pub offset_from_bottom: u64,
    pub viewport_rows: u64,
}
impl PaneScrollInfo {
    pub fn builder() -> builder::PaneScrollInfo {
        Default::default()
    }
}
#[doc = "`PaneSwapReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"no_neighbor\","]
#[doc = "    \"same_pane\","]
#[doc = "    \"not_found\","]
#[doc = "    \"cross_tab\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneSwapReason {
    #[serde(rename = "no_neighbor")]
    NoNeighbor,
    #[serde(rename = "same_pane")]
    SamePane,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "cross_tab")]
    CrossTab,
}
impl ::std::fmt::Display for PaneSwapReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NoNeighbor => f.write_str("no_neighbor"),
            Self::SamePane => f.write_str("same_pane"),
            Self::NotFound => f.write_str("not_found"),
            Self::CrossTab => f.write_str("cross_tab"),
        }
    }
}
impl ::std::str::FromStr for PaneSwapReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "no_neighbor" => Ok(Self::NoNeighbor),
            "same_pane" => Ok(Self::SamePane),
            "not_found" => Ok(Self::NotFound),
            "cross_tab" => Ok(Self::CrossTab),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneSwapReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneSwapReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneSwapReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneSwapResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changed\","]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"layout\","]
#[doc = "    \"source_pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSwapReason\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"target_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneSwapResult {
    pub changed: bool,
    pub focused_pane_id: ::std::string::String,
    pub layout: PaneLayoutSnapshot,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<PaneSwapReason>,
    pub source_pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target_pane_id: ::std::option::Option<::std::string::String>,
}
impl PaneSwapResult {
    pub fn builder() -> builder::PaneSwapResult {
        Default::default()
    }
}
#[doc = "`PaneZoomReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"single_pane\","]
#[doc = "    \"already_zoomed\","]
#[doc = "    \"already_unzoomed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PaneZoomReason {
    #[serde(rename = "single_pane")]
    SinglePane,
    #[serde(rename = "already_zoomed")]
    AlreadyZoomed,
    #[serde(rename = "already_unzoomed")]
    AlreadyUnzoomed,
}
impl ::std::fmt::Display for PaneZoomReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SinglePane => f.write_str("single_pane"),
            Self::AlreadyZoomed => f.write_str("already_zoomed"),
            Self::AlreadyUnzoomed => f.write_str("already_unzoomed"),
        }
    }
}
impl ::std::str::FromStr for PaneZoomReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_pane" => Ok(Self::SinglePane),
            "already_zoomed" => Ok(Self::AlreadyZoomed),
            "already_unzoomed" => Ok(Self::AlreadyUnzoomed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneZoomReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneZoomReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneZoomReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneZoomResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"changed\","]
#[doc = "    \"focus_changed\","]
#[doc = "    \"focused_pane_id\","]
#[doc = "    \"layout\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"zoom_changed\","]
#[doc = "    \"zoomed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"focus_changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneZoomReason\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"zoom_changed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"zoomed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneZoomResult {
    pub changed: bool,
    pub focus_changed: bool,
    pub focused_pane_id: ::std::string::String,
    pub layout: PaneLayoutSnapshot,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<PaneZoomReason>,
    pub zoom_changed: bool,
    pub zoomed: bool,
}
impl PaneZoomResult {
    pub fn builder() -> builder::PaneZoomResult {
        Default::default()
    }
}
#[doc = "`PluginActionContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"global\","]
#[doc = "    \"workspace\","]
#[doc = "    \"tab\","]
#[doc = "    \"pane\","]
#[doc = "    \"selection\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginActionContext {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "workspace")]
    Workspace,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "pane")]
    Pane,
    #[serde(rename = "selection")]
    Selection,
}
impl ::std::fmt::Display for PluginActionContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Global => f.write_str("global"),
            Self::Workspace => f.write_str("workspace"),
            Self::Tab => f.write_str("tab"),
            Self::Pane => f.write_str("pane"),
            Self::Selection => f.write_str("selection"),
        }
    }
}
impl ::std::str::FromStr for PluginActionContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "global" => Ok(Self::Global),
            "workspace" => Ok(Self::Workspace),
            "tab" => Ok(Self::Tab),
            "pane" => Ok(Self::Pane),
            "selection" => Ok(Self::Selection),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginActionContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginActionContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginActionContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginActionInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action_id\","]
#[doc = "    \"command\","]
#[doc = "    \"plugin_id\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contexts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginActionContext\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginActionInfo {
    pub action_id: ::std::string::String,
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub contexts: ::std::vec::Vec<PluginActionContext>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
    pub plugin_id: ::std::string::String,
    pub title: ::std::string::String,
}
impl PluginActionInfo {
    pub fn builder() -> builder::PluginActionInfo {
        Default::default()
    }
}
#[doc = "`PluginCommandLogInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"log_id\","]
#[doc = "    \"plugin_id\","]
#[doc = "    \"started_unix_ms\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"event\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"exit_code\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"finished_unix_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"log_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"started_unix_ms\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/PluginCommandStatus\""]
#[doc = "    },"]
#[doc = "    \"stderr\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stdout\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginCommandLogInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action_id: ::std::option::Option<::std::string::String>,
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub event: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exit_code: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub finished_unix_ms: ::std::option::Option<u64>,
    pub log_id: ::std::string::String,
    pub plugin_id: ::std::string::String,
    pub started_unix_ms: u64,
    pub status: PluginCommandStatus,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<::std::string::String>,
}
impl PluginCommandLogInfo {
    pub fn builder() -> builder::PluginCommandLogInfo {
        Default::default()
    }
}
#[doc = "`PluginCommandStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"running\","]
#[doc = "    \"succeeded\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginCommandStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for PluginCommandStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
            Self::Succeeded => f.write_str("succeeded"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for PluginCommandStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginCommandStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginCommandStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginCommandStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginInvocationContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"clicked_url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"correlation_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_pane_agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_pane_cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_pane_status\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"invocation_source\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"link_handler_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"selected_text\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tab_label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"workspace_cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"workspace_label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"worktree\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceWorktreeInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginInvocationContext {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clicked_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub correlation_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_agent: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_status: ::std::option::Option<AgentStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invocation_source: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub link_handler_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub selected_text: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub worktree: ::std::option::Option<WorkspaceWorktreeInfo>,
}
impl ::std::default::Default for PluginInvocationContext {
    fn default() -> Self {
        Self {
            clicked_url: Default::default(),
            correlation_id: Default::default(),
            focused_pane_agent: Default::default(),
            focused_pane_cwd: Default::default(),
            focused_pane_id: Default::default(),
            focused_pane_status: Default::default(),
            invocation_source: Default::default(),
            link_handler_id: Default::default(),
            selected_text: Default::default(),
            tab_id: Default::default(),
            tab_label: Default::default(),
            workspace_cwd: Default::default(),
            workspace_id: Default::default(),
            workspace_label: Default::default(),
            worktree: Default::default(),
        }
    }
}
impl PluginInvocationContext {
    pub fn builder() -> builder::PluginInvocationContext {
        Default::default()
    }
}
#[doc = "`PluginManifestAction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"id\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contexts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginActionContext\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginManifestAction {
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub contexts: ::std::vec::Vec<PluginActionContext>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
    pub title: ::std::string::String,
}
impl PluginManifestAction {
    pub fn builder() -> builder::PluginManifestAction {
        Default::default()
    }
}
#[doc = "`PluginManifestBuild`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginManifestBuild {
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
}
impl PluginManifestBuild {
    pub fn builder() -> builder::PluginManifestBuild {
        Default::default()
    }
}
#[doc = "`PluginManifestEventHook`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"on\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"on\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginManifestEventHook {
    pub command: ::std::vec::Vec<::std::string::String>,
    pub on: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
}
impl PluginManifestEventHook {
    pub fn builder() -> builder::PluginManifestEventHook {
        Default::default()
    }
}
#[doc = "`PluginManifestLinkHandler`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action\","]
#[doc = "    \"id\","]
#[doc = "    \"pattern\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginManifestLinkHandler {
    pub action: ::std::string::String,
    pub id: ::std::string::String,
    pub pattern: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
    pub title: ::std::string::String,
}
impl PluginManifestLinkHandler {
    pub fn builder() -> builder::PluginManifestLinkHandler {
        Default::default()
    }
}
#[doc = "`PluginManifestPane`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\","]
#[doc = "    \"id\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"height\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PopupSize\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"placement\": {"]
#[doc = "      \"default\": \"overlay\","]
#[doc = "      \"$ref\": \"#/$defs/PluginPanePlacement\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PluginPlatform\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PopupSize\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginManifestPane {
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<PopupSize>,
    pub id: ::std::string::String,
    #[serde(default = "defaults::plugin_manifest_pane_placement")]
    pub placement: PluginPanePlacement,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platforms: ::std::option::Option<::std::vec::Vec<PluginPlatform>>,
    pub title: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<PopupSize>,
}
impl PluginManifestPane {
    pub fn builder() -> builder::PluginManifestPane {
        Default::default()
    }
}
#[doc = "`PluginPaneInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"entrypoint\","]
#[doc = "    \"pane\","]
#[doc = "    \"plugin_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"entrypoint\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pane\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginPaneInfo {
    pub entrypoint: ::std::string::String,
    pub pane: PaneInfo,
    pub plugin_id: ::std::string::String,
}
impl PluginPaneInfo {
    pub fn builder() -> builder::PluginPaneInfo {
        Default::default()
    }
}
#[doc = "`PluginPanePlacement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"overlay\","]
#[doc = "    \"popup\","]
#[doc = "    \"split\","]
#[doc = "    \"tab\","]
#[doc = "    \"zoomed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginPanePlacement {
    #[serde(rename = "overlay")]
    Overlay,
    #[serde(rename = "popup")]
    Popup,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "zoomed")]
    Zoomed,
}
impl ::std::fmt::Display for PluginPanePlacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Overlay => f.write_str("overlay"),
            Self::Popup => f.write_str("popup"),
            Self::Split => f.write_str("split"),
            Self::Tab => f.write_str("tab"),
            Self::Zoomed => f.write_str("zoomed"),
        }
    }
}
impl ::std::str::FromStr for PluginPanePlacement {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "overlay" => Ok(Self::Overlay),
            "popup" => Ok(Self::Popup),
            "split" => Ok(Self::Split),
            "tab" => Ok(Self::Tab),
            "zoomed" => Ok(Self::Zoomed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginPanePlacement {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginPanePlacement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginPanePlacement {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginPlatform`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"linux\","]
#[doc = "    \"macos\","]
#[doc = "    \"windows\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginPlatform {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "windows")]
    Windows,
}
impl ::std::fmt::Display for PluginPlatform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Linux => f.write_str("linux"),
            Self::Macos => f.write_str("macos"),
            Self::Windows => f.write_str("windows"),
        }
    }
}
impl ::std::str::FromStr for PluginPlatform {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "linux" => Ok(Self::Linux),
            "macos" => Ok(Self::Macos),
            "windows" => Ok(Self::Windows),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginPlatform {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PluginSourceInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"installed_unix_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"default\": \"local\","]
#[doc = "      \"$ref\": \"#/$defs/PluginSourceKind\""]
#[doc = "    },"]
#[doc = "    \"managed_path\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"repo\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requested_ref\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"resolved_commit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"subdir\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginSourceInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub installed_unix_ms: ::std::option::Option<u64>,
    #[serde(default = "defaults::plugin_source_info_kind")]
    pub kind: PluginSourceKind,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub managed_path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub repo: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub requested_ref: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resolved_commit: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subdir: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PluginSourceInfo {
    fn default() -> Self {
        Self {
            installed_unix_ms: Default::default(),
            kind: defaults::plugin_source_info_kind(),
            managed_path: Default::default(),
            owner: Default::default(),
            repo: Default::default(),
            requested_ref: Default::default(),
            resolved_commit: Default::default(),
            subdir: Default::default(),
        }
    }
}
impl PluginSourceInfo {
    pub fn builder() -> builder::PluginSourceInfo {
        Default::default()
    }
}
#[doc = "`PluginSourceKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"local\","]
#[doc = "    \"github\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PluginSourceKind {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "github")]
    Github,
}
impl ::std::fmt::Display for PluginSourceKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
            Self::Github => f.write_str("github"),
        }
    }
}
impl ::std::str::FromStr for PluginSourceKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local" => Ok(Self::Local),
            "github" => Ok(Self::Github),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PluginSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginSourceKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PopupSize`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Outer popup size in terminal cells, including the border.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 65535.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Outer popup size as a percentage of the terminal area, for example 80%.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^(100|[1-9][0-9]?)%$\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PopupSize {
    Integer(u16),
    String(PopupSizeString),
}
impl ::std::str::FromStr for PopupSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for PopupSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PopupSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PopupSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PopupSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<u16> for PopupSize {
    fn from(value: u16) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<PopupSizeString> for PopupSize {
    fn from(value: PopupSizeString) -> Self {
        Self::String(value)
    }
}
#[doc = "Outer popup size as a percentage of the terminal area, for example 80%."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outer popup size as a percentage of the terminal area, for example 80%.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(100|[1-9][0-9]?)%$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PopupSizeString(::std::string::String);
impl ::std::ops::Deref for PopupSizeString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PopupSizeString> for ::std::string::String {
    fn from(value: PopupSizeString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PopupSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^(100|[1-9][0-9]?)%$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(100|[1-9][0-9]?)%$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PopupSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PopupSizeString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PopupSizeString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PopupSizeString {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ReadFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"text\","]
#[doc = "    \"ansi\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReadFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "ansi")]
    Ansi,
}
impl ::std::fmt::Display for ReadFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Ansi => f.write_str("ansi"),
        }
    }
}
impl ::std::str::FromStr for ReadFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "ansi" => Ok(Self::Ansi),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReadFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReadFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReadFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReadSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"visible\","]
#[doc = "    \"recent\","]
#[doc = "    \"recent_unwrapped\","]
#[doc = "    \"detection\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReadSource {
    #[serde(rename = "visible")]
    Visible,
    #[serde(rename = "recent")]
    Recent,
    #[serde(rename = "recent_unwrapped")]
    RecentUnwrapped,
    #[serde(rename = "detection")]
    Detection,
}
impl ::std::fmt::Display for ReadSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Visible => f.write_str("visible"),
            Self::Recent => f.write_str("recent"),
            Self::RecentUnwrapped => f.write_str("recent_unwrapped"),
            Self::Detection => f.write_str("detection"),
        }
    }
}
impl ::std::str::FromStr for ReadSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "visible" => Ok(Self::Visible),
            "recent" => Ok(Self::Recent),
            "recent_unwrapped" => Ok(Self::RecentUnwrapped),
            "detection" => Ok(Self::Detection),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReadSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReadSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReadSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponseResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"protocol\","]
#[doc = "        \"type\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"capabilities\": {"]
#[doc = "          \"default\": null,"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/ServerCapabilities\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"protocol\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint32\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pong\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"snapshot\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"snapshot\": {"]
#[doc = "          \"$ref\": \"#/$defs/SessionSnapshot\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"session_snapshot\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspace\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_info\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"root_pane\","]
#[doc = "        \"tab\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"root_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_created\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"workspaces\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_list\""]
#[doc = "        },"]
#[doc = "        \"workspaces\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source\","]
#[doc = "        \"type\","]
#[doc = "        \"worktrees\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeSourceInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_list\""]
#[doc = "        },"]
#[doc = "        \"worktrees\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"root_pane\","]
#[doc = "        \"tab\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace\","]
#[doc = "        \"worktree\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"root_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_created\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        },"]
#[doc = "        \"worktree\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"already_open\","]
#[doc = "        \"root_pane\","]
#[doc = "        \"tab\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace\","]
#[doc = "        \"worktree\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"already_open\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"root_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_opened\""]
#[doc = "        },"]
#[doc = "        \"workspace\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "        },"]
#[doc = "        \"worktree\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeInfo\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"forced\","]
#[doc = "        \"path\","]
#[doc = "        \"type\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"forced\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"path\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree_removed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tab\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_info\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"root_pane\","]
#[doc = "        \"tab\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"root_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"tab\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_created\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tabs\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tabs\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agent\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_info\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agent\","]
#[doc = "        \"argv\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentInfo\""]
#[doc = "        },"]
#[doc = "        \"argv\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_started\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agents\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agents\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_info\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"panes\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"panes\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_current\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"swap\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"swap\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSwapResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_swap\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"move_result\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"move_result\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneMoveResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_move\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"zoom\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_zoom\""]
#[doc = "        },"]
#[doc = "        \"zoom\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneZoomResult\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"layout\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"layout\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_layout\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"process_info\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"process_info\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneProcessInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_process_info\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"layout\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"layout\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutDescription\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout_export\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"layout\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"layout\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutDescription\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout_apply\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"layout\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"layout\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutDescription\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout_split_ratio_set\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"neighbor\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"neighbor\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneNeighborResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_neighbor\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"edges\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"edges\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneEdgesResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_edges\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"focus\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"focus\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneFocusDirectionResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_focus_direction\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"resize\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"resize\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneResizeResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_resize\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"read\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"read\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReadResult\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_read\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"cell_height_px\","]
#[doc = "        \"cell_width_px\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"cell_height_px\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint32\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"cell_width_px\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint32\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_graphics_info\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"explain\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"explain\": true,"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_explain\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"subscription_started\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"$ref\": \"#/$defs/EventEnvelope\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"wait_matched\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"read\","]
#[doc = "        \"revision\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"matched_line\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"read\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReadResult\""]
#[doc = "        },"]
#[doc = "        \"revision\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"uint64\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"output_matched\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"reason\","]
#[doc = "        \"shown\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$ref\": \"#/$defs/NotificationShowReason\""]
#[doc = "        },"]
#[doc = "        \"shown\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"notification_show\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"changed\","]
#[doc = "        \"reason\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"changed\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$ref\": \"#/$defs/ClientWindowTitleReason\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"client_window_title\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"details\","]
#[doc = "        \"target\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"details\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationInstallResult\""]
#[doc = "        },"]
#[doc = "        \"target\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationTarget\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"integration_install\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"details\","]
#[doc = "        \"target\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"details\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationUninstallResult\""]
#[doc = "        },"]
#[doc = "        \"target\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationTarget\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"integration_uninstall\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"manifests\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"manifests\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentManifestInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_manifest_reload\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"manifests\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"last_check_unix\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"uint64\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"last_result\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"manifests\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentManifestInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent_manifest_status\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin\": {"]
#[doc = "          \"$ref\": \"#/$defs/InstalledPluginInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_linked\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugins\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugins\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/InstalledPluginInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin_id\","]
#[doc = "        \"removed\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"removed\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_unlinked\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin\": {"]
#[doc = "          \"$ref\": \"#/$defs/InstalledPluginInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_enabled\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin\": {"]
#[doc = "          \"$ref\": \"#/$defs/InstalledPluginInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_disabled\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"actions\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"actions\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/PluginActionInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_action_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"action\","]
#[doc = "        \"context\","]
#[doc = "        \"log\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginActionInfo\""]
#[doc = "        },"]
#[doc = "        \"context\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginInvocationContext\""]
#[doc = "        },"]
#[doc = "        \"log\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginCommandLogInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_action_invoked\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"logs\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"logs\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/PluginCommandLogInfo\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_log_list\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin_pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_pane_opened\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"plugin_pane\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"plugin_pane\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPaneInfo\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_pane_focused\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"pane_id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin_pane_closed\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"diagnostics\","]
#[doc = "        \"status\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"diagnostics\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$ref\": \"#/$defs/ConfigReloadStatus\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"config_reload\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ok\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseResult {
    #[serde(rename = "pong")]
    Pong {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        capabilities: ::std::option::Option<ServerCapabilities>,
        protocol: u32,
        version: ::std::string::String,
    },
    #[serde(rename = "session_snapshot")]
    SessionSnapshot { snapshot: SessionSnapshot },
    #[serde(rename = "workspace_info")]
    WorkspaceInfo { workspace: WorkspaceInfo },
    #[serde(rename = "workspace_created")]
    WorkspaceCreated {
        root_pane: PaneInfo,
        tab: TabInfo,
        workspace: WorkspaceInfo,
    },
    #[serde(rename = "workspace_list")]
    WorkspaceList {
        workspaces: ::std::vec::Vec<WorkspaceInfo>,
    },
    #[serde(rename = "worktree_list")]
    WorktreeList {
        source: WorktreeSourceInfo,
        worktrees: ::std::vec::Vec<WorktreeInfo>,
    },
    #[serde(rename = "worktree_created")]
    WorktreeCreated {
        root_pane: PaneInfo,
        tab: TabInfo,
        workspace: WorkspaceInfo,
        worktree: WorktreeInfo,
    },
    #[serde(rename = "worktree_opened")]
    WorktreeOpened {
        already_open: bool,
        root_pane: PaneInfo,
        tab: TabInfo,
        workspace: WorkspaceInfo,
        worktree: WorktreeInfo,
    },
    #[serde(rename = "worktree_removed")]
    WorktreeRemoved {
        forced: bool,
        path: ::std::string::String,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "tab_info")]
    TabInfo { tab: TabInfo },
    #[serde(rename = "tab_created")]
    TabCreated { root_pane: PaneInfo, tab: TabInfo },
    #[serde(rename = "tab_list")]
    TabList { tabs: ::std::vec::Vec<TabInfo> },
    #[serde(rename = "agent_info")]
    AgentInfo { agent: AgentInfo },
    #[serde(rename = "agent_started")]
    AgentStarted {
        agent: AgentInfo,
        argv: ::std::vec::Vec<::std::string::String>,
    },
    #[serde(rename = "agent_list")]
    AgentList { agents: ::std::vec::Vec<AgentInfo> },
    #[serde(rename = "pane_info")]
    PaneInfo { pane: PaneInfo },
    #[serde(rename = "pane_list")]
    PaneList { panes: ::std::vec::Vec<PaneInfo> },
    #[serde(rename = "pane_current")]
    PaneCurrent { pane: PaneInfo },
    #[serde(rename = "pane_swap")]
    PaneSwap { swap: PaneSwapResult },
    #[serde(rename = "pane_move")]
    PaneMove { move_result: PaneMoveResult },
    #[serde(rename = "pane_zoom")]
    PaneZoom { zoom: PaneZoomResult },
    #[serde(rename = "pane_layout")]
    PaneLayout { layout: PaneLayoutSnapshot },
    #[serde(rename = "pane_process_info")]
    PaneProcessInfo { process_info: PaneProcessInfo },
    #[serde(rename = "layout_export")]
    LayoutExport { layout: LayoutDescription },
    #[serde(rename = "layout_apply")]
    LayoutApply { layout: LayoutDescription },
    #[serde(rename = "layout_split_ratio_set")]
    LayoutSplitRatioSet { layout: LayoutDescription },
    #[serde(rename = "pane_neighbor")]
    PaneNeighbor { neighbor: PaneNeighborResult },
    #[serde(rename = "pane_edges")]
    PaneEdges { edges: PaneEdgesResult },
    #[serde(rename = "pane_focus_direction")]
    PaneFocusDirection { focus: PaneFocusDirectionResult },
    #[serde(rename = "pane_resize")]
    PaneResize { resize: PaneResizeResult },
    #[serde(rename = "pane_read")]
    PaneRead { read: PaneReadResult },
    #[serde(rename = "pane_graphics_info")]
    PaneGraphicsInfo {
        cell_height_px: u32,
        cell_width_px: u32,
    },
    #[serde(rename = "agent_explain")]
    AgentExplain { explain: ::serde_json::Value },
    #[serde(rename = "subscription_started")]
    SubscriptionStarted,
    #[serde(rename = "wait_matched")]
    WaitMatched { event: EventEnvelope },
    #[serde(rename = "output_matched")]
    OutputMatched {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        matched_line: ::std::option::Option<::std::string::String>,
        pane_id: ::std::string::String,
        read: PaneReadResult,
        revision: u64,
    },
    #[serde(rename = "notification_show")]
    NotificationShow {
        reason: NotificationShowReason,
        shown: bool,
    },
    #[serde(rename = "client_window_title")]
    ClientWindowTitle {
        changed: bool,
        reason: ClientWindowTitleReason,
    },
    #[serde(rename = "integration_install")]
    IntegrationInstall {
        details: IntegrationInstallResult,
        target: IntegrationTarget,
    },
    #[serde(rename = "integration_uninstall")]
    IntegrationUninstall {
        details: IntegrationUninstallResult,
        target: IntegrationTarget,
    },
    #[serde(rename = "agent_manifest_reload")]
    AgentManifestReload {
        manifests: ::std::vec::Vec<AgentManifestInfo>,
    },
    #[serde(rename = "agent_manifest_status")]
    AgentManifestStatus {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        last_check_unix: ::std::option::Option<u64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        last_result: ::std::option::Option<::std::string::String>,
        manifests: ::std::vec::Vec<AgentManifestInfo>,
    },
    #[serde(rename = "plugin_linked")]
    PluginLinked { plugin: InstalledPluginInfo },
    #[serde(rename = "plugin_list")]
    PluginList {
        plugins: ::std::vec::Vec<InstalledPluginInfo>,
    },
    #[serde(rename = "plugin_unlinked")]
    PluginUnlinked {
        plugin_id: ::std::string::String,
        removed: bool,
    },
    #[serde(rename = "plugin_enabled")]
    PluginEnabled { plugin: InstalledPluginInfo },
    #[serde(rename = "plugin_disabled")]
    PluginDisabled { plugin: InstalledPluginInfo },
    #[serde(rename = "plugin_action_list")]
    PluginActionList {
        actions: ::std::vec::Vec<PluginActionInfo>,
    },
    #[serde(rename = "plugin_action_invoked")]
    PluginActionInvoked {
        action: PluginActionInfo,
        context: PluginInvocationContext,
        log: PluginCommandLogInfo,
    },
    #[serde(rename = "plugin_log_list")]
    PluginLogList {
        logs: ::std::vec::Vec<PluginCommandLogInfo>,
    },
    #[serde(rename = "plugin_pane_opened")]
    PluginPaneOpened { plugin_pane: PluginPaneInfo },
    #[serde(rename = "plugin_pane_focused")]
    PluginPaneFocused { plugin_pane: PluginPaneInfo },
    #[serde(rename = "plugin_pane_closed")]
    PluginPaneClosed { pane_id: ::std::string::String },
    #[serde(rename = "config_reload")]
    ConfigReload {
        diagnostics: ::std::vec::Vec<::std::string::String>,
        status: ConfigReloadStatus,
    },
    #[serde(rename = "ok")]
    Ok,
}
#[doc = "`ServerCapabilities`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"live_handoff\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detached_server_daemon\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"live_handoff\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ServerCapabilities {
    #[serde(default)]
    pub detached_server_daemon: bool,
    pub live_handoff: bool,
}
impl ServerCapabilities {
    pub fn builder() -> builder::ServerCapabilities {
        Default::default()
    }
}
#[doc = "`SessionSnapshot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agents\","]
#[doc = "    \"layouts\","]
#[doc = "    \"panes\","]
#[doc = "    \"protocol\","]
#[doc = "    \"tabs\","]
#[doc = "    \"version\","]
#[doc = "    \"workspaces\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"focused_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_tab_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"focused_workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"layouts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PaneLayoutSnapshot\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"panes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PaneInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"protocol\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tabs\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/TabInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"workspaces\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/WorkspaceInfo\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SessionSnapshot {
    pub agents: ::std::vec::Vec<AgentInfo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_tab_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub focused_workspace_id: ::std::option::Option<::std::string::String>,
    pub layouts: ::std::vec::Vec<PaneLayoutSnapshot>,
    pub panes: ::std::vec::Vec<PaneInfo>,
    pub protocol: u32,
    pub tabs: ::std::vec::Vec<TabInfo>,
    pub version: ::std::string::String,
    pub workspaces: ::std::vec::Vec<WorkspaceInfo>,
}
impl SessionSnapshot {
    pub fn builder() -> builder::SessionSnapshot {
        Default::default()
    }
}
#[doc = "`SplitDirection`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"right\","]
#[doc = "    \"down\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SplitDirection {
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "down")]
    Down,
}
impl ::std::fmt::Display for SplitDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Right => f.write_str("right"),
            Self::Down => f.write_str("down"),
        }
    }
}
impl ::std::str::FromStr for SplitDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "right" => Ok(Self::Right),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SplitDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SplitDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SplitDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SuccessResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SuccessResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"$ref\": \"#/$defs/ResponseResult\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SuccessResponse {
    pub id: ::std::string::String,
    pub result: ResponseResult,
}
impl SuccessResponse {
    pub fn builder() -> builder::SuccessResponse {
        Default::default()
    }
}
#[doc = "`TabInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent_status\","]
#[doc = "    \"focused\","]
#[doc = "    \"label\","]
#[doc = "    \"number\","]
#[doc = "    \"pane_count\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent_status\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "    },"]
#[doc = "    \"focused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"number\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"pane_count\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TabInfo {
    pub agent_status: AgentStatus,
    pub focused: bool,
    pub label: ::std::string::String,
    pub number: u32,
    pub pane_count: u32,
    pub tab_id: ::std::string::String,
    pub workspace_id: ::std::string::String,
}
impl TabInfo {
    pub fn builder() -> builder::TabInfo {
        Default::default()
    }
}
#[doc = "`WorkspaceInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active_tab_id\","]
#[doc = "    \"agent_status\","]
#[doc = "    \"focused\","]
#[doc = "    \"label\","]
#[doc = "    \"number\","]
#[doc = "    \"pane_count\","]
#[doc = "    \"tab_count\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active_tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"agent_status\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "    },"]
#[doc = "    \"focused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"number\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"pane_count\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tab_count\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 32,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"worktree\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceWorktreeInfo\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkspaceInfo {
    pub active_tab_id: ::std::string::String,
    pub agent_status: AgentStatus,
    pub focused: bool,
    pub label: ::std::string::String,
    pub number: u32,
    pub pane_count: u32,
    pub tab_count: u32,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub tokens: ::std::collections::HashMap<WorkspaceInfoTokensKey, ::std::string::String>,
    pub workspace_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub worktree: ::std::option::Option<WorkspaceWorktreeInfo>,
}
impl WorkspaceInfo {
    pub fn builder() -> builder::WorkspaceInfo {
        Default::default()
    }
}
#[doc = "`WorkspaceInfoTokensKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WorkspaceInfoTokensKey(::std::string::String);
impl ::std::ops::Deref for WorkspaceInfoTokensKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WorkspaceInfoTokensKey> for ::std::string::String {
    fn from(value: WorkspaceInfoTokensKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WorkspaceInfoTokensKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9_-]{1,32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9_-]{1,32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WorkspaceInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WorkspaceInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkspaceInfoTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WorkspaceInfoTokensKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`WorkspaceWorktreeInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"checkout_path\","]
#[doc = "    \"is_linked_worktree\","]
#[doc = "    \"repo_key\","]
#[doc = "    \"repo_name\","]
#[doc = "    \"repo_root\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"checkout_path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"is_linked_worktree\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"repo_key\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"repo_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"repo_root\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkspaceWorktreeInfo {
    pub checkout_path: ::std::string::String,
    pub is_linked_worktree: bool,
    pub repo_key: ::std::string::String,
    pub repo_name: ::std::string::String,
    pub repo_root: ::std::string::String,
}
impl WorkspaceWorktreeInfo {
    pub fn builder() -> builder::WorkspaceWorktreeInfo {
        Default::default()
    }
}
#[doc = "`WorktreeInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"is_bare\","]
#[doc = "    \"is_detached\","]
#[doc = "    \"is_linked_worktree\","]
#[doc = "    \"is_prunable\","]
#[doc = "    \"label\","]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"is_bare\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"is_detached\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"is_linked_worktree\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"is_prunable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"open_workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorktreeInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    pub is_bare: bool,
    pub is_detached: bool,
    pub is_linked_worktree: bool,
    pub is_prunable: bool,
    pub label: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open_workspace_id: ::std::option::Option<::std::string::String>,
    pub path: ::std::string::String,
}
impl WorktreeInfo {
    pub fn builder() -> builder::WorktreeInfo {
        Default::default()
    }
}
#[doc = "`WorktreeSourceInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"repo_key\","]
#[doc = "    \"repo_name\","]
#[doc = "    \"repo_root\","]
#[doc = "    \"source_checkout_path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"repo_key\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"repo_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"repo_root\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_checkout_path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_workspace_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorktreeSourceInfo {
    pub repo_key: ::std::string::String,
    pub repo_name: ::std::string::String,
    pub repo_root: ::std::string::String,
    pub source_checkout_path: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source_workspace_id: ::std::option::Option<::std::string::String>,
}
impl WorktreeSourceInfo {
    pub fn builder() -> builder::WorktreeSourceInfo {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AgentInfo {
        agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_session: ::std::result::Result<
            ::std::option::Option<super::AgentSessionInfo>,
            ::std::string::String,
        >,
        agent_status: ::std::result::Result<super::AgentStatus, ::std::string::String>,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        display_agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused: ::std::result::Result<bool, ::std::string::String>,
        foreground_cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        revision: ::std::result::Result<u64, ::std::string::String>,
        screen_detection_skipped:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        state_labels: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        terminal_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        terminal_title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_title_stripped: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tokens: ::std::result::Result<
            ::std::collections::HashMap<super::AgentInfoTokensKey, ::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentInfo {
        fn default() -> Self {
            Self {
                agent: Ok(Default::default()),
                agent_session: Ok(Default::default()),
                agent_status: Err("no value supplied for agent_status".to_string()),
                cwd: Ok(Default::default()),
                display_agent: Ok(Default::default()),
                focused: Err("no value supplied for focused".to_string()),
                foreground_cwd: Ok(Default::default()),
                name: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                revision: Err("no value supplied for revision".to_string()),
                screen_detection_skipped: Ok(Default::default()),
                state_labels: Ok(Default::default()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                terminal_id: Err("no value supplied for terminal_id".to_string()),
                terminal_title: Ok(Default::default()),
                terminal_title_stripped: Ok(Default::default()),
                title: Ok(Default::default()),
                tokens: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl AgentInfo {
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn agent_session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentSessionInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_session: {e}"));
            self
        }
        pub fn agent_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_status: {e}"));
            self
        }
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {e}"));
            self
        }
        pub fn display_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_agent: {e}"));
            self
        }
        pub fn focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused: {e}"));
            self
        }
        pub fn foreground_cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.foreground_cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for foreground_cwd: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {e}"));
            self
        }
        pub fn screen_detection_skipped<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.screen_detection_skipped = value.try_into().map_err(|e| {
                format!("error converting supplied value for screen_detection_skipped: {e}")
            });
            self
        }
        pub fn state_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.state_labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state_labels: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn terminal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terminal_id: {e}"));
            self
        }
        pub fn terminal_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terminal_title: {e}"));
            self
        }
        pub fn terminal_title_stripped<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_title_stripped = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_title_stripped: {e}")
            });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::AgentInfoTokensKey, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentInfo> for super::AgentInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                agent_session: value.agent_session?,
                agent_status: value.agent_status?,
                cwd: value.cwd?,
                display_agent: value.display_agent?,
                focused: value.focused?,
                foreground_cwd: value.foreground_cwd?,
                name: value.name?,
                pane_id: value.pane_id?,
                revision: value.revision?,
                screen_detection_skipped: value.screen_detection_skipped?,
                state_labels: value.state_labels?,
                tab_id: value.tab_id?,
                terminal_id: value.terminal_id?,
                terminal_title: value.terminal_title?,
                terminal_title_stripped: value.terminal_title_stripped?,
                title: value.title?,
                tokens: value.tokens?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::AgentInfo> for AgentInfo {
        fn from(value: super::AgentInfo) -> Self {
            Self {
                agent: Ok(value.agent),
                agent_session: Ok(value.agent_session),
                agent_status: Ok(value.agent_status),
                cwd: Ok(value.cwd),
                display_agent: Ok(value.display_agent),
                focused: Ok(value.focused),
                foreground_cwd: Ok(value.foreground_cwd),
                name: Ok(value.name),
                pane_id: Ok(value.pane_id),
                revision: Ok(value.revision),
                screen_detection_skipped: Ok(value.screen_detection_skipped),
                state_labels: Ok(value.state_labels),
                tab_id: Ok(value.tab_id),
                terminal_id: Ok(value.terminal_id),
                terminal_title: Ok(value.terminal_title),
                terminal_title_stripped: Ok(value.terminal_title_stripped),
                title: Ok(value.title),
                tokens: Ok(value.tokens),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentManifestInfo {
        active_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        cached_remote_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        local_override_shadowing_remote: ::std::result::Result<bool, ::std::string::String>,
        remote_last_checked_unix:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        remote_update_error: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remote_update_result: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        warning: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentManifestInfo {
        fn default() -> Self {
            Self {
                active_version: Ok(Default::default()),
                agent: Err("no value supplied for agent".to_string()),
                cached_remote_version: Ok(Default::default()),
                local_override_shadowing_remote: Err(
                    "no value supplied for local_override_shadowing_remote".to_string(),
                ),
                remote_last_checked_unix: Ok(Default::default()),
                remote_update_error: Ok(Default::default()),
                remote_update_result: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                source_kind: Err("no value supplied for source_kind".to_string()),
                warning: Ok(Default::default()),
            }
        }
    }
    impl AgentManifestInfo {
        pub fn active_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.active_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active_version: {e}"));
            self
        }
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn cached_remote_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cached_remote_version = value.try_into().map_err(|e| {
                format!("error converting supplied value for cached_remote_version: {e}")
            });
            self
        }
        pub fn local_override_shadowing_remote<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.local_override_shadowing_remote = value.try_into().map_err(|e| {
                format!("error converting supplied value for local_override_shadowing_remote: {e}")
            });
            self
        }
        pub fn remote_last_checked_unix<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_last_checked_unix = value.try_into().map_err(|e| {
                format!("error converting supplied value for remote_last_checked_unix: {e}")
            });
            self
        }
        pub fn remote_update_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_update_error = value.try_into().map_err(|e| {
                format!("error converting supplied value for remote_update_error: {e}")
            });
            self
        }
        pub fn remote_update_result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_update_result = value.try_into().map_err(|e| {
                format!("error converting supplied value for remote_update_result: {e}")
            });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn source_kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_kind: {e}"));
            self
        }
        pub fn warning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.warning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentManifestInfo> for super::AgentManifestInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentManifestInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active_version: value.active_version?,
                agent: value.agent?,
                cached_remote_version: value.cached_remote_version?,
                local_override_shadowing_remote: value.local_override_shadowing_remote?,
                remote_last_checked_unix: value.remote_last_checked_unix?,
                remote_update_error: value.remote_update_error?,
                remote_update_result: value.remote_update_result?,
                source: value.source?,
                source_kind: value.source_kind?,
                warning: value.warning?,
            })
        }
    }
    impl ::std::convert::From<super::AgentManifestInfo> for AgentManifestInfo {
        fn from(value: super::AgentManifestInfo) -> Self {
            Self {
                active_version: Ok(value.active_version),
                agent: Ok(value.agent),
                cached_remote_version: Ok(value.cached_remote_version),
                local_override_shadowing_remote: Ok(value.local_override_shadowing_remote),
                remote_last_checked_unix: Ok(value.remote_last_checked_unix),
                remote_update_error: Ok(value.remote_update_error),
                remote_update_result: Ok(value.remote_update_result),
                source: Ok(value.source),
                source_kind: Ok(value.source_kind),
                warning: Ok(value.warning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentSessionInfo {
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        kind: ::std::result::Result<super::AgentSessionRefKind, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentSessionInfo {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                source: Err("no value supplied for source".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AgentSessionInfo {
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentSessionRefKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentSessionInfo> for super::AgentSessionInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentSessionInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                kind: value.kind?,
                source: value.source?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AgentSessionInfo> for AgentSessionInfo {
        fn from(value: super::AgentSessionInfo) -> Self {
            Self {
                agent: Ok(value.agent),
                kind: Ok(value.kind),
                source: Ok(value.source),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventEnvelope {
        data: ::std::result::Result<super::EventData, ::std::string::String>,
        event: ::std::result::Result<super::EventKind, ::std::string::String>,
    }
    impl ::std::default::Default for EventEnvelope {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                event: Err("no value supplied for event".to_string()),
            }
        }
    }
    impl EventEnvelope {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventKind>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventEnvelope> for super::EventEnvelope {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventEnvelope,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                event: value.event?,
            })
        }
    }
    impl ::std::convert::From<super::EventEnvelope> for EventEnvelope {
        fn from(value: super::EventEnvelope) -> Self {
            Self {
                data: Ok(value.data),
                event: Ok(value.event),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstalledPluginInfo {
        actions: ::std::result::Result<
            ::std::vec::Vec<super::PluginManifestAction>,
            ::std::string::String,
        >,
        build: ::std::result::Result<
            ::std::vec::Vec<super::PluginManifestBuild>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enabled: ::std::result::Result<bool, ::std::string::String>,
        events: ::std::result::Result<
            ::std::vec::Vec<super::PluginManifestEventHook>,
            ::std::string::String,
        >,
        link_handlers: ::std::result::Result<
            ::std::vec::Vec<super::PluginManifestLinkHandler>,
            ::std::string::String,
        >,
        manifest_path: ::std::result::Result<::std::string::String, ::std::string::String>,
        min_herdr_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        panes: ::std::result::Result<
            ::std::vec::Vec<super::PluginManifestPane>,
            ::std::string::String,
        >,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        plugin_root: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::PluginSourceInfo, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
        warnings:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for InstalledPluginInfo {
        fn default() -> Self {
            Self {
                actions: Ok(Default::default()),
                build: Ok(Default::default()),
                description: Ok(Default::default()),
                enabled: Err("no value supplied for enabled".to_string()),
                events: Ok(Default::default()),
                link_handlers: Ok(Default::default()),
                manifest_path: Err("no value supplied for manifest_path".to_string()),
                min_herdr_version: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                panes: Ok(Default::default()),
                platforms: Ok(Default::default()),
                plugin_id: Err("no value supplied for plugin_id".to_string()),
                plugin_root: Err("no value supplied for plugin_root".to_string()),
                source: Ok(super::defaults::installed_plugin_info_source()),
                version: Err("no value supplied for version".to_string()),
                warnings: Ok(Default::default()),
            }
        }
    }
    impl InstalledPluginInfo {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginManifestAction>>,
            T::Error: ::std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {e}"));
            self
        }
        pub fn build<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginManifestBuild>>,
            T::Error: ::std::fmt::Display,
        {
            self.build = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for build: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginManifestEventHook>>,
            T::Error: ::std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {e}"));
            self
        }
        pub fn link_handlers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginManifestLinkHandler>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_handlers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_handlers: {e}"));
            self
        }
        pub fn manifest_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.manifest_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for manifest_path: {e}"));
            self
        }
        pub fn min_herdr_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.min_herdr_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_herdr_version: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn panes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginManifestPane>>,
            T::Error: ::std::fmt::Display,
        {
            self.panes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for panes: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
        pub fn plugin_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_root: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PluginSourceInfo>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
        pub fn warnings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.warnings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warnings: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InstalledPluginInfo> for super::InstalledPluginInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstalledPluginInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actions: value.actions?,
                build: value.build?,
                description: value.description?,
                enabled: value.enabled?,
                events: value.events?,
                link_handlers: value.link_handlers?,
                manifest_path: value.manifest_path?,
                min_herdr_version: value.min_herdr_version?,
                name: value.name?,
                panes: value.panes?,
                platforms: value.platforms?,
                plugin_id: value.plugin_id?,
                plugin_root: value.plugin_root?,
                source: value.source?,
                version: value.version?,
                warnings: value.warnings?,
            })
        }
    }
    impl ::std::convert::From<super::InstalledPluginInfo> for InstalledPluginInfo {
        fn from(value: super::InstalledPluginInfo) -> Self {
            Self {
                actions: Ok(value.actions),
                build: Ok(value.build),
                description: Ok(value.description),
                enabled: Ok(value.enabled),
                events: Ok(value.events),
                link_handlers: Ok(value.link_handlers),
                manifest_path: Ok(value.manifest_path),
                min_herdr_version: Ok(value.min_herdr_version),
                name: Ok(value.name),
                panes: Ok(value.panes),
                platforms: Ok(value.platforms),
                plugin_id: Ok(value.plugin_id),
                plugin_root: Ok(value.plugin_root),
                source: Ok(value.source),
                version: Ok(value.version),
                warnings: Ok(value.warnings),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntegrationInstallResult {
        messages:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for IntegrationInstallResult {
        fn default() -> Self {
            Self {
                messages: Err("no value supplied for messages".to_string()),
            }
        }
    }
    impl IntegrationInstallResult {
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IntegrationInstallResult> for super::IntegrationInstallResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IntegrationInstallResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                messages: value.messages?,
            })
        }
    }
    impl ::std::convert::From<super::IntegrationInstallResult> for IntegrationInstallResult {
        fn from(value: super::IntegrationInstallResult) -> Self {
            Self {
                messages: Ok(value.messages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntegrationUninstallResult {
        messages:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for IntegrationUninstallResult {
        fn default() -> Self {
            Self {
                messages: Err("no value supplied for messages".to_string()),
            }
        }
    }
    impl IntegrationUninstallResult {
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IntegrationUninstallResult> for super::IntegrationUninstallResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IntegrationUninstallResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                messages: value.messages?,
            })
        }
    }
    impl ::std::convert::From<super::IntegrationUninstallResult> for IntegrationUninstallResult {
        fn from(value: super::IntegrationUninstallResult) -> Self {
            Self {
                messages: Ok(value.messages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayoutDescription {
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        root: ::std::result::Result<super::LayoutNode, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        zoomed: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for LayoutDescription {
        fn default() -> Self {
            Self {
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                root: Err("no value supplied for root".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
                zoomed: Err("no value supplied for zoomed".to_string()),
            }
        }
    }
    impl LayoutDescription {
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LayoutNode>,
            T::Error: ::std::fmt::Display,
        {
            self.root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for root: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
        pub fn zoomed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.zoomed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zoomed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LayoutDescription> for super::LayoutDescription {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LayoutDescription,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                focused_pane_id: value.focused_pane_id?,
                root: value.root?,
                tab_id: value.tab_id?,
                workspace_id: value.workspace_id?,
                zoomed: value.zoomed?,
            })
        }
    }
    impl ::std::convert::From<super::LayoutDescription> for LayoutDescription {
        fn from(value: super::LayoutDescription) -> Self {
            Self {
                focused_pane_id: Ok(value.focused_pane_id),
                root: Ok(value.root),
                tab_id: Ok(value.tab_id),
                workspace_id: Ok(value.workspace_id),
                zoomed: Ok(value.zoomed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneEdgesResult {
        down: ::std::result::Result<bool, ::std::string::String>,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        left: ::std::result::Result<bool, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        right: ::std::result::Result<bool, ::std::string::String>,
        up: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for PaneEdgesResult {
        fn default() -> Self {
            Self {
                down: Err("no value supplied for down".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                left: Err("no value supplied for left".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                right: Err("no value supplied for right".to_string()),
                up: Err("no value supplied for up".to_string()),
            }
        }
    }
    impl PaneEdgesResult {
        pub fn down<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.down = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for down: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn left<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for left: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn right<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for right: {e}"));
            self
        }
        pub fn up<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.up = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for up: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneEdgesResult> for super::PaneEdgesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneEdgesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                down: value.down?,
                layout: value.layout?,
                left: value.left?,
                pane_id: value.pane_id?,
                right: value.right?,
                up: value.up?,
            })
        }
    }
    impl ::std::convert::From<super::PaneEdgesResult> for PaneEdgesResult {
        fn from(value: super::PaneEdgesResult) -> Self {
            Self {
                down: Ok(value.down),
                layout: Ok(value.layout),
                left: Ok(value.left),
                pane_id: Ok(value.pane_id),
                right: Ok(value.right),
                up: Ok(value.up),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneFocusDirectionResult {
        changed: ::std::result::Result<bool, ::std::string::String>,
        focused_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::PaneFocusDirectionReason>,
            ::std::string::String,
        >,
        source_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneFocusDirectionResult {
        fn default() -> Self {
            Self {
                changed: Err("no value supplied for changed".to_string()),
                focused_pane_id: Ok(Default::default()),
                layout: Err("no value supplied for layout".to_string()),
                reason: Ok(Default::default()),
                source_pane_id: Err("no value supplied for source_pane_id".to_string()),
            }
        }
    }
    impl PaneFocusDirectionResult {
        pub fn changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changed: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneFocusDirectionReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn source_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneFocusDirectionResult> for super::PaneFocusDirectionResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneFocusDirectionResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changed: value.changed?,
                focused_pane_id: value.focused_pane_id?,
                layout: value.layout?,
                reason: value.reason?,
                source_pane_id: value.source_pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneFocusDirectionResult> for PaneFocusDirectionResult {
        fn from(value: super::PaneFocusDirectionResult) -> Self {
            Self {
                changed: Ok(value.changed),
                focused_pane_id: Ok(value.focused_pane_id),
                layout: Ok(value.layout),
                reason: Ok(value.reason),
                source_pane_id: Ok(value.source_pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneInfo {
        agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_session: ::std::result::Result<
            ::std::option::Option<super::AgentSessionInfo>,
            ::std::string::String,
        >,
        agent_status: ::std::result::Result<super::AgentStatus, ::std::string::String>,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        display_agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused: ::std::result::Result<bool, ::std::string::String>,
        foreground_cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        revision: ::std::result::Result<u64, ::std::string::String>,
        scroll: ::std::result::Result<
            ::std::option::Option<super::PaneScrollInfo>,
            ::std::string::String,
        >,
        state_labels: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        terminal_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        terminal_title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_title_stripped: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tokens: ::std::result::Result<
            ::std::collections::HashMap<super::PaneInfoTokensKey, ::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneInfo {
        fn default() -> Self {
            Self {
                agent: Ok(Default::default()),
                agent_session: Ok(Default::default()),
                agent_status: Err("no value supplied for agent_status".to_string()),
                cwd: Ok(Default::default()),
                display_agent: Ok(Default::default()),
                focused: Err("no value supplied for focused".to_string()),
                foreground_cwd: Ok(Default::default()),
                label: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                revision: Err("no value supplied for revision".to_string()),
                scroll: Ok(Default::default()),
                state_labels: Ok(Default::default()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                terminal_id: Err("no value supplied for terminal_id".to_string()),
                terminal_title: Ok(Default::default()),
                terminal_title_stripped: Ok(Default::default()),
                title: Ok(Default::default()),
                tokens: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl PaneInfo {
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn agent_session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentSessionInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_session: {e}"));
            self
        }
        pub fn agent_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_status: {e}"));
            self
        }
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {e}"));
            self
        }
        pub fn display_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_agent: {e}"));
            self
        }
        pub fn focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused: {e}"));
            self
        }
        pub fn foreground_cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.foreground_cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for foreground_cwd: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {e}"));
            self
        }
        pub fn scroll<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneScrollInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.scroll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scroll: {e}"));
            self
        }
        pub fn state_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.state_labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state_labels: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn terminal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terminal_id: {e}"));
            self
        }
        pub fn terminal_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terminal_title: {e}"));
            self
        }
        pub fn terminal_title_stripped<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_title_stripped = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_title_stripped: {e}")
            });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::PaneInfoTokensKey, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneInfo> for super::PaneInfo {
        type Error = super::error::ConversionError;
        fn try_from(value: PaneInfo) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                agent_session: value.agent_session?,
                agent_status: value.agent_status?,
                cwd: value.cwd?,
                display_agent: value.display_agent?,
                focused: value.focused?,
                foreground_cwd: value.foreground_cwd?,
                label: value.label?,
                pane_id: value.pane_id?,
                revision: value.revision?,
                scroll: value.scroll?,
                state_labels: value.state_labels?,
                tab_id: value.tab_id?,
                terminal_id: value.terminal_id?,
                terminal_title: value.terminal_title?,
                terminal_title_stripped: value.terminal_title_stripped?,
                title: value.title?,
                tokens: value.tokens?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneInfo> for PaneInfo {
        fn from(value: super::PaneInfo) -> Self {
            Self {
                agent: Ok(value.agent),
                agent_session: Ok(value.agent_session),
                agent_status: Ok(value.agent_status),
                cwd: Ok(value.cwd),
                display_agent: Ok(value.display_agent),
                focused: Ok(value.focused),
                foreground_cwd: Ok(value.foreground_cwd),
                label: Ok(value.label),
                pane_id: Ok(value.pane_id),
                revision: Ok(value.revision),
                scroll: Ok(value.scroll),
                state_labels: Ok(value.state_labels),
                tab_id: Ok(value.tab_id),
                terminal_id: Ok(value.terminal_id),
                terminal_title: Ok(value.terminal_title),
                terminal_title_stripped: Ok(value.terminal_title_stripped),
                title: Ok(value.title),
                tokens: Ok(value.tokens),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneLayoutPane {
        focused: ::std::result::Result<bool, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        rect: ::std::result::Result<super::PaneLayoutRect, ::std::string::String>,
    }
    impl ::std::default::Default for PaneLayoutPane {
        fn default() -> Self {
            Self {
                focused: Err("no value supplied for focused".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                rect: Err("no value supplied for rect".to_string()),
            }
        }
    }
    impl PaneLayoutPane {
        pub fn focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn rect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutRect>,
            T::Error: ::std::fmt::Display,
        {
            self.rect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rect: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneLayoutPane> for super::PaneLayoutPane {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneLayoutPane,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                focused: value.focused?,
                pane_id: value.pane_id?,
                rect: value.rect?,
            })
        }
    }
    impl ::std::convert::From<super::PaneLayoutPane> for PaneLayoutPane {
        fn from(value: super::PaneLayoutPane) -> Self {
            Self {
                focused: Ok(value.focused),
                pane_id: Ok(value.pane_id),
                rect: Ok(value.rect),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneLayoutRect {
        height: ::std::result::Result<u16, ::std::string::String>,
        width: ::std::result::Result<u16, ::std::string::String>,
        x: ::std::result::Result<u16, ::std::string::String>,
        y: ::std::result::Result<u16, ::std::string::String>,
    }
    impl ::std::default::Default for PaneLayoutRect {
        fn default() -> Self {
            Self {
                height: Err("no value supplied for height".to_string()),
                width: Err("no value supplied for width".to_string()),
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl PaneLayoutRect {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {e}"));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneLayoutRect> for super::PaneLayoutRect {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneLayoutRect,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                width: value.width?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl ::std::convert::From<super::PaneLayoutRect> for PaneLayoutRect {
        fn from(value: super::PaneLayoutRect) -> Self {
            Self {
                height: Ok(value.height),
                width: Ok(value.width),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneLayoutSnapshot {
        area: ::std::result::Result<super::PaneLayoutRect, ::std::string::String>,
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        panes: ::std::result::Result<::std::vec::Vec<super::PaneLayoutPane>, ::std::string::String>,
        splits:
            ::std::result::Result<::std::vec::Vec<super::PaneLayoutSplit>, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        zoomed: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for PaneLayoutSnapshot {
        fn default() -> Self {
            Self {
                area: Err("no value supplied for area".to_string()),
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                panes: Err("no value supplied for panes".to_string()),
                splits: Err("no value supplied for splits".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
                zoomed: Err("no value supplied for zoomed".to_string()),
            }
        }
    }
    impl PaneLayoutSnapshot {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutRect>,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn panes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PaneLayoutPane>>,
            T::Error: ::std::fmt::Display,
        {
            self.panes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for panes: {e}"));
            self
        }
        pub fn splits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PaneLayoutSplit>>,
            T::Error: ::std::fmt::Display,
        {
            self.splits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for splits: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
        pub fn zoomed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.zoomed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zoomed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneLayoutSnapshot> for super::PaneLayoutSnapshot {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneLayoutSnapshot,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                focused_pane_id: value.focused_pane_id?,
                panes: value.panes?,
                splits: value.splits?,
                tab_id: value.tab_id?,
                workspace_id: value.workspace_id?,
                zoomed: value.zoomed?,
            })
        }
    }
    impl ::std::convert::From<super::PaneLayoutSnapshot> for PaneLayoutSnapshot {
        fn from(value: super::PaneLayoutSnapshot) -> Self {
            Self {
                area: Ok(value.area),
                focused_pane_id: Ok(value.focused_pane_id),
                panes: Ok(value.panes),
                splits: Ok(value.splits),
                tab_id: Ok(value.tab_id),
                workspace_id: Ok(value.workspace_id),
                zoomed: Ok(value.zoomed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneLayoutSplit {
        direction: ::std::result::Result<super::SplitDirection, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        ratio: ::std::result::Result<f32, ::std::string::String>,
        rect: ::std::result::Result<super::PaneLayoutRect, ::std::string::String>,
    }
    impl ::std::default::Default for PaneLayoutSplit {
        fn default() -> Self {
            Self {
                direction: Err("no value supplied for direction".to_string()),
                id: Err("no value supplied for id".to_string()),
                ratio: Err("no value supplied for ratio".to_string()),
                rect: Err("no value supplied for rect".to_string()),
            }
        }
    }
    impl PaneLayoutSplit {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SplitDirection>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f32>,
            T::Error: ::std::fmt::Display,
        {
            self.ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ratio: {e}"));
            self
        }
        pub fn rect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutRect>,
            T::Error: ::std::fmt::Display,
        {
            self.rect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rect: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneLayoutSplit> for super::PaneLayoutSplit {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneLayoutSplit,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                id: value.id?,
                ratio: value.ratio?,
                rect: value.rect?,
            })
        }
    }
    impl ::std::convert::From<super::PaneLayoutSplit> for PaneLayoutSplit {
        fn from(value: super::PaneLayoutSplit) -> Self {
            Self {
                direction: Ok(value.direction),
                id: Ok(value.id),
                ratio: Ok(value.ratio),
                rect: Ok(value.rect),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneMoveResult {
        changed: ::std::result::Result<bool, ::std::string::String>,
        closed_tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        closed_workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created_tab:
            ::std::result::Result<::std::option::Option<super::TabInfo>, ::std::string::String>,
        created_workspace: ::std::result::Result<
            ::std::option::Option<super::WorkspaceInfo>,
            ::std::string::String,
        >,
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        pane: ::std::result::Result<super::PaneInfo, ::std::string::String>,
        previous_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        previous_tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        previous_workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::PaneMoveReason>,
            ::std::string::String,
        >,
        source_layout: ::std::result::Result<
            ::std::option::Option<super::PaneLayoutSnapshot>,
            ::std::string::String,
        >,
        target_layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
    }
    impl ::std::default::Default for PaneMoveResult {
        fn default() -> Self {
            Self {
                changed: Err("no value supplied for changed".to_string()),
                closed_tab_id: Ok(Default::default()),
                closed_workspace_id: Ok(Default::default()),
                created_tab: Ok(Default::default()),
                created_workspace: Ok(Default::default()),
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                pane: Err("no value supplied for pane".to_string()),
                previous_pane_id: Err("no value supplied for previous_pane_id".to_string()),
                previous_tab_id: Err("no value supplied for previous_tab_id".to_string()),
                previous_workspace_id: Err(
                    "no value supplied for previous_workspace_id".to_string()
                ),
                reason: Ok(Default::default()),
                source_layout: Ok(Default::default()),
                target_layout: Err("no value supplied for target_layout".to_string()),
            }
        }
    }
    impl PaneMoveResult {
        pub fn changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changed: {e}"));
            self
        }
        pub fn closed_tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.closed_tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for closed_tab_id: {e}"));
            self
        }
        pub fn closed_workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.closed_workspace_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for closed_workspace_id: {e}")
            });
            self
        }
        pub fn created_tab<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TabInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_tab = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_tab: {e}"));
            self
        }
        pub fn created_workspace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WorkspaceInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_workspace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_workspace: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn pane<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneInfo>,
            T::Error: ::std::fmt::Display,
        {
            self.pane = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane: {e}"));
            self
        }
        pub fn previous_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for previous_pane_id: {e}"));
            self
        }
        pub fn previous_tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for previous_tab_id: {e}"));
            self
        }
        pub fn previous_workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_workspace_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for previous_workspace_id: {e}")
            });
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneMoveReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn source_layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneLayoutSnapshot>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_layout: {e}"));
            self
        }
        pub fn target_layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.target_layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_layout: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneMoveResult> for super::PaneMoveResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneMoveResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changed: value.changed?,
                closed_tab_id: value.closed_tab_id?,
                closed_workspace_id: value.closed_workspace_id?,
                created_tab: value.created_tab?,
                created_workspace: value.created_workspace?,
                focused_pane_id: value.focused_pane_id?,
                pane: value.pane?,
                previous_pane_id: value.previous_pane_id?,
                previous_tab_id: value.previous_tab_id?,
                previous_workspace_id: value.previous_workspace_id?,
                reason: value.reason?,
                source_layout: value.source_layout?,
                target_layout: value.target_layout?,
            })
        }
    }
    impl ::std::convert::From<super::PaneMoveResult> for PaneMoveResult {
        fn from(value: super::PaneMoveResult) -> Self {
            Self {
                changed: Ok(value.changed),
                closed_tab_id: Ok(value.closed_tab_id),
                closed_workspace_id: Ok(value.closed_workspace_id),
                created_tab: Ok(value.created_tab),
                created_workspace: Ok(value.created_workspace),
                focused_pane_id: Ok(value.focused_pane_id),
                pane: Ok(value.pane),
                previous_pane_id: Ok(value.previous_pane_id),
                previous_tab_id: Ok(value.previous_tab_id),
                previous_workspace_id: Ok(value.previous_workspace_id),
                reason: Ok(value.reason),
                source_layout: Ok(value.source_layout),
                target_layout: Ok(value.target_layout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneNeighborResult {
        direction: ::std::result::Result<super::PaneDirection, ::std::string::String>,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        neighbor_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneNeighborResult {
        fn default() -> Self {
            Self {
                direction: Err("no value supplied for direction".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                neighbor_pane_id: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneNeighborResult {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneDirection>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn neighbor_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.neighbor_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for neighbor_pane_id: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneNeighborResult> for super::PaneNeighborResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneNeighborResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                layout: value.layout?,
                neighbor_pane_id: value.neighbor_pane_id?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneNeighborResult> for PaneNeighborResult {
        fn from(value: super::PaneNeighborResult) -> Self {
            Self {
                direction: Ok(value.direction),
                layout: Ok(value.layout),
                neighbor_pane_id: Ok(value.neighbor_pane_id),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneProcessInfo {
        foreground_process_group_id:
            ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        foreground_processes: ::std::result::Result<
            ::std::vec::Vec<super::PaneProcessInfoProcess>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        shell_pid: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        tty: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneProcessInfo {
        fn default() -> Self {
            Self {
                foreground_process_group_id: Ok(Default::default()),
                foreground_processes: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                shell_pid: Ok(Default::default()),
                tty: Ok(Default::default()),
            }
        }
    }
    impl PaneProcessInfo {
        pub fn foreground_process_group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.foreground_process_group_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for foreground_process_group_id: {e}")
            });
            self
        }
        pub fn foreground_processes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PaneProcessInfoProcess>>,
            T::Error: ::std::fmt::Display,
        {
            self.foreground_processes = value.try_into().map_err(|e| {
                format!("error converting supplied value for foreground_processes: {e}")
            });
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn shell_pid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.shell_pid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shell_pid: {e}"));
            self
        }
        pub fn tty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tty: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneProcessInfo> for super::PaneProcessInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneProcessInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                foreground_process_group_id: value.foreground_process_group_id?,
                foreground_processes: value.foreground_processes?,
                pane_id: value.pane_id?,
                shell_pid: value.shell_pid?,
                tty: value.tty?,
            })
        }
    }
    impl ::std::convert::From<super::PaneProcessInfo> for PaneProcessInfo {
        fn from(value: super::PaneProcessInfo) -> Self {
            Self {
                foreground_process_group_id: Ok(value.foreground_process_group_id),
                foreground_processes: Ok(value.foreground_processes),
                pane_id: Ok(value.pane_id),
                shell_pid: Ok(value.shell_pid),
                tty: Ok(value.tty),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneProcessInfoProcess {
        argv: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        argv0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cmdline: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        pid: ::std::result::Result<u32, ::std::string::String>,
    }
    impl ::std::default::Default for PaneProcessInfoProcess {
        fn default() -> Self {
            Self {
                argv: Ok(Default::default()),
                argv0: Ok(Default::default()),
                cmdline: Ok(Default::default()),
                cwd: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                pid: Err("no value supplied for pid".to_string()),
            }
        }
    }
    impl PaneProcessInfoProcess {
        pub fn argv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.argv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for argv: {e}"));
            self
        }
        pub fn argv0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.argv0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for argv0: {e}"));
            self
        }
        pub fn cmdline<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cmdline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cmdline: {e}"));
            self
        }
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn pid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.pid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pid: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneProcessInfoProcess> for super::PaneProcessInfoProcess {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneProcessInfoProcess,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                argv: value.argv?,
                argv0: value.argv0?,
                cmdline: value.cmdline?,
                cwd: value.cwd?,
                name: value.name?,
                pid: value.pid?,
            })
        }
    }
    impl ::std::convert::From<super::PaneProcessInfoProcess> for PaneProcessInfoProcess {
        fn from(value: super::PaneProcessInfoProcess) -> Self {
            Self {
                argv: Ok(value.argv),
                argv0: Ok(value.argv0),
                cmdline: Ok(value.cmdline),
                cwd: Ok(value.cwd),
                name: Ok(value.name),
                pid: Ok(value.pid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReadResult {
        format: ::std::result::Result<super::ReadFormat, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        revision: ::std::result::Result<u64, ::std::string::String>,
        source: ::std::result::Result<super::ReadSource, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        truncated: ::std::result::Result<bool, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneReadResult {
        fn default() -> Self {
            Self {
                format: Err("no value supplied for format".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                revision: Err("no value supplied for revision".to_string()),
                source: Err("no value supplied for source".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                text: Err("no value supplied for text".to_string()),
                truncated: Err("no value supplied for truncated".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl PaneReadResult {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadFormat>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn revision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.revision = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for revision: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReadSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn truncated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.truncated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for truncated: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneReadResult> for super::PaneReadResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReadResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                pane_id: value.pane_id?,
                revision: value.revision?,
                source: value.source?,
                tab_id: value.tab_id?,
                text: value.text?,
                truncated: value.truncated?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReadResult> for PaneReadResult {
        fn from(value: super::PaneReadResult) -> Self {
            Self {
                format: Ok(value.format),
                pane_id: Ok(value.pane_id),
                revision: Ok(value.revision),
                source: Ok(value.source),
                tab_id: Ok(value.tab_id),
                text: Ok(value.text),
                truncated: Ok(value.truncated),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneResizeResult {
        changed: ::std::result::Result<bool, ::std::string::String>,
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::PaneResizeReason>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneResizeResult {
        fn default() -> Self {
            Self {
                changed: Err("no value supplied for changed".to_string()),
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                reason: Ok(Default::default()),
            }
        }
    }
    impl PaneResizeResult {
        pub fn changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changed: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneResizeReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneResizeResult> for super::PaneResizeResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneResizeResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changed: value.changed?,
                focused_pane_id: value.focused_pane_id?,
                layout: value.layout?,
                pane_id: value.pane_id?,
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::PaneResizeResult> for PaneResizeResult {
        fn from(value: super::PaneResizeResult) -> Self {
            Self {
                changed: Ok(value.changed),
                focused_pane_id: Ok(value.focused_pane_id),
                layout: Ok(value.layout),
                pane_id: Ok(value.pane_id),
                reason: Ok(value.reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneScrollInfo {
        max_offset_from_bottom: ::std::result::Result<u64, ::std::string::String>,
        offset_from_bottom: ::std::result::Result<u64, ::std::string::String>,
        viewport_rows: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for PaneScrollInfo {
        fn default() -> Self {
            Self {
                max_offset_from_bottom: Err(
                    "no value supplied for max_offset_from_bottom".to_string()
                ),
                offset_from_bottom: Err("no value supplied for offset_from_bottom".to_string()),
                viewport_rows: Err("no value supplied for viewport_rows".to_string()),
            }
        }
    }
    impl PaneScrollInfo {
        pub fn max_offset_from_bottom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_offset_from_bottom = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_offset_from_bottom: {e}")
            });
            self
        }
        pub fn offset_from_bottom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.offset_from_bottom = value.try_into().map_err(|e| {
                format!("error converting supplied value for offset_from_bottom: {e}")
            });
            self
        }
        pub fn viewport_rows<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.viewport_rows = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for viewport_rows: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneScrollInfo> for super::PaneScrollInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneScrollInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_offset_from_bottom: value.max_offset_from_bottom?,
                offset_from_bottom: value.offset_from_bottom?,
                viewport_rows: value.viewport_rows?,
            })
        }
    }
    impl ::std::convert::From<super::PaneScrollInfo> for PaneScrollInfo {
        fn from(value: super::PaneScrollInfo) -> Self {
            Self {
                max_offset_from_bottom: Ok(value.max_offset_from_bottom),
                offset_from_bottom: Ok(value.offset_from_bottom),
                viewport_rows: Ok(value.viewport_rows),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSwapResult {
        changed: ::std::result::Result<bool, ::std::string::String>,
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::PaneSwapReason>,
            ::std::string::String,
        >,
        source_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        target_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneSwapResult {
        fn default() -> Self {
            Self {
                changed: Err("no value supplied for changed".to_string()),
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                reason: Ok(Default::default()),
                source_pane_id: Err("no value supplied for source_pane_id".to_string()),
                target_pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneSwapResult {
        pub fn changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changed: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneSwapReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn source_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_pane_id: {e}"));
            self
        }
        pub fn target_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.target_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneSwapResult> for super::PaneSwapResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSwapResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changed: value.changed?,
                focused_pane_id: value.focused_pane_id?,
                layout: value.layout?,
                reason: value.reason?,
                source_pane_id: value.source_pane_id?,
                target_pane_id: value.target_pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSwapResult> for PaneSwapResult {
        fn from(value: super::PaneSwapResult) -> Self {
            Self {
                changed: Ok(value.changed),
                focused_pane_id: Ok(value.focused_pane_id),
                layout: Ok(value.layout),
                reason: Ok(value.reason),
                source_pane_id: Ok(value.source_pane_id),
                target_pane_id: Ok(value.target_pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneZoomResult {
        changed: ::std::result::Result<bool, ::std::string::String>,
        focus_changed: ::std::result::Result<bool, ::std::string::String>,
        focused_pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        layout: ::std::result::Result<super::PaneLayoutSnapshot, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::PaneZoomReason>,
            ::std::string::String,
        >,
        zoom_changed: ::std::result::Result<bool, ::std::string::String>,
        zoomed: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for PaneZoomResult {
        fn default() -> Self {
            Self {
                changed: Err("no value supplied for changed".to_string()),
                focus_changed: Err("no value supplied for focus_changed".to_string()),
                focused_pane_id: Err("no value supplied for focused_pane_id".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                reason: Ok(Default::default()),
                zoom_changed: Err("no value supplied for zoom_changed".to_string()),
                zoomed: Err("no value supplied for zoomed".to_string()),
            }
        }
    }
    impl PaneZoomResult {
        pub fn changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changed: {e}"));
            self
        }
        pub fn focus_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus_changed: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneLayoutSnapshot>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneZoomReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn zoom_changed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.zoom_changed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zoom_changed: {e}"));
            self
        }
        pub fn zoomed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.zoomed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zoomed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneZoomResult> for super::PaneZoomResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneZoomResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changed: value.changed?,
                focus_changed: value.focus_changed?,
                focused_pane_id: value.focused_pane_id?,
                layout: value.layout?,
                pane_id: value.pane_id?,
                reason: value.reason?,
                zoom_changed: value.zoom_changed?,
                zoomed: value.zoomed?,
            })
        }
    }
    impl ::std::convert::From<super::PaneZoomResult> for PaneZoomResult {
        fn from(value: super::PaneZoomResult) -> Self {
            Self {
                changed: Ok(value.changed),
                focus_changed: Ok(value.focus_changed),
                focused_pane_id: Ok(value.focused_pane_id),
                layout: Ok(value.layout),
                pane_id: Ok(value.pane_id),
                reason: Ok(value.reason),
                zoom_changed: Ok(value.zoom_changed),
                zoomed: Ok(value.zoomed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginActionInfo {
        action_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        contexts: ::std::result::Result<
            ::std::vec::Vec<super::PluginActionContext>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginActionInfo {
        fn default() -> Self {
            Self {
                action_id: Err("no value supplied for action_id".to_string()),
                command: Err("no value supplied for command".to_string()),
                contexts: Ok(Default::default()),
                description: Ok(Default::default()),
                platforms: Ok(Default::default()),
                plugin_id: Err("no value supplied for plugin_id".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl PluginActionInfo {
        pub fn action_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.action_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action_id: {e}"));
            self
        }
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn contexts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginActionContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.contexts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contexts: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginActionInfo> for super::PluginActionInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginActionInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_id: value.action_id?,
                command: value.command?,
                contexts: value.contexts?,
                description: value.description?,
                platforms: value.platforms?,
                plugin_id: value.plugin_id?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::PluginActionInfo> for PluginActionInfo {
        fn from(value: super::PluginActionInfo) -> Self {
            Self {
                action_id: Ok(value.action_id),
                command: Ok(value.command),
                contexts: Ok(value.contexts),
                description: Ok(value.description),
                platforms: Ok(value.platforms),
                plugin_id: Ok(value.plugin_id),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginCommandLogInfo {
        action_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        error: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exit_code: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        finished_unix_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        log_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        started_unix_ms: ::std::result::Result<u64, ::std::string::String>,
        status: ::std::result::Result<super::PluginCommandStatus, ::std::string::String>,
        stderr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stdout: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginCommandLogInfo {
        fn default() -> Self {
            Self {
                action_id: Ok(Default::default()),
                command: Err("no value supplied for command".to_string()),
                error: Ok(Default::default()),
                event: Ok(Default::default()),
                exit_code: Ok(Default::default()),
                finished_unix_ms: Ok(Default::default()),
                log_id: Err("no value supplied for log_id".to_string()),
                plugin_id: Err("no value supplied for plugin_id".to_string()),
                started_unix_ms: Err("no value supplied for started_unix_ms".to_string()),
                status: Err("no value supplied for status".to_string()),
                stderr: Ok(Default::default()),
                stdout: Ok(Default::default()),
            }
        }
    }
    impl PluginCommandLogInfo {
        pub fn action_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.action_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action_id: {e}"));
            self
        }
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn exit_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.exit_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_code: {e}"));
            self
        }
        pub fn finished_unix_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.finished_unix_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finished_unix_ms: {e}"));
            self
        }
        pub fn log_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.log_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_id: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
        pub fn started_unix_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.started_unix_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started_unix_ms: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PluginCommandStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn stderr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stderr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stderr: {e}"));
            self
        }
        pub fn stdout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stdout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stdout: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginCommandLogInfo> for super::PluginCommandLogInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginCommandLogInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_id: value.action_id?,
                command: value.command?,
                error: value.error?,
                event: value.event?,
                exit_code: value.exit_code?,
                finished_unix_ms: value.finished_unix_ms?,
                log_id: value.log_id?,
                plugin_id: value.plugin_id?,
                started_unix_ms: value.started_unix_ms?,
                status: value.status?,
                stderr: value.stderr?,
                stdout: value.stdout?,
            })
        }
    }
    impl ::std::convert::From<super::PluginCommandLogInfo> for PluginCommandLogInfo {
        fn from(value: super::PluginCommandLogInfo) -> Self {
            Self {
                action_id: Ok(value.action_id),
                command: Ok(value.command),
                error: Ok(value.error),
                event: Ok(value.event),
                exit_code: Ok(value.exit_code),
                finished_unix_ms: Ok(value.finished_unix_ms),
                log_id: Ok(value.log_id),
                plugin_id: Ok(value.plugin_id),
                started_unix_ms: Ok(value.started_unix_ms),
                status: Ok(value.status),
                stderr: Ok(value.stderr),
                stdout: Ok(value.stdout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginInvocationContext {
        clicked_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        correlation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_pane_agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_pane_cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_pane_status:
            ::std::result::Result<::std::option::Option<super::AgentStatus>, ::std::string::String>,
        invocation_source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        link_handler_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        selected_text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        worktree: ::std::result::Result<
            ::std::option::Option<super::WorkspaceWorktreeInfo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginInvocationContext {
        fn default() -> Self {
            Self {
                clicked_url: Ok(Default::default()),
                correlation_id: Ok(Default::default()),
                focused_pane_agent: Ok(Default::default()),
                focused_pane_cwd: Ok(Default::default()),
                focused_pane_id: Ok(Default::default()),
                focused_pane_status: Ok(Default::default()),
                invocation_source: Ok(Default::default()),
                link_handler_id: Ok(Default::default()),
                selected_text: Ok(Default::default()),
                tab_id: Ok(Default::default()),
                tab_label: Ok(Default::default()),
                workspace_cwd: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
                workspace_label: Ok(Default::default()),
                worktree: Ok(Default::default()),
            }
        }
    }
    impl PluginInvocationContext {
        pub fn clicked_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.clicked_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clicked_url: {e}"));
            self
        }
        pub fn correlation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.correlation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for correlation_id: {e}"));
            self
        }
        pub fn focused_pane_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_agent = value.try_into().map_err(|e| {
                format!("error converting supplied value for focused_pane_agent: {e}")
            });
            self
        }
        pub fn focused_pane_cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_cwd: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn focused_pane_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_status = value.try_into().map_err(|e| {
                format!("error converting supplied value for focused_pane_status: {e}")
            });
            self
        }
        pub fn invocation_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.invocation_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invocation_source: {e}"));
            self
        }
        pub fn link_handler_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_handler_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_handler_id: {e}"));
            self
        }
        pub fn selected_text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.selected_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for selected_text: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn tab_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_label: {e}"));
            self
        }
        pub fn workspace_cwd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_cwd: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
        pub fn workspace_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_label: {e}"));
            self
        }
        pub fn worktree<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WorkspaceWorktreeInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.worktree = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for worktree: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginInvocationContext> for super::PluginInvocationContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginInvocationContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clicked_url: value.clicked_url?,
                correlation_id: value.correlation_id?,
                focused_pane_agent: value.focused_pane_agent?,
                focused_pane_cwd: value.focused_pane_cwd?,
                focused_pane_id: value.focused_pane_id?,
                focused_pane_status: value.focused_pane_status?,
                invocation_source: value.invocation_source?,
                link_handler_id: value.link_handler_id?,
                selected_text: value.selected_text?,
                tab_id: value.tab_id?,
                tab_label: value.tab_label?,
                workspace_cwd: value.workspace_cwd?,
                workspace_id: value.workspace_id?,
                workspace_label: value.workspace_label?,
                worktree: value.worktree?,
            })
        }
    }
    impl ::std::convert::From<super::PluginInvocationContext> for PluginInvocationContext {
        fn from(value: super::PluginInvocationContext) -> Self {
            Self {
                clicked_url: Ok(value.clicked_url),
                correlation_id: Ok(value.correlation_id),
                focused_pane_agent: Ok(value.focused_pane_agent),
                focused_pane_cwd: Ok(value.focused_pane_cwd),
                focused_pane_id: Ok(value.focused_pane_id),
                focused_pane_status: Ok(value.focused_pane_status),
                invocation_source: Ok(value.invocation_source),
                link_handler_id: Ok(value.link_handler_id),
                selected_text: Ok(value.selected_text),
                tab_id: Ok(value.tab_id),
                tab_label: Ok(value.tab_label),
                workspace_cwd: Ok(value.workspace_cwd),
                workspace_id: Ok(value.workspace_id),
                workspace_label: Ok(value.workspace_label),
                worktree: Ok(value.worktree),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginManifestAction {
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        contexts: ::std::result::Result<
            ::std::vec::Vec<super::PluginActionContext>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginManifestAction {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                contexts: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                platforms: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl PluginManifestAction {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn contexts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PluginActionContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.contexts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contexts: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginManifestAction> for super::PluginManifestAction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginManifestAction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                contexts: value.contexts?,
                description: value.description?,
                id: value.id?,
                platforms: value.platforms?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::PluginManifestAction> for PluginManifestAction {
        fn from(value: super::PluginManifestAction) -> Self {
            Self {
                command: Ok(value.command),
                contexts: Ok(value.contexts),
                description: Ok(value.description),
                id: Ok(value.id),
                platforms: Ok(value.platforms),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginManifestBuild {
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginManifestBuild {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                platforms: Ok(Default::default()),
            }
        }
    }
    impl PluginManifestBuild {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginManifestBuild> for super::PluginManifestBuild {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginManifestBuild,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                platforms: value.platforms?,
            })
        }
    }
    impl ::std::convert::From<super::PluginManifestBuild> for PluginManifestBuild {
        fn from(value: super::PluginManifestBuild) -> Self {
            Self {
                command: Ok(value.command),
                platforms: Ok(value.platforms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginManifestEventHook {
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        on: ::std::result::Result<::std::string::String, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginManifestEventHook {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                on: Err("no value supplied for on".to_string()),
                platforms: Ok(Default::default()),
            }
        }
    }
    impl PluginManifestEventHook {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn on<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginManifestEventHook> for super::PluginManifestEventHook {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginManifestEventHook,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                on: value.on?,
                platforms: value.platforms?,
            })
        }
    }
    impl ::std::convert::From<super::PluginManifestEventHook> for PluginManifestEventHook {
        fn from(value: super::PluginManifestEventHook) -> Self {
            Self {
                command: Ok(value.command),
                on: Ok(value.on),
                platforms: Ok(value.platforms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginManifestLinkHandler {
        action: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        pattern: ::std::result::Result<::std::string::String, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginManifestLinkHandler {
        fn default() -> Self {
            Self {
                action: Err("no value supplied for action".to_string()),
                id: Err("no value supplied for id".to_string()),
                pattern: Err("no value supplied for pattern".to_string()),
                platforms: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl PluginManifestLinkHandler {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn pattern<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pattern: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginManifestLinkHandler> for super::PluginManifestLinkHandler {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginManifestLinkHandler,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                id: value.id?,
                pattern: value.pattern?,
                platforms: value.platforms?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::PluginManifestLinkHandler> for PluginManifestLinkHandler {
        fn from(value: super::PluginManifestLinkHandler) -> Self {
            Self {
                action: Ok(value.action),
                id: Ok(value.id),
                pattern: Ok(value.pattern),
                platforms: Ok(value.platforms),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginManifestPane {
        command:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        height:
            ::std::result::Result<::std::option::Option<super::PopupSize>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        placement: ::std::result::Result<super::PluginPanePlacement, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        width:
            ::std::result::Result<::std::option::Option<super::PopupSize>, ::std::string::String>,
    }
    impl ::std::default::Default for PluginManifestPane {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                description: Ok(Default::default()),
                height: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                placement: Ok(super::defaults::plugin_manifest_pane_placement()),
                platforms: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                width: Ok(Default::default()),
            }
        }
    }
    impl PluginManifestPane {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PopupSize>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PluginPanePlacement>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for placement: {e}"));
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::PluginPlatform>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PopupSize>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginManifestPane> for super::PluginManifestPane {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginManifestPane,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                description: value.description?,
                height: value.height?,
                id: value.id?,
                placement: value.placement?,
                platforms: value.platforms?,
                title: value.title?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::PluginManifestPane> for PluginManifestPane {
        fn from(value: super::PluginManifestPane) -> Self {
            Self {
                command: Ok(value.command),
                description: Ok(value.description),
                height: Ok(value.height),
                id: Ok(value.id),
                placement: Ok(value.placement),
                platforms: Ok(value.platforms),
                title: Ok(value.title),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginPaneInfo {
        entrypoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        pane: ::std::result::Result<super::PaneInfo, ::std::string::String>,
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginPaneInfo {
        fn default() -> Self {
            Self {
                entrypoint: Err("no value supplied for entrypoint".to_string()),
                pane: Err("no value supplied for pane".to_string()),
                plugin_id: Err("no value supplied for plugin_id".to_string()),
            }
        }
    }
    impl PluginPaneInfo {
        pub fn entrypoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.entrypoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entrypoint: {e}"));
            self
        }
        pub fn pane<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneInfo>,
            T::Error: ::std::fmt::Display,
        {
            self.pane = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginPaneInfo> for super::PluginPaneInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginPaneInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                entrypoint: value.entrypoint?,
                pane: value.pane?,
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginPaneInfo> for PluginPaneInfo {
        fn from(value: super::PluginPaneInfo) -> Self {
            Self {
                entrypoint: Ok(value.entrypoint),
                pane: Ok(value.pane),
                plugin_id: Ok(value.plugin_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginSourceInfo {
        installed_unix_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        kind: ::std::result::Result<super::PluginSourceKind, ::std::string::String>,
        managed_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        repo: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        requested_ref: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resolved_commit: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subdir: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginSourceInfo {
        fn default() -> Self {
            Self {
                installed_unix_ms: Ok(Default::default()),
                kind: Ok(super::defaults::plugin_source_info_kind()),
                managed_path: Ok(Default::default()),
                owner: Ok(Default::default()),
                repo: Ok(Default::default()),
                requested_ref: Ok(Default::default()),
                resolved_commit: Ok(Default::default()),
                subdir: Ok(Default::default()),
            }
        }
    }
    impl PluginSourceInfo {
        pub fn installed_unix_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.installed_unix_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for installed_unix_ms: {e}"));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PluginSourceKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn managed_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.managed_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for managed_path: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn repo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.repo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo: {e}"));
            self
        }
        pub fn requested_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.requested_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requested_ref: {e}"));
            self
        }
        pub fn resolved_commit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resolved_commit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resolved_commit: {e}"));
            self
        }
        pub fn subdir<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.subdir = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subdir: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginSourceInfo> for super::PluginSourceInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginSourceInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                installed_unix_ms: value.installed_unix_ms?,
                kind: value.kind?,
                managed_path: value.managed_path?,
                owner: value.owner?,
                repo: value.repo?,
                requested_ref: value.requested_ref?,
                resolved_commit: value.resolved_commit?,
                subdir: value.subdir?,
            })
        }
    }
    impl ::std::convert::From<super::PluginSourceInfo> for PluginSourceInfo {
        fn from(value: super::PluginSourceInfo) -> Self {
            Self {
                installed_unix_ms: Ok(value.installed_unix_ms),
                kind: Ok(value.kind),
                managed_path: Ok(value.managed_path),
                owner: Ok(value.owner),
                repo: Ok(value.repo),
                requested_ref: Ok(value.requested_ref),
                resolved_commit: Ok(value.resolved_commit),
                subdir: Ok(value.subdir),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerCapabilities {
        detached_server_daemon: ::std::result::Result<bool, ::std::string::String>,
        live_handoff: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ServerCapabilities {
        fn default() -> Self {
            Self {
                detached_server_daemon: Ok(Default::default()),
                live_handoff: Err("no value supplied for live_handoff".to_string()),
            }
        }
    }
    impl ServerCapabilities {
        pub fn detached_server_daemon<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.detached_server_daemon = value.try_into().map_err(|e| {
                format!("error converting supplied value for detached_server_daemon: {e}")
            });
            self
        }
        pub fn live_handoff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.live_handoff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for live_handoff: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerCapabilities> for super::ServerCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detached_server_daemon: value.detached_server_daemon?,
                live_handoff: value.live_handoff?,
            })
        }
    }
    impl ::std::convert::From<super::ServerCapabilities> for ServerCapabilities {
        fn from(value: super::ServerCapabilities) -> Self {
            Self {
                detached_server_daemon: Ok(value.detached_server_daemon),
                live_handoff: Ok(value.live_handoff),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SessionSnapshot {
        agents: ::std::result::Result<::std::vec::Vec<super::AgentInfo>, ::std::string::String>,
        focused_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focused_workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        layouts: ::std::result::Result<
            ::std::vec::Vec<super::PaneLayoutSnapshot>,
            ::std::string::String,
        >,
        panes: ::std::result::Result<::std::vec::Vec<super::PaneInfo>, ::std::string::String>,
        protocol: ::std::result::Result<u32, ::std::string::String>,
        tabs: ::std::result::Result<::std::vec::Vec<super::TabInfo>, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
        workspaces:
            ::std::result::Result<::std::vec::Vec<super::WorkspaceInfo>, ::std::string::String>,
    }
    impl ::std::default::Default for SessionSnapshot {
        fn default() -> Self {
            Self {
                agents: Err("no value supplied for agents".to_string()),
                focused_pane_id: Ok(Default::default()),
                focused_tab_id: Ok(Default::default()),
                focused_workspace_id: Ok(Default::default()),
                layouts: Err("no value supplied for layouts".to_string()),
                panes: Err("no value supplied for panes".to_string()),
                protocol: Err("no value supplied for protocol".to_string()),
                tabs: Err("no value supplied for tabs".to_string()),
                version: Err("no value supplied for version".to_string()),
                workspaces: Err("no value supplied for workspaces".to_string()),
            }
        }
    }
    impl SessionSnapshot {
        pub fn agents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.agents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agents: {e}"));
            self
        }
        pub fn focused_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_pane_id: {e}"));
            self
        }
        pub fn focused_tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused_tab_id: {e}"));
            self
        }
        pub fn focused_workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.focused_workspace_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for focused_workspace_id: {e}")
            });
            self
        }
        pub fn layouts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PaneLayoutSnapshot>>,
            T::Error: ::std::fmt::Display,
        {
            self.layouts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layouts: {e}"));
            self
        }
        pub fn panes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PaneInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.panes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for panes: {e}"));
            self
        }
        pub fn protocol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol: {e}"));
            self
        }
        pub fn tabs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TabInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.tabs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tabs: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
        pub fn workspaces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WorkspaceInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspaces = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspaces: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SessionSnapshot> for super::SessionSnapshot {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SessionSnapshot,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agents: value.agents?,
                focused_pane_id: value.focused_pane_id?,
                focused_tab_id: value.focused_tab_id?,
                focused_workspace_id: value.focused_workspace_id?,
                layouts: value.layouts?,
                panes: value.panes?,
                protocol: value.protocol?,
                tabs: value.tabs?,
                version: value.version?,
                workspaces: value.workspaces?,
            })
        }
    }
    impl ::std::convert::From<super::SessionSnapshot> for SessionSnapshot {
        fn from(value: super::SessionSnapshot) -> Self {
            Self {
                agents: Ok(value.agents),
                focused_pane_id: Ok(value.focused_pane_id),
                focused_tab_id: Ok(value.focused_tab_id),
                focused_workspace_id: Ok(value.focused_workspace_id),
                layouts: Ok(value.layouts),
                panes: Ok(value.panes),
                protocol: Ok(value.protocol),
                tabs: Ok(value.tabs),
                version: Ok(value.version),
                workspaces: Ok(value.workspaces),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SuccessResponse {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::ResponseResult, ::std::string::String>,
    }
    impl ::std::default::Default for SuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl SuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SuccessResponse> for super::SuccessResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::SuccessResponse> for SuccessResponse {
        fn from(value: super::SuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabInfo {
        agent_status: ::std::result::Result<super::AgentStatus, ::std::string::String>,
        focused: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        number: ::std::result::Result<u32, ::std::string::String>,
        pane_count: ::std::result::Result<u32, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TabInfo {
        fn default() -> Self {
            Self {
                agent_status: Err("no value supplied for agent_status".to_string()),
                focused: Err("no value supplied for focused".to_string()),
                label: Err("no value supplied for label".to_string()),
                number: Err("no value supplied for number".to_string()),
                pane_count: Err("no value supplied for pane_count".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl TabInfo {
        pub fn agent_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_status: {e}"));
            self
        }
        pub fn focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {e}"));
            self
        }
        pub fn pane_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_count: {e}"));
            self
        }
        pub fn tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_id: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TabInfo> for super::TabInfo {
        type Error = super::error::ConversionError;
        fn try_from(value: TabInfo) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_status: value.agent_status?,
                focused: value.focused?,
                label: value.label?,
                number: value.number?,
                pane_count: value.pane_count?,
                tab_id: value.tab_id?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabInfo> for TabInfo {
        fn from(value: super::TabInfo) -> Self {
            Self {
                agent_status: Ok(value.agent_status),
                focused: Ok(value.focused),
                label: Ok(value.label),
                number: Ok(value.number),
                pane_count: Ok(value.pane_count),
                tab_id: Ok(value.tab_id),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceInfo {
        active_tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        agent_status: ::std::result::Result<super::AgentStatus, ::std::string::String>,
        focused: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        number: ::std::result::Result<u32, ::std::string::String>,
        pane_count: ::std::result::Result<u32, ::std::string::String>,
        tab_count: ::std::result::Result<u32, ::std::string::String>,
        tokens: ::std::result::Result<
            ::std::collections::HashMap<super::WorkspaceInfoTokensKey, ::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        worktree: ::std::result::Result<
            ::std::option::Option<super::WorkspaceWorktreeInfo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorkspaceInfo {
        fn default() -> Self {
            Self {
                active_tab_id: Err("no value supplied for active_tab_id".to_string()),
                agent_status: Err("no value supplied for agent_status".to_string()),
                focused: Err("no value supplied for focused".to_string()),
                label: Err("no value supplied for label".to_string()),
                number: Err("no value supplied for number".to_string()),
                pane_count: Err("no value supplied for pane_count".to_string()),
                tab_count: Err("no value supplied for tab_count".to_string()),
                tokens: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
                worktree: Ok(Default::default()),
            }
        }
    }
    impl WorkspaceInfo {
        pub fn active_tab_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.active_tab_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active_tab_id: {e}"));
            self
        }
        pub fn agent_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_status: {e}"));
            self
        }
        pub fn focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focused: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {e}"));
            self
        }
        pub fn pane_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_count: {e}"));
            self
        }
        pub fn tab_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_count: {e}"));
            self
        }
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::WorkspaceInfoTokensKey, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
        pub fn worktree<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WorkspaceWorktreeInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.worktree = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for worktree: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceInfo> for super::WorkspaceInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active_tab_id: value.active_tab_id?,
                agent_status: value.agent_status?,
                focused: value.focused?,
                label: value.label?,
                number: value.number?,
                pane_count: value.pane_count?,
                tab_count: value.tab_count?,
                tokens: value.tokens?,
                workspace_id: value.workspace_id?,
                worktree: value.worktree?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceInfo> for WorkspaceInfo {
        fn from(value: super::WorkspaceInfo) -> Self {
            Self {
                active_tab_id: Ok(value.active_tab_id),
                agent_status: Ok(value.agent_status),
                focused: Ok(value.focused),
                label: Ok(value.label),
                number: Ok(value.number),
                pane_count: Ok(value.pane_count),
                tab_count: Ok(value.tab_count),
                tokens: Ok(value.tokens),
                workspace_id: Ok(value.workspace_id),
                worktree: Ok(value.worktree),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceWorktreeInfo {
        checkout_path: ::std::result::Result<::std::string::String, ::std::string::String>,
        is_linked_worktree: ::std::result::Result<bool, ::std::string::String>,
        repo_key: ::std::result::Result<::std::string::String, ::std::string::String>,
        repo_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        repo_root: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceWorktreeInfo {
        fn default() -> Self {
            Self {
                checkout_path: Err("no value supplied for checkout_path".to_string()),
                is_linked_worktree: Err("no value supplied for is_linked_worktree".to_string()),
                repo_key: Err("no value supplied for repo_key".to_string()),
                repo_name: Err("no value supplied for repo_name".to_string()),
                repo_root: Err("no value supplied for repo_root".to_string()),
            }
        }
    }
    impl WorkspaceWorktreeInfo {
        pub fn checkout_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.checkout_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for checkout_path: {e}"));
            self
        }
        pub fn is_linked_worktree<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_linked_worktree = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_linked_worktree: {e}")
            });
            self
        }
        pub fn repo_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_key: {e}"));
            self
        }
        pub fn repo_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_name: {e}"));
            self
        }
        pub fn repo_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_root: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkspaceWorktreeInfo> for super::WorkspaceWorktreeInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceWorktreeInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                checkout_path: value.checkout_path?,
                is_linked_worktree: value.is_linked_worktree?,
                repo_key: value.repo_key?,
                repo_name: value.repo_name?,
                repo_root: value.repo_root?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceWorktreeInfo> for WorkspaceWorktreeInfo {
        fn from(value: super::WorkspaceWorktreeInfo) -> Self {
            Self {
                checkout_path: Ok(value.checkout_path),
                is_linked_worktree: Ok(value.is_linked_worktree),
                repo_key: Ok(value.repo_key),
                repo_name: Ok(value.repo_name),
                repo_root: Ok(value.repo_root),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorktreeInfo {
        branch: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_bare: ::std::result::Result<bool, ::std::string::String>,
        is_detached: ::std::result::Result<bool, ::std::string::String>,
        is_linked_worktree: ::std::result::Result<bool, ::std::string::String>,
        is_prunable: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        open_workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorktreeInfo {
        fn default() -> Self {
            Self {
                branch: Ok(Default::default()),
                is_bare: Err("no value supplied for is_bare".to_string()),
                is_detached: Err("no value supplied for is_detached".to_string()),
                is_linked_worktree: Err("no value supplied for is_linked_worktree".to_string()),
                is_prunable: Err("no value supplied for is_prunable".to_string()),
                label: Err("no value supplied for label".to_string()),
                open_workspace_id: Ok(Default::default()),
                path: Err("no value supplied for path".to_string()),
            }
        }
    }
    impl WorktreeInfo {
        pub fn branch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.branch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branch: {e}"));
            self
        }
        pub fn is_bare<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_bare = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_bare: {e}"));
            self
        }
        pub fn is_detached<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_detached = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_detached: {e}"));
            self
        }
        pub fn is_linked_worktree<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_linked_worktree = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_linked_worktree: {e}")
            });
            self
        }
        pub fn is_prunable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_prunable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_prunable: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn open_workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_workspace_id: {e}"));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorktreeInfo> for super::WorktreeInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branch: value.branch?,
                is_bare: value.is_bare?,
                is_detached: value.is_detached?,
                is_linked_worktree: value.is_linked_worktree?,
                is_prunable: value.is_prunable?,
                label: value.label?,
                open_workspace_id: value.open_workspace_id?,
                path: value.path?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeInfo> for WorktreeInfo {
        fn from(value: super::WorktreeInfo) -> Self {
            Self {
                branch: Ok(value.branch),
                is_bare: Ok(value.is_bare),
                is_detached: Ok(value.is_detached),
                is_linked_worktree: Ok(value.is_linked_worktree),
                is_prunable: Ok(value.is_prunable),
                label: Ok(value.label),
                open_workspace_id: Ok(value.open_workspace_id),
                path: Ok(value.path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorktreeSourceInfo {
        repo_key: ::std::result::Result<::std::string::String, ::std::string::String>,
        repo_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        repo_root: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_checkout_path: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorktreeSourceInfo {
        fn default() -> Self {
            Self {
                repo_key: Err("no value supplied for repo_key".to_string()),
                repo_name: Err("no value supplied for repo_name".to_string()),
                repo_root: Err("no value supplied for repo_root".to_string()),
                source_checkout_path: Err("no value supplied for source_checkout_path".to_string()),
                source_workspace_id: Ok(Default::default()),
            }
        }
    }
    impl WorktreeSourceInfo {
        pub fn repo_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_key: {e}"));
            self
        }
        pub fn repo_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_name: {e}"));
            self
        }
        pub fn repo_root<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.repo_root = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repo_root: {e}"));
            self
        }
        pub fn source_checkout_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_checkout_path = value.try_into().map_err(|e| {
                format!("error converting supplied value for source_checkout_path: {e}")
            });
            self
        }
        pub fn source_workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_workspace_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for source_workspace_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<WorktreeSourceInfo> for super::WorktreeSourceInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeSourceInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                repo_key: value.repo_key?,
                repo_name: value.repo_name?,
                repo_root: value.repo_root?,
                source_checkout_path: value.source_checkout_path?,
                source_workspace_id: value.source_workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeSourceInfo> for WorktreeSourceInfo {
        fn from(value: super::WorktreeSourceInfo) -> Self {
            Self {
                repo_key: Ok(value.repo_key),
                repo_name: Ok(value.repo_name),
                repo_root: Ok(value.repo_root),
                source_checkout_path: Ok(value.source_checkout_path),
                source_workspace_id: Ok(value.source_workspace_id),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn installed_plugin_info_source() -> super::PluginSourceInfo {
        super::PluginSourceInfo {
            installed_unix_ms: Default::default(),
            kind: super::PluginSourceKind::Local,
            managed_path: Default::default(),
            owner: Default::default(),
            repo: Default::default(),
            requested_ref: Default::default(),
            resolved_commit: Default::default(),
            subdir: Default::default(),
        }
    }
    pub(super) fn plugin_manifest_pane_placement() -> super::PluginPanePlacement {
        super::PluginPanePlacement::Overlay
    }
    pub(super) fn plugin_source_info_kind() -> super::PluginSourceKind {
        super::PluginSourceKind::Local
    }
}
