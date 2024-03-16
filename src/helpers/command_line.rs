use crossterm::{
  style::{Color, ResetColor, SetForegroundColor},
  ExecutableCommand,
};
use reqwest::Client;
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
  pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
      let mut stdout: std::io::Stdout = stdout();

      // Decide on the print color
      let statement_color: Color = match self {
          Self::AICall => Color::Cyan,
          Self::UnitTest => Color::Magenta,
          Self::Issue => Color::Red,
      };

      // Print the agent statement in a specific color
      stdout.execute(SetForegroundColor(Color::Green)).unwrap();
      print!("Agent: {}: ", agent_pos);

      // Make selected color
      stdout.execute(SetForegroundColor(statement_color)).unwrap();
      println!("{}", agent_statement);

      // Reset color
      stdout.execute(ResetColor).unwrap();
  }
}

pub fn get_user_response(question: &str) -> String {
  let mut stdout: std::io::Stdout = stdout();
  
    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    
    // Reset Color
    stdout.execute(ResetColor).unwrap();

     // Read user input
     let mut user_response: String = String::new();
     stdin()
         .read_line(&mut user_response)
         .expect("Failed to read response");
 
     // Trim whitespace and return
     return user_response.trim().to_string();
}

pub async fn check_status_code(client:&Client, url: &str) -> Result<u16, reqwest::Error>{
  let response:reqwest::Response = client.get(url).send().await?;
  Ok(response.status().as_u16())
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_print_agent_msg() {
      PrintCommand::AICall
      .print_agent_message("Managing Agent", "This is a test statement");
  }

 
}