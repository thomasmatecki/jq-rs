use jq_rs::JqProgram;
use std::sync::Arc;
use std::thread;

use lazy_static::lazy_static; // 1.4.0

lazy_static! {
    static ref JQ_PROGRAM: Arc<JqProgram> = Arc::new(jq_rs::compile(".name").unwrap());
}

#[test]
fn test_multi_thread_compiled() {
    let handle0 = thread::spawn(|| {
        let mut c1 = JQ_PROGRAM.as_ref().clone();
        if let Ok(result) = c1.run(r#"{"name": "test"}"#) {
            assert_eq!(result, "\"test\"\n");
        } else {
            panic!("failed");
        }
    });

    let handle1 = thread::spawn(|| {
        let mut c1 = JQ_PROGRAM.as_ref().clone();
        if let Ok(result) = c1.run(r#"{"name": "test"}"#) {
            assert_eq!(result, "\"test\"\n");
        } else {
            panic!("failed");
        }
    });
}
