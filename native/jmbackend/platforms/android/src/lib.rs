pub use jmbackend::*;
use jni::objects::JClass;
use jni::objects::JString;
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "system" fn Java_opensource_jenny_Jni_init<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    params: JString<'local>,
) {
    let params: String = env.get_string(&params).unwrap().into();
    init_sync(params.as_str());
}

#[no_mangle]
pub unsafe extern "C" fn Java_opensource_jenny_Jni_invoke<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    params: JString<'local>,
) -> JString<'local> {
    let params: String = env.get_string(&params).unwrap().into();
    env.new_string(invoke(params.as_str())).unwrap()
}
