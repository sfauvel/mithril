#![doc = include_str!("../README.md")]

mod commands;

use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand, CommandFactory, Command, Arg};
use config::{builder::DefaultState, ConfigBuilder, Map, Source, Value, ValueKind};
use slog::{Drain, Level, Logger};
use slog_scope::debug;

use mithril_client_cli::common::StdResult;

use commands::{
    mithril_stake_distribution::MithrilStakeDistributionCommands, snapshot::SnapshotCommands,
};

#[derive(Parser, Debug, Clone)]
#[clap(name = "mithril-client")]
#[clap(
    about = "This program shows, downloads and verifies certified blockchain artifacts.",
    long_about = None
)]
#[command(version)]
pub struct Args {
    /// Available commands
    #[clap(subcommand)]
    command: ArtifactCommands,

    /// Run Mode.
    #[clap(long, env = "RUN_MODE", default_value = "dev")]
    run_mode: String,

    /// Verbosity level (-v=warning, -vv=info, -vvv=debug).
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Directory where configuration file is located.
    #[clap(long, default_value = "./config")]
    pub config_directory: PathBuf,

    /// Override configuration Aggregator endpoint URL.
    #[clap(long, env = "AGGREGATOR_ENDPOINT")]
    aggregator_endpoint: Option<String>,
}

impl Args {
    pub fn doc_markdown() {

        // See: https://github1s.com/clap-rs/clap/blob/HEAD/clap_builder/src/builder/command.rs#L1989

        let cmd: clap::Command = <Self as CommandFactory>::command();

        fn format_arg(arg: &Arg) -> String {
            let parameter=arg.get_id();
            let short_option =  arg.get_short().map(|c| format!("`-{}`", c)).unwrap_or(String::from(""));
            let long_option = arg.get_long().map(|c| format!("`--{}`", c)).unwrap_or(String::from(""));
          //  let env_variable =  format_value_names(arg);
            let env_variable = arg.get_env().map(|s| s.to_os_string().into_string().map(|s| format!("`{}`", s)).unwrap_or(String::from(""))).unwrap_or(String::from(""));
            let description = "?";
            let default_value:Vec<String> = arg.get_default_values().iter().map(|v| format!("{}", v.to_str().map(|s| format!("`{}`", s)).unwrap_or(String::from("-")))).collect();
            let default_value = default_value.join(" ");
            let example = arg.get_help().map(|s| s.to_string()).unwrap_or(String::from(""));
            let is_required = if arg.is_required_set() {":heavy_check_mark:"} else {"-"};
            format!("| `{parameter}` | {long_option} | {short_option} | {env_variable} | {description} | {default_value} | {example} | {is_required} |")
        }

        fn format_parameters(cmd: &Command) {
            if cmd.get_arguments().peekable().peek().is_some() {
                println!("### Configuration parameters");

                println!("| Parameter | Command line (long) |  Command line (short) | Environment variable | Description | Default value | Example | Mandatory |");
                println!("|-----------|---------------------|:---------------------:|----------------------|-------------|---------------|---------|:---------:|");
            
                for arg in cmd.get_arguments() {
                    println!("{}", format_arg(arg));
                }
            }
        }

        fn format_command(cmd: &Command, parent: Option<String>)  {
            println!("### {} {}\n", parent.clone().map(|s| format!("{} ", s)).unwrap_or(String::from("")), cmd.get_name());
            

            if cmd.get_subcommands().peekable().peek().is_some() {
            
                println!("{}", String::from("| Subcommand | Performed action |"));
                println!("{}", String::from("|------------|------------------|"));
        
                for command in cmd.get_subcommands() {
                    let doc = command.get_about().map(|s| s.to_string()).unwrap_or(String::from(""));
        
                    println!("| **{}** | {} |",
                        command.get_name(), doc
                    )
                }
                println!("");
            }
            format_parameters(&cmd);

            for sub_command in cmd.get_subcommands() {
                let p = Some(parent.clone().map(|s| format!("{} ", s)).unwrap_or(String::from("")) + cmd.get_name());
                format_command(sub_command, p);
            }
 
        }

        format_command(&cmd, None);


    }

    pub fn document() {

        // See: https://github1s.com/clap-rs/clap/blob/HEAD/clap_builder/src/builder/command.rs#L1989

        let cmd: clap::Command = <Self as CommandFactory>::command();

        fn format_value_names(arg: &Arg) -> String {
            
            fn format_required(arg: &Arg, value: &str) -> String {
                if arg.is_required_set() {
                    format!("<{}>", value.to_string())
                 } else {
                     format!("[{}]", value.to_string())
                 } 
            }

            let formatted_names = arg.get_value_names()
            .map(|v| {
                let b: Vec<String> = v.iter().map(|t| format_required(arg, t)).collect();
                format!("{}", b.join(" "))
            }
             ).unwrap_or(String::from("?"));
            formatted_names
            //format!("{} (postional={}, required={})", formatted_names, arg.is_positional(), arg.is_required_set())
        }

        fn format_arg(arg: &Arg) -> String {
            format!("  {}{} {} {}", 
                arg.get_short().map(|c| format!("-{}, ", c)).unwrap_or(String::from("")),
                arg.get_long().map(|c| format!("--{}", c)).unwrap_or(String::from("")),
                format_value_names(arg),
                arg.get_help().map(|s| s.to_string()).unwrap_or(String::from("")),
            )
        }

        fn format_command(command: &Command) -> String {
           // println!("POSITIONAL ARGUMENTS:");
           // for arg in cmd.get_positionals() {
           //     println!("{}", format_arg(arg));
           // }

            let posargs: Vec<String> = command.get_positionals().map(|arg| format!("{}",format_value_names(arg))).collect();
            let pa = posargs.join(" ");

            let subcmds: Vec<String> = command.get_subcommands().map(|cmd| format!("   {}",format_command(cmd))).collect();
            let s = subcmds.join("\n");

            let args: Vec<String> = command.get_arguments().map(|arg| format!("   {}",format_arg(arg))).collect();
            let a = args.join("\n");
           
            format!("{} {}\n{}\n{}", 
                command.get_name(), pa, a, s
            )
        }



        println!("ARGUMENTS:");
        for arg in cmd.get_arguments() {
            println!("{}", format_arg(arg));
        }

        println!("SUBCOMMANDS:");
        for command in cmd.get_subcommands() {
            println!("{}", format_command(command));
        }

    }

    pub async fn execute(&self) -> StdResult<()> {
        debug!("Run Mode: {}", self.run_mode);
        let filename = format!("{}/{}.json", self.config_directory.display(), self.run_mode);
        debug!("Reading configuration file '{}'.", filename);
        let config: ConfigBuilder<DefaultState> = config::Config::builder()
            .add_source(config::File::with_name(&filename).required(false))
            .add_source(self.clone())
            .set_default("download_dir", "")?;

        self.command.execute(config).await
    }

    fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::Error,
            1 => Level::Warning,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }
    }

    fn build_logger(&self) -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog::LevelFilter::new(drain, self.log_level()).fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        Logger::root(Arc::new(drain), slog::o!())
    }
}

impl Source for Args {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, config::ConfigError> {
        let mut map = Map::new();
        let namespace = "clap arguments".to_string();

        if let Some(aggregator_endpoint) = self.aggregator_endpoint.clone() {
            map.insert(
                "aggregator_endpoint".to_string(),
                Value::new(Some(&namespace), ValueKind::from(aggregator_endpoint)),
            );
        }

        Ok(map)
    }
}

#[derive(Subcommand, Debug, Clone)]
enum ArtifactCommands {
    #[clap(subcommand)]
    Snapshot(SnapshotCommands),

    #[clap(subcommand, alias("msd"))]
    MithrilStakeDistribution(MithrilStakeDistributionCommands),
}

impl ArtifactCommands {
    pub async fn execute(&self, config_builder: ConfigBuilder<DefaultState>) -> StdResult<()> {
        match self {
            Self::Snapshot(cmd) => cmd.execute(config_builder).await,
            Self::MithrilStakeDistribution(cmd) => cmd.execute(config_builder).await,
        }
    }
}

#[tokio::main]
async fn main() -> StdResult<()> {
    //Args::document();
    Args::doc_markdown();
    // Load args
    let args = Args::parse();
    let _guard = slog_scope::set_global_logger(args.build_logger());

    #[cfg(feature = "bundle_openssl")]
    openssl_probe::init_ssl_cert_env_vars();

    args.execute().await
}
