#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    CreateFolder,
    CreateFile,
    WriteFile,
    AppendFile,
    DeleteFile,
    RenameFile,
    MoveFile,

    RunCommand,

    OpenApplication,
    CloseApplication,

    OpenUrl,

    Wait,

    UserConfirmation,

    Unknown,
}
