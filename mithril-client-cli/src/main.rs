#![doc = include_str!("../README.md")]

mod commands;

use std::io::Write;
use std::{path::PathBuf, fs::File};
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
    pub fn doc_markdown() -> String {

        // See: https://github1s.com/clap-rs/clap/blob/HEAD/clap_builder/src/builder/command.rs#L1989

        let cmd: clap::Command = <Self as CommandFactory>::command();

        fn format_arg(arg: &Arg) -> String {
            let parameter=arg.get_id();

            let short_option = arg.get_short().map_or("".into(), |c| format!("`-{}`", c));
            let long_option = arg.get_long().map_or("".into(), |c| format!("`--{}`", c));
          //  let env_variable =  format_value_names(arg);
            let env_variable = arg.get_env().map_or("".into(), |s| s.to_os_string().into_string().map_or("".into(), |s| format!("`{}`", s)));
            let description = "?";
            let default_value:Vec<String> = arg.get_default_values().iter().map(|v| format!("{}", v.to_str().map_or("-".into(), |s| format!("`{}`", s)))).collect();
            let default_value = default_value.join(" ");
            let example = arg.get_help().map_or("".into(), |s| s.to_string());
            let is_required = if arg.is_required_set() {":heavy_check_mark:"} else {"-"};
            format!("| `{parameter}` | {long_option} | {short_option} | {env_variable} | {description} | {default_value} | {example} | {is_required} |")
        }

        fn format_parameters(cmd: &Command) -> String {
            if cmd.get_arguments().peekable().peek().is_some() {

                let parameters_lines: Vec<String> = cmd.get_arguments().map(|arg| format_arg(arg)).collect();

                let parameters_table = format!("Here is a list of the available parameters:\n### Configuration parameters\n\n{}\n{}\n{}\n",
                    "| Parameter | Command line (long) |  Command line (short) | Environment variable | Description | Default value | Example | Mandatory |",
                    "|-----------|---------------------|:---------------------:|----------------------|-------------|---------------|---------|:---------:|",
                    parameters_lines.join("\n"),
                );
                let parameters_explanation = format!("\n\
                    The configuration parameters can be set in either of the following ways:\n\
                    \n\
                    1. In a configuration file, depending on the `--run-mode` parameter. If the runtime mode is `testnet`, the file is located in `./conf/testnet.json`.\n\
                    \n\
                    2. The value can be overridden by an environment variable with the parameter name in uppercase.\n\
                    ");
                format!("{}\n{}", parameters_explanation, parameters_table)
            } else {
                String::from("")
            }
        }

        fn format_command(cmd: &Command, parent: Option<String>) -> String {
            let title = format!("### {} {}\n", parent.clone().map_or("".into(), |s| format!("{} ", s)), cmd.get_name());
            let description = format!("{}", cmd.get_about().map_or("".into(), |s| s.to_string()));

            let subcommands_table = if cmd.get_subcommands().peekable().peek().is_some() {
                let subcommands_lines: Vec<String> = cmd.get_subcommands().map(|command| {
                    format!("| **{}** | {} |",
                        command.get_name(),
                        command.get_about().map_or("".into(), |s| s.to_string())
                    )
                }).collect();

                let subcommands_table = format!("{}\n{}\n{}\n",
                    "| Subcommand | Performed action |",
                    "|------------|------------------|",
                    subcommands_lines.join("\n"),
                );

                println!("{}", subcommands_table);
                subcommands_table
            } else {
                String::from("")
            };

            let parameters = format_parameters(&cmd);

            for sub_command in cmd.get_subcommands() {
                let p = Some(parent.clone().map_or("".into(), |s| format!("{} ", s)) + cmd.get_name());
                format_command(sub_command, p);
            }

            let subcommands: Vec<String> = cmd.get_subcommands().map(|sub_command| {
                let p = Some(parent.clone().map_or("".into(), |s| format!("{} ", s)) + cmd.get_name());
                format_command(sub_command, p)
            }).collect();

            format!("{}\n{}\n{}\n{}\n{}", title, description, subcommands_table, parameters, subcommands.join("\n"))

        }

        format_command(&cmd, None)


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
    let doc = Args::doc_markdown();
    let mut buffer: File = File::create("generated_doc.md")?;
    buffer.write(b"Generated doc\n\n")?;
    buffer.write(doc.as_bytes())?;

    // Load args
    let args = Args::parse();
    let _guard = slog_scope::set_global_logger(args.build_logger());

    #[cfg(feature = "bundle_openssl")]
    openssl_probe::init_ssl_cert_env_vars();

    args.execute().await
}
