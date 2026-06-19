#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkSessionState {
    Idle,
    Connected,
    Executing,
    Paused,
    Completed,
    Stopped,
}

pub struct WorkSessionManager {
    state: WorkSessionState,
}

impl WorkSessionManager {
    pub fn new() -> Self {
        Self {
            state: WorkSessionState::Idle,
        }
    }

    pub fn state(&self) -> WorkSessionState {
        self.state
    }

    pub fn connect(&mut self) {
        self.state = WorkSessionState::Connected;
        println!("Connected to AI.");
    }

    pub fn start(&mut self) {
        self.state = WorkSessionState::Executing;
        println!("Work Session Started.");
    }

    pub fn pause(&mut self) {
        self.state = WorkSessionState::Paused;
        println!("Execution Paused.");
    }

    pub fn resume(&mut self) {
        self.state = WorkSessionState::Executing;
        println!("Execution Resumed.");
    }

    pub fn complete(&mut self) {
        self.state = WorkSessionState::Completed;
        println!("Execution Completed.");
    }

    pub fn stop(&mut self) {
        self.state = WorkSessionState::Stopped;
        println!("Execution Stopped.");
    }

    pub fn reset(&mut self) {
        self.state = WorkSessionState::Idle;
        println!("Returned to Idle.");
    }

    pub fn is_idle(&self) -> bool {
        self.state == WorkSessionState::Idle
    }

    pub fn is_executing(&self) -> bool {
        self.state == WorkSessionState::Executing
    }

    pub fn is_paused(&self) -> bool {
        self.state == WorkSessionState::Paused
    }
}
