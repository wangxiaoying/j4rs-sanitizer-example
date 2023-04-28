pub mod run_java;

#[no_mangle]
pub unsafe extern "C" fn j4rs_test() {
	let _ = run_java::init_naive_jvm();
	println!("success!");
}
