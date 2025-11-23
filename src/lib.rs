mod tasks;

use clap::{Parser, Subcommand};
use tasks::{add_task, list_tasks, mark_done, remove_task};

use crate::tasks::load_tasks;

#[derive(Parser)]
#[command(name = "mytodo", version = "1.0", about = "Aplikasi todo-list CLI sederhana")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Menambah tugas baru dengan deskripsi tertentu
    Add {
        description: String,
    },
    /// Menampilkan semua tugas
    List {
        #[arg(long)]
        pending:bool
    },
    /// Menandai tugas dengan nomor tertentu sebagai selesai
    Done {
        id: usize,
    },
    /// Menghapus tugas dengan nomor tertentu
    Remove {
        id: usize
    }
}

pub fn run(cli: Cli) -> Result<bool, String> {
    // Inisialisasi daftar tugas (untuk sementara, kosong di awal)
    let mut tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => return Err(e.to_string())
    };

    match cli.command {
        Some(Commands::Add { description }) => {
            add_task(&mut tasks, description);
        }
        Some(Commands::List { pending }) => {
            list_tasks(&tasks, pending);
        }
        Some(Commands::Done { id }) => {
            mark_done(&mut tasks, id);
        }
        Some(Commands::Remove { id }) => {
            remove_task(&mut tasks, id);
        }
        None => {
            println!("Tidak ada perintah. Gunakan `--help` untuk daftar perintah.");
        }
    }

    Ok(true)
}