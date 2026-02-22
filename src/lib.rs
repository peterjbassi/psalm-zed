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
        let root = worktree.root_path();
        let vendor_ls = format!("{root}/vendor/bin/psalm-language-server");
        let vendor_psalm = format!("{root}/vendor/bin/psalm");

        // Use sh -c with test -x to check executables, since read_text_file
        // cannot read binary/executable files from the Wasm sandbox.
        // The shell handles existence checks and launches the correct binary.
        let shell_cmd = format!(
            "if test -x '{vendor_ls}'; then exec '{vendor_ls}' --no-progress; \
             elif test -x '{vendor_psalm}'; then exec '{vendor_psalm}' --language-server --no-progress; \
             else echo 'psalm-zed: could not find psalm in {root}/vendor/bin/' >&2; exit 1; fi"
        );

        Ok(zed::Command {
            command: "sh".to_string(),
            args: vec!["-c".to_string(), shell_cmd],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(PsalmExtension);
