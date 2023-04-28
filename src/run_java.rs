use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use std::convert::TryFrom;
use std::fs;
use std::env;

pub fn init_naive_jvm() -> Jvm {
	let j4rs_base = env::var("J4RS_BASE_PATH").unwrap_or("/root/j4rs-sanitizer-example/target/debug".to_string());
    JvmBuilder::new().skip_setting_native_lib().with_base_path(j4rs_base.as_str()).build().unwrap()
}

pub fn init_jvm() -> Jvm {
    let path = fs::canonicalize("./test.jar").unwrap();
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    JvmBuilder::new().skip_setting_native_lib().classpath_entry(entry).build().unwrap()
}

pub fn run_java(in_str: &str) {
    let jvm = init_jvm();

    let str = InvocationArg::try_from(in_str).unwrap();
    let test = jvm.create_instance("pyo3.test.Pyo3Test", &[]).unwrap();
    jvm.invoke(&test, "run", &[str]).unwrap();
}
