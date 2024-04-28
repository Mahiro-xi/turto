use crate::{
    messages::{
        TurtoMessage,
        TurtoMessageKind::{InvalidUrl, Queue},
    },
    models::{
        alias::{Context, Error},
        queue_item::{QueueItem, QueueItemKind},
        url::ParsedUrl,
    },
};
use std::mem::replace;

pub enum QueueType {
    Front,
    Back,
}

pub async fn enqueue(ctx: Context<'_>, query: String, queue_type: QueueType) -> Result<(), Error> {
    let locale = ctx.locale();
    let Ok(parsed) = query.parse::<ParsedUrl>() else {
        ctx.say(TurtoMessage {
            locale,
            kind: InvalidUrl(None),
        })
        .await?;
        return Ok(());
    };
    ctx.defer().await?;

    let queue_item = QueueItem::new(parsed);

    let Some(queue_item_kind) = queue_item.query().await else {
        ctx.say(TurtoMessage {
            locale,
            kind: InvalidUrl(Some(&query)),
        })
        .await?;
        return Ok(());
    };

    let guild_id = ctx.guild_id().unwrap();
    let mut guild_data = ctx.data().guilds.entry(guild_id).or_default();

    let title = match queue_item_kind {
        QueueItemKind::Single(playlist_item) => {
            let title = playlist_item.title.clone();
            match queue_type {
                QueueType::Front => guild_data.playlist.push_front(playlist_item),
                QueueType::Back => guild_data.playlist.push_back(playlist_item),
            }
            drop(guild_data);
            title
        }
        QueueItemKind::Playlist(mut yt_playlist) => {
            let title = yt_playlist.title.take().unwrap_or_default();
            match queue_type {
                QueueType::Front => {
                    let new_playlist = yt_playlist.into_playlist();
                    let tail = replace(&mut guild_data.playlist, new_playlist);
                    guild_data.playlist.extend(tail);
                }
                QueueType::Back => guild_data.playlist.extend(yt_playlist),
            }
            drop(guild_data);
            title
        }
    };

    ctx.say(TurtoMessage {
        locale,
        kind: Queue { title: &title },
    })
    .await?;
    Ok(())
}