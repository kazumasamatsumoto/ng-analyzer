pub mod rules;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub ignore: Vec<String>,
    pub output: OutputConfig,
    pub rules: HashMap<String, RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub rules: HashMap<String, RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub enabled: bool,
    pub severity: String,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub formats: Vec<String>,
    pub path: PathBuf,
    pub include_recommendations: bool,
    pub include_metrics: bool,
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        
        profiles.insert("strict".to_string(), Profile {
            name: "Strict".to_string(),
            description: "Strict rules for production-ready code".to_string(),
            rules: create_strict_rules(),
        });
        
        profiles.insert("recommended".to_string(), Profile {
            name: "Recommended".to_string(),
            description: "Balanced rules for most projects".to_string(),
            rules: create_recommended_rules(),
        });
        
        profiles.insert("relaxed".to_string(), Profile {
            name: "Relaxed".to_string(),
            description: "Minimal rules for rapid development".to_string(),
            rules: create_relaxed_rules(),
        });

        Self {
            profiles,
            ignore: vec![
                "**/*.spec.ts".to_string(),
                "**/*.test.ts".to_string(),
                "**/node_modules/**".to_string(),
                "**/dist/**".to_string(),
                "**/.git/**".to_string(),
            ],
            output: OutputConfig {
                formats: vec!["json".to_string()],
                path: PathBuf::from("./reports"),
                include_recommendations: true,
                include_metrics: true,
            },
            rules: create_recommended_rules(),
        }
    }
}

impl Config {
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn create_default_config_file(path: &PathBuf, profile: &str) -> Result<()> {
        let mut config = Config::default();
        
        if let Some(selected_profile) = config.profiles.get(profile) {
            config.rules = selected_profile.rules.clone();
        }
        
        config.save_to_file(path)?;
        Ok(())
    }
}

fn create_strict_rules() -> HashMap<String, RuleConfig> {
    let mut rules = HashMap::new();
    
    rules.insert("component-complexity".to_string(), RuleConfig {
        enabled: true,
        severity: "error".to_string(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("max_complexity".to_string(), serde_json::Value::Number(serde_json::Number::from(8)));
            opts
        },
    });
    
    rules.insert("change-detection-strategy".to_string(), RuleConfig {
        enabled: true,
        severity: "warning".to_string(),
        options: HashMap::new(),
    });
    
    rules.insert("too-many-inputs".to_string(), RuleConfig {
        enabled: true,
        severity: "error".to_string(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("max_inputs".to_string(), serde_json::Value::Number(serde_json::Number::from(6)));
            opts
        },
    });
    
    rules.insert("missing-cleanup-pattern".to_string(), RuleConfig {
        enabled: true,
        severity: "error".to_string(),
        options: HashMap::new(),
    });
    
    rules.insert("circular-dependency".to_string(), RuleConfig {
        enabled: true,
        severity: "error".to_string(),
        options: HashMap::new(),
    });
    
    rules
}

fn create_recommended_rules() -> HashMap<String, RuleConfig> {
    let mut rules = HashMap::new();
    
    rules.insert("component-complexity".to_string(), RuleConfig {
        enabled: true,
        severity: "warning".to_string(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("max_complexity".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
            opts
        },
    });
    
    rules.insert("change-detection-strategy".to_string(), RuleConfig {
        enabled: true,
        severity: "info".to_string(),
        options: HashMap::new(),
    });
    
    rules.insert("too-many-inputs".to_string(), RuleConfig {
        enabled: true,
        severity: "warning".to_string(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("max_inputs".to_string(), serde_json::Value::Number(serde_json::Number::from(8)));
            opts
        },
    });
    
    rules.insert("missing-cleanup-pattern".to_string(), RuleConfig {
        enabled: true,
        severity: "warning".to_string(),
        options: HashMap::new(),
    });
    
    rules.insert("circular-dependency".to_string(), RuleConfig {
        enabled: true,
        severity: "error".to_string(),
        options: HashMap::new(),
    });
    
    rules
}

fn create_relaxed_rules() -> HashMap<String, RuleConfig> {
    let mut rules = HashMap::new();
    
    rules.insert("component-complexity".to_string(), RuleConfig {
        enabled: true,
        severity: "info".to_string(),
        options: {
            let mut opts = HashMap::new();
            opts.insert("max_complexity".to_string(), serde_json::Value::Number(serde_json::Number::from(15)));
            opts
        },
    });
    
    rules.insert("change-detection-strategy".to_string(), RuleConfig {
        enabled: false,
        severity: "info".to_string(),
        options: HashMap::new(),
    });
    
    rules.insert("circular-dependency".to_string(), RuleConfig {
        enabled: true,
        severity: "warning".to_string(),
        options: HashMap::new(),
    });
    
    rules
}