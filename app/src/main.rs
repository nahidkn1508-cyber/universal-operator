use common::types::action::Action;
use execution_planner::ExecutionPlanner;
use execution_queue::ExecutionQueue;
use executor::execution::executor::ExecutionEngine;
use parser::Parser;

fn main() {
    println!("Discovered AI Sessions:\n");

    for session in ai_session_manager::SessionManager::discover() {
        println!("ID: {}", session.id);
        println!("Application: {}", session.application);
        println!("Window: {}", session.window_title);
        println!("Connector: {}", session.connector);
        println!("--------------------------------");

        if let Some(mut connector) =
            ai_session_manager::create_connector(&session.connector, session.id.clone())
        {
            if connector.connect() {
                println!("✓ Connected");

                if let Some(response) =
                    ai_session_manager::ResponseListener::wait_for_response(&session.id)
                {
                    println!();
                    println!("AI Response Received:");
                    println!("{}", response);
                } else {
                    println!("No AI response yet.");
                }

                connector.disconnect();

                println!("Disconnected");
            }
        }
    }

    println!();
    println!("Browser Tabs:\n");

    for tab in ai_session_manager::browser_tabs() {
        println!("• {} | {}", tab.browser, tab.title);
        println!("  {}", tab.url);
    }

    println!();
    println!("==============================");
    println!("Universal AI Operator");
    println!("==============================");

    let ai_response = r#"
Create folder Demo
Create file Demo/hello.txt
Run command pwd
Run command ls
"#;

    println!();
    println!("AI Response:");
    println!("{}", ai_response);

    let parser = Parser::new();
    let planner = ExecutionPlanner::new();
    let executor = ExecutionEngine::new();

    let actions: Vec<Action> = parser.parse(ai_response);

    let planned = planner.create_plan(actions);

    let mut queue = ExecutionQueue::new();

    for action in planned {
        queue.push(action);
    }

    println!();
    println!("Executing...");
    println!();

    while let Some(action) = queue.pop() {
        executor.execute(&action);
    }

    println!();
    println!("Execution Finished.");
}
