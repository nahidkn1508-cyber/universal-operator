use browser_controller::BrowserController;
use execution_reporter::ExecutionReporter;
use std::cell::RefCell;

use application_controller::ApplicationController;
use common::types::action::{Action, ActionKind};
use permission_manager::PermissionManager;

use crate::{files::FileManager, terminal::TerminalManager};

pub struct ExecutionEngine {
    files: FileManager,
    terminal: TerminalManager,
    permissions: PermissionManager,
    applications: ApplicationController,
    browser: BrowserController,
    reporter: ExecutionReporter,
    working_directory: RefCell<Option<String>>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self {
            files: FileManager::new(),
            terminal: TerminalManager::new(),
            permissions: PermissionManager::new(),
            applications: ApplicationController::new(),
            browser: BrowserController::new(),
            reporter: ExecutionReporter::new(),
            working_directory: RefCell::new(None),
        }
    }

    pub fn execute(&self, action: &Action) {
        if !self.permissions.can_execute() {
            self.reporter.report_failure(
                "Permission Check",
                "Permission Validation",
                "Permission denied",
            );
            return;
        }

        match action.kind {
            ActionKind::CreateFolder => {
                if let Some(path) = &action.target {
                    match self.files.create_folder(path) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Created folder: {}", path)),
                        Err(e) => {
                            self.reporter
                                .report_failure(path, "Create Folder", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::CreateFile => {
                if let Some(path) = &action.target {
                    match self.files.create_file(path) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Created file: {}", path)),
                        Err(e) => {
                            self.reporter
                                .report_failure(path, "Create File", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::WriteFile => {
                if let (Some(path), Some(content)) = (&action.target, &action.content) {
                    match self.files.write_file(path, content) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Wrote file: {}", path)),
                        Err(e) => {
                            self.reporter
                                .report_failure(path, "Write File", &e.to_string());
                        }
                    }
                } else {
                    self.reporter.report_failure(
                        "Write File",
                        "Validation",
                        "Missing file path or content",
                    );
                }
            }

            ActionKind::AppendFile => {
                if let (Some(path), Some(content)) = (&action.target, &action.content) {
                    match self.files.append_file(path, content) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Appended file: {}", path)),
                        Err(e) => {
                            self.reporter
                                .report_failure(path, "Append File", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::DeleteFile => {
                if let Some(path) = &action.target {
                    match self.files.delete_file(path) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Deleted file: {}", path)),
                        Err(e) => {
                            self.reporter
                                .report_failure(path, "Delete File", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::RenameFile => {
                println!("RenameFile not yet implemented");
            }

            ActionKind::MoveFile => {
                println!("MoveFile not yet implemented");
            }

            ActionKind::RunCommand => {
                let command = action.command.as_deref().or(action.target.as_deref());

                if let Some(command) = command {
                    println!("> {}", command);

                    let dir = self
                        .working_directory
                        .borrow()
                        .clone()
                        .unwrap_or_else(|| ".".to_string());

                    match self.terminal.run_in(command, &dir) {
                        Ok(_) => self.reporter.report_success("Command completed"),
                        Err(e) => {
                            self.reporter
                                .report_failure(command, "Run Command", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::OpenApplication => {
                if let Some(app) = &action.target {
                    match self.applications.open(app) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Opened application: {}", app)),
                        Err(e) => {
                            self.reporter
                                .report_failure(app, "Open Application", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::CloseApplication => {
                if let Some(app) = &action.target {
                    match self.applications.close(app) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Closed application: {}", app)),
                        Err(e) => {
                            self.reporter
                                .report_failure(app, "Close Application", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::OpenUrl => {
                if let Some(url) = &action.target {
                    match self.browser.open(url) {
                        Ok(_) => self
                            .reporter
                            .report_success(&format!("Opened URL: {}", url)),
                        Err(e) => {
                            self.reporter
                                .report_failure(url, "Open URL", &e.to_string());
                        }
                    }
                }
            }

            ActionKind::Wait => {
                println!("Wait");
            }

            ActionKind::UserConfirmation => {
                println!("UserConfirmation");
            }

            ActionKind::Unknown => {
                println!("Unknown action");
            }
        }
    }
}
