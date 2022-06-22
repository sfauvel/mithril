#![allow(dead_code)]
use async_trait::async_trait;
use nom::IResult;
use std::error::Error;
use std::path::PathBuf;
use tokio::process::Command;

use crate::chain_observer::interface::*;
use crate::entities::{Epoch, StakeDistribution};

#[async_trait]
pub trait CliRunner {
    async fn launch_stake_distribution(&self) -> Result<String, Box<dyn Error + Sync + Send>>;
    async fn launch_epoch(&self) -> Result<String, Box<dyn Error + Sync + Send>>;
}
struct CardanoCliRunner {
    cli_path: PathBuf,
    socket_path: PathBuf,
}

impl CardanoCliRunner {
    pub fn new(cli_path: PathBuf, socket_path: PathBuf) -> Self {
        Self {
            cli_path,
            socket_path,
        }
    }

    fn get_command(&self) -> Command {
        let mut command = Command::new(&self.cli_path);
        command.env(
            "CARDANO_NODE_SOCKET_PATH",
            self.socket_path.to_string_lossy().as_ref(),
        );

        command
    }
}

#[async_trait]
impl CliRunner for CardanoCliRunner {
    async fn launch_stake_distribution(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
        let output = self
            .get_command()
            .arg("query")
            .arg("stake-distribution")
            .output()
            .await?;

        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }

    async fn launch_epoch(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
        todo!()
    }
}

pub struct CardanoCliChainObserver {
    cli_runner: Box<dyn CliRunner + Send + Sync>,
}

impl CardanoCliChainObserver {
    pub fn new(cli_runner: Box<dyn CliRunner + Send + Sync>) -> Self {
        Self { cli_runner }
    }

    // This is the only way I found to tell the compiler the correct types
    // and lifetimes for the function `double`.
    fn parse_string<'a>(&'a self, string: &'a str) -> IResult<&str, f64> {
        nom::number::complete::double(string)
    }
}

#[async_trait]
impl ChainObserver for CardanoCliChainObserver {
    async fn get_current_epoch(&self) -> Result<Option<Epoch>, ChainObserverError> {
        todo!()
    }

    async fn get_current_stake_distribution(
        &self,
    ) -> Result<Option<StakeDistribution>, ChainObserverError> {
        let output = self
            .cli_runner
            .launch_stake_distribution()
            .await
            .map_err(ChainObserverError::General)?;
        let mut stake_distribution = StakeDistribution::new();

        for (num, line) in output.lines().enumerate() {
            let words: Vec<&str> = line.split_ascii_whitespace().collect();

            if num < 3 || words.len() != 2 {
                continue;
            }

            if let Ok((_, f)) = self.parse_string(words[1]) {
                let stake: u64 = (f * 1_000_000_000.0).round() as u64;
                // TODO: the stake distribution shall not be indexed by position
                // use the real poolId instead, for now this must be a u32.
                //
                // The position is num - 2 since we ignore the first two lines
                // of the CLI output.
                if stake > 0 {
                    let _ = stake_distribution.insert(num as u64 - 2, stake);
                }
            }
        }

        Ok(Some(stake_distribution))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCliRunner {}

    #[async_trait]
    impl CliRunner for TestCliRunner {
        async fn launch_stake_distribution(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
            let output = r#"
                           PoolId                                 Stake frac
------------------------------------------------------------------------------
pool1qqyjr9pcrv97gwrueunug829fs5znw6p2wxft3fvqkgu5f4qlrg   2.493e-3
pool1qqfnw2fwajdnam7xsqhhrje5cgd8jcltzfrx655rd23eqlxjfef   2.164e-5
pool1qqnjh80kudcjphrxftj74x22q3a4uvw8wknlxptgs7gdqtstqad   8.068e-7
pool1qquwwu6680fr72y4779r2kpc7mxtch8rp2uhuqcc7v9p6q4f7ph   7.073e-7
pool1qpqvz90w7qsex2al2ejjej0rfgrwsguch307w8fraw7a7adf6g8   2.474e-11
pool1qptl80vq84xm28pt3t2lhpfzqag28csjhktxz5k6a74n260clmt   5.600e-7
pool1qpuckgzxwgdru9vvq3ydmuqa077ur783yn2uywz7zq2c29p506e   5.161e-5
pool1qz2vzszautc2c8mljnqre2857dpmheq7kgt6vav0s38tvvhxm6w   1.051e-6
            "#;

            Ok(output.to_string())
        }

        async fn launch_epoch(&self) -> Result<String, Box<dyn Error + Sync + Send>> {
            todo!()
        }
    }
    #[tokio::test]
    async fn test_get_current_stake_distribution() {
        let observer = CardanoCliChainObserver::new(Box::new(TestCliRunner {}));
        let results = observer
            .get_current_stake_distribution()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(7, results.len());
        assert_eq!(2_493_000, *results.get(&1).unwrap());
        assert_eq!(1_051, *results.get(&8).unwrap());
        assert!(results.get(&5).is_none());
    }
}
