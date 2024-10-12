use eyre::Result;

use crate::config::Settings;

/// Show current settings
///
/// This is the contents of ~/.config/mise/config.toml
///
/// Note that aliases are also stored in this file
/// but managed separately with `mise aliases`
#[derive(Debug, clap::Args)]
#[clap(visible_alias = "list", after_long_help = AFTER_LONG_HELP, verbatim_doc_comment)]
pub struct SettingsLs {
    /// Only display key names for each setting
    #[clap(long, verbatim_doc_comment)]
    pub keys: bool,
}

impl SettingsLs {
    pub fn run(self) -> Result<()> {
        let settings = Settings::try_get()?;
        let mut settings = settings.as_dict()?;
        for k in Settings::hidden_configs() {
            settings.remove(*k);
        }
        if self.keys {
            return self.print_keys(&settings);
        }
        miseprintln!("{}", settings);
        Ok(())
    }

    fn print_keys(&self, settings: &toml::Table) -> Result<()> {
        for (k, v) in settings {
            miseprintln!("{k}");
            if let toml::Value::Table(t) = v {
                for (subkey, _) in t {
                    miseprintln!("{k}.{subkey}");
                }
            }
        }
        Ok(())
    }
}

static AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>

    $ <bold>mise settings</bold>
    legacy_version_file = false
"#
);

#[cfg(test)]
mod tests {
    use crate::test::reset;

    #[test]
    fn test_settings_ls() {
        reset();
        assert_cli_snapshot!("settings", @r#"
        activate_aggressive = false
        all_compile = false
        always_keep_download = true
        always_keep_install = true
        asdf = true
        asdf_compat = false
        cache_prune_age = "0"
        cargo_binstall = true
        color = true
        disable_default_shorthands = false
        disable_hints = []
        disable_tools = []
        experimental = true
        fetch_remote_versions_cache = "1h"
        fetch_remote_versions_timeout = "10s"
        go_default_packages_file = "~/.default-go-packages"
        go_download_mirror = "https://dl.google.com/go"
        go_repo = "https://github.com/golang/go"
        go_set_gopath = false
        go_set_goroot = true
        go_skip_checksum = false
        http_timeout = "30s"
        jobs = 2
        legacy_version_file = true
        legacy_version_file_disable_tools = []
        libgit2 = true
        not_found_auto_install = true
        paranoid = false
        pin = false
        pipx_uvx = false
        plugin_autoupdate_last_check_duration = "20m"
        python_default_packages_file = "~/.default-python-packages"
        quiet = false
        raw = false
        trusted_config_paths = []
        use_versions_host = true
        verbose = true
        vfox = false
        yes = true

        [node]

        [python]
        default_packages_file = "~/.default-python-packages"
        pyenv_repo = "https://github.com/pyenv/pyenv.git"
        venv_auto_create = false
        venv_stdlib = false

        [ruby]
        default_packages_file = "~/.default-gems"
        ruby_build_repo = "https://github.com/rbenv/ruby-build.git"
        ruby_install = false
        ruby_install_repo = "https://github.com/postmodern/ruby-install.git"

        [status]
        missing_tools = "if_other_versions_installed"
        show_env = false
        show_tools = false
        "#);
    }

    #[test]
    fn test_settings_ls_keys() {
        reset();
        assert_cli_snapshot!("settings", "--keys", @r#"
        activate_aggressive
        all_compile
        always_keep_download
        always_keep_install
        asdf
        asdf_compat
        cache_prune_age
        cargo_binstall
        color
        disable_default_shorthands
        disable_hints
        disable_tools
        experimental
        fetch_remote_versions_cache
        fetch_remote_versions_timeout
        go_default_packages_file
        go_download_mirror
        go_repo
        go_set_gopath
        go_set_goroot
        go_skip_checksum
        http_timeout
        jobs
        legacy_version_file
        legacy_version_file_disable_tools
        libgit2
        node
        not_found_auto_install
        paranoid
        pin
        pipx_uvx
        plugin_autoupdate_last_check_duration
        python
        python.default_packages_file
        python.pyenv_repo
        python.venv_auto_create
        python.venv_stdlib
        python_default_packages_file
        quiet
        raw
        ruby
        ruby.default_packages_file
        ruby.ruby_build_repo
        ruby.ruby_install
        ruby.ruby_install_repo
        status
        status.missing_tools
        status.show_env
        status.show_tools
        trusted_config_paths
        use_versions_host
        verbose
        vfox
        yes
        "#);
    }
}
