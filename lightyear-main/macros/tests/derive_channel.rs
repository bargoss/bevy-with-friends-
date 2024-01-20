pub mod some_channel {
    use lightyear_macros::Channel;

    #[derive(Channel)]
    pub struct SomeChannel;
}

#[cfg(test)]
mod tests {
    use lightyear::prelude::{
        Channel, ChannelContainer, ChannelDirection, ChannelMode, ChannelSettings,
    };

    use super::some_channel::*;

    #[test]
    fn test_channel_derive() {
        let settings = ChannelSettings {
            mode: ChannelMode::UnorderedUnreliable,
            direction: ChannelDirection::Bidirectional,
        };
        let builder = SomeChannel::get_builder(settings);
        let channel_container: ChannelContainer = builder.build();
        assert_eq!(
            channel_container.setting.mode,
            ChannelMode::UnorderedUnreliable
        );
    }
}
