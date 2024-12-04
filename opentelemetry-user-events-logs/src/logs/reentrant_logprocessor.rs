use std::fmt::Debug;

use opentelemetry_sdk::logs::LogResult;

#[cfg(feature = "spec_unstable_logs_enabled")]
use opentelemetry_sdk::export::logs::LogExporter;

use crate::logs::exporter::*;

/// This export processor exports without synchronization.
/// This is currently only used in users_event exporter, where we know
/// that the underlying exporter is safe under concurrent calls

#[derive(Debug)]
pub struct ReentrantLogProcessor {
    event_exporter: UserEventsExporter,
}

impl ReentrantLogProcessor {
    /// constructor that accepts an exporter instance
    pub fn new(exporter: UserEventsExporter) -> Self {
        ReentrantLogProcessor {
            event_exporter: exporter,
        }
    }
}

impl opentelemetry_sdk::logs::LogProcessor for ReentrantLogProcessor {
    fn emit(
        &self,
        record: &mut opentelemetry_sdk::logs::LogRecord,
        instrumentation: &opentelemetry::InstrumentationScope,
    ) {
        _ = self.event_exporter.export_log_data(record, instrumentation);
    }

    // This is a no-op as this processor doesn't keep anything
    // in memory to be flushed out.
    fn force_flush(&self) -> LogResult<()> {
        Ok(())
    }

    // This is a no-op no special cleanup is required before
    // shutdown.
    fn shutdown(&self) -> LogResult<()> {
        Ok(())
    }

    #[cfg(feature = "spec_unstable_logs_enabled")]
    fn event_enabled(
        &self,
        level: opentelemetry::logs::Severity,
        target: &str,
        name: &str,
    ) -> bool {
        self.event_exporter.event_enabled(level, target, name)
    }
}
