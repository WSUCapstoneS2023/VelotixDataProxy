
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 67,
                        patch: 1,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.67.1 (d5a82bbd2 2023-02-07)".to_owned(),
                    commit_hash: Some("d5a82bbd26e1ad8b7401f6a718a9c57c96905483".to_owned()),
                    commit_date: Some("2023-02-07".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            