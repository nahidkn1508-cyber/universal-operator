#[derive(Debug)]
pub struct ExecutionReport {
    pub instruction: String,
    pub step: String,
    pub error: String,
}

pub struct ExecutionReporter;

impl ExecutionReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn report_success(&self, step: &str) {
        println!("✓ {}", step);
    }

    pub fn report_failure(&self, instruction: &str, step: &str, error: &str) -> ExecutionReport {
        let report = ExecutionReport {
            instruction: instruction.to_string(),
            step: step.to_string(),
            error: error.to_string(),
        };

        println!();
        println!("==============================");
        println!("Execution Failed");
        println!("Instruction : {}", report.instruction);
        println!("Step        : {}", report.step);
        println!("Error       : {}", report.error);
        println!("==============================");
        println!();

        report
    }
}
