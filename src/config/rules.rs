use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub name: String,
    pub description: String,
    pub category: String,
    pub default_severity: String,
    pub configurable_options: Vec<ConfigurableOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurableOption {
    pub name: String,
    pub description: String,
    pub option_type: String,
    pub default_value: serde_json::Value,
    pub possible_values: Option<Vec<serde_json::Value>>,
}

pub fn get_all_rule_definitions() -> Vec<RuleDefinition> {
    vec![
        RuleDefinition {
            name: "component-complexity".to_string(),
            description: "Checks if component complexity exceeds threshold".to_string(),
            category: "Code Quality".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "max_complexity".to_string(),
                    description: "Maximum allowed complexity score".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(10)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "change-detection-strategy".to_string(),
            description: "Suggests using OnPush change detection strategy".to_string(),
            category: "Performance".to_string(),
            default_severity: "info".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "too-many-inputs".to_string(),
            description: "Checks if component has too many input properties".to_string(),
            category: "Code Quality".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "max_inputs".to_string(),
                    description: "Maximum allowed number of inputs".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(8)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "too-many-outputs".to_string(),
            description: "Checks if component has too many output properties".to_string(),
            category: "Code Quality".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "max_outputs".to_string(),
                    description: "Maximum allowed number of outputs".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(5)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "missing-cleanup-pattern".to_string(),
            description: "Checks for proper cleanup patterns in components".to_string(),
            category: "Memory Management".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "template-conflict".to_string(),
            description: "Checks for conflicting template definitions".to_string(),
            category: "Code Quality".to_string(),
            default_severity: "error".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "circular-dependency".to_string(),
            description: "Detects circular dependencies between components and services".to_string(),
            category: "Architecture".to_string(),
            default_severity: "error".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "unused-dependency".to_string(),
            description: "Identifies unused dependencies".to_string(),
            category: "Code Quality".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "deep-dependency-chain".to_string(),
            description: "Checks for overly deep dependency chains".to_string(),
            category: "Architecture".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "max_depth".to_string(),
                    description: "Maximum allowed dependency depth".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(5)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "consider-state-management".to_string(),
            description: "Suggests centralized state management for complex applications".to_string(),
            category: "Architecture".to_string(),
            default_severity: "info".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "state_service_threshold".to_string(),
                    description: "Number of state services before suggesting centralized management".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(3)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "missing-unsubscribe-pattern".to_string(),
            description: "Checks for proper observable unsubscription patterns".to_string(),
            category: "Memory Management".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "high-default-change-detection".to_string(),
            description: "Warns about high usage of default change detection".to_string(),
            category: "Performance".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "threshold_percentage".to_string(),
                    description: "Percentage threshold for default change detection usage".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(70)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "consider-lazy-loading".to_string(),
            description: "Suggests implementing lazy loading for large applications".to_string(),
            category: "Performance".to_string(),
            default_severity: "info".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "component_threshold".to_string(),
                    description: "Number of components before suggesting lazy loading".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(10)),
                    possible_values: None,
                },
            ],
        },
        RuleDefinition {
            name: "potential-memory-leak".to_string(),
            description: "Identifies potential memory leak risks".to_string(),
            category: "Memory Management".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![],
        },
        RuleDefinition {
            name: "excessive-bindings".to_string(),
            description: "Checks for excessive property and event bindings".to_string(),
            category: "Performance".to_string(),
            default_severity: "warning".to_string(),
            configurable_options: vec![
                ConfigurableOption {
                    name: "max_bindings".to_string(),
                    description: "Maximum allowed number of bindings".to_string(),
                    option_type: "number".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(15)),
                    possible_values: None,
                },
            ],
        },
    ]
}

pub fn get_rule_definition(name: &str) -> Option<RuleDefinition> {
    get_all_rule_definitions().into_iter().find(|rule| rule.name == name)
}

pub fn get_rules_by_category(category: &str) -> Vec<RuleDefinition> {
    get_all_rule_definitions().into_iter()
        .filter(|rule| rule.category == category)
        .collect()
}

pub fn get_available_categories() -> Vec<String> {
    let mut categories: Vec<String> = get_all_rule_definitions().into_iter()
        .map(|rule| rule.category)
        .collect();
    categories.sort();
    categories.dedup();
    categories
}