use jq_rs::compile;
use jq_rs::Error;
use jq_rs::JqProgram;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::thread::{sleep, spawn};
use std::time::Duration;

#[test]
fn test_run_twice() {
    let jq_program = compile(".name").unwrap();

    let r0 = jq_program.run(r#"{"name": "test"}"#);
    let r1 = jq_program.run(r#"{"name": "test"}"#);

    if let Ok(result) = r1 {
        assert_eq!(result, "\"test\"\n");
    } else {
        panic!("failed");
    }

    if let Ok(result) = r0 {
        assert_eq!(result, "\"test\"\n");
    } else {
        panic!("failed");
    }
}

#[test]
fn test_run_in_spawned_thread_and_main() {
    let arc_p = Arc::new(JqProgram::new(".name"));
    let p_ref0 = arc_p.clone();

    let handle0 = spawn(move || {
        let r0 = p_ref0.run(r#"{"name": "test"}"#);

        if let Ok(result) = r0 {
            assert_eq!(result, "\"test\"\n");
        } else {
            panic!("failed");
        }
    });

    sleep(Duration::new(1, 0));

    assert_eq!(true, matches!(handle0.join(), Ok(_result)));

    let p_ref0 = arc_p.clone();

    if let Ok(result) = p_ref0.run(r#"{"name": "test"}"#) {
        assert_eq!(result, "\"test\"\n");
    } else {
        panic!("failed");
    }
}

#[test]
fn test_run_in_many_threads() {
    let arc_p = Arc::new(JqProgram::new(".name"));
    let threads: Vec<JoinHandle<Result<String, Error>>> = (1..1000) // Making this 10k causes problems
        .map(|_| {
            let p_ref0 = arc_p.clone();
            spawn(move || p_ref0.run(r#"{"name": "test"}"#))
        })
        .collect();

    for thread in threads {
        let result = thread.join();
        if let Ok(Ok(output)) = result {
            assert_eq!(output, "\"test\"\n");
        } else {
            panic!("failed");
        }
    }
}
