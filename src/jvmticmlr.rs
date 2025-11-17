use core::ffi::{c_char, c_void};
use jni_sys::{jint, jmethodID};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum jvmtiCMLRKind {
    JVMTI_CMLR_DUMMY = 1,
    JVMTI_CMLR_INLINE_INFO = 2,
}

pub type jvmtiCompiledMethodLoadRecordHeader = _jvmtiCompiledMethodLoadRecordHeader;
/*
 * Record that represents arbitrary information passed through JVMTI
 * CompiledMethodLoadEvent void pointer.
 */
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct _jvmtiCompiledMethodLoadRecordHeader {
    kind: jvmtiCMLRKind,    /* id for the kind of info passed in the record */
    majorinfoversion: jint, /* major and minor info version values. Init'ed */
    minorinfoversion: jint, /* to current version value in jvmtiExport.cpp. */
    next: *mut _jvmtiCompiledMethodLoadRecordHeader,
}

pub type PCStackInfo = _PCStackInfo;
/*
 * Record that gives information about the methods on the compile-time
 * stack at a specific pc address of a compiled method. Each element in
 * the methods array maps to same element in the bcis array.
*/
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct _PCStackInfo {
    pc: *mut c_void,         /* the pc address for this compiled method */
    numstackframes: jint,    /* number of methods on the stack */
    methods: *mut jmethodID, /* array of numstackframes method ids */
    bcis: *mut jint,         /* array of numstackframes bytecode indices */
}

pub type jvmtiCompiledMethodLoadInlineRecord = _jvmtiCompiledMethodLoadInlineRecord;

/*
 * Record that contains inlining information for each pc address of
 * an nmethod.
*/
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct _jvmtiCompiledMethodLoadInlineRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader, /* common header for casting */
    pub numpcs: jint,                                /* number of pc descriptors in this nmethod */
    pub pcinfo: *mut PCStackInfo,                    /* array of numpcs pc descriptors */
}

pub type jvmtiCompiledMethodLoadDummyRecord = _jvmtiCompiledMethodLoadDummyRecord;

/*
 * Dummy record used to test that we can pass records with different
 * information through the void pointer provided that they can be cast
 * to a jvmtiCompiledMethodLoadRecordHeader.
*/
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct _jvmtiCompiledMethodLoadDummyRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader, /* common header for casting */
    pub message: [c_char; 50],
}
