// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to hide warnings for unused imports.
#![allow(unused_imports)]
// An attribute to hide warnings for unused variables.
#![allow(unused_variables)]
// An attribute to hide warnings for unused mutable.
#![allow(unused_mut)]

use crate::core_qualification_dto::EvaluationContext;

// Abstract different Phase for Core Qualification
// Each Phase will execute these pre-defined methods following the accordingly sequence logically:
// #before --> #execute --> #after
// Ensure: Never throw ANY exception from any below methods
pub trait Phase {
    fn before(&self, context: &mut EvaluationContext);

    fn execute(&self, context: &mut EvaluationContext);

    fn after(&self, context: &mut EvaluationContext);
}

// Abstract different Mapper for Core Qualification's Mapping Phase
// Each Mapper will execute these pre-defined methods following the accordingly sequence logically:
// #before --> #map --> #after
// Ensure: Never throw ANY exception from any below methods
pub trait Mapper {
    fn before(&self, context: &mut EvaluationContext);

    fn map(&self, context: &mut EvaluationContext);

    fn after(&self, context: &mut EvaluationContext);
}

pub struct InitializationPhase;

impl Phase for InitializationPhase {
    fn before(&self, context: &mut EvaluationContext) {
        log::debug!("#InitializationPhase start.");
    }

    fn execute(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#InitializationPhase finished.");
    }
}

pub struct ValidationPhase;

impl Phase for ValidationPhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, context: &mut EvaluationContext) {
        for experiment in &mut context.experiment_list {
            let must_to_have_context_key = experiment.randomization_unit_key.to_string();
            if !context
                .context_map
                .contains_key(&String::from(must_to_have_context_key))
            {
                context.error_code = 1;
                context.error_message = format!(
                    "Missing context key {}",
                    experiment.randomization_unit_key.to_string()
                );
                log::error!("{}", context.error_message);
                break;
            }
        }
    }

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#ValidationPhase finished.");
    }
}

pub struct MappingPhase {
    pub mappers: Vec<Box<dyn Mapper>>,
}

impl Phase for MappingPhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, mut context: &mut EvaluationContext) {
        for individual_mapper in self.mappers.iter() {
            individual_mapper.before(context);
            individual_mapper.map(context);
            individual_mapper.after(context);
        }
    }

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#MappingPhase finished.");
    }
}

pub struct OptInMapper;

impl Mapper for OptInMapper {
    fn before(&self, context: &mut EvaluationContext) {}

    fn map(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#OptInMapper finished.");
    }
}

pub struct UidListMapper;

impl Mapper for UidListMapper {
    fn before(&self, context: &mut EvaluationContext) {}

    fn map(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#UidListMapper finished.");
    }
}

pub struct VariantRuleMapper;

impl Mapper for VariantRuleMapper {
    fn before(&self, context: &mut EvaluationContext) {}

    fn map(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#VariantRuleMapper finished.");
    }
}

pub struct VariantMapper;

impl Mapper for VariantMapper {
    fn before(&self, context: &mut EvaluationContext) {}

    fn map(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#VariantMapper finished.");
    }
}

pub struct CollisionResolvePhase;

impl Phase for CollisionResolvePhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#CollisionResolvePhase finished.");
    }
}

pub struct PrioritizationPhase;

impl Phase for PrioritizationPhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#PrioritizationPhase finished.");
    }
}

pub struct ContextPhase;

impl Phase for ContextPhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#ContextPhase finished.");
    }
}

pub struct ResultPackagedPhase;

impl Phase for ResultPackagedPhase {
    fn before(&self, context: &mut EvaluationContext) {}

    fn execute(&self, context: &mut EvaluationContext) {}

    fn after(&self, context: &mut EvaluationContext) {
        log::debug!("#ResultPackagePhase finished.");
    }
}

pub struct QualificationEngine {
    pub phases: Vec<Box<dyn Phase>>,
}

impl QualificationEngine {
    fn qualify(&self, context: &mut EvaluationContext) {
        for individual_phase in self.phases.iter() {
            individual_phase.before(context);
            individual_phase.execute(context);
            individual_phase.after(context);
            if context.error_code > 0 {
                break;
            }
        }
    }
}

impl Default for QualificationEngine {
    fn default() -> Self {
        QualificationEngine {
            phases: vec![
                Box::new(InitializationPhase),
                Box::new(ValidationPhase),
                Box::new(MappingPhase {
                    mappers: vec![
                        Box::new(OptInMapper),
                        Box::new(UidListMapper),
                        Box::new(VariantRuleMapper),
                    ],
                }),
                Box::new(CollisionResolvePhase),
                Box::new(PrioritizationPhase),
                Box::new(ContextPhase),
                Box::new(ResultPackagedPhase),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_qualification_dto::EvaluationResult;
    use crate::ep_dto::{Experiment, Target, Traffic, Variant, VariantRule};
    use mockall::predicate::*;
    use mockall::*;
    use std::any::Any;
    use std::collections::HashMap;

    #[ctor::ctor]
    fn init() {
        env_logger::builder().format_timestamp_nanos().init();
    }

    // Qualification Engine Unit Test
    #[test]
    fn qualification_engine_creation() {
        let engine = QualificationEngine::default();
        assert_eq!(engine.phases.len(), 7);
    }

    #[test]
    fn qualification_engine_qualify_happy_path() {
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
            randomization_unit_key: "LOOKUP_ID".to_string(),
        };
        let mut context_map: HashMap<String, String> = HashMap::new();
        context_map.insert("LOOKUP_ID".to_string(), "search_88ax9i5".to_string());
        context_map.insert("UID".to_string(), "1015529".to_string());
        context_map.insert("SITEID".to_string(), "100".to_string());
        context_map.insert("CHANNELID".to_string(), "6".to_string());
        context_map.insert("F90D".to_string(), "TRUE".to_string());
        let mut evaluation_context = EvaluationContext {
            experiment_list: vec![color_experiment],
            context_map,
            opt_in_variant_display_ids: vec!["0aX0".to_string()],
            ..Default::default()
        };
        let engine = QualificationEngine::default();

        engine.qualify(&mut evaluation_context);
        assert_eq!(evaluation_context.experiment_list.len(), 1);
        assert_eq!(evaluation_context.result.variant_result_map.len(), 0);
        assert_eq!(evaluation_context.result_by_phase.len(), 0);
        assert_eq!(evaluation_context.error_code, 0);
        assert_eq!(evaluation_context.error_message, "");
    }

    #[test]
    fn qualification_engine_qualify_missing_randomization_unit_context() {
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
            randomization_unit_key: "LOOKUP_ID".to_string(),
        };
        let mut context_map: HashMap<String, String> = HashMap::new();
        context_map.insert("UID".to_string(), "1015529".to_string());
        context_map.insert("SITEID".to_string(), "100".to_string());
        context_map.insert("CHANNELID".to_string(), "6".to_string());
        context_map.insert("F90D".to_string(), "TRUE".to_string());
        let mut evaluation_context = EvaluationContext {
            experiment_list: vec![color_experiment],
            context_map,
            opt_in_variant_display_ids: vec!["0aX0".to_string()],
            ..Default::default()
        };
        let engine = QualificationEngine::default();

        engine.qualify(&mut evaluation_context);
        assert_eq!(evaluation_context.experiment_list.len(), 1);
        assert_eq!(evaluation_context.result.variant_result_map.len(), 0);
        assert_eq!(evaluation_context.result_by_phase.len(), 0);
        assert_eq!(evaluation_context.error_code, 1);
        assert_eq!(
            evaluation_context.error_message,
            "Missing context key LOOKUP_ID"
        );
    }
}
