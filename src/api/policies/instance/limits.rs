pub mod limits {
    use std::collections::HashMap;

    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    #[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
    pub enum LimitType {
        AuthRegister,
        AuthLogin,
        AbsoluteMessage,
        AbsoluteRegister,
        Global,
        Ip,
        Channel,
        Error,
        Guild,
        Webhook,
    }

    impl std::fmt::Display for LimitType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                LimitType::AuthRegister => write!(f, "auth_register"),
                LimitType::AuthLogin => write!(f, "auth_login"),
                LimitType::AbsoluteMessage => write!(f, "absolute_message"),
                LimitType::AbsoluteRegister => write!(f, "absolute_register"),
                LimitType::Global => write!(f, "global"),
                LimitType::Ip => write!(f, "ip"),
                LimitType::Channel => write!(f, "channel"),
                LimitType::Error => write!(f, "error"),
                LimitType::Guild => write!(f, "guild"),
                LimitType::Webhook => write!(f, "webhook"),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub struct User {
        pub maxGuilds: u64,
        pub maxUsername: u64,
        pub maxFriends: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Guild {
        pub maxRoles: u64,
        pub maxEmojis: u64,
        pub maxMembers: u64,
        pub maxChannels: u64,
        pub maxChannelsInCategory: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Message {
        pub maxCharacters: u64,
        pub maxTTSCharacters: u64,
        pub maxReactions: u64,
        pub maxAttachmentSize: u64,
        pub maxBulkDelete: u64,
        pub maxEmbedDownloadSize: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Channel {
        pub maxPins: u64,
        pub maxTopic: u64,
        pub maxWebhooks: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Rate {
        pub enabled: bool,
        pub ip: Window,
        pub global: Window,
        pub error: Window,
        pub routes: Routes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Window {
        pub count: u64,
        pub window: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Routes {
        pub guild: Window,
        pub webhook: Window,
        pub channel: Window,
        pub auth: AuthRoutes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AuthRoutes {
        pub login: Window,
        pub register: Window,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AbsoluteRate {
        pub register: AbsoluteWindow,
        pub sendMessage: AbsoluteWindow,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AbsoluteWindow {
        pub limit: u64,
        pub window: u64,
        pub enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Config {
        pub user: User,
        pub guild: Guild,
        pub message: Message,
        pub channel: Channel,
        pub rate: Rate,
        pub absoluteRate: AbsoluteRate,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }

    impl std::fmt::Display for Limit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Bucket: {}, Limit: {}, Remaining: {}, Reset: {}",
                self.bucket, self.limit, self.remaining, self.reset
            )
        }
    }

    impl Limit {
        pub fn add_remaining(&mut self, remaining: i64) {
            if remaining < 0 {
                if ((self.remaining as i64 + remaining) as i64) <= 0 {
                    self.remaining = 0;
                    return;
                }
                self.remaining -= remaining.abs() as u64;
                return;
            }
            self.remaining += remaining.abs() as u64;
        }
    }

    pub struct Limits {
        pub limit_absolute_messages: Limit,
        pub limit_absolute_register: Limit,
        pub limit_auth_login: Limit,
        pub limit_auth_register: Limit,
        pub limit_ip: Limit,
        pub limit_global: Limit,
        pub limit_error: Limit,
        pub limit_guild: Limit,
        pub limit_webhook: Limit,
        pub limit_channel: Limit,
    }

    impl Limits {
        pub fn iter(&self) -> std::vec::IntoIter<Limit> {
            let mut limits: Vec<Limit> = Vec::new();
            limits.push(self.limit_absolute_messages.clone());
            limits.push(self.limit_absolute_register.clone());
            limits.push(self.limit_auth_login.clone());
            limits.push(self.limit_auth_register.clone());
            limits.push(self.limit_ip.clone());
            limits.push(self.limit_global.clone());
            limits.push(self.limit_error.clone());
            limits.push(self.limit_guild.clone());
            limits.push(self.limit_webhook.clone());
            limits.push(self.limit_channel.clone());
            limits.into_iter()
        }

        /// check_limits uses the API to get the current request limits of the instance.
        /// It returns a `Limits` struct containing all the limits.
        pub async fn check_limits(api_url: String) -> HashMap<LimitType, Limit> {
            let client = Client::new();
            let url_parsed = crate::URLBundle::parse_url(api_url) + "/policies/instance/limits";
            let result = client
                .get(url_parsed)
                .send()
                .await
                .unwrap_or_else(|e| panic!("An error occured while performing the request: {}", e))
                .text()
                .await
                .unwrap_or_else(|e| {
                    panic!(
                        "An error occured while parsing the request body string: {}",
                        e
                    )
                });
            let config: Config = from_str(&result).unwrap();
            // If config.rate.enabled is false, then add return a Limits struct with all limits set to u64::MAX
            let mut limits: HashMap<LimitType, Limit> = HashMap::new();
            if config.rate.enabled == false {
                limits.insert(
                    LimitType::AbsoluteMessage,
                    Limit {
                        bucket: LimitType::AbsoluteMessage,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::AbsoluteRegister,
                    Limit {
                        bucket: LimitType::AbsoluteRegister,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::AuthLogin,
                    Limit {
                        bucket: LimitType::AuthLogin,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::AuthRegister,
                    Limit {
                        bucket: LimitType::AuthRegister,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Ip,
                    Limit {
                        bucket: LimitType::Ip,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Global,
                    Limit {
                        bucket: LimitType::Global,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Error,
                    Limit {
                        bucket: LimitType::Error,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Guild,
                    Limit {
                        bucket: LimitType::Guild,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Webhook,
                    Limit {
                        bucket: LimitType::Webhook,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
                limits.insert(
                    LimitType::Channel,
                    Limit {
                        bucket: LimitType::Channel,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
            } else {
                limits.insert(
                    LimitType::AbsoluteMessage,
                    Limit {
                        bucket: LimitType::AbsoluteMessage,
                        limit: config.absoluteRate.sendMessage.limit,
                        remaining: config.absoluteRate.sendMessage.limit,
                        reset: config.absoluteRate.sendMessage.window,
                    },
                );
                limits.insert(
                    LimitType::AbsoluteRegister,
                    Limit {
                        bucket: LimitType::AbsoluteRegister,
                        limit: config.absoluteRate.register.limit,
                        remaining: config.absoluteRate.register.limit,
                        reset: config.absoluteRate.register.window,
                    },
                );
                limits.insert(
                    LimitType::AuthLogin,
                    Limit {
                        bucket: LimitType::AuthLogin,
                        limit: config.rate.routes.auth.login.count,
                        remaining: config.rate.routes.auth.login.count,
                        reset: config.rate.routes.auth.login.window,
                    },
                );
                limits.insert(
                    LimitType::AuthRegister,
                    Limit {
                        bucket: LimitType::AuthRegister,
                        limit: config.rate.routes.auth.register.count,
                        remaining: config.rate.routes.auth.register.count,
                        reset: config.rate.routes.auth.register.window,
                    },
                );
                limits.insert(
                    LimitType::Guild,
                    Limit {
                        bucket: LimitType::Guild,
                        limit: config.rate.routes.guild.count,
                        remaining: config.rate.routes.guild.count,
                        reset: config.rate.routes.guild.window,
                    },
                );
                limits.insert(
                    LimitType::Webhook,
                    Limit {
                        bucket: LimitType::Webhook,
                        limit: config.rate.routes.webhook.count,
                        remaining: config.rate.routes.webhook.count,
                        reset: config.rate.routes.webhook.window,
                    },
                );
                limits.insert(
                    LimitType::Channel,
                    Limit {
                        bucket: LimitType::Channel,
                        limit: config.rate.routes.channel.count,
                        remaining: config.rate.routes.channel.count,
                        reset: config.rate.routes.channel.window,
                    },
                );
                limits.insert(
                    LimitType::Ip,
                    Limit {
                        bucket: LimitType::Ip,
                        limit: config.rate.ip.count,
                        remaining: config.rate.ip.count,
                        reset: config.rate.ip.window,
                    },
                );
                limits.insert(
                    LimitType::Global,
                    Limit {
                        bucket: LimitType::Global,
                        limit: config.rate.global.count,
                        remaining: config.rate.global.count,
                        reset: config.rate.global.window,
                    },
                );
                limits.insert(
                    LimitType::Error,
                    Limit {
                        bucket: LimitType::Error,
                        limit: config.rate.error.count,
                        remaining: config.rate.error.count,
                        reset: config.rate.error.window,
                    },
                );
            }

            if !config.absoluteRate.register.enabled {
                limits.insert(
                    LimitType::AbsoluteRegister,
                    Limit {
                        bucket: LimitType::AbsoluteRegister,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
            }

            if !config.absoluteRate.sendMessage.enabled {
                limits.insert(
                    LimitType::AbsoluteMessage,
                    Limit {
                        bucket: LimitType::AbsoluteMessage,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                );
            }

            return limits;
        }
    }
}

#[cfg(test)]
mod instance_limits {
    use crate::api::limits::{Limit, LimitType};

    #[test]
    fn limit_below_zero() {
        let mut limit = Limit {
            bucket: LimitType::AbsoluteMessage,
            limit: 0,
            remaining: 1,
            reset: 0,
        };
        limit.add_remaining(-2);
        assert_eq!(0 as u64, limit.remaining);
        limit.add_remaining(-2123123);
        assert_eq!(0 as u64, limit.remaining);
    }
}
