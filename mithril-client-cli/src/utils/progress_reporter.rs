use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget};
use mithril_common::StdResult;
use slog_scope::warn;
use std::{
    ops::Deref,
    sync::RwLock,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Output type of a [ProgressPrinter] or a [DownloadProgressReporter]
pub enum ProgressOutputType {
    /// Output to json
    JsonReporter,
    /// Output to tty
    TTY,
    /// No output
    Hidden,
}

impl From<ProgressOutputType> for ProgressDrawTarget {
    fn from(value: ProgressOutputType) -> Self {
        match value {
            ProgressOutputType::JsonReporter => ProgressDrawTarget::hidden(),
            ProgressOutputType::TTY => ProgressDrawTarget::stdout(),
            ProgressOutputType::Hidden => ProgressDrawTarget::hidden(),
        }
    }
}

/// Wrapper of a indicatif [MultiProgress] to allow reporting to json.
pub struct ProgressPrinter {
    multi_progress: MultiProgress,
    output_type: ProgressOutputType,
    number_of_steps: u16,
}

impl ProgressPrinter {
    /// Instanciate a new progress printer
    pub fn new(output_type: ProgressOutputType, number_of_steps: u16) -> Self {
        Self {
            multi_progress: MultiProgress::with_draw_target(output_type.into()),
            output_type,
            number_of_steps,
        }
    }

    /// Report the current step
    pub fn report_step(&self, step_number: u16, text: &str) -> StdResult<()> {
        match self.output_type {
            ProgressOutputType::JsonReporter => println!(
                r#"{{"step_num": {step_number}, "total_steps": {}, "message": "{text}"}}"#,
                self.number_of_steps
            ),
            ProgressOutputType::TTY => self
                .multi_progress
                .println(format!("{step_number}/{} - {text}", self.number_of_steps))?,
            ProgressOutputType::Hidden => (),
        };

        Ok(())
    }
}

impl Deref for ProgressPrinter {
    type Target = MultiProgress;

    fn deref(&self) -> &Self::Target {
        &self.multi_progress
    }
}

/// Wrapper of a indicatif [ProgressBar] to allow reporting to json.
pub struct DownloadProgressReporter {
    progress_bar: ProgressBar,
    output_type: ProgressOutputType,
    last_json_report_instant: RwLock<Option<Instant>>,
}

impl DownloadProgressReporter {
    /// Instanciate a new progress reporter
    pub fn new(progress_bar: ProgressBar, output_type: ProgressOutputType) -> Self {
        Self {
            progress_bar,
            output_type,
            last_json_report_instant: RwLock::new(None),
        }
    }

    /// Report the current progress
    pub fn report(&self, actual_position: u64) {
        self.progress_bar.set_position(actual_position);

        if let ProgressOutputType::JsonReporter = self.output_type {
            let should_report = match self.get_remaining_time_since_last_json_report() {
                Some(remaining_time) => remaining_time > Duration::from_millis(333),
                None => true,
            };

            if should_report {
                println!(
                    r#"{{ "bytesDownloaded": {}, "bytesTotal": {}, "secondsLeft": {}.{}, "secondsElapsed": {}.{} }}"#,
                    self.progress_bar.position(),
                    self.progress_bar.length().unwrap_or(0),
                    self.progress_bar.eta().as_secs(),
                    self.progress_bar.eta().subsec_millis(),
                    self.progress_bar.elapsed().as_secs(),
                    self.progress_bar.elapsed().subsec_millis(),
                );

                match self.last_json_report_instant.write() {
                    Ok(mut instant) => *instant = Some(Instant::now()),
                    Err(error) => {
                        warn!("failed to update last json report instant, error: {error:?}")
                    }
                };
            }
        };
    }

    fn get_remaining_time_since_last_json_report(&self) -> Option<Duration> {
        match self.last_json_report_instant.read() {
            Ok(instant) => (*instant).map(|instant| instant.elapsed()),
            Err(_) => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use chrono::{Utc, NaiveDate};
    use serde_json::json;

    use super::*;

    use std::thread::{sleep, self};

    #[test]
    fn progress_printer_tty() {
        let progress_bar: ProgressPrinter = ProgressPrinter::new(ProgressOutputType::TTY, 6);

        let _ = progress_bar.report_step(1, "First step")
            .and(progress_bar.report_step(2, "Second step"))
            .and(progress_bar.report_step(3, "Third step"));
        
        // Note that the output is merge with the output of other test using TTY. 
        // There is probable a flush that not done.
        // What is strange, is that first steps are displayed together then second steps.
        // This is because rust run test in parallel. 
        // To avoid that add the option: --test-thread=1
    }

    #[test]
    fn progress_printer_json() {
        let progress_bar: ProgressPrinter = ProgressPrinter::new(ProgressOutputType::JsonReporter, 6);
        
        let _ = progress_bar.report_step(1, "First step")
            .and(progress_bar.report_step(2, "Second step"))
            .and(progress_bar.report_step(3, "Third step"));
    }

    #[test]
    fn progress_printer_tty_step_out_of_bound() {
        let progress_bar: ProgressPrinter = ProgressPrinter::new(ProgressOutputType::TTY, 6);

        let _ = progress_bar.report_step(1, "First step X")
            .and(progress_bar.report_step(2, "Second step X"))
            .and(progress_bar.report_step(10, "Step out of bound"));

        // It displays 10/6 - Step out of bound, there is no check on bounds.
    }

    #[test]
    fn progress_printer_tty_with_same_step_several_times() {
        let progress_bar: ProgressPrinter = ProgressPrinter::new(ProgressOutputType::TTY, 6);

        let _ = progress_bar.report_step(1, "First step 1")
            .and(progress_bar.report_step(1, "Second step 1"))
            .and(progress_bar.report_step(2, "First step 2"));

        // The two steps are displayed, there is no check done on that.
    }

    #[test]
    fn download_progress_reporter_check() {
        let progress_bar = ProgressBar::new(10);
        let download_progress_bar: DownloadProgressReporter = DownloadProgressReporter::new(progress_bar, ProgressOutputType::JsonReporter);
        println!("");
        download_progress_bar.report(2);
        sleep(Duration::from_millis(500));
        download_progress_bar.report(3);
        sleep(Duration::from_millis(500));
        download_progress_bar.report(4);
        sleep(Duration::from_millis(500));


        // We can see the progress bar with sleep.
        // Othrwise, the progress bar disappeared at the end of the test.
        // It may be overwrite by something else
    }

    #[test]
    fn download_progress_reporter_json_with_hidden_progress_bar() {
        let progress_bar = ProgressBar::hidden();
        progress_bar.set_length(10);
        let download_progress_bar: DownloadProgressReporter = DownloadProgressReporter::new(progress_bar, ProgressOutputType::JsonReporter);
        println!("");
        download_progress_bar.report(6);
        sleep(Duration::from_millis(500));
        download_progress_bar.report(7);
        sleep(Duration::from_millis(500));
        download_progress_bar.report(8);
        sleep(Duration::from_millis(500));

    }    

    #[test]
    fn progress_bar_eta() {
        let pos = 1;
        {
            let progress_bar = ProgressBar::new(pos * 2);
            thread::sleep(Duration::from_millis(123));
            progress_bar.set_position(pos);
            assert_eq!("0.123", format!("{}.{:0>3}", progress_bar.eta().as_secs(), progress_bar.eta().subsec_millis(),));
        }
        {
            let progress_bar = ProgressBar::new(pos * 2);
            thread::sleep(Duration::from_millis(1));
            progress_bar.set_position(pos);
            assert_eq!("0.001", format!("{}.{:0>3}", progress_bar.eta().as_secs(), progress_bar.eta().subsec_millis(),));
        }
    }

    #[test]
    fn format_json() {
        let json = format!(
            r#"{{ ""secondsLeft": {}.{} }}"#,
            42,
            687
        );
        assert_eq!(r#"{ ""secondsLeft": 42.687 }"#, json);
    }
    
    #[test]
    fn format_json_decimal_less_than_100() {
        let json = format!(
            r#"{{ ""secondsLeft": {}.{:0>3} }}"#,
            42,
            007
        );
        assert_eq!(r#"{ ""secondsLeft": 42.007 }"#, json);
    }

    #[test]
    fn build_json() {
        assert_eq!(r#"{"timestamp":"2023-12-13T15:34:04.818350196+00:00"}"#, json!(
            {
                "timestamp": NaiveDate::from_ymd_opt(2023, 12, 13).unwrap().and_hms_nano_opt(15, 34, 4, 818350196).unwrap().and_local_timezone(Utc).unwrap().to_rfc3339(),
            }).to_string());
        
        assert_eq!(r#"{"bytes_total":100}"#, json!(
            {
                "bytes_total": 100,
            }).to_string());


        assert_eq!(r#"{"seconds_left":5.325}"#, json!(
            {
                "seconds_left": format!("{}.{}", 5, 325).parse::<f64>().unwrap(), // It's probably wrong because 
            }).to_string());

            assert_eq!(r#"{"seconds_left":0.1}"#, json!(
                {
                    "seconds_left": "0.1".parse::<f64>().unwrap(),
                }).to_string());

                
            let five_seconds = Duration::new(5, 123);
            assert_eq!(r#"{"seconds_left":{"nanos":123,"secs":5}}"#, json!(
                {
                    "seconds_left": five_seconds,
                }).to_string());

    }
}
