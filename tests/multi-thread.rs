use jq_rs::compile;
use jq_rs::Error;
use jq_rs::JqProgram;
use std::ops::DerefMut;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

//use lazy_static::lazy_static; // 1.4.0

//#[test]
//fn test_run_twice() {
//    let mut jq_program = compile(".name").unwrap();
//
//    let r0 = jq_program.run(r#"{"name": "test"}"#);
//    let r1 = jq_program.run(r#"{"name": "test"}"#);
//
//    if let Ok(result) = r1 {
//        assert_eq!(result, "\"test\"\n");
//    } else {
//        panic!("failed");
//    }
//
//    if let Ok(result) = r0 {
//        assert_eq!(result, "\"test\"\n");
//    } else {
//        panic!("failed");
//    }
//}

#[derive(Clone)]
struct Program {
    jq_program: Arc<JqProgram>,
}

impl Program {
    fn new(prog_str: &str) -> Self {
        Program {
            jq_program: Arc::new(compile(prog_str).unwrap()),
        }
    }

    fn run(&self, input: &str) -> Result<String, Error> {
        self.jq_program.clone().run(input)
    }
}

#[test]
fn test_run_in_thread() {
    let p = Program::new(".name");
    let c0 = p.clone();

    let _handle0 = spawn(|| {
        let r0 = c0.run(r#"{"name": "test"}"#);

        if let Ok(result) = r0 {
            assert_eq!(result, "\"test\"\n");
            panic!("failed");
        }
    });

    sleep(Duration::new(5, 0));

    // Second thread
    //let handle1 = thread::spawn(move || {
    //    let r0 = jq_program.run(r#"{"name": "test"}"#);
    //    if let Ok(result) = r0 {
    //        assert_eq!(result, "\"test\"\n");
    //    } else {
    //        panic!("failed");
    //    }
    //});
}
//lazy_static! {
//    static ref JQ_PROGRAM: Arc<JqProgram> = Arc::new(jq_rs::compile(".name").unwrap());
//}
//
//#[test]
//fn test_multi_thread_compiled() {
//    let handle0 = thread::spawn(|| {
//        let mut c1 = JQ_PROGRAM.as_ref().clone();
//        if let Ok(result) = c1.run(r#"{"name": "test"}"#) {
//            assert_eq!(result, "\"test\"\n");
//        } else {
//            panic!("failed");
//        }
//    });
//
//    let handle1 = thread::spawn(|| {
//        let mut c1 = JQ_PROGRAM.as_ref().clone();
//        if let Ok(result) = c1.run(r#"{"name": "test"}"#) {
//            assert_eq!(result, "\"test\"\n");
//        } else {
//            panic!("failed");
//        }
//    });
//}
