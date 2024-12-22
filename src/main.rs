use std::io::{self, Write};

fn main() {
    let mut tod_do_list: Vec<String> = Vec::new();
    loop {
        // Print the menu
        println!("\n=====================");
        println!("\nTo-Do List");
        println!("\n=====================");
        println!("1. View Tasks");
        println!("2. Add Task");
        println!("3. Mark Task as Done");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        // Read the input from the user
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        // Match the choice
        match choice {
            "1" => view_tasks(&tod_do_list),
            "2" => add_task(&mut tod_do_list),
            "3" => mark_task_as_done(&mut tod_do_list),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    fn view_tasks(tod_do_list: &Vec<String>) {
        if tod_do_list.is_empty() {
            println!("You have no tasks.");
        } else {
            for (index, task) in tod_do_list.iter().enumerate() {
                println!("{}: {}", index + 1, task);
            }
        }
    }

    fn add_task(tod_do_list: &mut Vec<String>) {
        print!("Enter a task: ");
        io::stdout().flush().unwrap();
        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();
        tod_do_list.push(task.trim().to_string());
        println!("Task added.");
    }

    fn mark_task_as_done(tod_do_list: &mut Vec<String>) {
        if tod_do_list.is_empty() {
            println!("You have no tasks to mark as done.");
        } else {
            print!("Enter the task number to mark as done: ");
            io::stdout().flush().unwrap();
            let mut task_number = String::new();
            io::stdin().read_line(&mut task_number).unwrap();
            let task_number = task_number.trim().parse::<usize>().unwrap();
            if task_number > 0 && task_number <= tod_do_list.len() {
                tod_do_list.remove(task_number - 1);
                println!("Task marked as done.");
            } else {
                println!("Invalid task number.");
            }
        }
    }

}
