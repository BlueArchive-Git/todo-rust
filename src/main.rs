use std::io::{self, Write};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Task {
	id: u32,
	title: String,
	done: bool,
}
fn main() {
	let home = std::env::var("HOME").unwrap_or_default();
	let path = format!("{}/.todo.json", home);
	let mut tasks: Vec<Task> = match std::fs::read_to_string(&path) {
		Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
		Err(_) => Vec::new(),
	};
	let mut next_id: u32 = 1;
	let args: Vec<String> = std::env::args().collect();
	let cmd = match args.get(1) {
		Some(s) => s.as_str(),
		None => "",
	};
	match cmd {
		"list" => {
			println!("Tasks:");
			for task in &tasks {
				let mark = if task.done { "x" } else { " " };
				println!("[{}] {}", mark, task.title);
			}
		}
		"add" => {
			let titles: Vec<&str> = args.iter().skip(2).map(|s|s.as_str()).collect();
			let mut added: Vec<&str> = Vec::new();
			let mut skipped: Vec<&str> = Vec::new();
			for title in &titles {
				if tasks.iter().any(|t| t.title == *title) {
					skipped.push(title);
					continue;
				}
				tasks.push(Task {
					id: next_id,
					title: title.to_string(),
					done: false,
				});
				next_id += 1;
				added.push(title);
			}
			if !skipped.is_empty() {
				if skipped.len() == 1 {
					println!("{}: task already exists: {}", args[0], skipped[0]);
				} else {
					println!("{}: tasks already exists: {}", args[0], skipped.join(", "));
				}
			}
			if !added.is_empty() {
				if added.len() == 1 {
					println!("{}: added task: {}", args[0], added[0]);
				} else {
					println!("{}: added tasks: {}", args[0], added.join(", "));
				}
				let json = serde_json::to_string_pretty(&tasks).unwrap();
				std::fs::write(&path, json).unwrap();
			}
		}
		"done" => {
			let targets: Vec<&str> = args.iter().skip(2).map(|s|s.as_str()).collect();
			let mut find: Vec<&str> = Vec::new();
			let mut nonexist: Vec<&str> = Vec::new();
			for target in &targets {
				let found = tasks.iter_mut().find(|t| t.title == *target);
				match found {
					Some(task) => {
						task.done = true;
						find.push(target);
					}
					None => {
						nonexist.push(target);
					}
				}
			}
			if !nonexist.is_empty() {
				if nonexist.len() == 1 {
					println!("{}: task not found: {}", args[0], nonexist[0]);
				} else {
					println!("{}: tasks not found: {}", args[0], nonexist.join(", "));
				}
			}
			if !find.is_empty() {
				if find.len() == 1 {
					println!("{}: task completed: {}", args[0], find[0]);
				} else {
					println!("{}: tasks completed: {}", args[0], find.join(", "));
				}
				let json = serde_json::to_string_pretty(&tasks).unwrap();
				std::fs::write(&path, json).unwrap();
			} 
		}
		"rm" => {
			let targets: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();
			let mut removed: Vec<&str> = Vec::new();
			let mut not_found: Vec<&str> = Vec::new();
			let mut cancelled: Vec<&str> = Vec::new();
			for target in &targets {
				if !tasks.iter().any(|t| t.title == *target) {
					not_found.push(target);
					continue;
				}
				print!("{}: remove task: {}? (y/N) > ", args[0], target);
				io::stdout().flush().unwrap();
				let mut input = String::new();
				io::stdin().read_line(&mut input).expect("read failed");
				let input = input.trim().to_lowercase();
				if input == "y" || input == "yes" {
					tasks.retain(|t| t.title != *target);
					removed.push(target);
				} else {
					cancelled.push(target);
				}
			}
			if !not_found.is_empty() {
				if not_found.len() == 1 {
					println!("{}: task not found: {}", args[0], not_found[0]);
				} else {
					println!("{}: tasks not found: {}", args[0], not_found.join(", "));
				}
			}
			if !cancelled.is_empty() {
				if cancelled.len() == 1 {
					println!("{}: task cancelled: {}", args[0], cancelled[0]);
				} else {
					println!("{}: tasks cancelled: {}", args[0], cancelled.join(", "));
				}
			}
			if !removed.is_empty() {
				if removed.len() == 1 {
					println!("{}: task removed: {}", args[0], removed[0]);
				} else {
					println!("{}: tasks removed: {}", args[0], removed.join(", "));
				}
				let json = serde_json::to_string_pretty(&tasks).unwrap();
				std::fs::write(&path, json).unwrap();
			}
		}
		_ => {
			println!("{}: no such command: {}", args[0], cmd);
		}
	}
}
