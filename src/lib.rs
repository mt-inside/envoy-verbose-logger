use log::*;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

/* TODO
 * - rename to VerboseLogger
 * - add examples of Istio yaml to deploy and use
 * - add to the dynamic metadata, so they can be logged that way (https://themartian.hashnode.dev/http-request-body-logging-with-istio-and-envoy)
 * - take config to enable / disable the four parts, for performance
 * - take config with regex, match and print only that in body
 */

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    info!("plugin init");
    proxy_wasm::set_root_context(|_context_id: u32| -> Box<dyn RootContext> { Box::new(BodyLoggerRoot::new()) });
}}

#[derive(Debug)]
struct BodyLoggerRoot {}

impl BodyLoggerRoot {
    fn new() -> Self {
        Self {}
    }
}

impl Context for BodyLoggerRoot {}

impl RootContext for BodyLoggerRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(BodyLogger::new()))
    }
}

#[derive(Debug)]
struct BodyLogger {}

impl Context for BodyLogger {}

impl BodyLogger {
    fn new() -> Self {
        Self {}
    }
}

impl HttpContext for BodyLogger {
    fn on_http_request_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> proxy_wasm::types::Action {
        let headers = self.get_http_request_headers();
        info!("REQUEST headers follow (count {})", headers.len());
        headers.iter().for_each(|(name, value)| {
            info!("{} => {}", name, value);
        });

        proxy_wasm::types::Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> proxy_wasm::types::Action {
        let headers = self.get_http_response_headers();
        info!("RESPONSE headers follow (count {})", headers.len());
        headers.iter().for_each(|(name, value)| {
            info!("{} => {}", name, value);
        });

        proxy_wasm::types::Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream {
            return Action::Pause;
        }

        if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
            info!("REQUEST body follows (size {})", body_size);
            match String::from_utf8(body_bytes) {
                Ok(body_str) => info!("{}", body_str),
                Err(_) => info!("<non-utf8 body; not logging>"),
            };
        }

        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream {
            return Action::Pause;
        }

        if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
            info!("RESPONSE body follows (size {})", body_size);
            match String::from_utf8(body_bytes) {
                Ok(body_str) => info!("{}", body_str),
                Err(_) => info!("<non-utf8 body; not logging>"),
            };
        }

        Action::Continue
    }
}
