#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)] 
extern crate libc;
/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_TLimits {
    pub nonInductiveForLoops: u8,
    pub whileLoops: u8,
    pub doWhileLoops: u8,
    pub generalUniformIndexing: u8,
    pub generalAttributeMatrixVectorIndexing: u8,
    pub generalVaryingIndexing: u8,
    pub generalSamplerIndexing: u8,
    pub generalVariableIndexing: u8,
    pub generalConstantMatrixVectorIndexing: u8,
}
impl ::std::clone::Clone for Struct_TLimits {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_TLimits {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type TLimits = Struct_TLimits;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_TBuiltInResource {
    pub maxLights: ::libc::c_int,
    pub maxClipPlanes: ::libc::c_int,
    pub maxTextureUnits: ::libc::c_int,
    pub maxTextureCoords: ::libc::c_int,
    pub maxVertexAttribs: ::libc::c_int,
    pub maxVertexUniformComponents: ::libc::c_int,
    pub maxVaryingFloats: ::libc::c_int,
    pub maxVertexTextureImageUnits: ::libc::c_int,
    pub maxCombinedTextureImageUnits: ::libc::c_int,
    pub maxTextureImageUnits: ::libc::c_int,
    pub maxFragmentUniformComponents: ::libc::c_int,
    pub maxDrawBuffers: ::libc::c_int,
    pub maxVertexUniformVectors: ::libc::c_int,
    pub maxVaryingVectors: ::libc::c_int,
    pub maxFragmentUniformVectors: ::libc::c_int,
    pub maxVertexOutputVectors: ::libc::c_int,
    pub maxFragmentInputVectors: ::libc::c_int,
    pub minProgramTexelOffset: ::libc::c_int,
    pub maxProgramTexelOffset: ::libc::c_int,
    pub maxClipDistances: ::libc::c_int,
    pub maxComputeWorkGroupCountX: ::libc::c_int,
    pub maxComputeWorkGroupCountY: ::libc::c_int,
    pub maxComputeWorkGroupCountZ: ::libc::c_int,
    pub maxComputeWorkGroupSizeX: ::libc::c_int,
    pub maxComputeWorkGroupSizeY: ::libc::c_int,
    pub maxComputeWorkGroupSizeZ: ::libc::c_int,
    pub maxComputeUniformComponents: ::libc::c_int,
    pub maxComputeTextureImageUnits: ::libc::c_int,
    pub maxComputeImageUniforms: ::libc::c_int,
    pub maxComputeAtomicCounters: ::libc::c_int,
    pub maxComputeAtomicCounterBuffers: ::libc::c_int,
    pub maxVaryingComponents: ::libc::c_int,
    pub maxVertexOutputComponents: ::libc::c_int,
    pub maxGeometryInputComponents: ::libc::c_int,
    pub maxGeometryOutputComponents: ::libc::c_int,
    pub maxFragmentInputComponents: ::libc::c_int,
    pub maxImageUnits: ::libc::c_int,
    pub maxCombinedImageUnitsAndFragmentOutputs: ::libc::c_int,
    pub maxCombinedShaderOutputResources: ::libc::c_int,
    pub maxImageSamples: ::libc::c_int,
    pub maxVertexImageUniforms: ::libc::c_int,
    pub maxTessControlImageUniforms: ::libc::c_int,
    pub maxTessEvaluationImageUniforms: ::libc::c_int,
    pub maxGeometryImageUniforms: ::libc::c_int,
    pub maxFragmentImageUniforms: ::libc::c_int,
    pub maxCombinedImageUniforms: ::libc::c_int,
    pub maxGeometryTextureImageUnits: ::libc::c_int,
    pub maxGeometryOutputVertices: ::libc::c_int,
    pub maxGeometryTotalOutputComponents: ::libc::c_int,
    pub maxGeometryUniformComponents: ::libc::c_int,
    pub maxGeometryVaryingComponents: ::libc::c_int,
    pub maxTessControlInputComponents: ::libc::c_int,
    pub maxTessControlOutputComponents: ::libc::c_int,
    pub maxTessControlTextureImageUnits: ::libc::c_int,
    pub maxTessControlUniformComponents: ::libc::c_int,
    pub maxTessControlTotalOutputComponents: ::libc::c_int,
    pub maxTessEvaluationInputComponents: ::libc::c_int,
    pub maxTessEvaluationOutputComponents: ::libc::c_int,
    pub maxTessEvaluationTextureImageUnits: ::libc::c_int,
    pub maxTessEvaluationUniformComponents: ::libc::c_int,
    pub maxTessPatchComponents: ::libc::c_int,
    pub maxPatchVertices: ::libc::c_int,
    pub maxTessGenLevel: ::libc::c_int,
    pub maxViewports: ::libc::c_int,
    pub maxVertexAtomicCounters: ::libc::c_int,
    pub maxTessControlAtomicCounters: ::libc::c_int,
    pub maxTessEvaluationAtomicCounters: ::libc::c_int,
    pub maxGeometryAtomicCounters: ::libc::c_int,
    pub maxFragmentAtomicCounters: ::libc::c_int,
    pub maxCombinedAtomicCounters: ::libc::c_int,
    pub maxAtomicCounterBindings: ::libc::c_int,
    pub maxVertexAtomicCounterBuffers: ::libc::c_int,
    pub maxTessControlAtomicCounterBuffers: ::libc::c_int,
    pub maxTessEvaluationAtomicCounterBuffers: ::libc::c_int,
    pub maxGeometryAtomicCounterBuffers: ::libc::c_int,
    pub maxFragmentAtomicCounterBuffers: ::libc::c_int,
    pub maxCombinedAtomicCounterBuffers: ::libc::c_int,
    pub maxAtomicCounterBufferSize: ::libc::c_int,
    pub maxTransformFeedbackBuffers: ::libc::c_int,
    pub maxTransformFeedbackInterleavedComponents: ::libc::c_int,
    pub maxCullDistances: ::libc::c_int,
    pub maxCombinedClipAndCullDistances: ::libc::c_int,
    pub maxSamples: ::libc::c_int,
    pub limits: TLimits,
}
impl ::std::clone::Clone for Struct_TBuiltInResource {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_TBuiltInResource {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type TBuiltInResource = Struct_TBuiltInResource;
pub type Enum_EShLanguage = ::libc::c_uint;
pub const EShLangVertex: ::libc::c_uint = 0;
pub const EShLangTessControl: ::libc::c_uint = 1;
pub const EShLangTessEvaluation: ::libc::c_uint = 2;
pub const EShLangGeometry: ::libc::c_uint = 3;
pub const EShLangFragment: ::libc::c_uint = 4;
pub const EShLangCompute: ::libc::c_uint = 5;
pub const EShLangCount: ::libc::c_uint = 6;
pub type EShLanguage = Enum_EShLanguage;
pub type Enum_EShLanguageMask = ::libc::c_uint;
pub const EShLangVertexMask: ::libc::c_uint = 1;
pub const EShLangTessControlMask: ::libc::c_uint = 2;
pub const EShLangTessEvaluationMask: ::libc::c_uint = 4;
pub const EShLangGeometryMask: ::libc::c_uint = 8;
pub const EShLangFragmentMask: ::libc::c_uint = 16;
pub const EShLangComputeMask: ::libc::c_uint = 32;
pub type EShLanguageMask = Enum_EShLanguageMask;
pub type Enum_EShExecutable = ::libc::c_uint;
pub const EShExVertexFragment: ::libc::c_uint = 0;
pub const EShExFragment: ::libc::c_uint = 1;
pub type EShExecutable = Enum_EShExecutable;
pub type Enum_EShOptimizationLevel = ::libc::c_uint;
pub const EShOptNoGeneration: ::libc::c_uint = 0;
pub const EShOptNone: ::libc::c_uint = 1;
pub const EShOptSimple: ::libc::c_uint = 2;
pub const EShOptFull: ::libc::c_uint = 3;
pub type EShOptimizationLevel = Enum_EShOptimizationLevel;
pub type Enum_EShMessages = ::libc::c_uint;
pub const EShMsgDefault: ::libc::c_uint = 0;
pub const EShMsgRelaxedErrors: ::libc::c_uint = 1;
pub const EShMsgSuppressWarnings: ::libc::c_uint = 2;
pub const EShMsgAST: ::libc::c_uint = 4;
pub type EShMessages = Enum_EShMessages;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ShBinding {
    pub name: *const ::libc::c_char,
    pub binding: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ShBinding {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ShBinding {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ShBinding = Struct_ShBinding;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ShBindingTable {
    pub numBindings: ::libc::c_int,
    pub bindings: *mut ShBinding,
}
impl ::std::clone::Clone for Struct_ShBindingTable {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ShBindingTable {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ShBindingTable = Struct_ShBindingTable;
pub type ShHandle = *mut ::libc::c_void;
extern "C" {
    pub fn ShInitialize() -> ::libc::c_int;
    pub fn ShFinalize() -> ::libc::c_int;
    pub fn StageName(arg1: EShLanguage) -> *const ::libc::c_char;
    pub fn ShConstructCompiler(arg1: EShLanguage, debugOptions: ::libc::c_int)
     -> ShHandle;
    pub fn ShConstructLinker(arg1: EShExecutable, debugOptions: ::libc::c_int)
     -> ShHandle;
    pub fn ShConstructUniformMap() -> ShHandle;
    pub fn ShDestruct(arg1: ShHandle) -> ();
    pub fn ShCompile(arg1: ShHandle,
                     shaderStrings: *const *const ::libc::c_char,
                     numStrings: ::libc::c_int, lengths: *const ::libc::c_int,
                     arg2: EShOptimizationLevel,
                     resources: *const TBuiltInResource,
                     debugOptions: ::libc::c_int,
                     defaultVersion: ::libc::c_int, forwardCompatible: u8,
                     messages: EShMessages) -> ::libc::c_int;
    pub fn ShLink(arg1: ShHandle, h: *const ShHandle,
                  numHandles: ::libc::c_int, uniformMap: ShHandle,
                  uniformsAccessed: *mut *mut ::libc::c_short,
                  numUniformsAccessed: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ShLinkExt(arg1: ShHandle, h: *const ShHandle,
                     numHandles: ::libc::c_int) -> ::libc::c_int;
    pub fn ShSetEncryptionMethod(arg1: ShHandle) -> ();
    pub fn ShGetInfoLog(arg1: ShHandle) -> *const ::libc::c_char;
    pub fn ShGetExecutable(arg1: ShHandle) -> *const ::libc::c_void;
    pub fn ShSetVirtualAttributeBindings(arg1: ShHandle,
                                         arg2: *const ShBindingTable)
     -> ::libc::c_int;
    pub fn ShSetFixedAttributeBindings(arg1: ShHandle,
                                       arg2: *const ShBindingTable)
     -> ::libc::c_int;
    pub fn ShGetPhysicalAttributeBindings(arg1: ShHandle,
                                          arg2: *mut *const ShBindingTable)
     -> ::libc::c_int;
    pub fn ShExcludeAttributes(arg1: ShHandle, attributes: *mut ::libc::c_int,
                               count: ::libc::c_int) -> ::libc::c_int;
    pub fn ShGetUniformLocation(uniformMap: ShHandle,
                                name: *const ::libc::c_char) -> ::libc::c_int;
}

#[test]
fn it_initializes_and_finalizes() {
    unsafe {
        ShInitialize();
        ShFinalize();
    }
}
