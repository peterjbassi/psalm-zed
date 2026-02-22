use zed_extension_api::{self as zed, Result};

struct PsalmExtension;

impl zed::Extension for PsalmExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("psalm-language-server") {
            return Ok(zed::Command {
                command: path,
                args: vec!["--no-progress".to_string()],
                env: worktree.shell_env(),
            });
        }

        if let Some(path) = worktree.which("psalm") {
            return Ok(zed::Command {
                command: path,
                args: vec![
                    "--language-server".to_string(),
                    "--no-progress".to_string(),
                ],
                env: worktree.shell_env(),
            });
        }

        Err("Could not find psalm-language-server or psalm in PATH. \
             Install Psalm with: composer require --dev vimeo/psalm"
            .to_string())
    }
}

zed::register_extension!(PsalmExtension);
