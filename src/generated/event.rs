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
#[doc = "  \"title\": \"EventEnvelope\","]
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
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
}
