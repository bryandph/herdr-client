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
#[doc = "`AgentPromptParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"wait\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/AgentPromptWaitOptions\""]
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
pub struct AgentPromptParams {
    pub target: ::std::string::String,
    pub text: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wait: ::std::option::Option<AgentPromptWaitOptions>,
}
impl AgentPromptParams {
    pub fn builder() -> builder::AgentPromptParams {
        Default::default()
    }
}
#[doc = "`AgentPromptWaitOptions`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"until\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentPromptWaitOptions {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub until: ::std::vec::Vec<AgentStatus>,
}
impl ::std::default::Default for AgentPromptWaitOptions {
    fn default() -> Self {
        Self {
            timeout_ms: Default::default(),
            until: Default::default(),
        }
    }
}
impl AgentPromptWaitOptions {
    pub fn builder() -> builder::AgentPromptWaitOptions {
        Default::default()
    }
}
#[doc = "`AgentReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"default\": \"text\","]
#[doc = "      \"$ref\": \"#/$defs/ReadFormat\""]
#[doc = "    },"]
#[doc = "    \"lines\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReadSource\""]
#[doc = "    },"]
#[doc = "    \"strip_ansi\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentReadParams {
    #[serde(default = "defaults::agent_read_params_format")]
    pub format: ReadFormat,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lines: ::std::option::Option<u32>,
    pub source: ReadSource,
    #[serde(default = "defaults::default_bool::<true>")]
    pub strip_ansi: bool,
    pub target: ::std::string::String,
}
impl AgentReadParams {
    pub fn builder() -> builder::AgentReadParams {
        Default::default()
    }
}
#[doc = "`AgentRenameParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentRenameParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub target: ::std::string::String,
}
impl AgentRenameParams {
    pub fn builder() -> builder::AgentRenameParams {
        Default::default()
    }
}
#[doc = "`AgentSendKeysParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keys\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"keys\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentSendKeysParams {
    pub keys: ::std::vec::Vec<::std::string::String>,
    pub target: ::std::string::String,
}
impl AgentSendKeysParams {
    pub fn builder() -> builder::AgentSendKeysParams {
        Default::default()
    }
}
#[doc = "`AgentStartParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"name\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"description\": \"Startup timeout in milliseconds. Values must be greater than 3000 and at most 300000.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentStartParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub args: ::std::vec::Vec<::std::string::String>,
    pub kind: ::std::string::String,
    pub name: ::std::string::String,
    pub pane_id: ::std::string::String,
    #[doc = "Startup timeout in milliseconds. Values must be greater than 3000 and at most 300000."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<u64>,
}
impl AgentStartParams {
    pub fn builder() -> builder::AgentStartParams {
        Default::default()
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
#[doc = "`AgentTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentTarget {
    pub target: ::std::string::String,
}
impl AgentTarget {
    pub fn builder() -> builder::AgentTarget {
        Default::default()
    }
}
#[doc = "`AgentViewBuiltinField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"status\","]
#[doc = "    \"workspace_id\","]
#[doc = "    \"tab_id\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"agent\","]
#[doc = "    \"seen\","]
#[doc = "    \"state_change_seq\""]
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
pub enum AgentViewBuiltinField {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "workspace_id")]
    WorkspaceId,
    #[serde(rename = "tab_id")]
    TabId,
    #[serde(rename = "pane_id")]
    PaneId,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "seen")]
    Seen,
    #[serde(rename = "state_change_seq")]
    StateChangeSeq,
}
impl ::std::fmt::Display for AgentViewBuiltinField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Status => f.write_str("status"),
            Self::WorkspaceId => f.write_str("workspace_id"),
            Self::TabId => f.write_str("tab_id"),
            Self::PaneId => f.write_str("pane_id"),
            Self::Agent => f.write_str("agent"),
            Self::Seen => f.write_str("seen"),
            Self::StateChangeSeq => f.write_str("state_change_seq"),
        }
    }
}
impl ::std::str::FromStr for AgentViewBuiltinField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "status" => Ok(Self::Status),
            "workspace_id" => Ok(Self::WorkspaceId),
            "tab_id" => Ok(Self::TabId),
            "pane_id" => Ok(Self::PaneId),
            "agent" => Ok(Self::Agent),
            "seen" => Ok(Self::Seen),
            "state_change_seq" => Ok(Self::StateChangeSeq),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentViewBuiltinField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentViewBuiltinField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentViewBuiltinField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AgentViewBuiltinSortField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"workspace_order\","]
#[doc = "    \"tab_order\","]
#[doc = "    \"pane_order\","]
#[doc = "    \"attention\","]
#[doc = "    \"status\","]
#[doc = "    \"agent\","]
#[doc = "    \"seen\","]
#[doc = "    \"state_change_seq\""]
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
pub enum AgentViewBuiltinSortField {
    #[serde(rename = "workspace_order")]
    WorkspaceOrder,
    #[serde(rename = "tab_order")]
    TabOrder,
    #[serde(rename = "pane_order")]
    PaneOrder,
    #[serde(rename = "attention")]
    Attention,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "seen")]
    Seen,
    #[serde(rename = "state_change_seq")]
    StateChangeSeq,
}
impl ::std::fmt::Display for AgentViewBuiltinSortField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WorkspaceOrder => f.write_str("workspace_order"),
            Self::TabOrder => f.write_str("tab_order"),
            Self::PaneOrder => f.write_str("pane_order"),
            Self::Attention => f.write_str("attention"),
            Self::Status => f.write_str("status"),
            Self::Agent => f.write_str("agent"),
            Self::Seen => f.write_str("seen"),
            Self::StateChangeSeq => f.write_str("state_change_seq"),
        }
    }
}
impl ::std::str::FromStr for AgentViewBuiltinSortField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "workspace_order" => Ok(Self::WorkspaceOrder),
            "tab_order" => Ok(Self::TabOrder),
            "pane_order" => Ok(Self::PaneOrder),
            "attention" => Ok(Self::Attention),
            "status" => Ok(Self::Status),
            "agent" => Ok(Self::Agent),
            "seen" => Ok(Self::Seen),
            "state_change_seq" => Ok(Self::StateChangeSeq),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentViewBuiltinSortField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentViewBuiltinSortField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentViewBuiltinSortField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AgentViewClearParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
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
pub struct AgentViewClearParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AgentViewClearParams {
    fn default() -> Self {
        Self {
            source: Default::default(),
        }
    }
}
impl AgentViewClearParams {
    pub fn builder() -> builder::AgentViewClearParams {
        Default::default()
    }
}
#[doc = "`AgentViewContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"current_workspace_id\","]
#[doc = "    \"current_tab_id\""]
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
pub enum AgentViewContext {
    #[serde(rename = "current_workspace_id")]
    CurrentWorkspaceId,
    #[serde(rename = "current_tab_id")]
    CurrentTabId,
}
impl ::std::fmt::Display for AgentViewContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CurrentWorkspaceId => f.write_str("current_workspace_id"),
            Self::CurrentTabId => f.write_str("current_tab_id"),
        }
    }
}
impl ::std::str::FromStr for AgentViewContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "current_workspace_id" => Ok(Self::CurrentWorkspaceId),
            "current_tab_id" => Ok(Self::CurrentTabId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentViewContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentViewContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentViewContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AgentViewField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/AgentViewBuiltinField\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"token\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"token\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AgentViewField {
    AgentViewBuiltinField(AgentViewBuiltinField),
    Object { token: ::std::string::String },
}
impl ::std::convert::From<AgentViewBuiltinField> for AgentViewField {
    fn from(value: AgentViewBuiltinField) -> Self {
        Self::AgentViewBuiltinField(value)
    }
}
#[doc = "`AgentViewFilter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"filters\","]
#[doc = "        \"op\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"filters\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentViewFilter\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"all\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"filters\","]
#[doc = "        \"op\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"filters\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentViewFilter\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"any\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"filter\","]
#[doc = "        \"op\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"filter\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewFilter\""]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"not\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"field\","]
#[doc = "        \"op\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"field\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewField\""]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"eq\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewValue\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"field\","]
#[doc = "        \"op\","]
#[doc = "        \"values\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"field\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewField\""]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"in\""]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/AgentViewValue\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"field\","]
#[doc = "        \"op\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"field\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewField\""]
#[doc = "        },"]
#[doc = "        \"op\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"exists\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "op")]
pub enum AgentViewFilter {
    #[serde(rename = "all")]
    All {
        filters: ::std::vec::Vec<AgentViewFilter>,
    },
    #[serde(rename = "any")]
    Any {
        filters: ::std::vec::Vec<AgentViewFilter>,
    },
    #[serde(rename = "not")]
    Not {
        filter: ::std::boxed::Box<AgentViewFilter>,
    },
    #[serde(rename = "eq")]
    Eq {
        field: AgentViewField,
        value: AgentViewValue,
    },
    #[serde(rename = "in")]
    In {
        field: AgentViewField,
        values: ::std::vec::Vec<AgentViewValue>,
    },
    #[serde(rename = "exists")]
    Exists { field: AgentViewField },
}
#[doc = "`AgentViewSetParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"filter\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewFilter\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sort\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentViewSort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentViewSetParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: ::std::option::Option<AgentViewFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sort: ::std::vec::Vec<AgentViewSort>,
    pub source: ::std::string::String,
}
impl AgentViewSetParams {
    pub fn builder() -> builder::AgentViewSetParams {
        Default::default()
    }
}
#[doc = "`AgentViewSort`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"field\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"field\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentViewSortField\""]
#[doc = "    },"]
#[doc = "    \"order\": {"]
#[doc = "      \"default\": \"asc\","]
#[doc = "      \"$ref\": \"#/$defs/AgentViewSortOrder\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentViewSort {
    pub field: AgentViewSortField,
    #[serde(default = "defaults::agent_view_sort_order")]
    pub order: AgentViewSortOrder,
}
impl AgentViewSort {
    pub fn builder() -> builder::AgentViewSort {
        Default::default()
    }
}
#[doc = "`AgentViewSortField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/AgentViewBuiltinSortField\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"token\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"token\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AgentViewSortField {
    AgentViewBuiltinSortField(AgentViewBuiltinSortField),
    Object { token: ::std::string::String },
}
impl ::std::convert::From<AgentViewBuiltinSortField> for AgentViewSortField {
    fn from(value: AgentViewBuiltinSortField) -> Self {
        Self::AgentViewBuiltinSortField(value)
    }
}
#[doc = "`AgentViewSortOrder`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"asc\","]
#[doc = "    \"desc\""]
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
pub enum AgentViewSortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for AgentViewSortOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for AgentViewSortOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentViewSortOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AgentViewSortOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentViewSortOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AgentViewValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"context\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"context\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewContext\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AgentViewValue {
    String(::std::string::String),
    Boolean(bool),
    Uint64(u64),
    Object { context: AgentViewContext },
}
impl ::std::convert::From<bool> for AgentViewValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<u64> for AgentViewValue {
    fn from(value: u64) -> Self {
        Self::Uint64(value)
    }
}
#[doc = "`AgentWaitParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"target\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"until\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AgentWaitParams {
    pub target: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub until: ::std::vec::Vec<AgentStatus>,
}
impl AgentWaitParams {
    pub fn builder() -> builder::AgentWaitParams {
        Default::default()
    }
}
#[doc = "`ClientWindowTitleSetParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ClientWindowTitleSetParams {
    pub title: ::std::string::String,
}
impl ClientWindowTitleSetParams {
    pub fn builder() -> builder::ClientWindowTitleSetParams {
        Default::default()
    }
}
#[doc = "`EmptyParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct EmptyParams(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for EmptyParams {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<EmptyParams>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: EmptyParams) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for EmptyParams
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`EventMatch`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_created\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_updated\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_closed\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_renamed\""]
#[doc = "        },"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
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
#[doc = "        \"event\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace_moved\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"workspace_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
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
#[doc = "        \"event\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_created\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"tab_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_closed\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"tab_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_renamed\""]
#[doc = "        },"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"tab_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_moved\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"tab_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab_focused\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_created\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_closed\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_focused\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_moved\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_output_changed\""]
#[doc = "        },"]
#[doc = "        \"min_revision\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"uint64\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_exited\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_agent_detected\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"agent_status\","]
#[doc = "        \"event\","]
#[doc = "        \"pane_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent_status\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "        },"]
#[doc = "        \"event\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane_agent_status_changed\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "event")]
pub enum EventMatch {
    #[serde(rename = "workspace_created")]
    WorkspaceCreated {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "workspace_updated")]
    WorkspaceUpdated { workspace_id: ::std::string::String },
    #[serde(rename = "workspace_closed")]
    WorkspaceClosed { workspace_id: ::std::string::String },
    #[serde(rename = "workspace_renamed")]
    WorkspaceRenamed {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<::std::string::String>,
        workspace_id: ::std::string::String,
    },
    #[serde(rename = "workspace_moved")]
    WorkspaceMoved { workspace_id: ::std::string::String },
    #[serde(rename = "workspace_focused")]
    WorkspaceFocused { workspace_id: ::std::string::String },
    #[serde(rename = "tab_created")]
    TabCreated {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tab_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "tab_closed")]
    TabClosed { tab_id: ::std::string::String },
    #[serde(rename = "tab_renamed")]
    TabRenamed {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<::std::string::String>,
        tab_id: ::std::string::String,
    },
    #[serde(rename = "tab_moved")]
    TabMoved { tab_id: ::std::string::String },
    #[serde(rename = "tab_focused")]
    TabFocused { tab_id: ::std::string::String },
    #[serde(rename = "pane_created")]
    PaneCreated {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pane_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "pane_closed")]
    PaneClosed { pane_id: ::std::string::String },
    #[serde(rename = "pane_focused")]
    PaneFocused { pane_id: ::std::string::String },
    #[serde(rename = "pane_moved")]
    PaneMoved { pane_id: ::std::string::String },
    #[serde(rename = "pane_output_changed")]
    PaneOutputChanged {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        min_revision: ::std::option::Option<u64>,
        pane_id: ::std::string::String,
    },
    #[serde(rename = "pane_exited")]
    PaneExited { pane_id: ::std::string::String },
    #[serde(rename = "pane_agent_detected")]
    PaneAgentDetected {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent: ::std::option::Option<::std::string::String>,
        pane_id: ::std::string::String,
    },
    #[serde(rename = "pane_agent_status_changed")]
    PaneAgentStatusChanged {
        agent_status: AgentStatus,
        pane_id: ::std::string::String,
    },
}
#[doc = "`EventsSubscribeParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"subscriptions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"subscriptions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Subscription\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EventsSubscribeParams {
    pub subscriptions: ::std::vec::Vec<Subscription>,
}
impl EventsSubscribeParams {
    pub fn builder() -> builder::EventsSubscribeParams {
        Default::default()
    }
}
#[doc = "`EventsWaitParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"match_event\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"match_event\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventMatch\""]
#[doc = "    },"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EventsWaitParams {
    pub match_event: EventMatch,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<u64>,
}
impl EventsWaitParams {
    pub fn builder() -> builder::EventsWaitParams {
        Default::default()
    }
}
#[doc = "`IntegrationInstallParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"target\": {"]
#[doc = "      \"$ref\": \"#/$defs/IntegrationTarget\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct IntegrationInstallParams {
    pub target: IntegrationTarget,
}
impl IntegrationInstallParams {
    pub fn builder() -> builder::IntegrationInstallParams {
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
#[doc = "`IntegrationUninstallParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"target\": {"]
#[doc = "      \"$ref\": \"#/$defs/IntegrationTarget\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct IntegrationUninstallParams {
    pub target: IntegrationTarget,
}
impl IntegrationUninstallParams {
    pub fn builder() -> builder::IntegrationUninstallParams {
        Default::default()
    }
}
#[doc = "`LayoutApplyParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"root\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"root\": {"]
#[doc = "      \"$ref\": \"#/$defs/LayoutNode\""]
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
#[doc = "    \"workspace_id\": {"]
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
pub struct LayoutApplyParams {
    #[serde(default)]
    pub focus: bool,
    pub root: LayoutNode,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl LayoutApplyParams {
    pub fn builder() -> builder::LayoutApplyParams {
        Default::default()
    }
}
#[doc = "`LayoutExportParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LayoutExportParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for LayoutExportParams {
    fn default() -> Self {
        Self {
            pane_id: Default::default(),
            tab_id: Default::default(),
        }
    }
}
impl LayoutExportParams {
    pub fn builder() -> builder::LayoutExportParams {
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
#[doc = "`LayoutSetSplitRatioParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\","]
#[doc = "    \"ratio\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ratio\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
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
pub struct LayoutSetSplitRatioParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
    pub path: ::std::vec::Vec<bool>,
    pub ratio: f32,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tab_id: ::std::option::Option<::std::string::String>,
}
impl LayoutSetSplitRatioParams {
    pub fn builder() -> builder::LayoutSetSplitRatioParams {
        Default::default()
    }
}
#[doc = "`NotificationShowParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"body\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/ToastHerdrPosition\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"sound\": {"]
#[doc = "      \"$ref\": \"#/$defs/NotificationShowSound\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct NotificationShowParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub position: ::std::option::Option<ToastHerdrPosition>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sound: ::std::option::Option<NotificationShowSound>,
    pub title: ::std::string::String,
}
impl NotificationShowParams {
    pub fn builder() -> builder::NotificationShowParams {
        Default::default()
    }
}
#[doc = "`NotificationShowSound`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"none\","]
#[doc = "    \"done\","]
#[doc = "    \"request\""]
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
pub enum NotificationShowSound {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "request")]
    Request,
}
impl ::std::fmt::Display for NotificationShowSound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Done => f.write_str("done"),
            Self::Request => f.write_str("request"),
        }
    }
}
impl ::std::str::FromStr for NotificationShowSound {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "done" => Ok(Self::Done),
            "request" => Ok(Self::Request),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NotificationShowSound {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NotificationShowSound {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NotificationShowSound {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OutputMatch`"]
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
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"substring\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"regex\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum OutputMatch {
    #[serde(rename = "substring")]
    Substring(::std::string::String),
    #[serde(rename = "regex")]
    Regex(::std::string::String),
}
#[doc = "`PaneAgentState`"]
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
pub enum PaneAgentState {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for PaneAgentState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("idle"),
            Self::Working => f.write_str("working"),
            Self::Blocked => f.write_str("blocked"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for PaneAgentState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "idle" => Ok(Self::Idle),
            "working" => Ok(Self::Working),
            "blocked" => Ok(Self::Blocked),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneAgentState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneAgentState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneAgentState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneClearAgentAuthorityParams`"]
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
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
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
pub struct PaneClearAgentAuthorityParams {
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::string::String>,
}
impl PaneClearAgentAuthorityParams {
    pub fn builder() -> builder::PaneClearAgentAuthorityParams {
        Default::default()
    }
}
#[doc = "`PaneCurrentParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"caller_pane_id\": {"]
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
pub struct PaneCurrentParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub caller_pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneCurrentParams {
    fn default() -> Self {
        Self {
            caller_pane_id: Default::default(),
        }
    }
}
impl PaneCurrentParams {
    pub fn builder() -> builder::PaneCurrentParams {
        Default::default()
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
#[doc = "`PaneEdgesParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneEdgesParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneEdgesParams {
    fn default() -> Self {
        Self {
            pane_id: Default::default(),
        }
    }
}
impl PaneEdgesParams {
    pub fn builder() -> builder::PaneEdgesParams {
        Default::default()
    }
}
#[doc = "`PaneFocusDirectionParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneDirection\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneFocusDirectionParams {
    pub direction: PaneDirection,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl PaneFocusDirectionParams {
    pub fn builder() -> builder::PaneFocusDirectionParams {
        Default::default()
    }
}
#[doc = "`PaneGraphicsClearParams`"]
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
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneGraphicsClearParams {
    pub pane_id: ::std::string::String,
}
impl PaneGraphicsClearParams {
    pub fn builder() -> builder::PaneGraphicsClearParams {
        Default::default()
    }
}
#[doc = "`PaneGraphicsFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"png\","]
#[doc = "    \"rgb\","]
#[doc = "    \"rgba\""]
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
pub enum PaneGraphicsFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "rgba")]
    Rgba,
}
impl ::std::fmt::Display for PaneGraphicsFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Rgb => f.write_str("rgb"),
            Self::Rgba => f.write_str("rgba"),
        }
    }
}
impl ::std::str::FromStr for PaneGraphicsFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "rgb" => Ok(Self::Rgb),
            "rgba" => Ok(Self::Rgba),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneGraphicsFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneGraphicsFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneGraphicsFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneGraphicsPlacementParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"grid_cols\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"grid_rows\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"viewport_col\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"viewport_row\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneGraphicsPlacementParams {
    #[serde(default)]
    pub grid_cols: u32,
    #[serde(default)]
    pub grid_rows: u32,
    #[serde(default)]
    pub viewport_col: i32,
    #[serde(default)]
    pub viewport_row: i32,
}
impl ::std::default::Default for PaneGraphicsPlacementParams {
    fn default() -> Self {
        Self {
            grid_cols: Default::default(),
            grid_rows: Default::default(),
            viewport_col: Default::default(),
            viewport_row: Default::default(),
        }
    }
}
impl PaneGraphicsPlacementParams {
    pub fn builder() -> builder::PaneGraphicsPlacementParams {
        Default::default()
    }
}
#[doc = "`PaneGraphicsSetParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"format\","]
#[doc = "    \"image_height\","]
#[doc = "    \"image_width\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data_base64\": {"]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneGraphicsFormat\""]
#[doc = "    },"]
#[doc = "    \"image_height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"image_width\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"placement\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"grid_cols\": 0,"]
#[doc = "        \"grid_rows\": 0,"]
#[doc = "        \"viewport_col\": 0,"]
#[doc = "        \"viewport_row\": 0"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/$defs/PaneGraphicsPlacementParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneGraphicsSetParams {
    #[serde(default)]
    pub data_base64: ::std::string::String,
    pub format: PaneGraphicsFormat,
    pub image_height: u32,
    pub image_width: u32,
    pub pane_id: ::std::string::String,
    #[serde(default = "defaults::pane_graphics_set_params_placement")]
    pub placement: PaneGraphicsPlacementParams,
}
impl PaneGraphicsSetParams {
    pub fn builder() -> builder::PaneGraphicsSetParams {
        Default::default()
    }
}
#[doc = "`PaneLayoutParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneLayoutParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneLayoutParams {
    fn default() -> Self {
        Self {
            pane_id: Default::default(),
        }
    }
}
impl PaneLayoutParams {
    pub fn builder() -> builder::PaneLayoutParams {
        Default::default()
    }
}
#[doc = "`PaneListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"workspace_id\": {"]
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
pub struct PaneListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneListParams {
    fn default() -> Self {
        Self {
            workspace_id: Default::default(),
        }
    }
}
impl PaneListParams {
    pub fn builder() -> builder::PaneListParams {
        Default::default()
    }
}
#[doc = "`PaneMoveDestination`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"split\","]
#[doc = "        \"tab_id\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ratio\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"float\""]
#[doc = "        },"]
#[doc = "        \"split\": {"]
#[doc = "          \"$ref\": \"#/$defs/SplitDirection\""]
#[doc = "        },"]
#[doc = "        \"tab_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"target_pane_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"new_tab\""]
#[doc = "        },"]
#[doc = "        \"workspace_id\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tab_label\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"new_workspace\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum PaneMoveDestination {
    #[serde(rename = "tab")]
    Tab {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        ratio: ::std::option::Option<f32>,
        split: SplitDirection,
        tab_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        target_pane_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "new_tab")]
    NewTab {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        workspace_id: ::std::option::Option<::std::string::String>,
    },
    #[serde(rename = "new_workspace")]
    NewWorkspace {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        label: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tab_label: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`PaneMoveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"destination\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"destination\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneMoveDestination\""]
#[doc = "    },"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneMoveParams {
    pub destination: PaneMoveDestination,
    #[serde(default)]
    pub focus: bool,
    pub pane_id: ::std::string::String,
}
impl PaneMoveParams {
    pub fn builder() -> builder::PaneMoveParams {
        Default::default()
    }
}
#[doc = "`PaneNeighborParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneDirection\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneNeighborParams {
    pub direction: PaneDirection,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl PaneNeighborParams {
    pub fn builder() -> builder::PaneNeighborParams {
        Default::default()
    }
}
#[doc = "`PaneProcessInfoParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneProcessInfoParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneProcessInfoParams {
    fn default() -> Self {
        Self {
            pane_id: Default::default(),
        }
    }
}
impl PaneProcessInfoParams {
    pub fn builder() -> builder::PaneProcessInfoParams {
        Default::default()
    }
}
#[doc = "`PaneReadParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"default\": \"text\","]
#[doc = "      \"$ref\": \"#/$defs/ReadFormat\""]
#[doc = "    },"]
#[doc = "    \"lines\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReadSource\""]
#[doc = "    },"]
#[doc = "    \"strip_ansi\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReadParams {
    #[serde(default = "defaults::pane_read_params_format")]
    pub format: ReadFormat,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lines: ::std::option::Option<u32>,
    pub pane_id: ::std::string::String,
    pub source: ReadSource,
    #[serde(default = "defaults::default_bool::<true>")]
    pub strip_ansi: bool,
}
impl PaneReadParams {
    pub fn builder() -> builder::PaneReadParams {
        Default::default()
    }
}
#[doc = "`PaneReleaseAgentParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReleaseAgentParams {
    pub agent: ::std::string::String,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    pub source: ::std::string::String,
}
impl PaneReleaseAgentParams {
    pub fn builder() -> builder::PaneReleaseAgentParams {
        Default::default()
    }
}
#[doc = "`PaneRenameParams`"]
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
#[doc = "    \"label\": {"]
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
pub struct PaneRenameParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
}
impl PaneRenameParams {
    pub fn builder() -> builder::PaneRenameParams {
        Default::default()
    }
}
#[doc = "`PaneReportAgentParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\","]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"agent_session_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_session_path\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneAgentState\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReportAgentParams {
    pub agent: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session_path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    pub source: ::std::string::String,
    pub state: PaneAgentState,
}
impl PaneReportAgentParams {
    pub fn builder() -> builder::PaneReportAgentParams {
        Default::default()
    }
}
#[doc = "`PaneReportAgentSessionParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"agent_session_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_session_path\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"session_start_source\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReportAgentSessionParams {
    pub agent: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_session_path: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub session_start_source: ::std::option::Option<::std::string::String>,
    pub source: ::std::string::String,
}
impl PaneReportAgentSessionParams {
    pub fn builder() -> builder::PaneReportAgentSessionParams {
        Default::default()
    }
}
#[doc = "`PaneReportMetadataParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"applies_to_source\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"clear_display_agent\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"clear_state_labels\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"clear_title\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"display_agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"state_labels\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 16,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ttl_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"maximum\": 86400000.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneReportMetadataParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub applies_to_source: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub clear_display_agent: bool,
    #[serde(default)]
    pub clear_state_labels: bool,
    #[serde(default)]
    pub clear_title: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display_agent: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    pub source: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub state_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub tokens: ::std::collections::HashMap<
        PaneReportMetadataParamsTokensKey,
        ::std::option::Option<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl_ms: ::std::option::Option<::std::num::NonZeroU64>,
}
impl PaneReportMetadataParams {
    pub fn builder() -> builder::PaneReportMetadataParams {
        Default::default()
    }
}
#[doc = "`PaneReportMetadataParamsTokensKey`"]
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
pub struct PaneReportMetadataParamsTokensKey(::std::string::String);
impl ::std::ops::Deref for PaneReportMetadataParamsTokensKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PaneReportMetadataParamsTokensKey> for ::std::string::String {
    fn from(value: PaneReportMetadataParamsTokensKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PaneReportMetadataParamsTokensKey {
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
impl ::std::convert::TryFrom<&str> for PaneReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PaneReportMetadataParamsTokensKey {
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
#[doc = "`PaneResizeParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneDirection\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneResizeParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f32>,
    pub direction: PaneDirection,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl PaneResizeParams {
    pub fn builder() -> builder::PaneResizeParams {
        Default::default()
    }
}
#[doc = "`PaneSendInputParams`"]
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
#[doc = "    \"keys\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneSendInputParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub keys: ::std::vec::Vec<::std::string::String>,
    pub pane_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl PaneSendInputParams {
    pub fn builder() -> builder::PaneSendInputParams {
        Default::default()
    }
}
#[doc = "`PaneSendKeysParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keys\","]
#[doc = "    \"pane_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"keys\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneSendKeysParams {
    pub keys: ::std::vec::Vec<::std::string::String>,
    pub pane_id: ::std::string::String,
}
impl PaneSendKeysParams {
    pub fn builder() -> builder::PaneSendKeysParams {
        Default::default()
    }
}
#[doc = "`PaneSendTextParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pane_id\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneSendTextParams {
    pub pane_id: ::std::string::String,
    pub text: ::std::string::String,
}
impl PaneSendTextParams {
    pub fn builder() -> builder::PaneSendTextParams {
        Default::default()
    }
}
#[doc = "`PaneSplitParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"direction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/SplitDirection\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ratio\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"target_pane_id\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneSplitParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    pub direction: SplitDirection,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ratio: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target_pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl PaneSplitParams {
    pub fn builder() -> builder::PaneSplitParams {
        Default::default()
    }
}
#[doc = "`PaneSwapParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PaneDirection\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
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
pub struct PaneSwapParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direction: ::std::option::Option<PaneDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source_pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target_pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneSwapParams {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            pane_id: Default::default(),
            source_pane_id: Default::default(),
            target_pane_id: Default::default(),
        }
    }
}
impl PaneSwapParams {
    pub fn builder() -> builder::PaneSwapParams {
        Default::default()
    }
}
#[doc = "`PaneTarget`"]
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
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneTarget {
    pub pane_id: ::std::string::String,
}
impl PaneTarget {
    pub fn builder() -> builder::PaneTarget {
        Default::default()
    }
}
#[doc = "`PaneWaitForOutputParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"match\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"lines\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"match\": {"]
#[doc = "      \"$ref\": \"#/$defs/OutputMatch\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReadSource\""]
#[doc = "    },"]
#[doc = "    \"strip_ansi\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneWaitForOutputParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lines: ::std::option::Option<u32>,
    #[serde(rename = "match")]
    pub match_: OutputMatch,
    pub pane_id: ::std::string::String,
    pub source: ReadSource,
    #[serde(default = "defaults::default_bool::<true>")]
    pub strip_ansi: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<u64>,
}
impl PaneWaitForOutputParams {
    pub fn builder() -> builder::PaneWaitForOutputParams {
        Default::default()
    }
}
#[doc = "`PaneZoomMode`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"toggle\","]
#[doc = "    \"on\","]
#[doc = "    \"off\""]
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
pub enum PaneZoomMode {
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
impl ::std::fmt::Display for PaneZoomMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Toggle => f.write_str("toggle"),
            Self::On => f.write_str("on"),
            Self::Off => f.write_str("off"),
        }
    }
}
impl ::std::str::FromStr for PaneZoomMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "toggle" => Ok(Self::Toggle),
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PaneZoomMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PaneZoomMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PaneZoomMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PaneZoomParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"mode\": {"]
#[doc = "      \"default\": \"toggle\","]
#[doc = "      \"$ref\": \"#/$defs/PaneZoomMode\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
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
pub struct PaneZoomParams {
    #[serde(default = "defaults::pane_zoom_params_mode")]
    pub mode: PaneZoomMode,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pane_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PaneZoomParams {
    fn default() -> Self {
        Self {
            mode: defaults::pane_zoom_params_mode(),
            pane_id: Default::default(),
        }
    }
}
impl PaneZoomParams {
    pub fn builder() -> builder::PaneZoomParams {
        Default::default()
    }
}
#[doc = "`PingParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PingParams(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for PingParams {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<PingParams>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: PingParams) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for PingParams
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`PluginActionInvokeParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"action_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"action_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"context\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PluginInvocationContext\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
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
pub struct PluginActionInvokeParams {
    pub action_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<PluginInvocationContext>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub plugin_id: ::std::option::Option<::std::string::String>,
}
impl PluginActionInvokeParams {
    pub fn builder() -> builder::PluginActionInvokeParams {
        Default::default()
    }
}
#[doc = "`PluginActionListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"plugin_id\": {"]
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
pub struct PluginActionListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub plugin_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PluginActionListParams {
    fn default() -> Self {
        Self {
            plugin_id: Default::default(),
        }
    }
}
impl PluginActionListParams {
    pub fn builder() -> builder::PluginActionListParams {
        Default::default()
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
#[doc = "`PluginLinkParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"path\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"enabled\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PluginSourceInfo\""]
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
pub struct PluginLinkParams {
    #[serde(default = "defaults::default_bool::<true>")]
    pub enabled: bool,
    pub path: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PluginSourceInfo>,
}
impl PluginLinkParams {
    pub fn builder() -> builder::PluginLinkParams {
        Default::default()
    }
}
#[doc = "`PluginListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"plugin_id\": {"]
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
pub struct PluginListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub plugin_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PluginListParams {
    fn default() -> Self {
        Self {
            plugin_id: Default::default(),
        }
    }
}
impl PluginListParams {
    pub fn builder() -> builder::PluginListParams {
        Default::default()
    }
}
#[doc = "`PluginLogListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"limit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
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
pub struct PluginLogListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub plugin_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PluginLogListParams {
    fn default() -> Self {
        Self {
            limit: Default::default(),
            plugin_id: Default::default(),
        }
    }
}
impl PluginLogListParams {
    pub fn builder() -> builder::PluginLogListParams {
        Default::default()
    }
}
#[doc = "`PluginPaneCloseParams`"]
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
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginPaneCloseParams {
    pub pane_id: ::std::string::String,
}
impl PluginPaneCloseParams {
    pub fn builder() -> builder::PluginPaneCloseParams {
        Default::default()
    }
}
#[doc = "`PluginPaneFocusParams`"]
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
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginPaneFocusParams {
    pub pane_id: ::std::string::String,
}
impl PluginPaneFocusParams {
    pub fn builder() -> builder::PluginPaneFocusParams {
        Default::default()
    }
}
#[doc = "`PluginPaneOpenParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"entrypoint\","]
#[doc = "    \"plugin_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"direction\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/SplitDirection\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"entrypoint\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
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
#[doc = "    \"placement\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPanePlacement\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"target_pane_id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
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
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
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
pub struct PluginPaneOpenParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direction: ::std::option::Option<SplitDirection>,
    pub entrypoint: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<PopupSize>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<PluginPanePlacement>,
    pub plugin_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target_pane_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<PopupSize>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl PluginPaneOpenParams {
    pub fn builder() -> builder::PluginPaneOpenParams {
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
#[doc = "`PluginSetEnabledParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"plugin_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginSetEnabledParams {
    pub plugin_id: ::std::string::String,
}
impl PluginSetEnabledParams {
    pub fn builder() -> builder::PluginSetEnabledParams {
        Default::default()
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
#[doc = "`PluginUnlinkParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"plugin_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"plugin_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PluginUnlinkParams {
    pub plugin_id: ::std::string::String,
}
impl PluginUnlinkParams {
    pub fn builder() -> builder::PluginUnlinkParams {
        Default::default()
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
#[doc = "`Request`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ping\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PingParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"server.stop\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"server.live_handoff\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/ServerLiveHandoffParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"server.reload_config\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"server.agent_manifests\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"server.reload_agent_manifests\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"notification.show\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/NotificationShowParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"client.window_title.set\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/ClientWindowTitleSetParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"client.window_title.clear\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"session.snapshot\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.create\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceCreateParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.get\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.focus\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.rename\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceRenameParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.move\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceMoveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.report_metadata\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceReportMetadataParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.close\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorkspaceTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree.create\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeCreateParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree.open\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeOpenParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"worktree.remove\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/WorktreeRemoveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.create\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabCreateParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.get\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.focus\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.rename\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabRenameParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.move\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabMoveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"tab.close\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/TabTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.get\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.read\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.explain\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.send_keys\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentSendKeysParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.rename\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentRenameParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.view.set\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewSetParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.view.clear\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentViewClearParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.focus\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.start\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentStartParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.prompt\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentPromptParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"agent.wait\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/AgentWaitParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.split\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSplitParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.swap\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSwapParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.move\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneMoveParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.zoom\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneZoomParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.layout\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneLayoutParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.process_info\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneProcessInfoParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout.export\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutExportParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout.apply\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutApplyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"layout.set_split_ratio\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/LayoutSetSplitRatioParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.neighbor\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneNeighborParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.edges\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneEdgesParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.focus_direction\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneFocusDirectionParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.resize\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneResizeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.current\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneCurrentParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.get\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.focus\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.rename\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneRenameParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.send_text\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSendTextParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.send_keys\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSendKeysParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.send_input\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneSendInputParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.read\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReadParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.graphics.set\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneGraphicsSetParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.graphics.clear\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneGraphicsClearParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.graphics.info\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.report_agent\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReportAgentParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.report_agent_session\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReportAgentSessionParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.report_metadata\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReportMetadataParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.clear_agent_authority\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneClearAgentAuthorityParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.release_agent\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneReleaseAgentParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.close\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneTarget\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"popup.close\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EmptyParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"events.subscribe\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EventsSubscribeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"events.wait\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/EventsWaitParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.wait_for_output\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PaneWaitForOutputParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"integration.install\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationInstallParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"integration.uninstall\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/IntegrationUninstallParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.link\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginLinkParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.unlink\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginUnlinkParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.enable\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginSetEnabledParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.disable\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginSetEnabledParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.action.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginActionListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.action.invoke\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginActionInvokeParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.log.list\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginLogListParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.pane.open\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPaneOpenParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.pane.focus\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPaneFocusParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"method\","]
#[doc = "        \"params\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"method\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"plugin.pane.close\""]
#[doc = "        },"]
#[doc = "        \"params\": {"]
#[doc = "          \"$ref\": \"#/$defs/PluginPaneCloseParams\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Request {
    Variant0 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PingParams,
    },
    Variant1 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant2 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: ServerLiveHandoffParams,
    },
    Variant3 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant4 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant5 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant6 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: NotificationShowParams,
    },
    Variant7 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: ClientWindowTitleSetParams,
    },
    Variant8 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant9 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant10 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceCreateParams,
    },
    Variant11 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant12 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceTarget,
    },
    Variant13 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceTarget,
    },
    Variant14 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceRenameParams,
    },
    Variant15 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceMoveParams,
    },
    Variant16 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceReportMetadataParams,
    },
    Variant17 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorkspaceTarget,
    },
    Variant18 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorktreeListParams,
    },
    Variant19 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorktreeCreateParams,
    },
    Variant20 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorktreeOpenParams,
    },
    Variant21 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: WorktreeRemoveParams,
    },
    Variant22 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabCreateParams,
    },
    Variant23 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabListParams,
    },
    Variant24 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabTarget,
    },
    Variant25 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabTarget,
    },
    Variant26 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabRenameParams,
    },
    Variant27 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabMoveParams,
    },
    Variant28 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: TabTarget,
    },
    Variant29 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant30 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentTarget,
    },
    Variant31 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentReadParams,
    },
    Variant32 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentTarget,
    },
    Variant33 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentSendKeysParams,
    },
    Variant34 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentRenameParams,
    },
    Variant35 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentViewSetParams,
    },
    Variant36 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentViewClearParams,
    },
    Variant37 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentTarget,
    },
    Variant38 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentStartParams,
    },
    Variant39 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentPromptParams,
    },
    Variant40 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: AgentWaitParams,
    },
    Variant41 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneSplitParams,
    },
    Variant42 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneSwapParams,
    },
    Variant43 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneMoveParams,
    },
    Variant44 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneZoomParams,
    },
    Variant45 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneLayoutParams,
    },
    Variant46 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneProcessInfoParams,
    },
    Variant47 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: LayoutExportParams,
    },
    Variant48 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: LayoutApplyParams,
    },
    Variant49 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: LayoutSetSplitRatioParams,
    },
    Variant50 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneNeighborParams,
    },
    Variant51 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneEdgesParams,
    },
    Variant52 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneFocusDirectionParams,
    },
    Variant53 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneResizeParams,
    },
    Variant54 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneListParams,
    },
    Variant55 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneCurrentParams,
    },
    Variant56 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneTarget,
    },
    Variant57 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneTarget,
    },
    Variant58 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneRenameParams,
    },
    Variant59 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneSendTextParams,
    },
    Variant60 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneSendKeysParams,
    },
    Variant61 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneSendInputParams,
    },
    Variant62 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneReadParams,
    },
    Variant63 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneGraphicsSetParams,
    },
    Variant64 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneGraphicsClearParams,
    },
    Variant65 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneTarget,
    },
    Variant66 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneReportAgentParams,
    },
    Variant67 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneReportAgentSessionParams,
    },
    Variant68 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneReportMetadataParams,
    },
    Variant69 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneClearAgentAuthorityParams,
    },
    Variant70 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneReleaseAgentParams,
    },
    Variant71 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneTarget,
    },
    Variant72 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EmptyParams,
    },
    Variant73 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EventsSubscribeParams,
    },
    Variant74 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: EventsWaitParams,
    },
    Variant75 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PaneWaitForOutputParams,
    },
    Variant76 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: IntegrationInstallParams,
    },
    Variant77 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: IntegrationUninstallParams,
    },
    Variant78 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginLinkParams,
    },
    Variant79 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginListParams,
    },
    Variant80 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginUnlinkParams,
    },
    Variant81 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginSetEnabledParams,
    },
    Variant82 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginSetEnabledParams,
    },
    Variant83 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginActionListParams,
    },
    Variant84 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginActionInvokeParams,
    },
    Variant85 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginLogListParams,
    },
    Variant86 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginPaneOpenParams,
    },
    Variant87 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginPaneFocusParams,
    },
    Variant88 {
        id: ::std::string::String,
        method: ::std::string::String,
        params: PluginPaneCloseParams,
    },
}
#[doc = "`ServerLiveHandoffParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"expected_protocol\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint32\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"expected_version\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"import_exe\": {"]
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
pub struct ServerLiveHandoffParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expected_protocol: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expected_version: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub import_exe: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ServerLiveHandoffParams {
    fn default() -> Self {
        Self {
            expected_protocol: Default::default(),
            expected_version: Default::default(),
            import_exe: Default::default(),
        }
    }
}
impl ServerLiveHandoffParams {
    pub fn builder() -> builder::ServerLiveHandoffParams {
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
#[doc = "`Subscription`"]
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
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"workspace.created\""]
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
#[doc = "          \"const\": \"workspace.updated\""]
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
#[doc = "          \"const\": \"workspace.metadata_updated\""]
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
#[doc = "          \"const\": \"workspace.renamed\""]
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
#[doc = "          \"const\": \"workspace.moved\""]
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
#[doc = "          \"const\": \"workspace.closed\""]
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
#[doc = "          \"const\": \"workspace.focused\""]
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
#[doc = "          \"const\": \"worktree.created\""]
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
#[doc = "          \"const\": \"worktree.opened\""]
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
#[doc = "          \"const\": \"worktree.removed\""]
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
#[doc = "          \"const\": \"tab.created\""]
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
#[doc = "          \"const\": \"tab.closed\""]
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
#[doc = "          \"const\": \"tab.focused\""]
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
#[doc = "          \"const\": \"tab.renamed\""]
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
#[doc = "          \"const\": \"tab.moved\""]
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
#[doc = "          \"const\": \"pane.created\""]
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
#[doc = "          \"const\": \"pane.closed\""]
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
#[doc = "          \"const\": \"pane.updated\""]
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
#[doc = "          \"const\": \"pane.focused\""]
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
#[doc = "          \"const\": \"pane.moved\""]
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
#[doc = "          \"const\": \"pane.exited\""]
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
#[doc = "          \"const\": \"pane.agent_detected\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"match\","]
#[doc = "        \"pane_id\","]
#[doc = "        \"source\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"lines\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"uint32\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"match\": {"]
#[doc = "          \"$ref\": \"#/$defs/OutputMatch\""]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"$ref\": \"#/$defs/ReadSource\""]
#[doc = "        },"]
#[doc = "        \"strip_ansi\": {"]
#[doc = "          \"default\": true,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.output_matched\""]
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
#[doc = "        \"agent_status\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/$defs/AgentStatus\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"pane_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"pane.agent_status_changed\""]
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
#[doc = "          \"const\": \"pane.scroll_changed\""]
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
#[doc = "          \"const\": \"layout.updated\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Subscription {
    #[serde(rename = "workspace.created")]
    WorkspaceCreated,
    #[serde(rename = "workspace.updated")]
    WorkspaceUpdated,
    #[serde(rename = "workspace.metadata_updated")]
    WorkspaceMetadataUpdated,
    #[serde(rename = "workspace.renamed")]
    WorkspaceRenamed,
    #[serde(rename = "workspace.moved")]
    WorkspaceMoved,
    #[serde(rename = "workspace.closed")]
    WorkspaceClosed,
    #[serde(rename = "workspace.focused")]
    WorkspaceFocused,
    #[serde(rename = "worktree.created")]
    WorktreeCreated,
    #[serde(rename = "worktree.opened")]
    WorktreeOpened,
    #[serde(rename = "worktree.removed")]
    WorktreeRemoved,
    #[serde(rename = "tab.created")]
    TabCreated,
    #[serde(rename = "tab.closed")]
    TabClosed,
    #[serde(rename = "tab.focused")]
    TabFocused,
    #[serde(rename = "tab.renamed")]
    TabRenamed,
    #[serde(rename = "tab.moved")]
    TabMoved,
    #[serde(rename = "pane.created")]
    PaneCreated,
    #[serde(rename = "pane.closed")]
    PaneClosed,
    #[serde(rename = "pane.updated")]
    PaneUpdated,
    #[serde(rename = "pane.focused")]
    PaneFocused,
    #[serde(rename = "pane.moved")]
    PaneMoved,
    #[serde(rename = "pane.exited")]
    PaneExited,
    #[serde(rename = "pane.agent_detected")]
    PaneAgentDetected,
    #[serde(rename = "pane.output_matched")]
    PaneOutputMatched {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        lines: ::std::option::Option<u32>,
        #[serde(rename = "match")]
        match_: OutputMatch,
        pane_id: ::std::string::String,
        source: ReadSource,
        #[serde(default = "defaults::default_bool::<true>")]
        strip_ansi: bool,
    },
    #[serde(rename = "pane.agent_status_changed")]
    PaneAgentStatusChanged {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        agent_status: ::std::option::Option<AgentStatus>,
        pane_id: ::std::string::String,
    },
    #[serde(rename = "pane.scroll_changed")]
    PaneScrollChanged { pane_id: ::std::string::String },
    #[serde(rename = "layout.updated")]
    LayoutUpdated,
}
#[doc = "`TabCreateParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TabCreateParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TabCreateParams {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            env: Default::default(),
            focus: Default::default(),
            label: Default::default(),
            workspace_id: Default::default(),
        }
    }
}
impl TabCreateParams {
    pub fn builder() -> builder::TabCreateParams {
        Default::default()
    }
}
#[doc = "`TabListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"workspace_id\": {"]
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
pub struct TabListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TabListParams {
    fn default() -> Self {
        Self {
            workspace_id: Default::default(),
        }
    }
}
impl TabListParams {
    pub fn builder() -> builder::TabListParams {
        Default::default()
    }
}
#[doc = "`TabMoveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"insert_index\","]
#[doc = "    \"tab_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"insert_index\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TabMoveParams {
    pub insert_index: u32,
    pub tab_id: ::std::string::String,
}
impl TabMoveParams {
    pub fn builder() -> builder::TabMoveParams {
        Default::default()
    }
}
#[doc = "`TabRenameParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"label\","]
#[doc = "    \"tab_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TabRenameParams {
    pub label: ::std::string::String,
    pub tab_id: ::std::string::String,
}
impl TabRenameParams {
    pub fn builder() -> builder::TabRenameParams {
        Default::default()
    }
}
#[doc = "`TabTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tab_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"tab_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TabTarget {
    pub tab_id: ::std::string::String,
}
impl TabTarget {
    pub fn builder() -> builder::TabTarget {
        Default::default()
    }
}
#[doc = "`ToastHerdrPosition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"top-left\","]
#[doc = "    \"top-right\","]
#[doc = "    \"bottom-left\","]
#[doc = "    \"bottom-right\""]
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
pub enum ToastHerdrPosition {
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
}
impl ::std::fmt::Display for ToastHerdrPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TopLeft => f.write_str("top-left"),
            Self::TopRight => f.write_str("top-right"),
            Self::BottomLeft => f.write_str("bottom-left"),
            Self::BottomRight => f.write_str("bottom-right"),
        }
    }
}
impl ::std::str::FromStr for ToastHerdrPosition {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "top-left" => Ok(Self::TopLeft),
            "top-right" => Ok(Self::TopRight),
            "bottom-left" => Ok(Self::BottomLeft),
            "bottom-right" => Ok(Self::BottomRight),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToastHerdrPosition {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ToastHerdrPosition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToastHerdrPosition {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WorkspaceCreateParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"env\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
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
pub struct WorkspaceCreateParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WorkspaceCreateParams {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            env: Default::default(),
            focus: Default::default(),
            label: Default::default(),
        }
    }
}
impl WorkspaceCreateParams {
    pub fn builder() -> builder::WorkspaceCreateParams {
        Default::default()
    }
}
#[doc = "`WorkspaceMoveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"insert_index\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"insert_index\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkspaceMoveParams {
    pub insert_index: u32,
    pub workspace_id: ::std::string::String,
}
impl WorkspaceMoveParams {
    pub fn builder() -> builder::WorkspaceMoveParams {
        Default::default()
    }
}
#[doc = "`WorkspaceRenameParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"label\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
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
pub struct WorkspaceRenameParams {
    pub label: ::std::string::String,
    pub workspace_id: ::std::string::String,
}
impl WorkspaceRenameParams {
    pub fn builder() -> builder::WorkspaceRenameParams {
        Default::default()
    }
}
#[doc = "`WorkspaceReportMetadataParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"tokens\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"seq\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tokens\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 16,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9_-]{1,32}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ttl_ms\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"maximum\": 86400000.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkspaceReportMetadataParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seq: ::std::option::Option<u64>,
    pub source: ::std::string::String,
    pub tokens: ::std::collections::HashMap<
        WorkspaceReportMetadataParamsTokensKey,
        ::std::option::Option<::std::string::String>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl_ms: ::std::option::Option<::std::num::NonZeroU64>,
    pub workspace_id: ::std::string::String,
}
impl WorkspaceReportMetadataParams {
    pub fn builder() -> builder::WorkspaceReportMetadataParams {
        Default::default()
    }
}
#[doc = "`WorkspaceReportMetadataParamsTokensKey`"]
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
pub struct WorkspaceReportMetadataParamsTokensKey(::std::string::String);
impl ::std::ops::Deref for WorkspaceReportMetadataParamsTokensKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WorkspaceReportMetadataParamsTokensKey> for ::std::string::String {
    fn from(value: WorkspaceReportMetadataParamsTokensKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WorkspaceReportMetadataParamsTokensKey {
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
impl ::std::convert::TryFrom<&str> for WorkspaceReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WorkspaceReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkspaceReportMetadataParamsTokensKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WorkspaceReportMetadataParamsTokensKey {
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
#[doc = "`WorkspaceTarget`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTarget {
    pub workspace_id: ::std::string::String,
}
impl WorkspaceTarget {
    pub fn builder() -> builder::WorkspaceTarget {
        Default::default()
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
#[doc = "`WorktreeCreateParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"base\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"branch\": {"]
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
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorktreeCreateParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WorktreeCreateParams {
    fn default() -> Self {
        Self {
            base: Default::default(),
            branch: Default::default(),
            cwd: Default::default(),
            focus: Default::default(),
            label: Default::default(),
            path: Default::default(),
            workspace_id: Default::default(),
        }
    }
}
impl WorktreeCreateParams {
    pub fn builder() -> builder::WorktreeCreateParams {
        Default::default()
    }
}
#[doc = "`WorktreeListParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cwd\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorktreeListParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WorktreeListParams {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            workspace_id: Default::default(),
        }
    }
}
impl WorktreeListParams {
    pub fn builder() -> builder::WorktreeListParams {
        Default::default()
    }
}
#[doc = "`WorktreeOpenParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
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
#[doc = "    \"focus\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
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
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorktreeOpenParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub focus: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WorktreeOpenParams {
    fn default() -> Self {
        Self {
            branch: Default::default(),
            cwd: Default::default(),
            focus: Default::default(),
            label: Default::default(),
            path: Default::default(),
            workspace_id: Default::default(),
        }
    }
}
impl WorktreeOpenParams {
    pub fn builder() -> builder::WorktreeOpenParams {
        Default::default()
    }
}
#[doc = "`WorktreeRemoveParams`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"force\": {"]
#[doc = "      \"default\": false,"]
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
pub struct WorktreeRemoveParams {
    #[serde(default)]
    pub force: bool,
    pub workspace_id: ::std::string::String,
}
impl WorktreeRemoveParams {
    pub fn builder() -> builder::WorktreeRemoveParams {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AgentPromptParams {
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        wait: ::std::result::Result<
            ::std::option::Option<super::AgentPromptWaitOptions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentPromptParams {
        fn default() -> Self {
            Self {
                target: Err("no value supplied for target".to_string()),
                text: Err("no value supplied for text".to_string()),
                wait: Ok(Default::default()),
            }
        }
    }
    impl AgentPromptParams {
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
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
        pub fn wait<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentPromptWaitOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.wait = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wait: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentPromptParams> for super::AgentPromptParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentPromptParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target: value.target?,
                text: value.text?,
                wait: value.wait?,
            })
        }
    }
    impl ::std::convert::From<super::AgentPromptParams> for AgentPromptParams {
        fn from(value: super::AgentPromptParams) -> Self {
            Self {
                target: Ok(value.target),
                text: Ok(value.text),
                wait: Ok(value.wait),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentPromptWaitOptions {
        timeout_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        until: ::std::result::Result<::std::vec::Vec<super::AgentStatus>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentPromptWaitOptions {
        fn default() -> Self {
            Self {
                timeout_ms: Ok(Default::default()),
                until: Ok(Default::default()),
            }
        }
    }
    impl AgentPromptWaitOptions {
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {e}"));
            self
        }
        pub fn until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.until = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for until: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentPromptWaitOptions> for super::AgentPromptWaitOptions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentPromptWaitOptions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                timeout_ms: value.timeout_ms?,
                until: value.until?,
            })
        }
    }
    impl ::std::convert::From<super::AgentPromptWaitOptions> for AgentPromptWaitOptions {
        fn from(value: super::AgentPromptWaitOptions) -> Self {
            Self {
                timeout_ms: Ok(value.timeout_ms),
                until: Ok(value.until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentReadParams {
        format: ::std::result::Result<super::ReadFormat, ::std::string::String>,
        lines: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        source: ::std::result::Result<super::ReadSource, ::std::string::String>,
        strip_ansi: ::std::result::Result<bool, ::std::string::String>,
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentReadParams {
        fn default() -> Self {
            Self {
                format: Ok(super::defaults::agent_read_params_format()),
                lines: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                strip_ansi: Ok(super::defaults::default_bool::<true>()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl AgentReadParams {
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
        pub fn lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lines: {e}"));
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
        pub fn strip_ansi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.strip_ansi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strip_ansi: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentReadParams> for super::AgentReadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentReadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                lines: value.lines?,
                source: value.source?,
                strip_ansi: value.strip_ansi?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::AgentReadParams> for AgentReadParams {
        fn from(value: super::AgentReadParams) -> Self {
            Self {
                format: Ok(value.format),
                lines: Ok(value.lines),
                source: Ok(value.source),
                strip_ansi: Ok(value.strip_ansi),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentRenameParams {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentRenameParams {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl AgentRenameParams {
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
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentRenameParams> for super::AgentRenameParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentRenameParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::AgentRenameParams> for AgentRenameParams {
        fn from(value: super::AgentRenameParams) -> Self {
            Self {
                name: Ok(value.name),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentSendKeysParams {
        keys: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentSendKeysParams {
        fn default() -> Self {
            Self {
                keys: Err("no value supplied for keys".to_string()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl AgentSendKeysParams {
        pub fn keys<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keys = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keys: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentSendKeysParams> for super::AgentSendKeysParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentSendKeysParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keys: value.keys?,
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::AgentSendKeysParams> for AgentSendKeysParams {
        fn from(value: super::AgentSendKeysParams) -> Self {
            Self {
                keys: Ok(value.keys),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentStartParams {
        args: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        timeout_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentStartParams {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                name: Err("no value supplied for name".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                timeout_ms: Ok(Default::default()),
            }
        }
    }
    impl AgentStartParams {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {e}"));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
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
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentStartParams> for super::AgentStartParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentStartParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                kind: value.kind?,
                name: value.name?,
                pane_id: value.pane_id?,
                timeout_ms: value.timeout_ms?,
            })
        }
    }
    impl ::std::convert::From<super::AgentStartParams> for AgentStartParams {
        fn from(value: super::AgentStartParams) -> Self {
            Self {
                args: Ok(value.args),
                kind: Ok(value.kind),
                name: Ok(value.name),
                pane_id: Ok(value.pane_id),
                timeout_ms: Ok(value.timeout_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentTarget {
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentTarget {
        fn default() -> Self {
            Self {
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl AgentTarget {
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentTarget> for super::AgentTarget {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentTarget,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::AgentTarget> for AgentTarget {
        fn from(value: super::AgentTarget) -> Self {
            Self {
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentViewClearParams {
        source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentViewClearParams {
        fn default() -> Self {
            Self {
                source: Ok(Default::default()),
            }
        }
    }
    impl AgentViewClearParams {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentViewClearParams> for super::AgentViewClearParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentViewClearParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::AgentViewClearParams> for AgentViewClearParams {
        fn from(value: super::AgentViewClearParams) -> Self {
            Self {
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentViewSetParams {
        filter: ::std::result::Result<
            ::std::option::Option<super::AgentViewFilter>,
            ::std::string::String,
        >,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::vec::Vec<super::AgentViewSort>, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentViewSetParams {
        fn default() -> Self {
            Self {
                filter: Ok(Default::default()),
                label: Ok(Default::default()),
                sort: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl AgentViewSetParams {
        pub fn filter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentViewFilter>>,
            T::Error: ::std::fmt::Display,
        {
            self.filter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter: {e}"));
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
        pub fn sort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentViewSort>>,
            T::Error: ::std::fmt::Display,
        {
            self.sort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sort: {e}"));
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
    }
    impl ::std::convert::TryFrom<AgentViewSetParams> for super::AgentViewSetParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentViewSetParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                filter: value.filter?,
                label: value.label?,
                sort: value.sort?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::AgentViewSetParams> for AgentViewSetParams {
        fn from(value: super::AgentViewSetParams) -> Self {
            Self {
                filter: Ok(value.filter),
                label: Ok(value.label),
                sort: Ok(value.sort),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentViewSort {
        field: ::std::result::Result<super::AgentViewSortField, ::std::string::String>,
        order: ::std::result::Result<super::AgentViewSortOrder, ::std::string::String>,
    }
    impl ::std::default::Default for AgentViewSort {
        fn default() -> Self {
            Self {
                field: Err("no value supplied for field".to_string()),
                order: Ok(super::defaults::agent_view_sort_order()),
            }
        }
    }
    impl AgentViewSort {
        pub fn field<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentViewSortField>,
            T::Error: ::std::fmt::Display,
        {
            self.field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field: {e}"));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentViewSortOrder>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentViewSort> for super::AgentViewSort {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentViewSort,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                field: value.field?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::AgentViewSort> for AgentViewSort {
        fn from(value: super::AgentViewSort) -> Self {
            Self {
                field: Ok(value.field),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentWaitParams {
        target: ::std::result::Result<::std::string::String, ::std::string::String>,
        timeout_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        until: ::std::result::Result<::std::vec::Vec<super::AgentStatus>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentWaitParams {
        fn default() -> Self {
            Self {
                target: Err("no value supplied for target".to_string()),
                timeout_ms: Ok(Default::default()),
                until: Ok(Default::default()),
            }
        }
    }
    impl AgentWaitParams {
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {e}"));
            self
        }
        pub fn until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.until = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for until: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentWaitParams> for super::AgentWaitParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentWaitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target: value.target?,
                timeout_ms: value.timeout_ms?,
                until: value.until?,
            })
        }
    }
    impl ::std::convert::From<super::AgentWaitParams> for AgentWaitParams {
        fn from(value: super::AgentWaitParams) -> Self {
            Self {
                target: Ok(value.target),
                timeout_ms: Ok(value.timeout_ms),
                until: Ok(value.until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientWindowTitleSetParams {
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ClientWindowTitleSetParams {
        fn default() -> Self {
            Self {
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl ClientWindowTitleSetParams {
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
    impl ::std::convert::TryFrom<ClientWindowTitleSetParams> for super::ClientWindowTitleSetParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientWindowTitleSetParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ClientWindowTitleSetParams> for ClientWindowTitleSetParams {
        fn from(value: super::ClientWindowTitleSetParams) -> Self {
            Self {
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventsSubscribeParams {
        subscriptions:
            ::std::result::Result<::std::vec::Vec<super::Subscription>, ::std::string::String>,
    }
    impl ::std::default::Default for EventsSubscribeParams {
        fn default() -> Self {
            Self {
                subscriptions: Err("no value supplied for subscriptions".to_string()),
            }
        }
    }
    impl EventsSubscribeParams {
        pub fn subscriptions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Subscription>>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriptions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriptions: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsSubscribeParams> for super::EventsSubscribeParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsSubscribeParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subscriptions: value.subscriptions?,
            })
        }
    }
    impl ::std::convert::From<super::EventsSubscribeParams> for EventsSubscribeParams {
        fn from(value: super::EventsSubscribeParams) -> Self {
            Self {
                subscriptions: Ok(value.subscriptions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventsWaitParams {
        match_event: ::std::result::Result<super::EventMatch, ::std::string::String>,
        timeout_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
    }
    impl ::std::default::Default for EventsWaitParams {
        fn default() -> Self {
            Self {
                match_event: Err("no value supplied for match_event".to_string()),
                timeout_ms: Ok(Default::default()),
            }
        }
    }
    impl EventsWaitParams {
        pub fn match_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventMatch>,
            T::Error: ::std::fmt::Display,
        {
            self.match_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_event: {e}"));
            self
        }
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsWaitParams> for super::EventsWaitParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsWaitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                match_event: value.match_event?,
                timeout_ms: value.timeout_ms?,
            })
        }
    }
    impl ::std::convert::From<super::EventsWaitParams> for EventsWaitParams {
        fn from(value: super::EventsWaitParams) -> Self {
            Self {
                match_event: Ok(value.match_event),
                timeout_ms: Ok(value.timeout_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntegrationInstallParams {
        target: ::std::result::Result<super::IntegrationTarget, ::std::string::String>,
    }
    impl ::std::default::Default for IntegrationInstallParams {
        fn default() -> Self {
            Self {
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl IntegrationInstallParams {
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IntegrationTarget>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IntegrationInstallParams> for super::IntegrationInstallParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IntegrationInstallParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::IntegrationInstallParams> for IntegrationInstallParams {
        fn from(value: super::IntegrationInstallParams) -> Self {
            Self {
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntegrationUninstallParams {
        target: ::std::result::Result<super::IntegrationTarget, ::std::string::String>,
    }
    impl ::std::default::Default for IntegrationUninstallParams {
        fn default() -> Self {
            Self {
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl IntegrationUninstallParams {
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IntegrationTarget>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IntegrationUninstallParams> for super::IntegrationUninstallParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IntegrationUninstallParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target: value.target?,
            })
        }
    }
    impl ::std::convert::From<super::IntegrationUninstallParams> for IntegrationUninstallParams {
        fn from(value: super::IntegrationUninstallParams) -> Self {
            Self {
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayoutApplyParams {
        focus: ::std::result::Result<bool, ::std::string::String>,
        root: ::std::result::Result<super::LayoutNode, ::std::string::String>,
        tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LayoutApplyParams {
        fn default() -> Self {
            Self {
                focus: Ok(Default::default()),
                root: Err("no value supplied for root".to_string()),
                tab_id: Ok(Default::default()),
                tab_label: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl LayoutApplyParams {
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
    }
    impl ::std::convert::TryFrom<LayoutApplyParams> for super::LayoutApplyParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LayoutApplyParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                focus: value.focus?,
                root: value.root?,
                tab_id: value.tab_id?,
                tab_label: value.tab_label?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::LayoutApplyParams> for LayoutApplyParams {
        fn from(value: super::LayoutApplyParams) -> Self {
            Self {
                focus: Ok(value.focus),
                root: Ok(value.root),
                tab_id: Ok(value.tab_id),
                tab_label: Ok(value.tab_label),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayoutExportParams {
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LayoutExportParams {
        fn default() -> Self {
            Self {
                pane_id: Ok(Default::default()),
                tab_id: Ok(Default::default()),
            }
        }
    }
    impl LayoutExportParams {
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<LayoutExportParams> for super::LayoutExportParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LayoutExportParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
                tab_id: value.tab_id?,
            })
        }
    }
    impl ::std::convert::From<super::LayoutExportParams> for LayoutExportParams {
        fn from(value: super::LayoutExportParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
                tab_id: Ok(value.tab_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayoutSetSplitRatioParams {
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        path: ::std::result::Result<::std::vec::Vec<bool>, ::std::string::String>,
        ratio: ::std::result::Result<f32, ::std::string::String>,
        tab_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LayoutSetSplitRatioParams {
        fn default() -> Self {
            Self {
                pane_id: Ok(Default::default()),
                path: Err("no value supplied for path".to_string()),
                ratio: Err("no value supplied for ratio".to_string()),
                tab_id: Ok(Default::default()),
            }
        }
    }
    impl LayoutSetSplitRatioParams {
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
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
    }
    impl ::std::convert::TryFrom<LayoutSetSplitRatioParams> for super::LayoutSetSplitRatioParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LayoutSetSplitRatioParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
                path: value.path?,
                ratio: value.ratio?,
                tab_id: value.tab_id?,
            })
        }
    }
    impl ::std::convert::From<super::LayoutSetSplitRatioParams> for LayoutSetSplitRatioParams {
        fn from(value: super::LayoutSetSplitRatioParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
                path: Ok(value.path),
                ratio: Ok(value.ratio),
                tab_id: Ok(value.tab_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NotificationShowParams {
        body: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        position: ::std::result::Result<
            ::std::option::Option<super::ToastHerdrPosition>,
            ::std::string::String,
        >,
        sound: ::std::result::Result<
            ::std::option::Option<super::NotificationShowSound>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for NotificationShowParams {
        fn default() -> Self {
            Self {
                body: Ok(Default::default()),
                position: Ok(Default::default()),
                sound: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl NotificationShowParams {
        pub fn body<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body: {e}"));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToastHerdrPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {e}"));
            self
        }
        pub fn sound<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NotificationShowSound>>,
            T::Error: ::std::fmt::Display,
        {
            self.sound = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound: {e}"));
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
    impl ::std::convert::TryFrom<NotificationShowParams> for super::NotificationShowParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NotificationShowParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                body: value.body?,
                position: value.position?,
                sound: value.sound?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::NotificationShowParams> for NotificationShowParams {
        fn from(value: super::NotificationShowParams) -> Self {
            Self {
                body: Ok(value.body),
                position: Ok(value.position),
                sound: Ok(value.sound),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneClearAgentAuthorityParams {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneClearAgentAuthorityParams {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
                seq: Ok(Default::default()),
                source: Ok(Default::default()),
            }
        }
    }
    impl PaneClearAgentAuthorityParams {
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
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneClearAgentAuthorityParams>
        for super::PaneClearAgentAuthorityParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneClearAgentAuthorityParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
                seq: value.seq?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::PaneClearAgentAuthorityParams> for PaneClearAgentAuthorityParams {
        fn from(value: super::PaneClearAgentAuthorityParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
                seq: Ok(value.seq),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneCurrentParams {
        caller_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneCurrentParams {
        fn default() -> Self {
            Self {
                caller_pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneCurrentParams {
        pub fn caller_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.caller_pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for caller_pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneCurrentParams> for super::PaneCurrentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneCurrentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                caller_pane_id: value.caller_pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneCurrentParams> for PaneCurrentParams {
        fn from(value: super::PaneCurrentParams) -> Self {
            Self {
                caller_pane_id: Ok(value.caller_pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneEdgesParams {
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneEdgesParams {
        fn default() -> Self {
            Self {
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneEdgesParams {
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneEdgesParams> for super::PaneEdgesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneEdgesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneEdgesParams> for PaneEdgesParams {
        fn from(value: super::PaneEdgesParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneFocusDirectionParams {
        direction: ::std::result::Result<super::PaneDirection, ::std::string::String>,
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneFocusDirectionParams {
        fn default() -> Self {
            Self {
                direction: Err("no value supplied for direction".to_string()),
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneFocusDirectionParams {
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
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneFocusDirectionParams> for super::PaneFocusDirectionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneFocusDirectionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneFocusDirectionParams> for PaneFocusDirectionParams {
        fn from(value: super::PaneFocusDirectionParams) -> Self {
            Self {
                direction: Ok(value.direction),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneGraphicsClearParams {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneGraphicsClearParams {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneGraphicsClearParams {
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
    impl ::std::convert::TryFrom<PaneGraphicsClearParams> for super::PaneGraphicsClearParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneGraphicsClearParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneGraphicsClearParams> for PaneGraphicsClearParams {
        fn from(value: super::PaneGraphicsClearParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneGraphicsPlacementParams {
        grid_cols: ::std::result::Result<u32, ::std::string::String>,
        grid_rows: ::std::result::Result<u32, ::std::string::String>,
        viewport_col: ::std::result::Result<i32, ::std::string::String>,
        viewport_row: ::std::result::Result<i32, ::std::string::String>,
    }
    impl ::std::default::Default for PaneGraphicsPlacementParams {
        fn default() -> Self {
            Self {
                grid_cols: Ok(Default::default()),
                grid_rows: Ok(Default::default()),
                viewport_col: Ok(Default::default()),
                viewport_row: Ok(Default::default()),
            }
        }
    }
    impl PaneGraphicsPlacementParams {
        pub fn grid_cols<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.grid_cols = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grid_cols: {e}"));
            self
        }
        pub fn grid_rows<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.grid_rows = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grid_rows: {e}"));
            self
        }
        pub fn viewport_col<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.viewport_col = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for viewport_col: {e}"));
            self
        }
        pub fn viewport_row<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.viewport_row = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for viewport_row: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneGraphicsPlacementParams> for super::PaneGraphicsPlacementParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneGraphicsPlacementParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                grid_cols: value.grid_cols?,
                grid_rows: value.grid_rows?,
                viewport_col: value.viewport_col?,
                viewport_row: value.viewport_row?,
            })
        }
    }
    impl ::std::convert::From<super::PaneGraphicsPlacementParams> for PaneGraphicsPlacementParams {
        fn from(value: super::PaneGraphicsPlacementParams) -> Self {
            Self {
                grid_cols: Ok(value.grid_cols),
                grid_rows: Ok(value.grid_rows),
                viewport_col: Ok(value.viewport_col),
                viewport_row: Ok(value.viewport_row),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneGraphicsSetParams {
        data_base64: ::std::result::Result<::std::string::String, ::std::string::String>,
        format: ::std::result::Result<super::PaneGraphicsFormat, ::std::string::String>,
        image_height: ::std::result::Result<u32, ::std::string::String>,
        image_width: ::std::result::Result<u32, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        placement: ::std::result::Result<super::PaneGraphicsPlacementParams, ::std::string::String>,
    }
    impl ::std::default::Default for PaneGraphicsSetParams {
        fn default() -> Self {
            Self {
                data_base64: Ok(Default::default()),
                format: Err("no value supplied for format".to_string()),
                image_height: Err("no value supplied for image_height".to_string()),
                image_width: Err("no value supplied for image_width".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                placement: Ok(super::defaults::pane_graphics_set_params_placement()),
            }
        }
    }
    impl PaneGraphicsSetParams {
        pub fn data_base64<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data_base64 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_base64: {e}"));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneGraphicsFormat>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn image_height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.image_height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_height: {e}"));
            self
        }
        pub fn image_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.image_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_width: {e}"));
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
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneGraphicsPlacementParams>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for placement: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneGraphicsSetParams> for super::PaneGraphicsSetParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneGraphicsSetParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data_base64: value.data_base64?,
                format: value.format?,
                image_height: value.image_height?,
                image_width: value.image_width?,
                pane_id: value.pane_id?,
                placement: value.placement?,
            })
        }
    }
    impl ::std::convert::From<super::PaneGraphicsSetParams> for PaneGraphicsSetParams {
        fn from(value: super::PaneGraphicsSetParams) -> Self {
            Self {
                data_base64: Ok(value.data_base64),
                format: Ok(value.format),
                image_height: Ok(value.image_height),
                image_width: Ok(value.image_width),
                pane_id: Ok(value.pane_id),
                placement: Ok(value.placement),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneLayoutParams {
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneLayoutParams {
        fn default() -> Self {
            Self {
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneLayoutParams {
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneLayoutParams> for super::PaneLayoutParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneLayoutParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneLayoutParams> for PaneLayoutParams {
        fn from(value: super::PaneLayoutParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneListParams {
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneListParams {
        fn default() -> Self {
            Self {
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl PaneListParams {
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
    }
    impl ::std::convert::TryFrom<PaneListParams> for super::PaneListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneListParams> for PaneListParams {
        fn from(value: super::PaneListParams) -> Self {
            Self {
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneMoveParams {
        destination: ::std::result::Result<super::PaneMoveDestination, ::std::string::String>,
        focus: ::std::result::Result<bool, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneMoveParams {
        fn default() -> Self {
            Self {
                destination: Err("no value supplied for destination".to_string()),
                focus: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneMoveParams {
        pub fn destination<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneMoveDestination>,
            T::Error: ::std::fmt::Display,
        {
            self.destination = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for destination: {e}"));
            self
        }
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
    impl ::std::convert::TryFrom<PaneMoveParams> for super::PaneMoveParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneMoveParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                destination: value.destination?,
                focus: value.focus?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneMoveParams> for PaneMoveParams {
        fn from(value: super::PaneMoveParams) -> Self {
            Self {
                destination: Ok(value.destination),
                focus: Ok(value.focus),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneNeighborParams {
        direction: ::std::result::Result<super::PaneDirection, ::std::string::String>,
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneNeighborParams {
        fn default() -> Self {
            Self {
                direction: Err("no value supplied for direction".to_string()),
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneNeighborParams {
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
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneNeighborParams> for super::PaneNeighborParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneNeighborParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneNeighborParams> for PaneNeighborParams {
        fn from(value: super::PaneNeighborParams) -> Self {
            Self {
                direction: Ok(value.direction),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneProcessInfoParams {
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneProcessInfoParams {
        fn default() -> Self {
            Self {
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneProcessInfoParams {
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneProcessInfoParams> for super::PaneProcessInfoParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneProcessInfoParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneProcessInfoParams> for PaneProcessInfoParams {
        fn from(value: super::PaneProcessInfoParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReadParams {
        format: ::std::result::Result<super::ReadFormat, ::std::string::String>,
        lines: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::ReadSource, ::std::string::String>,
        strip_ansi: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for PaneReadParams {
        fn default() -> Self {
            Self {
                format: Ok(super::defaults::pane_read_params_format()),
                lines: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                source: Err("no value supplied for source".to_string()),
                strip_ansi: Ok(super::defaults::default_bool::<true>()),
            }
        }
    }
    impl PaneReadParams {
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
        pub fn lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lines: {e}"));
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
        pub fn strip_ansi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.strip_ansi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strip_ansi: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneReadParams> for super::PaneReadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                lines: value.lines?,
                pane_id: value.pane_id?,
                source: value.source?,
                strip_ansi: value.strip_ansi?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReadParams> for PaneReadParams {
        fn from(value: super::PaneReadParams) -> Self {
            Self {
                format: Ok(value.format),
                lines: Ok(value.lines),
                pane_id: Ok(value.pane_id),
                source: Ok(value.source),
                strip_ansi: Ok(value.strip_ansi),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReleaseAgentParams {
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneReleaseAgentParams {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                seq: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl PaneReleaseAgentParams {
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
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
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
    }
    impl ::std::convert::TryFrom<PaneReleaseAgentParams> for super::PaneReleaseAgentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReleaseAgentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                pane_id: value.pane_id?,
                seq: value.seq?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReleaseAgentParams> for PaneReleaseAgentParams {
        fn from(value: super::PaneReleaseAgentParams) -> Self {
            Self {
                agent: Ok(value.agent),
                pane_id: Ok(value.pane_id),
                seq: Ok(value.seq),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneRenameParams {
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneRenameParams {
        fn default() -> Self {
            Self {
                label: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneRenameParams {
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
    }
    impl ::std::convert::TryFrom<PaneRenameParams> for super::PaneRenameParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneRenameParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneRenameParams> for PaneRenameParams {
        fn from(value: super::PaneRenameParams) -> Self {
            Self {
                label: Ok(value.label),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReportAgentParams {
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        agent_session_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_session_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        state: ::std::result::Result<super::PaneAgentState, ::std::string::String>,
    }
    impl ::std::default::Default for PaneReportAgentParams {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                agent_session_id: Ok(Default::default()),
                agent_session_path: Ok(Default::default()),
                message: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                seq: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                state: Err("no value supplied for state".to_string()),
            }
        }
    }
    impl PaneReportAgentParams {
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
        pub fn agent_session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_session_id: {e}"));
            self
        }
        pub fn agent_session_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session_path = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_session_path: {e}")
            });
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
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
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
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
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneAgentState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneReportAgentParams> for super::PaneReportAgentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReportAgentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                agent_session_id: value.agent_session_id?,
                agent_session_path: value.agent_session_path?,
                message: value.message?,
                pane_id: value.pane_id?,
                seq: value.seq?,
                source: value.source?,
                state: value.state?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReportAgentParams> for PaneReportAgentParams {
        fn from(value: super::PaneReportAgentParams) -> Self {
            Self {
                agent: Ok(value.agent),
                agent_session_id: Ok(value.agent_session_id),
                agent_session_path: Ok(value.agent_session_path),
                message: Ok(value.message),
                pane_id: Ok(value.pane_id),
                seq: Ok(value.seq),
                source: Ok(value.source),
                state: Ok(value.state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReportAgentSessionParams {
        agent: ::std::result::Result<::std::string::String, ::std::string::String>,
        agent_session_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_session_path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        session_start_source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneReportAgentSessionParams {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                agent_session_id: Ok(Default::default()),
                agent_session_path: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                seq: Ok(Default::default()),
                session_start_source: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl PaneReportAgentSessionParams {
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
        pub fn agent_session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_session_id: {e}"));
            self
        }
        pub fn agent_session_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_session_path = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_session_path: {e}")
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
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
            self
        }
        pub fn session_start_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_start_source = value.try_into().map_err(|e| {
                format!("error converting supplied value for session_start_source: {e}")
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
    }
    impl ::std::convert::TryFrom<PaneReportAgentSessionParams> for super::PaneReportAgentSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReportAgentSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                agent_session_id: value.agent_session_id?,
                agent_session_path: value.agent_session_path?,
                pane_id: value.pane_id?,
                seq: value.seq?,
                session_start_source: value.session_start_source?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReportAgentSessionParams> for PaneReportAgentSessionParams {
        fn from(value: super::PaneReportAgentSessionParams) -> Self {
            Self {
                agent: Ok(value.agent),
                agent_session_id: Ok(value.agent_session_id),
                agent_session_path: Ok(value.agent_session_path),
                pane_id: Ok(value.pane_id),
                seq: Ok(value.seq),
                session_start_source: Ok(value.session_start_source),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneReportMetadataParams {
        agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        applies_to_source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        clear_display_agent: ::std::result::Result<bool, ::std::string::String>,
        clear_state_labels: ::std::result::Result<bool, ::std::string::String>,
        clear_title: ::std::result::Result<bool, ::std::string::String>,
        display_agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        state_labels: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tokens: ::std::result::Result<
            ::std::collections::HashMap<
                super::PaneReportMetadataParamsTokensKey,
                ::std::option::Option<::std::string::String>,
            >,
            ::std::string::String,
        >,
        ttl_ms: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneReportMetadataParams {
        fn default() -> Self {
            Self {
                agent: Ok(Default::default()),
                applies_to_source: Ok(Default::default()),
                clear_display_agent: Ok(Default::default()),
                clear_state_labels: Ok(Default::default()),
                clear_title: Ok(Default::default()),
                display_agent: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                seq: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                state_labels: Ok(Default::default()),
                title: Ok(Default::default()),
                tokens: Ok(Default::default()),
                ttl_ms: Ok(Default::default()),
            }
        }
    }
    impl PaneReportMetadataParams {
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
        pub fn applies_to_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.applies_to_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for applies_to_source: {e}"));
            self
        }
        pub fn clear_display_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.clear_display_agent = value.try_into().map_err(|e| {
                format!("error converting supplied value for clear_display_agent: {e}")
            });
            self
        }
        pub fn clear_state_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.clear_state_labels = value.try_into().map_err(|e| {
                format!("error converting supplied value for clear_state_labels: {e}")
            });
            self
        }
        pub fn clear_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.clear_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clear_title: {e}"));
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
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
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
                ::std::collections::HashMap<
                    super::PaneReportMetadataParamsTokensKey,
                    ::std::option::Option<::std::string::String>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneReportMetadataParams> for super::PaneReportMetadataParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneReportMetadataParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                applies_to_source: value.applies_to_source?,
                clear_display_agent: value.clear_display_agent?,
                clear_state_labels: value.clear_state_labels?,
                clear_title: value.clear_title?,
                display_agent: value.display_agent?,
                pane_id: value.pane_id?,
                seq: value.seq?,
                source: value.source?,
                state_labels: value.state_labels?,
                title: value.title?,
                tokens: value.tokens?,
                ttl_ms: value.ttl_ms?,
            })
        }
    }
    impl ::std::convert::From<super::PaneReportMetadataParams> for PaneReportMetadataParams {
        fn from(value: super::PaneReportMetadataParams) -> Self {
            Self {
                agent: Ok(value.agent),
                applies_to_source: Ok(value.applies_to_source),
                clear_display_agent: Ok(value.clear_display_agent),
                clear_state_labels: Ok(value.clear_state_labels),
                clear_title: Ok(value.clear_title),
                display_agent: Ok(value.display_agent),
                pane_id: Ok(value.pane_id),
                seq: Ok(value.seq),
                source: Ok(value.source),
                state_labels: Ok(value.state_labels),
                title: Ok(value.title),
                tokens: Ok(value.tokens),
                ttl_ms: Ok(value.ttl_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneResizeParams {
        amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        direction: ::std::result::Result<super::PaneDirection, ::std::string::String>,
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneResizeParams {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                direction: Err("no value supplied for direction".to_string()),
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneResizeParams {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
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
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneResizeParams> for super::PaneResizeParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneResizeParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                direction: value.direction?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneResizeParams> for PaneResizeParams {
        fn from(value: super::PaneResizeParams) -> Self {
            Self {
                amount: Ok(value.amount),
                direction: Ok(value.direction),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSendInputParams {
        keys: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneSendInputParams {
        fn default() -> Self {
            Self {
                keys: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                text: Ok(Default::default()),
            }
        }
    }
    impl PaneSendInputParams {
        pub fn keys<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keys = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keys: {e}"));
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneSendInputParams> for super::PaneSendInputParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSendInputParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keys: value.keys?,
                pane_id: value.pane_id?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSendInputParams> for PaneSendInputParams {
        fn from(value: super::PaneSendInputParams) -> Self {
            Self {
                keys: Ok(value.keys),
                pane_id: Ok(value.pane_id),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSendKeysParams {
        keys: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneSendKeysParams {
        fn default() -> Self {
            Self {
                keys: Err("no value supplied for keys".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneSendKeysParams {
        pub fn keys<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.keys = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keys: {e}"));
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
    impl ::std::convert::TryFrom<PaneSendKeysParams> for super::PaneSendKeysParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSendKeysParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keys: value.keys?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSendKeysParams> for PaneSendKeysParams {
        fn from(value: super::PaneSendKeysParams) -> Self {
            Self {
                keys: Ok(value.keys),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSendTextParams {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneSendTextParams {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl PaneSendTextParams {
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
    }
    impl ::std::convert::TryFrom<PaneSendTextParams> for super::PaneSendTextParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSendTextParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSendTextParams> for PaneSendTextParams {
        fn from(value: super::PaneSendTextParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSplitParams {
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        direction: ::std::result::Result<super::SplitDirection, ::std::string::String>,
        env: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        ratio: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        target_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneSplitParams {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                direction: Err("no value supplied for direction".to_string()),
                env: Ok(Default::default()),
                focus: Ok(Default::default()),
                ratio: Ok(Default::default()),
                target_pane_id: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl PaneSplitParams {
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
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {e}"));
            self
        }
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
            self
        }
        pub fn ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ratio: {e}"));
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
    }
    impl ::std::convert::TryFrom<PaneSplitParams> for super::PaneSplitParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSplitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                direction: value.direction?,
                env: value.env?,
                focus: value.focus?,
                ratio: value.ratio?,
                target_pane_id: value.target_pane_id?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSplitParams> for PaneSplitParams {
        fn from(value: super::PaneSplitParams) -> Self {
            Self {
                cwd: Ok(value.cwd),
                direction: Ok(value.direction),
                env: Ok(value.env),
                focus: Ok(value.focus),
                ratio: Ok(value.ratio),
                target_pane_id: Ok(value.target_pane_id),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneSwapParams {
        direction: ::std::result::Result<
            ::std::option::Option<super::PaneDirection>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        target_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneSwapParams {
        fn default() -> Self {
            Self {
                direction: Ok(Default::default()),
                pane_id: Ok(Default::default()),
                source_pane_id: Ok(Default::default()),
                target_pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneSwapParams {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PaneDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
        pub fn source_pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    impl ::std::convert::TryFrom<PaneSwapParams> for super::PaneSwapParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneSwapParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                pane_id: value.pane_id?,
                source_pane_id: value.source_pane_id?,
                target_pane_id: value.target_pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneSwapParams> for PaneSwapParams {
        fn from(value: super::PaneSwapParams) -> Self {
            Self {
                direction: Ok(value.direction),
                pane_id: Ok(value.pane_id),
                source_pane_id: Ok(value.source_pane_id),
                target_pane_id: Ok(value.target_pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneTarget {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneTarget {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PaneTarget {
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
    impl ::std::convert::TryFrom<PaneTarget> for super::PaneTarget {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneTarget,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneTarget> for PaneTarget {
        fn from(value: super::PaneTarget) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneWaitForOutputParams {
        lines: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        match_: ::std::result::Result<super::OutputMatch, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::ReadSource, ::std::string::String>,
        strip_ansi: ::std::result::Result<bool, ::std::string::String>,
        timeout_ms: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
    }
    impl ::std::default::Default for PaneWaitForOutputParams {
        fn default() -> Self {
            Self {
                lines: Ok(Default::default()),
                match_: Err("no value supplied for match_".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                source: Err("no value supplied for source".to_string()),
                strip_ansi: Ok(super::defaults::default_bool::<true>()),
                timeout_ms: Ok(Default::default()),
            }
        }
    }
    impl PaneWaitForOutputParams {
        pub fn lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lines: {e}"));
            self
        }
        pub fn match_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OutputMatch>,
            T::Error: ::std::fmt::Display,
        {
            self.match_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_: {e}"));
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
        pub fn strip_ansi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.strip_ansi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strip_ansi: {e}"));
            self
        }
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneWaitForOutputParams> for super::PaneWaitForOutputParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneWaitForOutputParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                lines: value.lines?,
                match_: value.match_?,
                pane_id: value.pane_id?,
                source: value.source?,
                strip_ansi: value.strip_ansi?,
                timeout_ms: value.timeout_ms?,
            })
        }
    }
    impl ::std::convert::From<super::PaneWaitForOutputParams> for PaneWaitForOutputParams {
        fn from(value: super::PaneWaitForOutputParams) -> Self {
            Self {
                lines: Ok(value.lines),
                match_: Ok(value.match_),
                pane_id: Ok(value.pane_id),
                source: Ok(value.source),
                strip_ansi: Ok(value.strip_ansi),
                timeout_ms: Ok(value.timeout_ms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneZoomParams {
        mode: ::std::result::Result<super::PaneZoomMode, ::std::string::String>,
        pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaneZoomParams {
        fn default() -> Self {
            Self {
                mode: Ok(super::defaults::pane_zoom_params_mode()),
                pane_id: Ok(Default::default()),
            }
        }
    }
    impl PaneZoomParams {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneZoomMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn pane_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneZoomParams> for super::PaneZoomParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneZoomParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneZoomParams> for PaneZoomParams {
        fn from(value: super::PaneZoomParams) -> Self {
            Self {
                mode: Ok(value.mode),
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginActionInvokeParams {
        action_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        context: ::std::result::Result<
            ::std::option::Option<super::PluginInvocationContext>,
            ::std::string::String,
        >,
        plugin_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginActionInvokeParams {
        fn default() -> Self {
            Self {
                action_id: Err("no value supplied for action_id".to_string()),
                context: Ok(Default::default()),
                plugin_id: Ok(Default::default()),
            }
        }
    }
    impl PluginActionInvokeParams {
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
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PluginInvocationContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginActionInvokeParams> for super::PluginActionInvokeParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginActionInvokeParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_id: value.action_id?,
                context: value.context?,
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginActionInvokeParams> for PluginActionInvokeParams {
        fn from(value: super::PluginActionInvokeParams) -> Self {
            Self {
                action_id: Ok(value.action_id),
                context: Ok(value.context),
                plugin_id: Ok(value.plugin_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginActionListParams {
        plugin_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginActionListParams {
        fn default() -> Self {
            Self {
                plugin_id: Ok(Default::default()),
            }
        }
    }
    impl PluginActionListParams {
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginActionListParams> for super::PluginActionListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginActionListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginActionListParams> for PluginActionListParams {
        fn from(value: super::PluginActionListParams) -> Self {
            Self {
                plugin_id: Ok(value.plugin_id),
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
    pub struct PluginLinkParams {
        enabled: ::std::result::Result<bool, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<
            ::std::option::Option<super::PluginSourceInfo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginLinkParams {
        fn default() -> Self {
            Self {
                enabled: Ok(super::defaults::default_bool::<true>()),
                path: Err("no value supplied for path".to_string()),
                source: Ok(Default::default()),
            }
        }
    }
    impl PluginLinkParams {
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
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PluginSourceInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginLinkParams> for super::PluginLinkParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginLinkParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                path: value.path?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::PluginLinkParams> for PluginLinkParams {
        fn from(value: super::PluginLinkParams) -> Self {
            Self {
                enabled: Ok(value.enabled),
                path: Ok(value.path),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginListParams {
        plugin_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginListParams {
        fn default() -> Self {
            Self {
                plugin_id: Ok(Default::default()),
            }
        }
    }
    impl PluginListParams {
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginListParams> for super::PluginListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginListParams> for PluginListParams {
        fn from(value: super::PluginListParams) -> Self {
            Self {
                plugin_id: Ok(value.plugin_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginLogListParams {
        limit: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        plugin_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginLogListParams {
        fn default() -> Self {
            Self {
                limit: Ok(Default::default()),
                plugin_id: Ok(Default::default()),
            }
        }
    }
    impl PluginLogListParams {
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn plugin_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.plugin_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plugin_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PluginLogListParams> for super::PluginLogListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginLogListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                limit: value.limit?,
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginLogListParams> for PluginLogListParams {
        fn from(value: super::PluginLogListParams) -> Self {
            Self {
                limit: Ok(value.limit),
                plugin_id: Ok(value.plugin_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginPaneCloseParams {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginPaneCloseParams {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PluginPaneCloseParams {
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
    impl ::std::convert::TryFrom<PluginPaneCloseParams> for super::PluginPaneCloseParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginPaneCloseParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginPaneCloseParams> for PluginPaneCloseParams {
        fn from(value: super::PluginPaneCloseParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginPaneFocusParams {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginPaneFocusParams {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
            }
        }
    }
    impl PluginPaneFocusParams {
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
    impl ::std::convert::TryFrom<PluginPaneFocusParams> for super::PluginPaneFocusParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginPaneFocusParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginPaneFocusParams> for PluginPaneFocusParams {
        fn from(value: super::PluginPaneFocusParams) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginPaneOpenParams {
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        direction: ::std::result::Result<
            ::std::option::Option<super::SplitDirection>,
            ::std::string::String,
        >,
        entrypoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        env: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        height:
            ::std::result::Result<::std::option::Option<super::PopupSize>, ::std::string::String>,
        placement: ::std::result::Result<
            ::std::option::Option<super::PluginPanePlacement>,
            ::std::string::String,
        >,
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        target_pane_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width:
            ::std::result::Result<::std::option::Option<super::PopupSize>, ::std::string::String>,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PluginPaneOpenParams {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                direction: Ok(Default::default()),
                entrypoint: Err("no value supplied for entrypoint".to_string()),
                env: Ok(Default::default()),
                focus: Ok(Default::default()),
                height: Ok(Default::default()),
                placement: Ok(Default::default()),
                plugin_id: Err("no value supplied for plugin_id".to_string()),
                target_pane_id: Ok(Default::default()),
                width: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl PluginPaneOpenParams {
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
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SplitDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
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
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {e}"));
            self
        }
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PluginPanePlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for placement: {e}"));
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
    }
    impl ::std::convert::TryFrom<PluginPaneOpenParams> for super::PluginPaneOpenParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginPaneOpenParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                direction: value.direction?,
                entrypoint: value.entrypoint?,
                env: value.env?,
                focus: value.focus?,
                height: value.height?,
                placement: value.placement?,
                plugin_id: value.plugin_id?,
                target_pane_id: value.target_pane_id?,
                width: value.width?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginPaneOpenParams> for PluginPaneOpenParams {
        fn from(value: super::PluginPaneOpenParams) -> Self {
            Self {
                cwd: Ok(value.cwd),
                direction: Ok(value.direction),
                entrypoint: Ok(value.entrypoint),
                env: Ok(value.env),
                focus: Ok(value.focus),
                height: Ok(value.height),
                placement: Ok(value.placement),
                plugin_id: Ok(value.plugin_id),
                target_pane_id: Ok(value.target_pane_id),
                width: Ok(value.width),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PluginSetEnabledParams {
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginSetEnabledParams {
        fn default() -> Self {
            Self {
                plugin_id: Err("no value supplied for plugin_id".to_string()),
            }
        }
    }
    impl PluginSetEnabledParams {
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
    impl ::std::convert::TryFrom<PluginSetEnabledParams> for super::PluginSetEnabledParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginSetEnabledParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginSetEnabledParams> for PluginSetEnabledParams {
        fn from(value: super::PluginSetEnabledParams) -> Self {
            Self {
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
    pub struct PluginUnlinkParams {
        plugin_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PluginUnlinkParams {
        fn default() -> Self {
            Self {
                plugin_id: Err("no value supplied for plugin_id".to_string()),
            }
        }
    }
    impl PluginUnlinkParams {
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
    impl ::std::convert::TryFrom<PluginUnlinkParams> for super::PluginUnlinkParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PluginUnlinkParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                plugin_id: value.plugin_id?,
            })
        }
    }
    impl ::std::convert::From<super::PluginUnlinkParams> for PluginUnlinkParams {
        fn from(value: super::PluginUnlinkParams) -> Self {
            Self {
                plugin_id: Ok(value.plugin_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerLiveHandoffParams {
        expected_protocol: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
        expected_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        import_exe: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ServerLiveHandoffParams {
        fn default() -> Self {
            Self {
                expected_protocol: Ok(Default::default()),
                expected_version: Ok(Default::default()),
                import_exe: Ok(Default::default()),
            }
        }
    }
    impl ServerLiveHandoffParams {
        pub fn expected_protocol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u32>>,
            T::Error: ::std::fmt::Display,
        {
            self.expected_protocol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expected_protocol: {e}"));
            self
        }
        pub fn expected_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.expected_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expected_version: {e}"));
            self
        }
        pub fn import_exe<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.import_exe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for import_exe: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ServerLiveHandoffParams> for super::ServerLiveHandoffParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServerLiveHandoffParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expected_protocol: value.expected_protocol?,
                expected_version: value.expected_version?,
                import_exe: value.import_exe?,
            })
        }
    }
    impl ::std::convert::From<super::ServerLiveHandoffParams> for ServerLiveHandoffParams {
        fn from(value: super::ServerLiveHandoffParams) -> Self {
            Self {
                expected_protocol: Ok(value.expected_protocol),
                expected_version: Ok(value.expected_version),
                import_exe: Ok(value.import_exe),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabCreateParams {
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        env: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TabCreateParams {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                env: Ok(Default::default()),
                focus: Ok(Default::default()),
                label: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl TabCreateParams {
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
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {e}"));
            self
        }
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
    }
    impl ::std::convert::TryFrom<TabCreateParams> for super::TabCreateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TabCreateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                env: value.env?,
                focus: value.focus?,
                label: value.label?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabCreateParams> for TabCreateParams {
        fn from(value: super::TabCreateParams) -> Self {
            Self {
                cwd: Ok(value.cwd),
                env: Ok(value.env),
                focus: Ok(value.focus),
                label: Ok(value.label),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabListParams {
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TabListParams {
        fn default() -> Self {
            Self {
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl TabListParams {
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
    }
    impl ::std::convert::TryFrom<TabListParams> for super::TabListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TabListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabListParams> for TabListParams {
        fn from(value: super::TabListParams) -> Self {
            Self {
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabMoveParams {
        insert_index: ::std::result::Result<u32, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TabMoveParams {
        fn default() -> Self {
            Self {
                insert_index: Err("no value supplied for insert_index".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
            }
        }
    }
    impl TabMoveParams {
        pub fn insert_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.insert_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for insert_index: {e}"));
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
    }
    impl ::std::convert::TryFrom<TabMoveParams> for super::TabMoveParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TabMoveParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                insert_index: value.insert_index?,
                tab_id: value.tab_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabMoveParams> for TabMoveParams {
        fn from(value: super::TabMoveParams) -> Self {
            Self {
                insert_index: Ok(value.insert_index),
                tab_id: Ok(value.tab_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabRenameParams {
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TabRenameParams {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                tab_id: Err("no value supplied for tab_id".to_string()),
            }
        }
    }
    impl TabRenameParams {
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
    }
    impl ::std::convert::TryFrom<TabRenameParams> for super::TabRenameParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TabRenameParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                tab_id: value.tab_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabRenameParams> for TabRenameParams {
        fn from(value: super::TabRenameParams) -> Self {
            Self {
                label: Ok(value.label),
                tab_id: Ok(value.tab_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TabTarget {
        tab_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TabTarget {
        fn default() -> Self {
            Self {
                tab_id: Err("no value supplied for tab_id".to_string()),
            }
        }
    }
    impl TabTarget {
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
    }
    impl ::std::convert::TryFrom<TabTarget> for super::TabTarget {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TabTarget,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tab_id: value.tab_id?,
            })
        }
    }
    impl ::std::convert::From<super::TabTarget> for TabTarget {
        fn from(value: super::TabTarget) -> Self {
            Self {
                tab_id: Ok(value.tab_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceCreateParams {
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        env: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorkspaceCreateParams {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                env: Ok(Default::default()),
                focus: Ok(Default::default()),
                label: Ok(Default::default()),
            }
        }
    }
    impl WorkspaceCreateParams {
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
        pub fn env<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.env = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for env: {e}"));
            self
        }
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
    }
    impl ::std::convert::TryFrom<WorkspaceCreateParams> for super::WorkspaceCreateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceCreateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                env: value.env?,
                focus: value.focus?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceCreateParams> for WorkspaceCreateParams {
        fn from(value: super::WorkspaceCreateParams) -> Self {
            Self {
                cwd: Ok(value.cwd),
                env: Ok(value.env),
                focus: Ok(value.focus),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceMoveParams {
        insert_index: ::std::result::Result<u32, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceMoveParams {
        fn default() -> Self {
            Self {
                insert_index: Err("no value supplied for insert_index".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl WorkspaceMoveParams {
        pub fn insert_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u32>,
            T::Error: ::std::fmt::Display,
        {
            self.insert_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for insert_index: {e}"));
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
    impl ::std::convert::TryFrom<WorkspaceMoveParams> for super::WorkspaceMoveParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceMoveParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                insert_index: value.insert_index?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceMoveParams> for WorkspaceMoveParams {
        fn from(value: super::WorkspaceMoveParams) -> Self {
            Self {
                insert_index: Ok(value.insert_index),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceRenameParams {
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceRenameParams {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl WorkspaceRenameParams {
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
    impl ::std::convert::TryFrom<WorkspaceRenameParams> for super::WorkspaceRenameParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceRenameParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceRenameParams> for WorkspaceRenameParams {
        fn from(value: super::WorkspaceRenameParams) -> Self {
            Self {
                label: Ok(value.label),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceReportMetadataParams {
        seq: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        tokens: ::std::result::Result<
            ::std::collections::HashMap<
                super::WorkspaceReportMetadataParamsTokensKey,
                ::std::option::Option<::std::string::String>,
            >,
            ::std::string::String,
        >,
        ttl_ms: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceReportMetadataParams {
        fn default() -> Self {
            Self {
                seq: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                tokens: Err("no value supplied for tokens".to_string()),
                ttl_ms: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl WorkspaceReportMetadataParams {
        pub fn seq<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seq = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seq: {e}"));
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
        pub fn tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::WorkspaceReportMetadataParamsTokensKey,
                    ::std::option::Option<::std::string::String>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tokens: {e}"));
            self
        }
        pub fn ttl_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl_ms: {e}"));
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
    impl ::std::convert::TryFrom<WorkspaceReportMetadataParams>
        for super::WorkspaceReportMetadataParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceReportMetadataParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                seq: value.seq?,
                source: value.source?,
                tokens: value.tokens?,
                ttl_ms: value.ttl_ms?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceReportMetadataParams> for WorkspaceReportMetadataParams {
        fn from(value: super::WorkspaceReportMetadataParams) -> Self {
            Self {
                seq: Ok(value.seq),
                source: Ok(value.source),
                tokens: Ok(value.tokens),
                ttl_ms: Ok(value.ttl_ms),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkspaceTarget {
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorkspaceTarget {
        fn default() -> Self {
            Self {
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl WorkspaceTarget {
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
    impl ::std::convert::TryFrom<WorkspaceTarget> for super::WorkspaceTarget {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkspaceTarget,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorkspaceTarget> for WorkspaceTarget {
        fn from(value: super::WorkspaceTarget) -> Self {
            Self {
                workspace_id: Ok(value.workspace_id),
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
    pub struct WorktreeCreateParams {
        base: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        branch: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorktreeCreateParams {
        fn default() -> Self {
            Self {
                base: Ok(Default::default()),
                branch: Ok(Default::default()),
                cwd: Ok(Default::default()),
                focus: Ok(Default::default()),
                label: Ok(Default::default()),
                path: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl WorktreeCreateParams {
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {e}"));
            self
        }
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
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
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
    }
    impl ::std::convert::TryFrom<WorktreeCreateParams> for super::WorktreeCreateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeCreateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base: value.base?,
                branch: value.branch?,
                cwd: value.cwd?,
                focus: value.focus?,
                label: value.label?,
                path: value.path?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeCreateParams> for WorktreeCreateParams {
        fn from(value: super::WorktreeCreateParams) -> Self {
            Self {
                base: Ok(value.base),
                branch: Ok(value.branch),
                cwd: Ok(value.cwd),
                focus: Ok(value.focus),
                label: Ok(value.label),
                path: Ok(value.path),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorktreeListParams {
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorktreeListParams {
        fn default() -> Self {
            Self {
                cwd: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl WorktreeListParams {
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
    }
    impl ::std::convert::TryFrom<WorktreeListParams> for super::WorktreeListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cwd: value.cwd?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeListParams> for WorktreeListParams {
        fn from(value: super::WorktreeListParams) -> Self {
            Self {
                cwd: Ok(value.cwd),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorktreeOpenParams {
        branch: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cwd: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        focus: ::std::result::Result<bool, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorktreeOpenParams {
        fn default() -> Self {
            Self {
                branch: Ok(Default::default()),
                cwd: Ok(Default::default()),
                focus: Ok(Default::default()),
                label: Ok(Default::default()),
                path: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl WorktreeOpenParams {
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
        pub fn focus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.focus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for focus: {e}"));
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
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
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
    }
    impl ::std::convert::TryFrom<WorktreeOpenParams> for super::WorktreeOpenParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeOpenParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branch: value.branch?,
                cwd: value.cwd?,
                focus: value.focus?,
                label: value.label?,
                path: value.path?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeOpenParams> for WorktreeOpenParams {
        fn from(value: super::WorktreeOpenParams) -> Self {
            Self {
                branch: Ok(value.branch),
                cwd: Ok(value.cwd),
                focus: Ok(value.focus),
                label: Ok(value.label),
                path: Ok(value.path),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorktreeRemoveParams {
        force: ::std::result::Result<bool, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WorktreeRemoveParams {
        fn default() -> Self {
            Self {
                force: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl WorktreeRemoveParams {
        pub fn force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.force = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for force: {e}"));
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
    impl ::std::convert::TryFrom<WorktreeRemoveParams> for super::WorktreeRemoveParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorktreeRemoveParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                force: value.force?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::WorktreeRemoveParams> for WorktreeRemoveParams {
        fn from(value: super::WorktreeRemoveParams) -> Self {
            Self {
                force: Ok(value.force),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn agent_read_params_format() -> super::ReadFormat {
        super::ReadFormat::Text
    }
    pub(super) fn agent_view_sort_order() -> super::AgentViewSortOrder {
        super::AgentViewSortOrder::Asc
    }
    pub(super) fn pane_graphics_set_params_placement() -> super::PaneGraphicsPlacementParams {
        super::PaneGraphicsPlacementParams {
            grid_cols: 0_u32,
            grid_rows: 0_u32,
            viewport_col: 0_i32,
            viewport_row: 0_i32,
        }
    }
    pub(super) fn pane_read_params_format() -> super::ReadFormat {
        super::ReadFormat::Text
    }
    pub(super) fn pane_zoom_params_mode() -> super::PaneZoomMode {
        super::PaneZoomMode::Toggle
    }
    pub(super) fn plugin_source_info_kind() -> super::PluginSourceKind {
        super::PluginSourceKind::Local
    }
}
