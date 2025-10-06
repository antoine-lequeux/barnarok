use barnarok::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "barnarok")]
#[command(about = "A Rust chess engine", long_about = None)]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands
{
    Explore
    {
        #[arg(short, long)]
        depth: usize,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        fen: Option<String>,
    },
    Play
    {
        #[arg(short, long)]
        wstrat: String,
        #[arg(short, long)]
        bstrat: String,
    },
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command
    {
        Commands::Explore { depth, verbose, fen } =>
        {
            let board_res = match fen
            {
                Some(fen_content) => Board::from_fen(fen_content),
                None => Board::new(),
            };
            match board_res
            {
                Ok(mut board) =>
                {
                    board.display();
                    println!(
                        "number of reachable positions after {} half-moves: {}",
                        depth,
                        launch_explore(&mut board, *depth, *verbose)
                    );
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Commands::Play { wstrat, bstrat } => match play(*&wstrat.as_str(), *&bstrat.as_str())
        {
            Ok(result) => match result
            {
                GameResult::White => println!("White wins by checkmate."),
                GameResult::Black => println!("Black wins by checkmate."),
                GameResult::Stalemate => println!("The game ends in a draw by stalemate."),
            },
            Err(err) => eprintln!("{}", err),
        },
    }
}
