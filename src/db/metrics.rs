// `DbOperationTimer` is only reached through the `db_metric!` macro, so it stays unreferenced
// until that macro is used to instrument a query.
#![allow(dead_code)]

/// Database metrics collection utilities
use std::time::Instant;

#[cfg(feature = "enable_metrics")]
use crate::CONFIG;

/// Database operation tracker for metrics
pub struct DbOperationTimer {
    start_time: Instant,
    operation: String,
}

impl DbOperationTimer {
    pub fn new(operation: &str) -> Self {
        Self {
            start_time: Instant::now(),
            operation: operation.to_owned(),
        }
    }

    pub fn finish(self) {
        let duration = self.start_time.elapsed();
        crate::metrics::observe_db_query_duration(&self.operation, duration.as_secs_f64());
    }
}

/// Macro to instrument database operations
#[macro_export]
macro_rules! db_metric {
    ($operation:expr, $code:block) => {{
        #[cfg(feature = "enable_metrics")]
        let timer = $crate::db::metrics::DbOperationTimer::new($operation);

        let result = $code;

        #[cfg(feature = "enable_metrics")]
        timer.finish();

        result
    }};
}

#[cfg(feature = "enable_metrics")]
fn db_type_name(db_type: &crate::db::DbConnType) -> &'static str {
    match db_type {
        #[cfg(mysql)]
        crate::db::DbConnType::Mysql => "mysql",
        #[cfg(postgresql)]
        crate::db::DbConnType::Postgresql => "postgresql",
        #[cfg(sqlite)]
        crate::db::DbConnType::Sqlite => "sqlite",
    }
}

/// Publish the connection pool gauges from the live pool.
///
/// r2d2 keeps no idle counter, so only the connections currently checked out and the configured
/// pool ceiling are reported. Anything else would have to be invented.
#[cfg(feature = "enable_metrics")]
pub fn update_pool_metrics(pool: &crate::db::DbPool) {
    crate::metrics::update_db_connections(
        crate::db::ACTIVE_DB_TYPE.get().map_or("unknown", db_type_name),
        i64::try_from(pool.connections_in_use()).unwrap_or(i64::MAX),
        i64::from(CONFIG.database_max_conns()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_db_operation_timer() {
        let timer = DbOperationTimer::new("test_query");
        thread::sleep(Duration::from_millis(1));
        timer.finish();
    }
}
