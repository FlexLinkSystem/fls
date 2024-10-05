use colored::Colorize;
use std::time::Instant;

pub struct Logger
{
    name : String,
    timer : Instant
}

impl Logger {
    pub fn new(node_name: String)->Self
    {

        Self { name: node_name , timer : Instant::now()}
    }

    pub fn info(&self, str : String)
    {
        println!("[{}][{}] {}", self.timer.elapsed().as_millis().to_string().bright_blue(), self.name.clone().bright_blue(), str.bright_blue())
    }

    pub fn error(&self, str : String)
    {
        println!("[{}][{}] {}", self.timer.elapsed().as_millis().to_string().bright_red(), self.name.clone().bright_red(), str.bright_red())
    }
}