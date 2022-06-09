use mithril_common::crypto_helper::Bytes;
use mithril_common::entities::{Beacon, Certificate, CertificatePending};
use mithril_common::fake_data;

use crate::dependency::{BeaconStoreWrapper, MultiSignerWrapper, SnapshotStoreWrapper};
use crate::dependency::{CertificatePendingStoreWrapper, CertificateStoreWrapper};
use crate::snapshot_uploaders::{SnapshotLocation, SnapshotUploader};
use crate::{BeaconStore, Snapshotter};

use super::{AggregatorRunnerTrait, RuntimeError};
use chrono::{DateTime, Utc};
use hex::ToHex;
use mithril_common::entities::Snapshot;
use slog_scope::{debug, error, info, trace, warn};
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[cfg(test)]
use mockall::automock;

#[derive(Clone, Debug, PartialEq)]
pub struct IdleState {
    current_beacon: Option<Beacon>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SigningState {
    current_beacon: Beacon,
    certificate_pending: CertificatePending,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AggregatorState {
    Idle(IdleState),
    Signing(SigningState),
}

impl Display for AggregatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AggregatorState::Idle(_) => write!(f, "idle"),
            AggregatorState::Signing(_) => write!(f, "signing"),
        }
    }
}

/// AggregatorRuntime
pub struct AggregatorRuntime {
    /// the internal state of the automate
    state: AggregatorState,

    /// time between each state machine execution
    state_sleep: Duration,

    /// specific runner for this state machine
    runner: Arc<dyn AggregatorRunnerTrait>,
}

impl AggregatorRuntime {
    pub fn get_state(&self) -> String {
        self.state.to_string()
    }

    pub async fn new(
        state_sleep: Duration,
        init_state: Option<AggregatorState>,
        runner: Arc<dyn AggregatorRunnerTrait>,
    ) -> Result<Self, RuntimeError> {
        info!("initializing runtime");

        let state = if init_state.is_none() {
            trace!("idle state, no current beacon");
            AggregatorState::Idle(IdleState {
                current_beacon: None,
            })
        } else {
            trace!("got initial state from caller");
            init_state.unwrap()
        };

        Ok::<Self, RuntimeError>(Self {
            state_sleep,
            state,
            runner,
        })
    }

    pub async fn run(&mut self) {
        info!("Starting runtime");
        loop {
            if let Err(e) = self.cycle().await {
                error!("{:?}", e)
            }

            info!("Sleeping…");
            sleep(self.state_sleep).await;
        }
    }

    pub async fn cycle(&mut self) -> Result<(), RuntimeError> {
        match self.state.clone() {
            AggregatorState::Idle(state) => {
                if let Some(beacon) = self.runner.is_new_beacon(state.current_beacon.as_ref())? {
                    let new_state = self.from_idle_to_signing(beacon).await?;
                    self.state = AggregatorState::Signing(new_state);
                }
            }
            AggregatorState::Signing(_state) => {}
        }
        Ok(())
    }

    /// transition
    /// from IDLE state to SIGNING
    async fn from_idle_to_signing(
        &mut self,
        new_beacon: Beacon,
    ) -> Result<SigningState, RuntimeError> {
        info!("transiting from IDLE to SIGNING state");
        let digester_result = self.runner.compute_digest(&new_beacon).await?;
        let certificate = self
            .runner
            .create_pending_certificate(digester_result)
            .await?;
        let state = SigningState {
            current_beacon: new_beacon,
            certificate_pending: certificate,
        };

        Ok(state)
    }
}
/// AggregatorRuntime factory
// TODO: Fix this by implementing an Aggregator Config that implements the From trait for a general Config
/*
       pub fn new(
           interval: u32,
           network: String,
           db_directory: PathBuf,
           snapshot_directory: PathBuf,
           beacon_store: BeaconStoreWrapper,
           multi_signer: MultiSignerWrapper,
           snapshot_store: SnapshotStoreWrapper,
           snapshot_uploader: Box<dyn SnapshotUploader>,
           certificate_pending_store: CertificatePendingStoreWrapper,
           certificate_store: CertificateStoreWrapper,
       ) -> Self {
           Self {
               interval,
               network,
               db_directory,
               snapshot_directory,
               beacon_store,
               multi_signer,
               snapshot_store,
               snapshot_uploader,
               certificate_pending_store,
               certificate_store,
           }
       }
    /// Run snapshotter loop
    pub async fn run(&self) {
        info!("Starting runtime");

        loop {
            if let Err(e) = self.do_work().await {
                error!("{:?}", e)
            }

            info!("Sleeping for {}", self.interval);
            sleep(Duration::from_millis(self.interval.into())).await;
        }
    }

    async fn do_work(&self) -> Result<(), RuntimeError> {
        let snapshotter =
            Snapshotter::new(self.db_directory.clone(), self.snapshot_directory.clone());
        let digester = ImmutableDigester::new(self.db_directory.clone(), slog_scope::logger());

        info!("Computing digest"; "db_directory" => self.db_directory.display());
        let digest_result = tokio::task::spawn_blocking(move || digester.compute_digest())
            .await
            .map_err(|e| RuntimeError::General(e.to_string()))?;
        match digest_result {
            Ok(digest_result) => {
                let mut beacon = fake_data::beacon();
                beacon.immutable_file_number = digest_result.last_immutable_file_number;
                let message = &digest_result.digest.clone().into_bytes();

                match self.manage_trigger_snapshot(message, &beacon).await {
                    Ok(Some(certificate)) => {
                        info!(
                            "Snapshotting immutables up to `{}` in an archive",
                            &beacon.immutable_file_number
                        );

                        let snapshot_name =
                            format!("{}.{}.tar.gz", self.network, &digest_result.digest);

                        let snapshot_path = tokio::task::spawn_blocking(
                            move || -> Result<PathBuf, SnapshotError> {
                                snapshotter.snapshot(&snapshot_name)
                            },
                        )
                        .await
                        .map_err(|e| RuntimeError::General(e.to_string()))??;

                        info!("Uploading snapshot archive");
                        let uploaded_snapshot_location = self
                            .snapshot_uploader
                            .upload_snapshot(&snapshot_path)
                            .await
                            .map_err(RuntimeError::SnapshotUploader)?;

                        info!(
                            "Snapshot archive uploaded, location: `{}`",
                            &uploaded_snapshot_location
                        );

                        let new_snapshot = build_new_snapshot(
                            digest_result.digest,
                            certificate.hash.to_owned(),
                            &snapshot_path,
                            uploaded_snapshot_location,
                        )?;

                        info!("Storing snapshot data"; "snapshot" => format!("{:?}", new_snapshot));
                        let mut snapshot_store = self.snapshot_store.write().await;
                        snapshot_store.add_snapshot(new_snapshot).await?;

                        info!("Storing certificate data"; "certificate" => format!("{:?}", certificate));
                        let mut certificate_store = self.certificate_store.write().await;
                        certificate_store
                            .save(certificate)
                            .await
                            .map_err(|e| RuntimeError::CertificateStore(e.to_string()))?;

                        Ok(())
                    }
                    Ok(None) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                let mut beacon_store = self.beacon_store.write().await;
                beacon_store.reset_current_beacon().await?;
                Err(RuntimeError::Digester(err))
            }
        }
    }

    async fn manage_trigger_snapshot(
        &self,
        message: &Bytes,
        beacon: &Beacon,
    ) -> Result<Option<Certificate>, RuntimeError> {
        let mut multi_signer = self.multi_signer.write().await;
        match multi_signer.get_multi_signature().await {
            Ok(None) => {
                {
                    let mut beacon_store = self.beacon_store.write().await;
                    beacon_store.set_current_beacon(beacon.clone()).await?;
                }
                multi_signer
                    .update_current_message(message.to_owned())
                    .await?;
                match multi_signer.create_multi_signature().await {
                    Ok(Some(_)) => {
                        let message = multi_signer
                            .get_current_message()
                            .await
                            .unwrap()
                            .encode_hex::<String>();
                        debug!(
                            "A multi signature has been created for message: {}",
                            message
                        );
                        let previous_hash = "".to_string();
                        Ok(multi_signer
                            .create_certificate(beacon.clone(), previous_hash)
                            .await?)
                    }
                    Ok(None) => {
                        warn!("Not ready to create a multi signature: quorum is not reached yet");
                        Ok(None)
                    }
                    Err(e) => {
                        warn!("Error while creating a multi signature: {}", e);
                        Err(RuntimeError::MultiSigner(e))
                    }
                }
            }
            Ok(_) => {
                let mut beacon_store = self.beacon_store.write().await;
                beacon_store.reset_current_beacon().await?;
                Ok(None)
            }
            Err(err) => {
                let mut beacon_store = self.beacon_store.write().await;
                beacon_store.reset_current_beacon().await?;
                Err(RuntimeError::MultiSigner(err))
            }
        }
    }
}
    */

fn build_new_snapshot(
    digest: String,
    certificate_hash: String,
    snapshot_filepath: &Path,
    uploaded_snapshot_location: SnapshotLocation,
) -> Result<Snapshot, RuntimeError> {
    let timestamp: DateTime<Utc> = Utc::now();
    let created_at = format!("{:?}", timestamp);

    let mut tar_gz = File::open(&snapshot_filepath)?;
    let size: u64 = tar_gz.seek(SeekFrom::End(0))?;

    Ok(Snapshot::new(
        digest,
        certificate_hash,
        size,
        created_at,
        vec![uploaded_snapshot_location],
    ))
}

#[cfg(test)]
mod tests {
    use super::super::runner::MockAggregatorRunner;
    use super::*;
    use mithril_common::digesters::DigesterResult;
    use mithril_common::fake_data;

    async fn init_runtime(
        init_state: Option<AggregatorState>,
        runner: MockAggregatorRunner,
    ) -> AggregatorRuntime {
        AggregatorRuntime::new(Duration::from_millis(100), init_state, Arc::new(runner))
            .await
            .unwrap()
    }

    #[tokio::test]
    pub async fn idle_check_no_new_beacon_with_current_beacon() {
        let mut runner = MockAggregatorRunner::new();
        runner
            .expect_is_new_beacon()
            .times(1)
            .returning(|_| Ok(None));
        let mut runtime = init_runtime(
            Some(AggregatorState::Idle(IdleState {
                current_beacon: Some(fake_data::beacon()),
            })),
            runner,
        )
        .await;

        let _ = runtime.cycle().await.unwrap();
        assert_eq!("idle".to_string(), runtime.get_state());
    }

    #[tokio::test]
    pub async fn idle_check_no_new_beacon_with_no_current_beacon() {
        let mut runner = MockAggregatorRunner::new();
        runner
            .expect_is_new_beacon()
            .times(1)
            .returning(|_| Ok(Some(fake_data::beacon())));
        runner.expect_compute_digest().times(1).returning(|_| {
            Ok(DigesterResult {
                digest: "whatever".to_string(),
                last_immutable_file_number: 123,
            })
        });
        runner
            .expect_create_pending_certificate()
            .times(1)
            .returning(|_| Ok(fake_data::certificate_pending()));
        let mut runtime = init_runtime(
            Some(AggregatorState::Idle(IdleState {
                current_beacon: None,
            })),
            runner,
        )
        .await;

        let _ = runtime.cycle().await.unwrap();
        assert_eq!("signing".to_string(), runtime.get_state());
    }
}
