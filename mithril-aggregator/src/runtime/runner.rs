use std::path::PathBuf;

use crate::DependencyManager;
use async_trait::async_trait;
use mithril_common::digesters::{Digester, DigesterResult, ImmutableDigester};
use mithril_common::entities::{Beacon, CertificatePending};

#[allow(unused_imports)]
use slog_scope::{debug, error, info, trace, warn};
use std::path::Path;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

use super::RuntimeError;
pub struct AggregatorConfig {
    /// Interval between each snapshot, in seconds
    pub interval: u32,

    /// Cardano network
    pub network: String,

    /// DB directory to snapshot
    pub db_directory: PathBuf,

    /// Directory to store snapshot
    pub snapshot_directory: PathBuf,

    pub dependencies: Arc<DependencyManager>,
}

impl AggregatorConfig {
    pub fn new(
        interval: u32,
        network: &str,
        db_directory: &Path,
        snapshot_directory: &Path,
        dependencies: Arc<DependencyManager>,
    ) -> Self {
        Self {
            interval,
            network: network.to_string(),
            db_directory: db_directory.to_path_buf(),
            snapshot_directory: snapshot_directory.to_path_buf(),
            dependencies,
        }
    }
}

#[async_trait]
pub trait AggregatorRunnerTrait: Sync + Send {
    /// Return the current beacon if it is newer than the given one.
    fn is_new_beacon(&self, beacon: Option<&Beacon>) -> Result<Option<Beacon>, RuntimeError>;
    async fn compute_digest(&self, new_beacon: &Beacon) -> Result<DigesterResult, RuntimeError>;
    async fn create_pending_certificate(
        &self,
        digester_result: DigesterResult,
    ) -> Result<CertificatePending, RuntimeError>;
}

pub struct AggregatorRunner {
    config: AggregatorConfig,
}

impl AggregatorRunner {
    pub fn new(config: AggregatorConfig) -> Self {
        Self { config }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
impl AggregatorRunnerTrait for AggregatorRunner {
    /// Is there a new beacon?
    /// returns a new beacon if there is one more recent than the given one
    fn is_new_beacon<'a>(
        &self,
        beacon: Option<&'a Beacon>,
    ) -> Result<Option<Beacon>, RuntimeError> {
        info!("checking if there is a new beacon");
        warn!("using fake data for the new beacon");
        let current_beacon = mithril_common::fake_data::beacon();

        if beacon.is_none() || current_beacon > *beacon.unwrap() {
            Ok(Some(current_beacon))
        } else {
            Ok(None)
        }
    }

    async fn compute_digest(&self, new_beacon: &Beacon) -> Result<DigesterResult, RuntimeError> {
        trace!("running runner::compute_digester");
        let digester =
            ImmutableDigester::new(self.config.db_directory.clone(), slog_scope::logger());
        info!("Computing digest"; "db_directory" => self.config.db_directory.display());
        let digest_result = tokio::task::spawn_blocking(move || digester.compute_digest())
            .await
            .map_err(|e| RuntimeError::General(e.to_string()))??;

        if digest_result.last_immutable_file_number != new_beacon.immutable_file_number {
            error!("digest beacon is different than the given beacon");
            Err(RuntimeError::General(
                format!("The digest has been computed for a different immutable ({}) file than the one given in the beacon ({}).", digest_result.last_immutable_file_number, new_beacon.immutable_file_number)
            ))
        } else {
            trace!("digest last immutable file number and new beacon file number are consistent");
            Ok(digest_result)
        }
    }

    async fn create_pending_certificate(
        &self,
        digester_result: DigesterResult,
    ) -> Result<CertificatePending, RuntimeError> {
        todo!()
    }
}
