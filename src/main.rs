use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new todo item by asking the user to input the todo content: `t add content`. Please wrap your todo item with double-quotes "".
    Add {
        #[clap(value_parser)]
        todo: String,
    },
    /// Prints out a list of stored todos to the user: `t list`
    List,
    /// Allows the user to delete a todo item by passing it an id: `t delete id`
    Delete {
        #[clap(value_parser)]
        id: u32,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    /// An id is needed so that a single todo item can be retrieved for deletion.
    id: u32,
    content: String,
}

impl Todo {
    fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TodoList {
    todo_list: Vec<Todo>,
}

impl TodoList {
    fn save(&self) -> anyhow::Result<()> {
        let file = File::create("./t.json")?;
        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    fn create(&mut self, content: &str) {
        let max = self.todo_list.iter().map(|todo| todo.id).max().unwrap_or(0);
        self.todo_list.push(Todo::new(max + 1, content.into()));
        self.save().expect("save should not fail");
    }

    fn delete(&mut self, id: u32) {
        self.todo_list.retain(|todo| todo.id != id);
        self.save().expect("save should not fail");
    }
}

impl Display for TodoList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, todo) in self.todo_list.iter().enumerate() {
            write!(f, "{todo}")?;

            if i != self.todo_list.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let json_file = "./t.json";

    let file = File::open(json_file)?;
    let mut todo_list = serde_json::from_reader::<_, TodoList>(file)?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { todo } => {
            todo_list.create(todo);
            println!("{todo}")
        }
        Commands::List => {
            println!("{todo_list}")
        }
        Commands::Delete { id } => {
            todo_list.delete(*id);
        }
    }
    Ok(())
}
