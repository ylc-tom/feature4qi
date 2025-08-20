// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to hide warnings for unused imports.
#![allow(unused_imports)]

use crate::core_qualification_dto::EvaluationContext;

// Abstract different Phase for Core Qualification
// Each Phase will execute these pre-defined methods following the accordingly sequence logically:
// #before --> #execute --> #after
pub trait Phase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext;

    fn execute(&self, context: EvaluationContext) -> EvaluationContext;

    fn after(&self, context: EvaluationContext) -> EvaluationContext;
}

// Abstract different Mapper for Core Qualification's Mapping Phase
// Each Mapper will execute these pre-defined methods following the accordingly sequence logically:
// #before --> #execute --> #after
pub trait Mapper {
    fn before(&self, context: EvaluationContext) -> EvaluationContext;

    fn execute(&self, context: EvaluationContext) -> EvaluationContext;

    fn after(&self, context: EvaluationContext) -> EvaluationContext;
}

pub struct InitializationPhase;

impl Phase for InitializationPhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct MappingPhase {
    pub mappers: Vec<Box<dyn Mapper>>,
}

impl Phase for MappingPhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, mut context: EvaluationContext) -> EvaluationContext {
        for individual_mapper in self.mappers.iter() {
            context = individual_mapper.before(context);
            context = individual_mapper.execute(context);
            context = individual_mapper.after(context);
        }
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct OptInMapper;

impl Mapper for OptInMapper {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct UidListMapper;

impl Mapper for UidListMapper {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct VariantRuleMapper;

impl Mapper for VariantRuleMapper {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct VariantMapper;

impl Mapper for VariantMapper {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct CollisionResolvePhase;

impl Phase for CollisionResolvePhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct PrioritizationPhase;

impl Phase for PrioritizationPhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct ContextPhase;

impl Phase for ContextPhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct ResultPackagePhase;

impl Phase for ResultPackagePhase {
    fn before(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn execute(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }

    fn after(&self, context: EvaluationContext) -> EvaluationContext {
        context
    }
}

pub struct QualificationEngine {
    pub phases: Vec<Box<dyn Phase>>,
}

impl QualificationEngine {
    fn qualify(&self, mut context: EvaluationContext) -> EvaluationContext {
        for individual_phase in self.phases.iter() {
            context = individual_phase.before(context);
            context = individual_phase.execute(context);
            context = individual_phase.after(context);
        }
        context
    }
}

impl Default for QualificationEngine {
    fn default() -> Self {
        QualificationEngine {
            phases: vec![
                Box::new(InitializationPhase),
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
                Box::new(ResultPackagePhase),
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

    // Qualification Engine Unit Test
    #[test]
    fn qualification_engine_creation() {
        let engine = QualificationEngine::default();
        assert_eq!(engine.phases.len(), 6);
    }

    #[test]
    fn qualification_engine_qualify() {
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
        let mut context_map:HashMap<&'static str, &'static str> = HashMap::new();
        context_map.insert("SITEID", "100");
        context_map.insert("CHANNELID", "6");
        context_map.insert("F90D", "TRUE");
        let evaluation_context = EvaluationContext {
            experiment_list: vec![color_experiment],
            context_map,
            error_code: 0,
            error_message: "".to_string(),
            result_by_phase: HashMap::new(),
            result: EvaluationResult {
                variant_result_map: HashMap::new(),
            },
        };
        let engine = QualificationEngine::default();
        let result_evaluation_context = engine.qualify(evaluation_context);
        assert_eq!(result_evaluation_context.experiment_list.len(), 1);
        assert_eq!(result_evaluation_context.result.variant_result_map.len(), 0);
        assert_eq!(result_evaluation_context.result_by_phase.len(), 0);
    }
}
