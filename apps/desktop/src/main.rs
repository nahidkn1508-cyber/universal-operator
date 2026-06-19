use ai_session_manager::ResponseListener;
use execution_engine::ExecutionEngine;
use execution_planner::ExecutionPlanner;
use execution_queue::ExecutionQueue;
use parser::Parser;

fn main() {
    println!("====================================");
    println!("Universal AI Operator");
    println!("====================================");

    let response = match ResponseListener::wait_for_response("chatgpt") {
        Some(text) => text,
        None => {
            println!("No AI response received.");
            return;
        }
    };

    println!("\nAI Response:\n{}\n", response);

    let parser = Parser::new();
    let planner = ExecutionPlanner::new();

    let actions = parser.parse(&response);

    println!("Parsed {} actions", actions.len());

    let planned = planner.create_plan(actions);

    let mut queue = ExecutionQueue::new();

    for action in planned {
        queue.push(action);
    }

    while let Some(action) = queue.pop() {
        ExecutionEngine::execute(action);
    }

    println!("Work session completed.");
}