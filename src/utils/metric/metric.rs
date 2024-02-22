pub use super::keccak256::Keccak256Record;
use revm_utils::time_utils::instant::Instant;

static mut METRIC_RECORDER: Option<Keccak256Record> = None;

// This function will be called directly during program initialization.
#[ctor::ctor]
unsafe fn init() {
    METRIC_RECORDER = Some(Keccak256Record::default());
}

pub fn get_keccak256_record() -> Keccak256Record {
    unsafe {
        let record = METRIC_RECORDER.unwrap_or_default();
        METRIC_RECORDER = Some(Keccak256Record::default());

        record
    }
}

pub struct Keccak256Recorder {
    start: Instant,
}

impl Keccak256Recorder {
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }
}

impl Drop for Keccak256Recorder {
    fn drop(&mut self) {
        unsafe {
            let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
            _record.count = _record.count.checked_add(1).expect("overflow");

            let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
            _record.time_cycles = _record.time_cycles.checked_add(time_cycles).expect("overflow");
        }
    }
}
