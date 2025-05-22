use jni::JNIEnv;
use jni::objects::{JClass, JValue};
//<RustJNI>
// primitive imports
use jni::sys::{jstring, jint, jfloat, jobject, jboolean};
//</RustJNI>

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_sayHello(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = "Hello from Rust!";

    env.new_string(output)
        .expect("Couldn't create Java string!")
        .into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_sumIntNumbers(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    a + b
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_subtract(
    _env: JNIEnv, 
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    a - b
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_multiply(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    a * b
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_divide(
    mut env: JNIEnv,
    _class: JClass,
    dividend: jfloat,
    divisor: jfloat
) -> jobject {
    let (result, status) = if divisor == 0.0 {
        (0.0, false)
    } else {
        (dividend / divisor, true)
    };

    let class = env.find_class("com/example/calculatorapp/DivisionResponse")
        .expect("Couldn't find class!");

    let response = env.new_object(
        class,
        "(FZ)V",
        &[JValue::Float(result), JValue::Bool(status as jboolean)]
    ).expect("Couldn't build object!");
    
    response.as_raw()
}