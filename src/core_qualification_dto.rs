// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to hide warnings for unused imports.
#![allow(unused_imports)]

use crate::ep_dto::Experiment;
use std::collections::HashMap;

// The Evaluation Context to be propagated according to a sequence Evaluable Phase
#[derive(Debug, PartialEq)]
pub struct EvaluationContext {
    // Intake
    pub experiment_list: Vec<Experiment>,
    pub context_map: HashMap<&'static str, &'static str>,

    // Output
    pub error_code: i32,
    pub error_message: String,
    pub result_by_phase: HashMap<&'static str, EvaluationResult>,
    pub result: EvaluationResult,
}

// Evaluation Result by individual variant id
#[derive(Debug, PartialEq)]
pub struct EvaluationResult {
    pub variant_result_map: HashMap<i32, QualificationResult>,
}

#[derive(Debug, PartialEq)]
pub struct QualificationResult {
    pub qualification_result_type: QualificationResultType,
    pub qualification_result_reason: String,
}

// Qualification Result by individual variant
#[derive(Debug, PartialEq)]
pub enum QualificationResultType {
    Deferred,     // qualified at this stage and looking for next stage
    Qualified,    // final result is qualified
    NotQualified, // final result is not qualified
    Error,        // something wrong during the variant qualification
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    // Evaluation Context Creation Unit Test
    #[test]
    fn evaluation_context_creation() {
        let evaluation_context = EvaluationContext {
            experiment_list: vec![],
            context_map: HashMap::new(),
            error_code: 0,
            error_message: "".to_string(),
            result_by_phase: HashMap::new(),
            result: EvaluationResult {
                variant_result_map: HashMap::new()
            }
        };
        assert_eq!(evaluation_context.result_by_phase.len(), 0);
        assert_eq!(evaluation_context, evaluation_context);
    }
}
