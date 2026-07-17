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
#[doc = "`PaneAgentStatusChangedEvent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agent_status\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"agent_status\": {"]
#[doc = "      \"$ref\": \"#/$defs/AgentStatus\""]
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
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneAgentStatusChangedEvent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<::std::string::String>,
    pub agent_status: AgentStatus,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display_agent: ::std::option::Option<::std::string::String>,
    pub pane_id: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub state_labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub workspace_id: ::std::string::String,
}
impl PaneAgentStatusChangedEvent {
    pub fn builder() -> builder::PaneAgentStatusChangedEvent {
        Default::default()
    }
}
#[doc = "`PaneOutputMatchedEvent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"matched_line\","]
#[doc = "    \"pane_id\","]
#[doc = "    \"read\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"matched_line\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"read\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneReadResult\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneOutputMatchedEvent {
    pub matched_line: ::std::string::String,
    pub pane_id: ::std::string::String,
    pub read: PaneReadResult,
}
impl PaneOutputMatchedEvent {
    pub fn builder() -> builder::PaneOutputMatchedEvent {
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
#[doc = "`PaneScrollChangedEvent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pane_id\","]
#[doc = "    \"scroll\","]
#[doc = "    \"workspace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pane_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scroll\": {"]
#[doc = "      \"$ref\": \"#/$defs/PaneScrollInfo\""]
#[doc = "    },"]
#[doc = "    \"workspace_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PaneScrollChangedEvent {
    pub pane_id: ::std::string::String,
    pub scroll: PaneScrollInfo,
    pub workspace_id: ::std::string::String,
}
impl PaneScrollChangedEvent {
    pub fn builder() -> builder::PaneScrollChangedEvent {
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
#[doc = "`SubscriptionEventData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/PaneOutputMatchedEvent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/PaneAgentStatusChangedEvent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/PaneScrollChangedEvent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SubscriptionEventData {
    OutputMatchedEvent(PaneOutputMatchedEvent),
    AgentStatusChangedEvent(PaneAgentStatusChangedEvent),
    ScrollChangedEvent(PaneScrollChangedEvent),
}
impl ::std::convert::From<PaneOutputMatchedEvent> for SubscriptionEventData {
    fn from(value: PaneOutputMatchedEvent) -> Self {
        Self::OutputMatchedEvent(value)
    }
}
impl ::std::convert::From<PaneAgentStatusChangedEvent> for SubscriptionEventData {
    fn from(value: PaneAgentStatusChangedEvent) -> Self {
        Self::AgentStatusChangedEvent(value)
    }
}
impl ::std::convert::From<PaneScrollChangedEvent> for SubscriptionEventData {
    fn from(value: PaneScrollChangedEvent) -> Self {
        Self::ScrollChangedEvent(value)
    }
}
#[doc = "`SubscriptionEventEnvelope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SubscriptionEventEnvelope\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"event\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"$ref\": \"#/$defs/SubscriptionEventData\""]
#[doc = "    },"]
#[doc = "    \"event\": {"]
#[doc = "      \"$ref\": \"#/$defs/SubscriptionEventKind\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SubscriptionEventEnvelope {
    pub data: SubscriptionEventData,
    pub event: SubscriptionEventKind,
}
impl SubscriptionEventEnvelope {
    pub fn builder() -> builder::SubscriptionEventEnvelope {
        Default::default()
    }
}
#[doc = "`SubscriptionEventKind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pane.output_matched\","]
#[doc = "    \"pane.agent_status_changed\","]
#[doc = "    \"pane.scroll_changed\""]
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
pub enum SubscriptionEventKind {
    #[serde(rename = "pane.output_matched")]
    PaneOutputMatched,
    #[serde(rename = "pane.agent_status_changed")]
    PaneAgentStatusChanged,
    #[serde(rename = "pane.scroll_changed")]
    PaneScrollChanged,
}
impl ::std::fmt::Display for SubscriptionEventKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PaneOutputMatched => f.write_str("pane.output_matched"),
            Self::PaneAgentStatusChanged => f.write_str("pane.agent_status_changed"),
            Self::PaneScrollChanged => f.write_str("pane.scroll_changed"),
        }
    }
}
impl ::std::str::FromStr for SubscriptionEventKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pane.output_matched" => Ok(Self::PaneOutputMatched),
            "pane.agent_status_changed" => Ok(Self::PaneAgentStatusChanged),
            "pane.scroll_changed" => Ok(Self::PaneScrollChanged),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubscriptionEventKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SubscriptionEventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubscriptionEventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct PaneAgentStatusChangedEvent {
        agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_status: ::std::result::Result<super::AgentStatus, ::std::string::String>,
        display_agent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        state_labels: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneAgentStatusChangedEvent {
        fn default() -> Self {
            Self {
                agent: Ok(Default::default()),
                agent_status: Err("no value supplied for agent_status".to_string()),
                display_agent: Ok(Default::default()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                state_labels: Ok(Default::default()),
                title: Ok(Default::default()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl PaneAgentStatusChangedEvent {
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
    impl ::std::convert::TryFrom<PaneAgentStatusChangedEvent> for super::PaneAgentStatusChangedEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneAgentStatusChangedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                agent_status: value.agent_status?,
                display_agent: value.display_agent?,
                pane_id: value.pane_id?,
                state_labels: value.state_labels?,
                title: value.title?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneAgentStatusChangedEvent> for PaneAgentStatusChangedEvent {
        fn from(value: super::PaneAgentStatusChangedEvent) -> Self {
            Self {
                agent: Ok(value.agent),
                agent_status: Ok(value.agent_status),
                display_agent: Ok(value.display_agent),
                pane_id: Ok(value.pane_id),
                state_labels: Ok(value.state_labels),
                title: Ok(value.title),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaneOutputMatchedEvent {
        matched_line: ::std::result::Result<::std::string::String, ::std::string::String>,
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        read: ::std::result::Result<super::PaneReadResult, ::std::string::String>,
    }
    impl ::std::default::Default for PaneOutputMatchedEvent {
        fn default() -> Self {
            Self {
                matched_line: Err("no value supplied for matched_line".to_string()),
                pane_id: Err("no value supplied for pane_id".to_string()),
                read: Err("no value supplied for read".to_string()),
            }
        }
    }
    impl PaneOutputMatchedEvent {
        pub fn matched_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.matched_line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for matched_line: {e}"));
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
        pub fn read<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneReadResult>,
            T::Error: ::std::fmt::Display,
        {
            self.read = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for read: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaneOutputMatchedEvent> for super::PaneOutputMatchedEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneOutputMatchedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                matched_line: value.matched_line?,
                pane_id: value.pane_id?,
                read: value.read?,
            })
        }
    }
    impl ::std::convert::From<super::PaneOutputMatchedEvent> for PaneOutputMatchedEvent {
        fn from(value: super::PaneOutputMatchedEvent) -> Self {
            Self {
                matched_line: Ok(value.matched_line),
                pane_id: Ok(value.pane_id),
                read: Ok(value.read),
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
    pub struct PaneScrollChangedEvent {
        pane_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        scroll: ::std::result::Result<super::PaneScrollInfo, ::std::string::String>,
        workspace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PaneScrollChangedEvent {
        fn default() -> Self {
            Self {
                pane_id: Err("no value supplied for pane_id".to_string()),
                scroll: Err("no value supplied for scroll".to_string()),
                workspace_id: Err("no value supplied for workspace_id".to_string()),
            }
        }
    }
    impl PaneScrollChangedEvent {
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
        pub fn scroll<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaneScrollInfo>,
            T::Error: ::std::fmt::Display,
        {
            self.scroll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scroll: {e}"));
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
    impl ::std::convert::TryFrom<PaneScrollChangedEvent> for super::PaneScrollChangedEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaneScrollChangedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pane_id: value.pane_id?,
                scroll: value.scroll?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::PaneScrollChangedEvent> for PaneScrollChangedEvent {
        fn from(value: super::PaneScrollChangedEvent) -> Self {
            Self {
                pane_id: Ok(value.pane_id),
                scroll: Ok(value.scroll),
                workspace_id: Ok(value.workspace_id),
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
    pub struct SubscriptionEventEnvelope {
        data: ::std::result::Result<super::SubscriptionEventData, ::std::string::String>,
        event: ::std::result::Result<super::SubscriptionEventKind, ::std::string::String>,
    }
    impl ::std::default::Default for SubscriptionEventEnvelope {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                event: Err("no value supplied for event".to_string()),
            }
        }
    }
    impl SubscriptionEventEnvelope {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionEventData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionEventKind>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscriptionEventEnvelope> for super::SubscriptionEventEnvelope {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscriptionEventEnvelope,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                event: value.event?,
            })
        }
    }
    impl ::std::convert::From<super::SubscriptionEventEnvelope> for SubscriptionEventEnvelope {
        fn from(value: super::SubscriptionEventEnvelope) -> Self {
            Self {
                data: Ok(value.data),
                event: Ok(value.event),
            }
        }
    }
}
