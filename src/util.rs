use flexi_logger::Logger;

#[allow(dead_code)]
pub(crate) fn init_log() {
    let _logger = Logger::try_with_env_or_str("debug")
        .expect("logger env failed")
        //.write_mode(WriteMode::Async)
        .log_to_stdout()
        .start()
        .expect("logger start error");
}
