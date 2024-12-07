use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Add {
        id: String,
        password: String,
        description: String,
    },
    List,
    Search {
        id: String,
    },
    Delete {
        id: String,
    },
}