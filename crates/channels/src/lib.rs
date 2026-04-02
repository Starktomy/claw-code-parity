pub mod manager;
pub mod traits;
pub mod slack;
pub mod discord;

pub use manager::ChannelManager;
pub use traits::{Channel, ChannelMessage, SendMessage};
pub use slack::SlackChannel;
pub use discord::DiscordChannel;
