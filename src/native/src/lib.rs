use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_App_shareMemory(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // Get the input string from Java
    let input_str: String = env
        .get_string(&input)
        .expect("Couldn't get Java string!")
        .into();

    // Modify the string (shared memory simulation)
    let output_str = format!("Rust says: {} (modified!)", input_str);

    // Convert back to Java string
    return env.new_string(output_str)
        .expect("Couldn't create Java string!")
        .into_raw();
}