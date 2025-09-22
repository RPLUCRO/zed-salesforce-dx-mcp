use zed_extension_api::{self as zed, ContextServerId};

struct SalesforceMcp;

impl zed::Extension for SalesforceMcp {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command, String> {
        // We only registered one server: "salesforce"
        if context_server_id.as_ref() != "salesforce" {
            return Err("unknown context server id".into());
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
