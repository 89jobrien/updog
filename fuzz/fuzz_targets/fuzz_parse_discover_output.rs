#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // parse_discover_output must never panic on arbitrary UTF-8 input.
        // It must return Ok(_) or Err(TraceError::Parse(_)) — never Unavailable, never panic.
        let result = ail::adapters::coursers::parse_discover_output(s);
        // Assert Unavailable is never returned from parse (only from binary-not-found path)
        if let Err(agent_loop::TraceError::Unavailable(_)) = result {
            panic!("parse_discover_output returned Unavailable — should be Parse");
        }
    }
});
