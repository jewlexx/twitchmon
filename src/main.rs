use twitchchat::{
    connector::Connector, twitch::Capability::*, AsyncRunner, UserConfig, ANONYMOUS_LOGIN,
};

fn main() {
    let (name, token) = ANONYMOUS_LOGIN;

    let config = UserConfig {
        name: name.into(),
        token: token.into(),
        capabilities: vec![Membership, Tags, Commands],
    };

    AsyncRunner::connect(, &config);
}
