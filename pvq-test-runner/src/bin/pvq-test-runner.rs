use clap::Parser;
use sp_core::hexdisplay::HexDisplay;
use tracing_subscriber::prelude::*;

use pvq_test_runner::TestRunner;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to the PolkaVM program to execute
    #[arg(short, long)]
    program: std::path::PathBuf,

    /// Chain to use for execution
    #[arg(short, long, value_enum)]
    chain: Chain,

    /// Print test data and expected result without executing the test
    #[arg(long)]
    print_data: bool,

    /// Entrypoint index to use for execution
    #[arg(short, long)]
    entrypoint_idx: u8,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Chain {
    Poc,
    Acala,
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Poc => write!(f, "poc"),
            Chain::Acala => write!(f, "acala"),
        }
    }
}

fn main() {
    let registry = tracing_subscriber::registry();

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    registry
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .try_init()
        .expect("Failed to initialize tracing");

    let cli = Cli::parse();

    let program_str = cli.program.to_string_lossy();
    let input_data = TestRunner::prepare_input_data(&program_str, &cli.chain.to_string(), cli.entrypoint_idx);
    let expected_result = TestRunner::expected_result(&program_str, &cli.chain.to_string(), cli.entrypoint_idx);

    if cli.print_data {
        println!("=== Test Data ===");
        println!("Program: {}", program_str);
        println!("Chain: {}", cli.chain);
        println!("Input data (hex): {}", HexDisplay::from(&input_data));
        println!("Input data (bytes): {:?}", input_data);
        println!("Expected result (hex): {}", HexDisplay::from(&expected_result));
        println!("Expected result (bytes): {:?}", expected_result);
        return;
    }

    let blob = std::fs::read(&cli.program).expect("Failed to read program");

    tracing::info!("Input data: {:?}", input_data);

    let mut runner = TestRunner::new();
    let res = runner.execute_program(&blob, &input_data).unwrap();

    let metadata = pvq_test_runner::extensions::metadata();
    let metadata_json = serde_json::to_string(&metadata).expect("Failed to serialize metadata");
    tracing::info!("Metadata: {}", metadata_json);

    tracing::info!("Result: {:?}", res);
}
