use serde::{Deserialize, Serialize};
// use serde_json::Result as SerdeResult;

#[derive(Deserialize, Serialize)]
pub struct Task {
  pub description: String,
  pub done: bool
}

/// Menambah tugas baru ke daftar
pub fn add_task(tasks: &mut Vec<Task>, desc: String) {
  let task = Task { description: desc, done: false };
  tasks.push(task);
  println!("Tugas baru ditambahkan");
}

/// Menampilkan semua tugas dalam daftar
pub fn list_tasks(tasks: &Vec<Task>, pending: bool) {
  if tasks.is_empty() {
    println!("(Belum ada tugas)");
    return;
  }

  println!("Daftar Tugas:");
  for (i, task) in tasks.iter().enumerate() {
    if pending && task.done { continue; }

    let status = if task.done { "[x]" } else { "[]" };
    println!("{}. {} {}", i+1, status, task.description);
  }
}

/// Menandai tugas selesai
pub fn mark_done(tasks: &mut Vec<Task>, id: usize) {
  if id == 0 || id > tasks.len() {
    println!("Error: Nomor tugas {} tidak valid!", id);
    return;
  }

  let index = id - 1;
  tasks[index].done = true;
  println!("Tugas {} telah ditandai selesai.", id);
}

/// Menghapus tugas
pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
  if id == 0 || id > tasks.len() {
    println!("Error: Nomor tugas {} tidak valid!", id);
    return;
  }

  let index = id - 1;
  tasks.remove(index);
  println!("Tugas {} dihapus", id);
}

// fn load_tasks_one() -> Result<Vec<Task>, String> {
//   let tasks_json = std::fs::read_to_string("tasks.json");
//   if let Err(e) = tasks_json {
//     return Err(e.to_string());
//   }

//   let validated_str = tasks_json.unwrap_or("[]".to_string());
//   let tasks = serde_json::from_str(&validated_str);
//   if let Err(e) = tasks {
//     return Err(e.to_string());
//   }

//   return Ok(tasks.unwrap());
// }

// fn load_tasks_second() -> SerdeResult<Vec<Task>> {
//   let tasks_json = std::fs::read_to_string("tasks.json");
//   if let Err(_) = tasks_json {
//     return Ok(Vec::new());
//   }

//   let tasks: Vec<Task> = serde_json::from_str(&tasks_json.unwrap())?;
//   Ok(tasks)
// }

pub fn load_tasks() -> Result<Vec<Task>, String> {
  let tasks_json: Result<String, std::io::Error> = std::fs::read_to_string("tasks.json");
  let tasks_json: String = match tasks_json {
      Ok(s) => s,
      Err(e) => return Err(e.to_string()),
  };

  let tasks: Result<Vec<Task>, serde_json::Error>= serde_json::from_str(&tasks_json);
  let tasks: Vec<Task> = match tasks {
      Ok(t) => t,
      Err(e) => return Err(e.to_string()),
  };
  
  Ok(tasks)
}