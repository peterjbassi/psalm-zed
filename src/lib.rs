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
        let vendor_ls = format!("{}/vendor/bin/psalm-language-server", worktree.root_path());
        let vendor_psalm = format!("{}/vendor/bin/psalm", worktree.root_path());

        // Check project vendor/bin first, then fall back to PATH
        let (command, use_language_server_flag) =
            if worktree.read_text_file("vendor/bin/psalm-language-server").is_ok() {
                (vendor_ls, false)
            } else if worktree.read_text_file("vendor/bin/psalm").is_ok() {
                (vendor_psalm, true)
            } else if let Some(path) = worktree.which("psalm-language-server") {
                (path, false)
            } else if let Some(path) = worktree.which("psalm") {
                (path, true)
            } else {
                return Err("Could not find psalm-language-server or psalm. \
                    Install Psalm with: composer require --dev vimeo/psalm"
                    .to_string());
            };

        let mut args = Vec::new();
        if use_language_server_flag {
            args.push("--language-server".to_string());
        }
        args.push("--no-progress".to_string());

        Ok(zed::Command {
            command,
            args,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(PsalmExtension);
