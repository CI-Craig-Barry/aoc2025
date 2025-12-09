
pub mod utils {
  use colored::Colorize;

  pub struct ExecDetails {
    pub day: u8,
    pub sample: String,
    pub input: String,
    pub task1_function: fn(input: &String) -> i64,
    pub task2_function: fn(input: &String) -> i64,
    pub task1_sample_expected: i64,
    pub task1_input_expected: i64,
    pub task2_sample_expected: i64,
    pub task2_input_expected: i64,
  }

  fn assert_result(
    task: &str,
    result: i64,
    expected: i64
  )
  {
    if result == expected {
      println!("{} {}", task, "Success".green().bold());
    }
    else {
      println!("{} {} Found {} but expected {}", task, "Failed".red().bold(), result, expected);
    }
  }

  pub fn exec(details: &ExecDetails)
  {
    let mut result: i64;
    println!("------ Day: {} ------", details.day);
    result = (details.task1_function)(&details.sample);
    assert_result("Task 1 Sample", result, details.task1_sample_expected);
    result = (details.task1_function)(&details.input);
    assert_result("Task 1 Input", result, details.task1_input_expected);
    result = (details.task2_function)(&details.sample);
    assert_result("Task 2 Sample", result, details.task2_sample_expected);
    result = (details.task2_function)(&details.input);
    assert_result("Task 2 Input", result, details.task2_input_expected);
    println!();
  }
}