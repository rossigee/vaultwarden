#[cfg(feature = "enable_metrics")]
mod metrics_enabled_tests {
    use crate::metrics::*;

    #[test]
    fn test_http_metrics_collection() {
        increment_http_requests("GET", "/api/sync", 200);
        increment_http_requests("POST", "/api/accounts/register", 201);
        increment_http_requests("GET", "/api/sync", 500);

        observe_http_request_duration("GET", "/api/sync", 0.150);
        observe_http_request_duration("POST", "/api/accounts/register", 0.300);

        let metrics_text = gather_metrics().unwrap();
        assert!(metrics_text.contains("vaultwarden_http_requests_total"), "{metrics_text}");
        assert!(metrics_text.contains("vaultwarden_http_request_duration_seconds"), "{metrics_text}");
    }

    #[test]
    fn test_database_metrics_collection() {
        update_db_connections("sqlite", 5, 10);
        update_db_connections("postgresql", 8, 2);

        observe_db_query_duration("select", 0.025);
        observe_db_query_duration("insert", 0.045);
        observe_db_query_duration("update", 0.030);

        let metrics_text = gather_metrics().unwrap();
        assert!(metrics_text.contains("vaultwarden_db_query_duration_seconds"), "{metrics_text}");
    }

    #[test]
    fn test_database_connection_gauges_are_exported() {
        // A dedicated label keeps the other tests in this module, which share the global
        // registry and reuse "sqlite", from racing with the values asserted here.
        update_db_connections("gauges_unittest", 3, 10);

        let metrics_text = gather_metrics().unwrap();

        assert!(
            metrics_text.contains("vaultwarden_db_connections_in_use{database=\"gauges_unittest\"} 3"),
            "{metrics_text}"
        );
        assert!(
            metrics_text.contains("vaultwarden_db_connections_max{database=\"gauges_unittest\"} 10"),
            "{metrics_text}"
        );

        // r2d2 exposes no idle counter, so these must not be published as if they were real.
        assert!(!metrics_text.contains("vaultwarden_db_connections_active"), "{metrics_text}");
        assert!(!metrics_text.contains("vaultwarden_db_connections_idle"), "{metrics_text}");
    }

    #[test]
    fn test_authentication_metrics() {
        increment_auth_attempts("password", "success");
        increment_auth_attempts("password", "failed");
        increment_auth_attempts("webauthn", "success");
        increment_auth_attempts("2fa", "failed");

        update_user_sessions("authenticated", 150);
        update_user_sessions("anonymous", 5);

        let metrics_text = gather_metrics().unwrap();
        assert!(metrics_text.contains("vaultwarden_auth_attempts_total"), "{metrics_text}");
        assert!(metrics_text.contains("vaultwarden_user_sessions_active"), "{metrics_text}");
    }

    #[test]
    fn test_build_info_initialization() {
        init_build_info();

        update_uptime(std::time::SystemTime::now());

        let metrics_text = gather_metrics().unwrap();
        assert!(metrics_text.contains("vaultwarden_build_info"), "{metrics_text}");
        assert!(metrics_text.contains("vaultwarden_uptime_seconds"), "{metrics_text}");
    }

    #[test]
    fn test_metrics_gathering() {
        increment_http_requests("GET", "/api/sync", 200);
        update_db_connections("gathering_unittest", 1, 5);
        init_build_info();

        let metrics_text = gather_metrics().unwrap();

        assert!(metrics_text.contains("# HELP"));
        assert!(metrics_text.contains("# TYPE"));
    }

    #[test]
    fn test_path_normalization() {
        increment_http_requests("GET", "/api/sync", 200);
        increment_http_requests("GET", "/api/accounts/123/profile", 200);
        increment_http_requests("POST", "/api/organizations/456/users", 201);
        increment_http_requests("PUT", "/api/ciphers/789", 200);

        let metrics_text = gather_metrics().unwrap();
        assert!(!metrics_text.is_empty());
    }

    #[test]
    fn test_concurrent_metrics_collection() {
        use std::thread;

        let handles: Vec<_> = (0..10i32)
            .map(|i| {
                thread::spawn(move || {
                    increment_http_requests("GET", "/api/sync", 200);
                    observe_http_request_duration("GET", "/api/sync", 0.1 + (f64::from(i) * 0.01));
                    update_db_connections("concurrent_unittest", i64::from(i), 10 - i64::from(i));
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert!(gather_metrics().is_ok());
    }
}

#[cfg(not(feature = "enable_metrics"))]
mod metrics_disabled_tests {
    use crate::metrics::*;

    #[test]
    fn test_no_op_implementations() {
        increment_http_requests("GET", "/api/sync", 200);
        observe_http_request_duration("GET", "/api/sync", 0.150);
        update_db_connections("sqlite", 5, 10);
        observe_db_query_duration("select", 0.025);
        increment_auth_attempts("password", "success");
        update_user_sessions("authenticated", 150);
        init_build_info();

        update_uptime(std::time::SystemTime::now());

        assert_eq!(gather_metrics().unwrap(), "Metrics not enabled");
    }

    #[test]
    fn test_concurrent_no_op_calls() {
        use std::thread;

        let handles: Vec<_> = (0..5i64)
            .map(|i| {
                thread::spawn(move || {
                    increment_http_requests("GET", "/test", 200);
                    observe_http_request_duration("GET", "/test", 0.1);
                    update_db_connections("test", i, 5 - i);
                    increment_auth_attempts("password", "success");
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(gather_metrics().unwrap(), "Metrics not enabled");
    }
}
