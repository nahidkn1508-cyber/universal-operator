use std::io;
use std::process::Command;

pub struct MouseController;

impl MouseController {
    pub fn new() -> Self {
        Self
    }

    pub fn move_to(&self, x: i32, y: i32) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"do shell script "python3 -c 'from Quartz.CoreGraphics import CGWarpMouseCursorPosition; CGWarpMouseCursorPosition(({}, {}))'"#,
                x, y
            ))
            .status()?;

        Ok(())
    }

    pub fn left_click(&self) -> io::Result<()> {
        println!("Left click");
        Ok(())
    }

    pub fn right_click(&self) -> io::Result<()> {
        println!("Right click");
        Ok(())
    }

    pub fn double_click(&self) -> io::Result<()> {
        println!("Double click");
        Ok(())
    }

    pub fn drag(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> io::Result<()> {
        println!(
            "Drag from ({}, {}) to ({}, {})",
            start_x, start_y, end_x, end_y
        );

        Ok(())
    }

    pub fn scroll(&self, amount: i32) -> io::Result<()> {
        println!("Scroll {}", amount);
        Ok(())
    }
}
