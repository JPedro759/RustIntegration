use jni::JNIEnv;
use jni::objects::{JClass, JValue};
//<RustJNI>
// primitive imports
use jni::sys::{jstring, jint, jfloat, jobject, jboolean};
//</RustJNI>

// Import the modules directory
mod modules;

use modules::operations;

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_sayHello(
    env: JNIEnv,
    _class: JClass
) -> jstring {
    let output = "Hello from Rust!";

    env.new_string(output).expect("Couldn't create Java string!").into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_sumIntNumbers(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    operations::sum(a, b)
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_subtract(
    _env: JNIEnv, 
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    operations::sub(a, b)
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_multiply(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint
) -> jint {
    operations::multi(a, b)
}

#[no_mangle]
pub extern "C" fn Java_com_example_calculatorapp_MainActivity_divide(
    mut env: JNIEnv,
    _class: JClass,
    dividend: jfloat,
    divisor: jfloat
) -> jobject {
    let result = operations::div(dividend, divisor);

    let class = env.find_class("com/example/calculatorapp/DivisionResponse")
        .expect("Couldn't find class!");
    
    let ctor_args = &[JValue::Float(result.value), JValue::Bool(result.status as jboolean)];

    let response = env.new_object(class, "(FZ)V", ctor_args).expect("Couldn't build object!");

    response.as_raw()
}