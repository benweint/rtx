use std::collections::HashMap;
use std::default::Default;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use color_eyre::eyre::Result;

use crate::config::config_file::{ConfigFile, ConfigFileType};
use crate::config::settings::SettingsBuilder;
use crate::config::{AliasMap, Settings};
use crate::plugins::PluginName;
use crate::tool::Tool;
use crate::toolset::{ToolSource, ToolVersionRequest, Toolset};

#[derive(Debug)]
pub struct LegacyVersionFile {
    path: PathBuf,
    toolset: Toolset,
}

impl LegacyVersionFile {
    pub fn parse(settings: &Settings, path: PathBuf, plugins: &[&Arc<Tool>]) -> Result<Self> {
        let mut toolset = Toolset::new(ToolSource::LegacyVersionFile(path.clone()));

        for plugin in plugins {
            let version = plugin.parse_legacy_file(&path, settings)?;
            for version in version.split_whitespace() {
                toolset.add_version(
                    ToolVersionRequest::new(plugin.name.to_string(), version),
                    Default::default(),
                );
            }
        }

        Ok(Self { toolset, path })
    }
}

impl ConfigFile for LegacyVersionFile {
    fn get_type(&self) -> ConfigFileType {
        ConfigFileType::LegacyVersion
    }

    fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    fn plugins(&self) -> HashMap<PluginName, String> {
        Default::default()
    }

    fn env(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn path_dirs(&self) -> Vec<PathBuf> {
        vec![]
    }

    fn remove_plugin(&mut self, _plugin_name: &PluginName) {
        unimplemented!()
    }

    fn replace_versions(&mut self, _plugin_name: &PluginName, _versions: &[String]) {
        unimplemented!()
    }

    fn save(&self) -> Result<()> {
        unimplemented!()
    }

    fn dump(&self) -> String {
        unimplemented!()
    }

    fn to_toolset(&self) -> &Toolset {
        &self.toolset
    }

    fn settings(&self) -> SettingsBuilder {
        SettingsBuilder::default()
    }

    fn aliases(&self) -> AliasMap {
        AliasMap::default()
    }

    fn watch_files(&self) -> Vec<PathBuf> {
        vec![self.path.clone()]
    }
}

impl Display for LegacyVersionFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LegacyVersionFile({})", self.path.display())
    }
}
