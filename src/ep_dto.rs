// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::collections::HashMap;

const BASE_HASHING_CONSTANT: &str = "EXPT";

// Experiment DTO
#[derive(Debug, PartialEq)]
pub struct Experiment {
    pub name: String,
    pub context_expression: String,
    pub hashing_constant: String,
    pub experiment_id: i32,
    pub experiment_flags: i32,
    pub variant_rules: Vec<VariantRule>,
    pub variants: Vec<Variant>,
    pub base_mod: Traffic,
}

// Target under Feature Flag Variant Rule
#[derive(Debug, PartialEq)]
pub struct Target {
    pub variant_mod_map: HashMap<i32, Traffic>,
}

// Variant Rule for supporting Feature Flag
#[derive(Debug, PartialEq)]
pub struct VariantRule {
    pub rule_id: i32,
    pub context_expression: String,
    pub target: Target,
}

// Traffic Sample
#[derive(Debug, PartialEq)]
pub struct Traffic {
    pub spectrum: String,
}

// Variant DTO
#[derive(Debug, PartialEq)]
pub struct Variant {
    pub name: String,
    pub value: String,
    pub variant_id: i32,
    pub variant_display_id: String,
    pub variant_flags: i32,
    pub variant_mod: Traffic,
    pub whitelisted_uids: Vec<String>,
}

// Unit tests for DTO creation
#[cfg(test)]
mod tests {
    use super::*; // Import items from the parent module

    #[test]
    fn constant_value() {
        assert_eq!(BASE_HASHING_CONSTANT, "EXPT");
    }

    // Experiment Creation Unit Test
    #[test]
    fn experiment_creation() {
        let color_red_variant = Variant {
            name: "Red Variant".to_string(),
            value: "#FF0000".to_string(),
            variant_id: 1024,
            variant_display_id: "0aX0".to_string(),
            variant_flags: 0,
            variant_mod: Traffic { spectrum: "1111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000".to_string() },
            whitelisted_uids: vec!["1038812".to_string()]
        };
        let color_blue_variant = Variant {
            name: "Blue Variant".to_string(),
            value: "#00FF00".to_string(),
            variant_id: 1025,
            variant_display_id: "0aX1".to_string(),
            variant_flags: 0,
            variant_mod: Traffic { spectrum: "0000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111".to_string() },
            whitelisted_uids: vec!["1015529".to_string()]
        };
        let mut first_variant_mod_map = HashMap::new();
        first_variant_mod_map.insert(1024, Traffic { spectrum: "0000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111".to_string() });
        first_variant_mod_map.insert(1025, Traffic { spectrum: "1111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000".to_string() });
        let first_variant_rule = VariantRule {
            rule_id: 0,
            context_expression: "AND(IN(SITEID, 0), IN(CHANNELID, 1))".to_string(),
            target: Target {
                variant_mod_map: first_variant_mod_map,
            },
        };
        let color_experiment = Experiment {
            name: "Color Experiment".to_string(),
            context_expression: "AND(IN(SITEID, 0, 77), IN(CHANNELID, 1, 5, 6), EQ(F90D, \"TRUE\"))".to_string(),
            hashing_constant: "0XF23AC".to_string(),
            experiment_id: 65536,
            experiment_flags: 0,
            variant_rules: vec![first_variant_rule],
            variants: vec![color_red_variant, color_blue_variant],
            base_mod: Traffic { spectrum: "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string() },
        };
        assert_eq!(color_experiment.name, "Color Experiment");
        assert_eq!(color_experiment.variants.len(), 2);
        assert_eq!(color_experiment.variant_rules.len(), 1);
        assert_eq!(color_experiment, color_experiment);
    }
}
