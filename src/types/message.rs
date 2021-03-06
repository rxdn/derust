use super::user::User;
use super::guild::GuildMember;
use super::channel::ChannelMention;
use serde::{Serialize, Deserialize};
use super::role::Role;
use crate::types::{Snowflake, Timestamp};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MentionedUsers {
    pub(crate) id: Snowflake,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: String,
    pub(crate) verified: bool,
    pub(crate) email: String,
    pub(crate) flags: i64,
    pub(crate) premium_type: i64,
    pub(crate) public_flags: i64,
    pub(crate) member: GuildMember,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub(crate) id: Snowflake,
    pub(crate) channel_id: Snowflake,
    pub(crate) guild_id: Option<Snowflake>,
    pub(crate) author: User,
    pub(crate) member: GuildMember,
    pub(crate) content: String,
    pub(crate) timestamp: Timestamp,
    pub(crate) edited_timestamp: Option<Timestamp>,
    pub(crate) tts: bool,
    pub(crate) mention_everyone: bool,
    pub(crate) mentions: Vec<MentionedUsers>,
    pub(crate) mention_roles: Vec<String>,
    pub(crate) mention_channels: Option<Vec<ChannelMention>>,
    pub(crate) attachments: Vec<Attachment>,
    pub(crate) embeds: Vec<Embed>,
    pub(crate) reactions: Vec<MessageReaction>,
    pub(crate) nonce: Option<String>,
    pub(crate) pinned: bool,
    pub(crate) webhook_id: Option<Snowflake>,
    pub(crate) r#type: MessageType,
    pub(crate) activity: Option<MessageActivity>,
    pub(crate) application: Option<MessageApplication>,
    pub(crate) message_reference: Option<MessageReference>,
    pub(crate) flags: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageReaction {
    pub(crate) count: i32,
    pub(crate) me: bool,
    pub(crate) emoji: Emoji,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageActivity {
    pub(crate) r#type: MessageActivityType,
    pub(crate) party_id: Option<Snowflake>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Default = 0,
    AddRecipient = 1,
    RemoveRecipient = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserBoost = 8,
    UserBoostTier1 = 9,
    UserBoostTier2 = 10,
    UserBoostTier3 = 11,
    AddChannelFollow = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageApplication {
    pub(crate) id: Snowflake,
    pub(crate) cover_image: Option<String>,
    pub(crate) description: String,
    pub(crate) icon: Option<String>,
    pub(crate) name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct MessageReference {
    pub(crate) message_id: Snowflake,
    pub(crate) channel_id: Snowflake,
    pub(crate) guild_id: Snowflake,
}

pub struct AllowedMentions {
    pub(crate) parse: Vec<String>,
    pub(crate) roles: Vec<Snowflake>,
    pub(crate) users: Vec<Snowflake>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Attachment {
    pub(crate) id: Snowflake,
    pub(crate) filename: String,
    pub(crate) size: i32,
    pub(crate) url: String,
    pub(crate) proxy_url: String,
    pub(crate) height: i32,
    pub(crate) width: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Embed {
    pub(crate) title: Option<String>,
    pub(crate) r#type: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) timestamp: Option<Timestamp>,
    pub(crate) color: Option<i32>,
    pub(crate) footer: Option<Vec<EmbedFooter>>,
    pub(crate) image: Option<EmbedImage>,
    pub(crate) thumbnail: Option<EmbedThumbnail>,
    pub(crate) video: Option<EmbedVideo>,
    pub(crate) provider: Option<EmbedProvider>,
    pub(crate) author: Option<EmbedAuthor>,
    pub(crate) fields: Vec<EmbedField>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedFooter {
    pub(crate) text: String,
    pub(crate) icon_url: Option<String>,
    pub(crate) proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedImage {
    pub(crate) url: Option<String>,
    pub(crate) proxy_url: Option<String>,
    pub(crate) height: Option<i32>,
    pub(crate) width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedProvider {
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedAuthor {
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) icon_url: Option<String>,
    pub(crate) proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedField {
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) inline: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedThumbnail {
    pub(crate) url: Option<String>,
    pub(crate) proxy_url: Option<String>,
    pub(crate) height: Option<i32>,
    pub(crate) width: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EmbedVideo {
    pub(crate) url: Option<String>,
    pub(crate) height: Option<i32>,
    pub(crate) width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Emoji {
    pub(crate) id: Snowflake,
    pub(crate) name: String,
    pub(crate) roles: Option<Role>,
    pub(crate) user: Option<User>,
    pub(crate) require_colons: Option<bool>,
    pub(crate) managed: Option<bool>,
    pub(crate) animated: Option<bool>,
    pub(crate) available: Option<bool>,
}