/// Metrics middleware for automatic HTTP request instrumentation
use rocket::{
    Data, Request, Response, Route,
    fairing::{Fairing, Info, Kind},
};
use std::time::Instant;

pub struct MetricsFairing;

#[rocket::async_trait]
impl Fairing for MetricsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Metrics Collection",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.local_cache(|| RequestTimer {
            start_time: Instant::now(),
        });
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let timer = req.local_cache(|| RequestTimer {
            start_time: Instant::now(),
        });
        let duration = timer.start_time.elapsed();
        let method = req.method().as_str();
        let path = route_label(req.route());
        let status = res.status().code;

        // Record metrics
        crate::metrics::increment_http_requests(method, &path, status);
        crate::metrics::observe_http_request_duration(method, &path, duration.as_secs_f64());
    }
}

struct RequestTimer {
    start_time: Instant,
}

/// Label for requests that match no route, so scanner probes share one series
const UNMATCHED_ROUTE: &str = "unmatched";

/// Use the matched route template (for example `/api/accounts/<uuid>`) as the path label.
/// The label set is then bounded by the routes Vaultwarden defines, not by client-supplied paths.
fn route_label(route: Option<&Route>) -> String {
    route.map_or_else(|| UNMATCHED_ROUTE.to_owned(), |route| route.uri.path().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Method;
    use rocket::route::dummy_handler;

    #[test]
    fn test_route_label_uses_route_template() {
        let route = Route::new(Method::Get, "/api/accounts/<uuid>", dummy_handler);
        assert_eq!(route_label(Some(&route)), "/api/accounts/<uuid>");
    }

    #[test]
    fn test_route_label_unmatched() {
        assert_eq!(route_label(None), UNMATCHED_ROUTE);
    }
}
