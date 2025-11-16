#![allow(non_snake_case, non_camel_case_types, unused)]
#![warn(rust_2018_idioms, missing_debug_implementations)]

use core::ffi::{c_char, c_uchar, c_uint, c_void};

use bitflags::bitflags;
use jni_sys::{
    jboolean, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jmethodID, jobject, jvalue,
    JNIEnv, JNIEnv_, JNINativeInterface_,
};
use jni_sys_macros::jni_to_union;

pub const JVMTI_VERSION_1: jint = 0x30010000;
pub const JVMTI_VERSION_1_0: jint = 0x30010000;
pub const JVMTI_VERSION_1_1: jint = 0x30010100;
pub const JVMTI_VERSION_1_2: jint = 0x30010200;

pub type Agent_OnLoad =
    extern "system" fn(vm: *mut JNIEnv, options: *mut c_char, reserved: *mut c_void) -> jint;
pub type Agent_OnAttach =
    extern "system" fn(vm: *mut JNIEnv, options: *mut c_char, reserved: *mut c_void) -> jint;
pub type Agent_OnUnload = extern "system" fn(vm: *mut jni_sys::JavaVM);

pub type jvmtiEnv = *const jvmtiInterface_1_;

pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;

#[derive(Debug)]
pub struct _jrawMonitorID {}
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = JNINativeInterface_;

pub const JVMTI_THREAD_STATE_ALIVE: c_uint = 1;
pub const JVMTI_THREAD_STATE_TERMINATED: c_uint = 2;
pub const JVMTI_THREAD_STATE_RUNNABLE: c_uint = 4;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: c_uint = 1024;
pub const JVMTI_THREAD_STATE_WAITING: c_uint = 128;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: c_uint = 16;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: c_uint = 32;
pub const JVMTI_THREAD_STATE_SLEEPING: c_uint = 64;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: c_uint = 256;
pub const JVMTI_THREAD_STATE_PARKED: c_uint = 512;
pub const JVMTI_THREAD_STATE_SUSPENDED: c_uint = 1048576;
pub const JVMTI_THREAD_STATE_INTERRUPTED: c_uint = 2097152;
pub const JVMTI_THREAD_STATE_IN_NATIVE: c_uint = 4194304;
pub const JVMTI_THREAD_STATE_VENDOR_1: c_uint = 268435456;
pub const JVMTI_THREAD_STATE_VENDOR_2: c_uint = 536870912;
pub const JVMTI_THREAD_STATE_VENDOR_3: c_uint = 1073741824;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: c_uint = 1207;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: c_uint = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: c_uint = 2;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: c_uint = 5;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: c_uint = 1025;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: c_uint = 145;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: c_uint = 161;

pub const JVMTI_THREAD_MIN_PRIORITY: c_uint = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: c_uint = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: c_uint = 10;

pub const JVMTI_HEAP_FILTER_TAGGED: c_uint = 4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: c_uint = 8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: c_uint = 16;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: c_uint = 32;

pub const JVMTI_VISIT_OBJECTS: c_uint = 256;
pub const JVMTI_VISIT_ABORT: c_uint = 32768;

/* Heap Reference Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiHeapReferenceKind {
    JVMTI_HEAP_REFERENCE_CLASS = 1,
    JVMTI_HEAP_REFERENCE_FIELD = 2,
    JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_HEAP_REFERENCE_CLASS_LOADER = 4,
    JVMTI_HEAP_REFERENCE_SIGNERS = 5,
    JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_HEAP_REFERENCE_INTERFACE = 7,
    JVMTI_HEAP_REFERENCE_STATIC_FIELD = 8,
    JVMTI_HEAP_REFERENCE_CONSTANT_POOL = 9,
    JVMTI_HEAP_REFERENCE_SUPERCLASS = 10,
    JVMTI_HEAP_REFERENCE_JNI_GLOBAL = 21,
    JVMTI_HEAP_REFERENCE_SYSTEM_CLASS = 22,
    JVMTI_HEAP_REFERENCE_MONITOR = 23,
    JVMTI_HEAP_REFERENCE_STACK_LOCAL = 24,
    JVMTI_HEAP_REFERENCE_JNI_LOCAL = 25,
    JVMTI_HEAP_REFERENCE_THREAD = 26,
    JVMTI_HEAP_REFERENCE_OTHER = 27,
}

/* Primitive Type Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiPrimitiveType {
    JVMTI_PRIMITIVE_TYPE_BOOLEAN = 90,
    JVMTI_PRIMITIVE_TYPE_BYTE = 66,
    JVMTI_PRIMITIVE_TYPE_CHAR = 67,
    JVMTI_PRIMITIVE_TYPE_SHORT = 83,
    JVMTI_PRIMITIVE_TYPE_INT = 73,
    JVMTI_PRIMITIVE_TYPE_LONG = 74,
    JVMTI_PRIMITIVE_TYPE_FLOAT = 70,
    JVMTI_PRIMITIVE_TYPE_DOUBLE = 68,
}
/* Heap Object Filter Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiHeapObjectFilter {
    JVMTI_HEAP_OBJECT_TAGGED = 1,
    JVMTI_HEAP_OBJECT_UNTAGGED = 2,
    JVMTI_HEAP_OBJECT_EITHER = 3,
}
/* Heap Root Kind Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiHeapRootKind {
    JVMTI_HEAP_ROOT_JNI_GLOBAL = 1,
    JVMTI_HEAP_ROOT_SYSTEM_CLASS = 2,
    JVMTI_HEAP_ROOT_MONITOR = 3,
    JVMTI_HEAP_ROOT_STACK_LOCAL = 4,
    JVMTI_HEAP_ROOT_JNI_LOCAL = 5,
    JVMTI_HEAP_ROOT_THREAD = 6,
    JVMTI_HEAP_ROOT_OTHER = 7,
}
/* Object Reference Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiObjectReferenceKind {
    JVMTI_REFERENCE_CLASS = 1,
    JVMTI_REFERENCE_FIELD = 2,
    JVMTI_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_REFERENCE_CLASS_LOADER = 4,
    JVMTI_REFERENCE_SIGNERS = 5,
    JVMTI_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_REFERENCE_INTERFACE = 7,
    JVMTI_REFERENCE_STATIC_FIELD = 8,
    JVMTI_REFERENCE_CONSTANT_POOL = 9,
}
/* Iteration Control Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiIterationControl {
    JVMTI_ITERATION_CONTINUE = 1,
    JVMTI_ITERATION_IGNORE = 2,
    JVMTI_ITERATION_ABORT = 0,
}
/* Class Status Flags */
pub const JVMTI_CLASS_STATUS_VERIFIED: c_uint = 1;
const JVMTI_CLASS_STATUS_PREPARED: c_uint = 2;
const JVMTI_CLASS_STATUS_INITIALIZED: c_uint = 4;
const JVMTI_CLASS_STATUS_ERROR: c_uint = 8;
const JVMTI_CLASS_STATUS_ARRAY: c_uint = 16;
const JVMTI_CLASS_STATUS_PRIMITIVE: c_uint = 32;

/* Event Enable/Disable */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiEventMode {
    JVMTI_ENABLE = 1,
    JVMTI_DISABLE = 0,
}

/* Extension Function/Event Parameter Types */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiParamTypes {
    JVMTI_TYPE_JBYTE = 101,
    JVMTI_TYPE_JCHAR = 102,
    JVMTI_TYPE_JSHORT = 103,
    JVMTI_TYPE_JINT = 104,
    JVMTI_TYPE_JLONG = 105,
    JVMTI_TYPE_JFLOAT = 106,
    JVMTI_TYPE_JDOUBLE = 107,
    JVMTI_TYPE_JBOOLEAN = 108,
    JVMTI_TYPE_JOBJECT = 109,
    JVMTI_TYPE_JTHREAD = 110,
    JVMTI_TYPE_JCLASS = 111,
    JVMTI_TYPE_JVALUE = 112,
    JVMTI_TYPE_JFIELDID = 113,
    JVMTI_TYPE_JMETHODID = 114,
    JVMTI_TYPE_CCHAR = 115,
    JVMTI_TYPE_CVOID = 116,
    JVMTI_TYPE_JNIENV = 117,
}

/* Extension Function/Event Parameter Kinds */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiParamKind {
    JVMTI_KIND_IN = 91,
    JVMTI_KIND_IN_PTR = 92,
    JVMTI_KIND_IN_BUF = 93,
    JVMTI_KIND_ALLOC_BUF = 94,
    JVMTI_KIND_ALLOC_ALLOC_BUF = 95,
    JVMTI_KIND_OUT = 96,
    JVMTI_KIND_OUT_BUF = 97,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
/* Timer Kinds */
pub enum jvmtiTimerKind {
    JVMTI_TIMER_USER_CPU = 30,
    JVMTI_TIMER_TOTAL_CPU = 31,
    JVMTI_TIMER_ELAPSED = 32,
}

/* Phases of execution */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiPhase {
    JVMTI_PHASE_ONLOAD = 1,
    JVMTI_PHASE_PRIMORDIAL = 2,
    JVMTI_PHASE_START = 6,
    JVMTI_PHASE_LIVE = 4,
    JVMTI_PHASE_DEAD = 8,
}
/* Version Interface Types */
pub const JVMTI_VERSION_INTERFACE_JNI: c_uint = 0x00000000;
pub const JVMTI_VERSION_INTERFACE_JVMTI: c_uint = 0x30000000;
/* Version Masks */

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: c_uint = 0x70000000;
pub const JVMTI_VERSION_MASK_MAJOR: c_uint = 0x0FFF0000;
pub const JVMTI_VERSION_MASK_MINOR: c_uint = 0x0000FF00;
pub const JVMTI_VERSION_MASK_MICRO: c_uint = 0x000000FF;

/* Version Shifts */
pub const JVMTI_VERSION_SHIFT_MAJOR: c_uint = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: c_uint = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: c_uint = 0;

/* Verbose Flag Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiVerboseFlag {
    JVMTI_VERBOSE_OTHER = 0,
    JVMTI_VERBOSE_GC = 1,
    JVMTI_VERBOSE_CLASS = 2,
    JVMTI_VERBOSE_JNI = 4,
}

/* JLocation Format Enumeration */
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiJlocationFormat {
    JVMTI_JLOCATION_JVMBCI = 1,
    JVMTI_JLOCATION_MACHINEPC = 2,
    JVMTI_JLOCATION_OTHER = 0,
}
/* Resource Exhaustion Flags */

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: jint = 0x0001;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: jint = 0x0002;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: jint = 0x0004;

bitflags! {
    /// Represents the capabilities of the JVMTI agent.
    #[derive(Clone,  Debug)]
    #[repr(C)]
    pub struct JVMTI_RESOURCE_EXHAUSTED_FLAGS: jint {
        const OOM_ERROR = JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR;
        const JAVA_HEAP = 0x0002;
        const THREADS = 0x0004;
    }
}

/* Errors */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum jvmtiError {
    JVMTI_ERROR_NONE = 0,
    JVMTI_ERROR_INVALID_THREAD = 10,
    JVMTI_ERROR_INVALID_THREAD_GROUP = 11,
    JVMTI_ERROR_INVALID_PRIORITY = 12,
    JVMTI_ERROR_THREAD_NOT_SUSPENDED = 13,
    JVMTI_ERROR_THREAD_SUSPENDED = 14,
    JVMTI_ERROR_THREAD_NOT_ALIVE = 15,
    JVMTI_ERROR_INVALID_OBJECT = 20,
    JVMTI_ERROR_INVALID_CLASS = 21,
    JVMTI_ERROR_CLASS_NOT_PREPARED = 22,
    JVMTI_ERROR_INVALID_METHODID = 23,
    JVMTI_ERROR_INVALID_LOCATION = 24,
    JVMTI_ERROR_INVALID_FIELDID = 25,
    JVMTI_ERROR_NO_MORE_FRAMES = 31,
    JVMTI_ERROR_OPAQUE_FRAME = 32,
    JVMTI_ERROR_TYPE_MISMATCH = 34,
    JVMTI_ERROR_INVALID_SLOT = 35,
    JVMTI_ERROR_DUPLICATE = 40,
    JVMTI_ERROR_NOT_FOUND = 41,
    JVMTI_ERROR_INVALID_MONITOR = 50,
    JVMTI_ERROR_NOT_MONITOR_OWNER = 51,
    JVMTI_ERROR_INTERRUPT = 52,
    JVMTI_ERROR_INVALID_CLASS_FORMAT = 60,
    JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION = 61,
    JVMTI_ERROR_FAILS_VERIFICATION = 62,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED = 63,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED = 64,
    JVMTI_ERROR_INVALID_TYPESTATE = 65,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED = 66,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED = 67,
    JVMTI_ERROR_UNSUPPORTED_VERSION = 68,
    JVMTI_ERROR_NAMES_DONT_MATCH = 69,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED = 70,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED = 71,
    JVMTI_ERROR_UNMODIFIABLE_CLASS = 79,
    JVMTI_ERROR_NOT_AVAILABLE = 98,
    JVMTI_ERROR_MUST_POSSESS_CAPABILITY = 99,
    JVMTI_ERROR_NULL_POINTER = 100,
    JVMTI_ERROR_ABSENT_INFORMATION = 101,
    JVMTI_ERROR_INVALID_EVENT_TYPE = 102,
    JVMTI_ERROR_ILLEGAL_ARGUMENT = 103,
    JVMTI_ERROR_NATIVE_METHOD = 104,
    JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED = 106,
    JVMTI_ERROR_OUT_OF_MEMORY = 110,
    JVMTI_ERROR_ACCESS_DENIED = 111,
    JVMTI_ERROR_WRONG_PHASE = 112,
    JVMTI_ERROR_INTERNAL = 113,
    JVMTI_ERROR_UNATTACHED_THREAD = 115,
    JVMTI_ERROR_INVALID_ENVIRONMENT = 116,
    // JVMTI_ERROR_MAX = 116
}
/* Event IDs */

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum jvmtiEvent {
    // JVMTI_MIN_EVENT_TYPE_VAL = 50,
    JVMTI_EVENT_VM_INIT = 50,
    JVMTI_EVENT_VM_DEATH = 51,
    JVMTI_EVENT_THREAD_START = 52,
    JVMTI_EVENT_THREAD_END = 53,
    JVMTI_EVENT_CLASS_FILE_LOAD_HOOK = 54,
    JVMTI_EVENT_CLASS_LOAD = 55,
    JVMTI_EVENT_CLASS_PREPARE = 56,
    JVMTI_EVENT_VM_START = 57,
    JVMTI_EVENT_EXCEPTION = 58,
    JVMTI_EVENT_EXCEPTION_CATCH = 59,
    JVMTI_EVENT_SINGLE_STEP = 60,
    JVMTI_EVENT_FRAME_POP = 61,
    JVMTI_EVENT_BREAKPOINT = 62,
    JVMTI_EVENT_FIELD_ACCESS = 63,
    JVMTI_EVENT_FIELD_MODIFICATION = 64,
    JVMTI_EVENT_METHOD_ENTRY = 65,
    JVMTI_EVENT_METHOD_EXIT = 66,
    JVMTI_EVENT_NATIVE_METHOD_BIND = 67,
    JVMTI_EVENT_COMPILED_METHOD_LOAD = 68,
    JVMTI_EVENT_COMPILED_METHOD_UNLOAD = 69,
    JVMTI_EVENT_DYNAMIC_CODE_GENERATED = 70,
    JVMTI_EVENT_DATA_DUMP_REQUEST = 71,
    JVMTI_EVENT_MONITOR_WAIT = 73,
    JVMTI_EVENT_MONITOR_WAITED = 74,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTER = 75,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTERED = 76,
    JVMTI_EVENT_RESOURCE_EXHAUSTED = 80,
    JVMTI_EVENT_GARBAGE_COLLECTION_START = 81,
    JVMTI_EVENT_GARBAGE_COLLECTION_FINISH = 82,
    JVMTI_EVENT_OBJECT_FREE = 83,
    JVMTI_EVENT_VM_OBJECT_ALLOC = 84,
    JVMTI_EVENT_SAMPLED_OBJECT_ALLOC = 86,
    JVMTI_EVENT_VIRTUAL_THREAD_START = 87,
    JVMTI_EVENT_VIRTUAL_THREAD_END = 88,
    // JVMTI_MAX_EVENT_TYPE_VAL = 88
}

pub type jvmtiThreadInfo = _jvmtiThreadInfo;
pub type jvmtiMonitorStackDepthInfo = _jvmtiMonitorStackDepthInfo;
pub type jvmtiThreadGroupInfo = _jvmtiThreadGroupInfo;
pub type jvmtiFrameInfo = _jvmtiFrameInfo;
pub type jvmtiStackInfo = _jvmtiStackInfo;
pub type jvmtiHeapReferenceInfoField = _jvmtiHeapReferenceInfoField;
pub type jvmtiHeapReferenceInfoArray = _jvmtiHeapReferenceInfoArray;
pub type jvmtiHeapReferenceInfoConstantPool = _jvmtiHeapReferenceInfoConstantPool;
pub type jvmtiHeapReferenceInfoStackLocal = _jvmtiHeapReferenceInfoStackLocal;
pub type jvmtiHeapReferenceInfoJniLocal = _jvmtiHeapReferenceInfoJniLocal;
pub type jvmtiHeapReferenceInfoReserved = _jvmtiHeapReferenceInfoReserved;
pub type jvmtiHeapReferenceInfo = _jvmtiHeapReferenceInfo;
pub type jvmtiHeapCallbacks = _jvmtiHeapCallbacks;
pub type jvmtiClassDefinition = _jvmtiClassDefinition;
pub type jvmtiMonitorUsage = _jvmtiMonitorUsage;
pub type jvmtiLineNumberEntry = _jvmtiLineNumberEntry;
pub type jvmtiLocalVariableEntry = _jvmtiLocalVariableEntry;
pub type jvmtiParamInfo = _jvmtiParamInfo;
pub type jvmtiExtensionFunctionInfo = _jvmtiExtensionFunctionInfo;
pub type jvmtiExtensionEventInfo = _jvmtiExtensionEventInfo;
pub type jvmtiTimerInfo = _jvmtiTimerInfo;
pub type jvmtiAddrLocationMap = _jvmtiAddrLocationMap;

pub type jvmtiStartFunction =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, arg: *mut c_void);
pub type jvmtiHeapIterationCallback = unsafe extern "system" fn(
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    length: jint,
    user_data: *mut c_void,
) -> jint;
pub type jvmtiHeapReferenceCallback = unsafe extern "system" fn(
    reference_kind: jvmtiHeapReferenceKind,
    reference_info: *const jvmtiHeapReferenceInfo,
    class_tag: jlong,
    referrer_class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    referrer_tag_ptr: *mut jlong,
    length: jint,
    user_data: *mut c_void,
) -> jint;
pub type jvmtiPrimitiveFieldCallback = unsafe extern "system" fn(
    kind: jvmtiHeapReferenceKind,
    info: *const jvmtiHeapReferenceInfo,
    object_class_tag: jlong,
    object_tag_ptr: *mut jlong,
    value: jvalue,
    value_type: jvmtiPrimitiveType,
    user_data: *mut c_void,
) -> jint;
pub type jvmtiArrayPrimitiveValueCallback = unsafe extern "system" fn(
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    element_count: jint,
    element_type: jvmtiPrimitiveType,
    elements: *mut c_void,
    user_data: *mut c_void,
) -> jint;
pub type jvmtiStringPrimitiveValueCallback = unsafe extern "system" fn(
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    value: *mut jchar,
    value_length: jint,
    user_data: *mut c_void,
) -> jint;
pub type jvmtiReservedCallback = unsafe extern "system" fn() -> jint;
pub type jvmtiHeapObjectCallback = unsafe extern "system" fn(
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    user_data: *mut c_void,
) -> jvmtiIterationControl;
pub type jvmtiHeapRootCallback = unsafe extern "system" fn(
    root_kind: jvmtiHeapRootKind,
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    user_data: *mut c_void,
) -> jvmtiIterationControl;
pub type jvmtiStackReferenceCallback = unsafe extern "system" fn(
    root_kind: jvmtiHeapRootKind,
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    thread_tag: jlong,
    depth: jint,
    method: jmethodID,
    slot: jint,
    user_data: *mut c_void,
) -> jvmtiIterationControl;
pub type jvmtiObjectReferenceCallback = unsafe extern "system" fn(
    reference_kind: jvmtiObjectReferenceKind,
    class_tag: jlong,
    size: jlong,
    tag_ptr: *mut jlong,
    referrer_tag: jlong,
    referrer_index: jint,
    user_data: *mut c_void,
) -> jvmtiIterationControl;
pub type jvmtiExtensionFunction = unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError;
pub type jvmtiExtensionEvent = unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiThreadInfo {
    pub name: *mut c_char,
    pub priority: jint,
    pub is_daemon: bool,
    pub thread_group: jthreadGroup,
    pub context_class_loader: jobject,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiMonitorStackDepthInfo {
    pub monitor: jobject,
    pub stack_depth: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiThreadGroupInfo {
    pub parent: jthreadGroup,
    pub name: *mut c_char,
    pub max_priority: jint,
    pub is_daemon: jboolean,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiFrameInfo {
    pub method: jmethodID,
    pub location: jlocation,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiStackInfo {
    pub thread: jthread,
    pub state: jint,
    pub frame_buffer: *mut _jvmtiFrameInfo,
    pub frame_count: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoField {
    pub index: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoArray {
    pub index: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoConstantPool {
    pub index: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoStackLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
    pub location: jlocation,
    pub slot: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoJniLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapReferenceInfoReserved {
    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union _jvmtiHeapReferenceInfo {
    pub field: jvmtiHeapReferenceInfoField,
    pub array: jvmtiHeapReferenceInfoArray,
    pub constant_pool: jvmtiHeapReferenceInfoConstantPool,
    pub stack_local: jvmtiHeapReferenceInfoStackLocal,
    pub jni_local: jvmtiHeapReferenceInfoJniLocal,
    pub other: jvmtiHeapReferenceInfoReserved,
}

impl core::fmt::Debug for _jvmtiHeapReferenceInfo {
    // TODO: think about how can this be represented better
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let field = unsafe { self.field };
        f.debug_struct("_jvmtiHeapReferenceInfo")
            .field("field", unsafe { &self.field })
            .field("array", unsafe { &self.array })
            .field("constant_pool", unsafe { &self.constant_pool })
            .field("stack_local", unsafe { &self.stack_local })
            .field("jni_local", unsafe { &self.jni_local })
            .field("other", unsafe { &self.other })
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiHeapCallbacks {
    pub heap_iteration_callback: jvmtiHeapIterationCallback,
    pub heap_reference_callback: jvmtiHeapReferenceCallback,
    pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
    pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
    pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
    pub reserved5: jvmtiReservedCallback,
    pub reserved6: jvmtiReservedCallback,
    pub reserved7: jvmtiReservedCallback,
    pub reserved8: jvmtiReservedCallback,
    pub reserved9: jvmtiReservedCallback,
    pub reserved10: jvmtiReservedCallback,
    pub reserved11: jvmtiReservedCallback,
    pub reserved12: jvmtiReservedCallback,
    pub reserved13: jvmtiReservedCallback,
    pub reserved14: jvmtiReservedCallback,
    pub reserved15: jvmtiReservedCallback,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiClassDefinition {
    pub klass: jclass,
    pub class_byte_count: jint,
    pub class_bytes: *const c_uchar,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiMonitorUsage {
    pub owner: jthread,
    pub entry_count: jint,
    pub waiter_count: jint,
    pub waiters: *mut jthread,
    pub notify_waiter_count: jint,
    pub notify_waiters: *mut jthread,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiLocalVariableEntry {
    pub start_location: jlocation,
    pub length: jint,
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub generic_signature: *mut c_char,
    pub slot: jint,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiParamInfo {
    pub name: *mut c_char,
    pub kind: jvmtiParamKind,
    pub base_type: jvmtiParamTypes,
    pub null_ok: jboolean,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiExtensionFunctionInfo {
    pub func: jvmtiExtensionFunction,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
    pub error_count: jint,
    pub errors: *mut jvmtiError,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiExtensionEventInfo {
    pub extension_event_index: jint,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiTimerInfo {
    pub max_value: jlong,
    pub may_skip_forward: jboolean,
    pub may_skip_backward: jboolean,
    pub kind: jvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}

bitflags! {
    /// Represents the capabilities of the JVMTI agent.
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct jvmtiCapabilities: u128 {
        const CAN_TAG_OBJECTS = 1 << 0;
        const CAN_GENERATE_FIELD_MODIFICATION_EVENTS = 1 << 1;
        const CAN_GENERATE_FIELD_ACCESS_EVENTS = 1 << 2;
        const CAN_GET_BYTECODES = 1 << 3;
        const CAN_GET_SYNTHETIC_ATTRIBUTE = 1 << 4;
        const CAN_GET_OWNED_MONITOR_INFO = 1 << 5;
        const CAN_GET_CURRENT_CONTENDED_MONITOR = 1 << 6;
        const CAN_GET_MONITOR_INFO = 1 << 7;
        const CAN_POP_FRAME = 1 << 8;
        const CAN_REDEFINE_CLASSES = 1 << 9;
        const CAN_SIGNAL_THREAD = 1 << 10;
        const CAN_GET_SOURCE_FILE_NAME = 1 << 11;
        const CAN_GET_LINE_NUMBERS = 1 << 12;
        const CAN_GET_SOURCE_DEBUG_EXTENSION = 1 << 13;
        const CAN_ACCESS_LOCAL_VARIABLES = 1 << 14;
        const CAN_MAINTAIN_ORIGINAL_METHOD_ORDER = 1 << 15;
        const CAN_GENERATE_SINGLE_STEP_EVENTS = 1 << 16;
        const CAN_GENERATE_EXCEPTION_EVENTS = 1 << 17;
        const CAN_GENERATE_FRAME_POP_EVENTS = 1 << 18;
        const CAN_GENERATE_BREAKPOINT_EVENTS = 1 << 19;
        const CAN_SUSPEND = 1 << 20;
        const CAN_REDEFINE_ANY_CLASS = 1 << 21;
        const CAN_GET_CURRENT_THREAD_CPU_TIME = 1 << 22;
        const CAN_GET_THREAD_CPU_TIME = 1 << 23;
        const CAN_GENERATE_METHOD_ENTRY_EVENTS = 1 << 24;
        const CAN_GENERATE_METHOD_EXIT_EVENTS = 1 << 25;
        const CAN_GENERATE_ALL_CLASS_HOOK_EVENTS = 1 << 26;
        const CAN_GENERATE_COMPILED_METHOD_LOAD_EVENTS = 1 << 27;
        const CAN_GENERATE_MONITOR_EVENTS = 1 << 28;
        const CAN_GENERATE_VM_OBJECT_ALLOC_EVENTS = 1 << 29;
        const CAN_GENERATE_NATIVE_METHOD_BIND_EVENTS = 1 << 30;
        const CAN_GENERATE_GARBAGE_COLLECTION_EVENTS = 1 << 31;
        const CAN_GENERATE_OBJECT_FREE_EVENTS = 1 << 32; // 33rd capability
        const CAN_FORCE_EARLY_RETURN = 1 << 33; // 34th capability
        const CAN_GET_OWNED_MONITOR_STACK_DEPTH_INFO = 1 << 34;
        const CAN_GET_CONSTANT_POOL = 1 << 35;
        const CAN_SET_NATIVE_METHOD_PREFIX = 1 << 36;
        const CAN_RETRANSFORM_CLASSES = 1 << 37;
        const CAN_RETRANSFORM_ANY_CLASS = 1 << 38;
        const CAN_GENERATE_RESOURCE_EXHAUSTION_HEAP_EVENTS = 1 << 39;
        const CAN_GENERATE_RESOURCE_EXHAUSTION_THREADS_EVENTS = 1 << 40;
        // TODO: Think how to represent new fields better
        const CAN_GENERATE_EARLY_VMSTART = 1 << 41;
        const CAN_GENERATE_EARLY_CLASS_HOOK_EVENTS = 1 << 42;
        const CAN_GENERATE_SAMPLED_OBJECT_ALLOC_EVENTS = 1 << 43;
        const CAN_SUPPORT_VIRTUAL_THREADS = 1 << 44;

    }
}

/* Event Definitions */

pub type jvmtiEventReserved = extern "system" fn();
pub type jvmtiEventBreakpoint = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
);

pub type jvmtiEventClassFileLoadHook = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    class_being_redefined: jclass,
    loader: jobject,
    name: *const c_char,
    protection_domain: jobject,
    class_data_len: jint,
    class_data: *const c_uchar,
    new_class_data_len: *mut jint,
    new_class_data: *mut *mut c_uint,
);

pub type jvmtiEventClassLoad = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    klass: jclass,
);

pub type jvmtiEventClassPrepare = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    klass: jclass,
);

pub type jvmtiEventCompiledMethodLoad = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    code_size: jint,
    code_addr: *const c_void,
    map_length: jint,
    map: *const jvmtiAddrLocationMap,
    compile_info: *const c_void,
);

pub type jvmtiEventCompiledMethodUnload = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    method: jmethodID,
    code_addr: *const c_void,
);

pub type jvmtiEventDataDumpRequest = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);

pub type jvmtiEventDynamicCodeGenerated = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    name: *const c_char,
    address: *const c_void,
    length: jint,
);

pub type jvmtiEventException = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    exception: jobject,
    catch_method: jmethodID,
    catch_location: jlocation,
);

pub type jvmtiEventExceptionCatch = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    exception: jobject,
);

pub type jvmtiEventFieldAccess = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    field_klass: jclass,
    object: jobject,
    field: jfieldID,
);

pub type jvmtiEventFieldModification = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
    field_klass: jclass,
    object: jobject,
    field: jfieldID,
    signature_type: c_char,
    new_value: jvalue,
);

pub type jvmtiEventFramePop = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    was_popped_by_exception: jboolean,
);

pub type jvmtiEventGarbageCollectionFinish = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);
pub type jvmtiEventGarbageCollectionStart = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv);
pub type jvmtiEventMethodEntry = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
);

pub type jvmtiEventMethodExit = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    was_popped_by_exception: jboolean,
    return_value: jvalue,
);

pub type jvmtiEventMonitorContendedEnter = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
);

pub type jvmtiEventMonitorContendedEntered = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
);
pub type jvmtiEventMonitorWait = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
    timeout: jlong,
);

pub type jvmtiEventMonitorWaited = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
    timed_out: jboolean,
);

pub type jvmtiEventNativeMethodBind = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: *mut c_void,
    address: *mut c_void,
    new_address_ptr: *mut *mut c_void,
);

pub type jvmtiEventObjectFree = unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, tag: jlong);
pub type jvmtiEventResourceExhausted = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    flags: JVMTI_RESOURCE_EXHAUSTED_FLAGS,
    reserved: *const c_void,
    description: *const c_char,
);

pub type jvmtiEventSampledObjectAlloc = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
    object_class: jclass,
    size: jlong,
);

pub type jvmtiEventSingleStep = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    method: jmethodID,
    location: jlocation,
);

pub type jvmtiEventThreadEnd =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventThreadStart =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVirtualThreadStart =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVirtualThreadEnd =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVMDeath =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv);

pub type jvmtiEventVMInit =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread);

pub type jvmtiEventVMObjectAlloc = unsafe extern "system" fn(
    jvmti_env: *mut jvmtiEnv,
    jni_env: *mut JNIEnv,
    thread: jthread,
    object: jobject,
    object_klass: jclass,
    size: jlong,
);

pub type jvmtiEventVMStart =
    unsafe extern "system" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv);

/* Event Callback Structure */

#[jni_to_union]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct jvmtiEventCallbacks {
    /*   50 : VM Initialization Event */
    #[jni_added("1.0")]
    pub VMInit: Option<jvmtiEventVMInit>,
    /*   51 : VM Death Event */
    #[jni_added("1.0")]
    pub VMDeath: Option<jvmtiEventVMDeath>,
    /*   52 : Thread Start */
    #[jni_added("1.0")]
    pub ThreadStart: Option<jvmtiEventThreadStart>,
    /*   53 : Thread End */
    #[jni_added("1.0")]
    pub ThreadEnd: Option<jvmtiEventThreadEnd>,
    /*   54 : Class File Load Hook */
    #[jni_added("1.0")]
    pub ClassFileLoadHook: Option<jvmtiEventClassFileLoadHook>,
    /*   55 : Class Load */
    #[jni_added("1.0")]
    pub ClassLoad: Option<jvmtiEventClassLoad>,
    /*   56 : Class Prepare */
    #[jni_added("1.0")]
    pub ClassPrepare: Option<jvmtiEventClassPrepare>,
    /*   57 : VM Start Event */
    #[jni_added("1.0")]
    pub VMStart: Option<jvmtiEventVMStart>,
    /*   58 : Exception */
    #[jni_added("1.0")]
    pub Exception: Option<jvmtiEventException>,
    /*   59 : Exception Catch */
    #[jni_added("1.0")]
    pub ExceptionCatch: Option<jvmtiEventExceptionCatch>,
    /*   60 : Single Step */
    #[jni_added("1.0")]
    pub SingleStep: Option<jvmtiEventSingleStep>,
    /*   61 : Frame Pop */
    #[jni_added("1.0")]
    pub FramePop: Option<jvmtiEventFramePop>,
    /*   62 : Breakpoint */
    #[jni_added("1.0")]
    pub Breakpoint: Option<jvmtiEventBreakpoint>,
    /*   63 : Field Access */
    #[jni_added("1.0")]
    pub FieldAccess: Option<jvmtiEventFieldAccess>,
    /*   64 : Field Modification */
    #[jni_added("1.0")]
    pub FieldModification: Option<jvmtiEventFieldModification>,
    /*   65 : Method Entry */
    #[jni_added("1.0")]
    pub MethodEntry: Option<jvmtiEventMethodEntry>,
    /*   66 : Method Exit */
    #[jni_added("1.0")]
    pub MethodExit: Option<jvmtiEventMethodExit>,
    /*   67 : Native Method Bind */
    #[jni_added("1.0")]
    pub NativeMethodBind: Option<jvmtiEventNativeMethodBind>,
    /*   68 : Compiled Method Load */
    #[jni_added("1.0")]
    pub CompiledMethodLoad: Option<jvmtiEventCompiledMethodLoad>,
    /*   69 : Compiled Method Unload */
    #[jni_added("1.0")]
    pub CompiledMethodUnload: Option<jvmtiEventCompiledMethodUnload>,
    /*   70 : Dynamic Code Generated */
    #[jni_added("1.0")]
    pub DynamicCodeGenerated: Option<jvmtiEventDynamicCodeGenerated>,
    /*   71 : Data Dump Request */
    #[jni_added("1.0")]
    pub DataDumpRequest: Option<jvmtiEventDataDumpRequest>,
    /*   72 */
    pub reserved72: Option<jvmtiEventReserved>,
    /*   73 : Monitor Wait */
    #[jni_added("1.0")]
    pub MonitorWait: Option<jvmtiEventMonitorWait>,
    /*   74 : Monitor Waited */
    #[jni_added("1.0")]
    pub MonitorWaited: Option<jvmtiEventMonitorWaited>,
    /*   75 : Monitor Contended Enter */
    #[jni_added("1.0")]
    pub MonitorContendedEnter: Option<jvmtiEventMonitorContendedEnter>,
    /*   76 : Monitor Contended Entered */
    #[jni_added("1.0")]
    pub MonitorContendedEntered: Option<jvmtiEventMonitorContendedEntered>,
    /*   77 */
    pub reserved77: Option<jvmtiEventReserved>,
    /*   78 */
    pub reserved78: Option<jvmtiEventReserved>,
    /*   79 */
    pub reserved79: Option<jvmtiEventReserved>,
    /*   80 : Resource Exhausted */
    #[jni_added("1.1")]
    pub ResourceExhausted: Option<jvmtiEventResourceExhausted>,
    /*   81 : Garbage Collection Start */
    #[jni_added("1.0")]
    pub GarbageCollectionStart: Option<jvmtiEventGarbageCollectionStart>,
    /*   82 : Garbage Collection Finish */
    #[jni_added("1.0")]
    pub GarbageCollectionFinish: Option<jvmtiEventGarbageCollectionFinish>,
    /*   83 : Object Free */
    #[jni_added("1.0")]
    pub ObjectFree: Option<jvmtiEventObjectFree>,
    /*   84 : VM Object Allocation */
    #[jni_added("1.0")]
    pub VMObjectAlloc: Option<jvmtiEventVMObjectAlloc>,
    #[jni_added("reserved")]
    pub reserved85: Option<jvmtiEventReserved>,
    /*   86 : Sampled Object Allocation */
    #[jni_added("11")]
    pub SampledObjectAlloc: Option<jvmtiEventSampledObjectAlloc>,
    /*   87 : Virtual Thread Start */
    #[jni_added("21")]
    pub VirtualThreadStart: Option<jvmtiEventVirtualThreadStart>,
    /*   88 : Virtual Thread End */
    #[jni_added("21")]
    pub VirtualThreadEnd: Option<jvmtiEventVirtualThreadEnd>,
}

#[jni_to_union]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct jvmtiInterface_1_ {
    #[jni_added("reserved")]
    pub reserved1: *mut c_void,
    #[jni_added("1.0")]
    pub SetEventNotificationMode: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        mode: jvmtiEventMode,
        event_type: jvmtiEvent,
        event_thread: jthread,
        ...
    ) -> jvmtiError,
    #[jni_added("9")]
    pub GetAllModules: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module_count_ptr: *mut jint,
        modules_ptr: *mut *mut jthread,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetAllThreads: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        threads_count_ptr: *mut jint,
        threads_ptr: *mut *mut jthread,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SuspendThread: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub ResumeThread: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub StopThread: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        exception: jobject,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub InterruptThread:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadInfo: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        info_ptr: *mut jvmtiThreadInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetOwnedMonitorInfo: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        owned_monitor_count_ptr: *mut jint,
        owned_monitors_ptr: *mut *mut jobject,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub GetCurrentContendedMonitor: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        monitor_ptr: *mut jobject,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub RunAgentThread: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        _proc: jvmtiStartFunction,
        arg: *const c_void,
        priority: jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetTopThreadGroups: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        group_count_ptr: *mut jint,
        groups_ptr: *mut *mut jthreadGroup,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadGroupInfo: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        group: jthreadGroup,
        info_ptr: *mut jvmtiThreadGroupInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadGroupChildren: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        group: jthreadGroup,
        thread_count_ptr: *mut jint,
        threads_ptr: *mut *mut jthread,
        group_count_ptr: *mut jint,
        groups_ptr: *mut *mut jthreadGroup,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetFrameCount: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        count_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadState: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        thread_state_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetCurrentThread:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetFrameLocation: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        method_ptr: *mut jmethodID,
        location_ptr: *mut jlocation,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub NotifyFramePop:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalObject: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jobject,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalInt: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalLong: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalFloat: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jfloat,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalDouble: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jdouble,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetLocalObject: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jobject,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub SetLocalInt: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetLocalLong: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jlong,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetLocalFloat: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jfloat,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetLocalDouble: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jdouble,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub CreateRawMonitor: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        name: *const c_char,
        monitor_ptr: *mut jrawMonitorID,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub DestroyRawMonitor:
        unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    #[jni_added("1.0")]
    pub RawMonitorEnter:
        unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    #[jni_added("1.0")]
    pub RawMonitorExit:
        unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    #[jni_added("1.0")]
    pub RawMonitorWait: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        monitor: jrawMonitorID,
        millis: jlong,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub RawMonitorNotify:
        unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    #[jni_added("1.0")]
    pub RawMonitorNotifyAll:
        unsafe extern "system" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetBreakpoint: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        location: jlocation,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub ClearBreakpoint: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        location: jlocation,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub GetNamedModule: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        class_loader: jobject,
        package_name: *const c_char,
        module_ptr: *mut jobject,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetFieldAccessWatch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    #[jni_added("1.0")]
    pub ClearFieldAccessWatch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetFieldModificationWatch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    #[jni_added("1.0")]
    pub ClearFieldModificationWatch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    #[jni_added("1.1")]
    pub IsModifiableClass: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_modifiable_class_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub Allocate: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        size: jlong,
        mem_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub Deallocate: unsafe extern "system" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassSignature: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassStatus: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        status_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetSourceFileName: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        source_name_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassModifiers: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassMethods: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        method_count_ptr: *mut jint,
        methods_ptr: *mut *mut jmethodID,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassFields: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field_count_ptr: *mut jint,
        fields_ptr: *mut *mut jfieldID,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetImplementedInterfaces: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        interface_count_ptr: *mut jint,
        interfaces_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsInterface: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_interface_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsArrayClass: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_array_class_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassLoader: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        classloader_ptr: *mut jobject,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetObjectHashCode: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        hash_code_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetObjectMonitorUsage: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        info_ptr: *mut jvmtiMonitorUsage,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetFieldName: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        name_ptr: *mut *mut c_char,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetFieldDeclaringClass: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        declaring_class_ptr: *mut jclass,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetFieldModifiers: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsFieldSynthetic: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        is_synthetic_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetMethodName: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        name_ptr: *mut *mut c_char,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetMethodDeclaringClass: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        declaring_class_ptr: *mut jclass,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetMethodModifiers: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("25")]
    pub ClearAllFramePops:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetMaxLocals: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        max_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetArgumentsSize: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        size_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLineNumberTable: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        entry_count_ptr: *mut jint,
        table_ptr: *mut *mut jvmtiLineNumberEntry,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetMethodLocation: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        start_location_ptr: *mut jlocation,
        end_location_ptr: *mut jlocation,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLocalVariableTable: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        entry_count_ptr: *mut jint,
        table_ptr: *mut *mut jvmtiLocalVariableEntry,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub SetNativeMethodPrefix:
        unsafe extern "system" fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError,
    #[jni_added("1.1")]
    pub SetNativeMethodPrefixes: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        prefix_count: jint,
        prefixes: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetBytecodes: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        bytecode_count_ptr: *mut jint,
        bytecodes_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsMethodNative: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_native_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsMethodSynthetic: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_synthetic_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetLoadedClasses: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        class_count_ptr: *mut jint,
        classes_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetClassLoaderClasses: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        initiating_loader: jobject,
        class_count_ptr: *mut jint,
        classes_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub PopFrame: unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnObject: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        value: jobject,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnInt:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnLong:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnFloat:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnDouble: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        value: jdouble,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnVoid:
        unsafe extern "system" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.0")]
    pub RedefineClasses: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        class_count: jint,
        class_definitions: *const jvmtiClassDefinition,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetVersionNumber:
        unsafe extern "system" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetCapabilities: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *mut jvmtiCapabilities,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetSourceDebugExtension: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        source_debug_extension_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IsMethodObsolete: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_obsolete_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SuspendThreadList: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        request_count: jint,
        request_list: *const jthread,
        results: *mut jvmtiError,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub ResumeThreadList: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        request_count: jint,
        request_list: *const jthread,
        results: *mut jvmtiError,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleReads: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        to_module: jobject,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleExports: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        pkg_name: *const c_char,
        to_module: jobject,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleOpens: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        pkg_name: *const c_char,
        to_module: jobject,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleUses: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        service: jclass,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleProvides: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        service: jclass,
        impl_class: jclass,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub IsModifiableModule: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        is_modifiable_module_ptr: *mut jboolean,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetAllStackTraces: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        max_frame_count: jint,
        stack_info_ptr: *mut *mut jvmtiStackInfo,
        thread_count_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadListStackTraces: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread_count: jint,
        thread_list: *const jthread,
        max_frame_count: jint,
        stack_info_ptr: *mut *mut jvmtiStackInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadLocalStorage: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        data_ptr: *mut *mut c_void,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetThreadLocalStorage: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetStackTrace: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        start_depth: jint,
        max_frame_count: jint,
        frame_buffer: *mut jvmtiFrameInfo,
        count_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved105: *mut c_void,
    #[jni_added("1.0")]
    pub GetTag: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        tag_ptr: *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetTag:
        unsafe extern "system" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError,
    #[jni_added("1.0")]
    pub ForceGarbageCollection: unsafe extern "system" fn(env: *mut jvmtiEnv) -> jvmtiError,
    #[jni_added("1.0")]
    pub IterateOverObjectsReachableFromObject: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        object_reference_callback: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IterateOverReachableObjects: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        heap_root_callback: jvmtiHeapRootCallback,
        stack_ref_callback: jvmtiStackReferenceCallback,
        object_ref_callback: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IterateOverHeap: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub IterateOverInstancesOfClass: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved113: *mut c_void,
    #[jni_added("1.0")]
    pub GetObjectsWithTags: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        tag_count: jint,
        tags: *const jlong,
        count_ptr: *mut jint,
        object_result_ptr: *mut *mut jobject,
        tag_result_ptr: *mut *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub FollowReferences: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        heap_filter: jint,
        klass: jclass,
        initial_object: jobject,
        callbacks: *const jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub IterateThroughHeap: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        heap_filter: jint,
        klass: jclass,
        callbacks: *const jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved117: *mut c_void,
    #[jni_added("21")]
    pub SuspendAllVirtualThreads: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        except_count: jint,
        except_list: *const jthread,
    ) -> jvmtiError,
    #[jni_added("21")]
    pub ResumeAllVirtualThreads: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        except_count: jint,
        except_list: *const jthread,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetJNIFunctionTable: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        function_table: *const JNINativeInterface_,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetJNIFunctionTable: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        function_table: *mut *mut JNINativeInterface_,
    ) -> jvmtiError,

    #[jni_added("1.0")]
    pub SetEventCallbacks: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        callbacks: *const jvmtiEventCallbacks,
        size_of_callbacks: jint,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GenerateEvents:
        unsafe extern "system" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetExtensionFunctions: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        extension_count_ptr: *mut jint,
        extensions: *mut *mut jvmtiExtensionFunctionInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetExtensionEvents: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        extension_count_ptr: *mut jint,
        extensions: *mut *mut jvmtiExtensionEventInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetExtensionEventCallback: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        extension_event_index: jint,
        callback: jvmtiExtensionEvent,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub DisposeEnvironment: unsafe extern "system" fn(env: *mut jvmtiEnv) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetErrorName: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        error: jvmtiError,
        name_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetJLocationFormat: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        format_ptr: *mut jvmtiJlocationFormat,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetSystemProperties: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        count_ptr: *mut jint,
        property_ptr: *mut *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetSystemProperty: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        property: *const c_char,
        value_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetSystemProperty: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        property: *const c_char,
        value: *const c_char,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetPhase:
        unsafe extern "system" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetCurrentThreadCpuTimerInfo:
        unsafe extern "system" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetCurrentThreadCpuTime:
        unsafe extern "system" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadCpuTimerInfo:
        unsafe extern "system" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetThreadCpuTime: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        nanos_ptr: *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetTimerInfo:
        unsafe extern "system" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetTime: unsafe extern "system" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetPotentialCapabilities: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *mut jvmtiCapabilities,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved141: *mut c_void,
    #[jni_added("1.0")]
    pub AddCapabilities: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *const jvmtiCapabilities,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub RelinquishCapabilities: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *const jvmtiCapabilities,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetAvailableProcessors:
        unsafe extern "system" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetClassVersionNumbers: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        minor_version_ptr: *mut jint,
        major_version_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetConstantPool: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        constant_pool_count_ptr: *mut jint,
        constant_pool_byte_count_ptr: *mut jint,
        constant_pool_bytes_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetEnvironmentLocalStorage:
        unsafe extern "system" fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError,
    #[jni_added("1.0")]
    pub SetEnvironmentLocalStorage:
        unsafe extern "system" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError,
    #[jni_added("1.0")]
    pub AddToBootstrapClassLoaderSearch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
    #[jni_added("1.1")]
    pub SetVerboseFlag: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        flag: jvmtiVerboseFlag,
        value: jboolean,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub AddToSystemClassLoaderSearch:
        unsafe extern "system" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
    #[jni_added("1.1")]
    pub RetransformClasses: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        class_count: jint,
        classes: *const jclass,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetOwnedMonitorStackDepthInfo: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        monitor_info_count_ptr: *mut jint,
        monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
    ) -> jvmtiError,
    #[jni_added("1.0")]
    pub GetObjectSize: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        size_ptr: *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.2")]
    pub GetLocalInstance: unsafe extern "system" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        value_ptr: *mut jobject,
    ) -> jvmtiError,
    #[jni_added("11")]
    pub SetHeapSamplingInterval:
        unsafe extern "system" fn(env: *mut jvmtiEnv, sampling_interval: jint) -> jvmtiError,
}

#[repr(C)]
#[derive(Copy, Debug)]
pub struct _jvmtiEnv {
    pub functions: *const jvmtiInterface_1_,
}

impl Clone for _jvmtiEnv {
    fn clone(&self) -> Self {
        *self
    }
}
