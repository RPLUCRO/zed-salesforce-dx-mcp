use zed_extension_api::{self as zed, LanguageServerId};

struct SalesforceMcp;

impl zed::Extension for SalesforceMcp {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // We only registered one server: "salesforce"
        if language_server_id.to_string() != "salesforce" {
            return Err("unknown language server id".into());
        }

        // Build: npx -y @salesforce/mcp --orgs DEFAULT_TARGET_ORG --toolsets all
        Ok(zed::Command {
            command: "npx".into(),
            args: vec![
                "-y".into(),
                "@salesforce/mcp".into(),
                "--orgs".into(),
                "DEFAULT_TARGET_ORG".into(), // change if you want aliases, see below
                "--toolsets".into(),
                "all".into(),
            ],
            env: std::env::vars().collect(), // inherit PATH, SFDX config, etc.
        })
    }
}

zed::register_extension!(SalesforceMcp);
