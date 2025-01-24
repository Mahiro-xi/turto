use crate::{
    config::message_template::get_template,
    models::{autoleave::AutoleaveType, guild::volume::GuildVolume},
    utils::misc::ToEmoji,
};
use serenity::{
    model::prelude::{ChannelId, UserId},
    prelude::Mentionable,
};
use std::fmt::Display;

pub struct TurtoMessage<'a> {
    pub locale: Option<&'a str>,
    pub kind: TurtoMessageKind<'a>,
}

use TurtoMessageKind::*;
pub enum TurtoMessageKind<'a> {
    NotPlaying,
    UserNotInVoiceChannel,
    BotNotInVoiceChannel,
    DifferentVoiceChannel { bot: ChannelId },
    Play { title: &'a str },
    Pause { title: &'a str },
    Skip { title: Option<&'a str> },
    Stop { title: &'a str },
    Join(ChannelId),
    Leave(ChannelId),
    Queue { title: &'a str },
    Remove { title: &'a str },
    RemoveAll,
    InvalidRemove { length: usize },
    InvalidRangeRemove { from: usize, to: usize, length: usize },
    InvalidUrl(Option<&'a str>),
    SetVolume(GuildVolume),
    SetAutoleave(AutoleaveType),
    SeekSuccess,
    InvalidSeek { seek_limit: u64 },
    SeekNotAllow { backward: bool },
    SeekNotLongEnough { title: &'a str, length: u64 },
    AdministratorOnly,
    Ban { success: bool, user: UserId },
    Unban { success: bool, user: UserId },
    BannedUserResponse,
    Shuffle,
    SetRepeat(bool),
    EmptyPlaylist,
    InvalidPlaylistPage { total_pages: usize },
    RemoveMany { removed_number: usize },
}

macro_rules! render {
    ($f:expr, $template:expr, $locale:expr $(, ($key:expr, $value:expr))* $(,)?) => {{
        $f.write_str(&get_template($template, $locale).renderer()
        $(
            .add_arg($key, $value)
        )*
        .render())
    }};
}

impl Display for TurtoMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = self.locale;
        match &self.kind {
            NotPlaying => render!(f, "not_playing", locale),
            UserNotInVoiceChannel => render!(f, "user_not_in_voice_channel", locale),
            BotNotInVoiceChannel => render!(f, "bot_not_in_voice_channel", locale),
            DifferentVoiceChannel { bot } => render!(
                f,
                "different_voice_channel",
                locale,
                ("bot_voice_channel", &bot.mention())
            ),
            Play { title } => render!(f, "play", locale, ("title", title)),
            Pause { title } => render!(f, "pause", locale, ("title", title)),
            Stop { title } => render!(f, "stop", locale, ("title", title)),
            Skip { title } => match title {
                Some(title) => render!(f, "skip", locale, ("title", title)),
                None => render!(f, "skip_success", locale),
            },
            Join(channel) => render!(f, "join", locale, ("voice_channel", &channel.mention())),
            Leave(channel) => render!(f, "leave", locale, ("voice_channel", &channel.mention())),
            Queue { title } => render!(f, "queue", locale, ("title", title)),
            Remove { title } => render!(f, "remove", locale, ("title", title)),
            RemoveAll => render!(f, "remove_all", locale),
            InvalidRemove { length } => {
                render!(
                    f,
                    "invalid_remove_index",
                    locale,
                    ("playlist_length", length)
                )
            }
            InvalidRangeRemove { from, to, length } => render!(
                f,
                "invalid_remove_range",
                locale,
                ("from", from),
                ("to", to),
                ("playlist_length", length)
            ),
            InvalidUrl(url) => match url {
                Some(url) => render!(f, "url_not_found", locale, ("url", url)),
                None => render!(f, "invalid_url", locale),
            },
            SetVolume(val) => render!(f, "volume", locale, ("volume", &val.to_emoji())),
            SetAutoleave(res) => match res {
                AutoleaveType::On => {
                    render!(f, "toggle_autoleave", locale, ("autoleave_status", &"on"))
                }
                AutoleaveType::Empty => render!(
                    f,
                    "toggle_autoleave",
                    locale,
                    ("autoleave_status", &"empty")
                ),
                AutoleaveType::Silent => render!(
                    f,
                    "toggle_autoleave",
                    locale,
                    ("autoleave_status", &"slient")
                ),
                AutoleaveType::Off => {
                    render!(f, "toggle_autoleave", locale, ("autoleave_status", &"off"))
                }
            },
            SeekSuccess => render!(f, "seek_success", locale),
            InvalidSeek { seek_limit } => {
                render!(f, "invalid_seek", locale, ("seek_limit", seek_limit))
            }
            SeekNotAllow { backward } => match *backward {
                true => render!(f, "backward_seek_not_allow", locale),
                false => render!(f, "seek_not_allow", locale),
            },
            SeekNotLongEnough { title, length } => render!(
                f,
                "seek_not_long_enough",
                locale,
                ("title", title),
                ("length", length)
            ),
            AdministratorOnly => render!(f, "administrator_only", locale),
            Ban { success, user } => match *success {
                true => render!(f, "user_got_banned", locale, ("user", &user.mention())),
                false => render!(f, "user_already_banned", locale, ("user", &user.mention())),
            },
            Unban { success, user } => match *success {
                true => render!(f, "user_got_unbanned", locale, ("user", &user.mention())),
                false => render!(f, "user_not_banned", locale, ("user", &user.mention())),
            },
            BannedUserResponse => render!(f, "banned_user_repsonse", locale),
            Shuffle => render!(f, "shuffle", locale),
            SetRepeat(repeat) => match repeat {
                true => render!(f, "toggle_repeat", locale, ("repeat_status", &"✅")),
                false => render!(f, "toggle_repeat", locale, ("repeat_status", &"❎")),
            },
            EmptyPlaylist => render!(f, "empty_playlist", locale),
            InvalidPlaylistPage { total_pages } => render!(
                f,
                "invalid_playlist_page",
                locale,
                ("total_pages", &total_pages.to_emoji()),
            ),
            RemoveMany { removed_number } => render!(
                f,
                "remove_many",
                locale,
                ("removed_number", &removed_number.to_emoji())
            ),
        }
    }
}

impl From<TurtoMessage<'_>> for String {
    fn from(value: TurtoMessage) -> Self {
        value.to_string()
    }
}
