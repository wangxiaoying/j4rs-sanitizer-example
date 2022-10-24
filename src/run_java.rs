use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use std::convert::TryFrom;
use std::fs;

pub fn init_naive_jvm() -> Jvm {
    JvmBuilder::new().skip_setting_native_lib().build().unwrap()
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
