#![doc = r" SPIR-V enums"]
#![doc = r""]
#![doc = r" This file is auto-generated, do not modify manually."]
use crate::{core::*, module::*, op::*, stream::*};
use bitflags::bitflags;
use core::fmt::{self, Debug, Display};
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct ImageOperands : u32 { const NONE = 0x0000 ; const BIAS = 0x0001 ; const LOD = 0x0002 ; const GRAD = 0x0004 ; const CONST_OFFSET = 0x0008 ; const OFFSET = 0x0010 ; const CONST_OFFSETS = 0x0020 ; const SAMPLE = 0x0040 ; const MIN_LOD = 0x0080 ; const MAKE_TEXEL_AVAILABLE = 0x0100 ; const MAKE_TEXEL_VISIBLE = 0x0200 ; const NON_PRIVATE_TEXEL = 0x0400 ; const VOLATILE_TEXEL = 0x0800 ; const SIGN_EXTEND = 0x1000 ; const ZERO_EXTEND = 0x2000 ; const NONTEMPORAL = 0x4000 ; const OFFSETS = 0x10000 ; } }
impl Word for ImageOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct FPFastMathMode : u32 { const NONE = 0x0000 ; const NOT_NA_N = 0x0001 ; const NOT_INF = 0x0002 ; const NSZ = 0x0004 ; const ALLOW_RECIP = 0x0008 ; const FAST = 0x0010 ; const ALLOW_CONTRACT = 0x10000 ; const ALLOW_REASSOC = 0x20000 ; const ALLOW_TRANSFORM = 0x40000 ; } }
impl Word for FPFastMathMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct SelectionControl : u32 { const NONE = 0x0000 ; const FLATTEN = 0x0001 ; const DONT_FLATTEN = 0x0002 ; } }
impl Word for SelectionControl {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct LoopControl : u32 { const NONE = 0x0000 ; const UNROLL = 0x0001 ; const DONT_UNROLL = 0x0002 ; const DEPENDENCY_INFINITE = 0x0004 ; const DEPENDENCY_LENGTH = 0x0008 ; const MIN_ITERATIONS = 0x0010 ; const MAX_ITERATIONS = 0x0020 ; const ITERATION_MULTIPLE = 0x0040 ; const PEEL_COUNT = 0x0080 ; const PARTIAL_COUNT = 0x0100 ; const INITIATION_INTERVAL_ALTERA = 0x10000 ; const MAX_CONCURRENCY_ALTERA = 0x20000 ; const DEPENDENCY_ARRAY_ALTERA = 0x40000 ; const PIPELINE_ENABLE_ALTERA = 0x80000 ; const LOOP_COALESCE_ALTERA = 0x100000 ; const MAX_INTERLEAVING_ALTERA = 0x200000 ; const SPECULATED_ITERATIONS_ALTERA = 0x400000 ; const NO_FUSION_ALTERA = 0x800000 ; const LOOP_COUNT_ALTERA = 0x1000000 ; const MAX_REINVOCATION_DELAY_ALTERA = 0x2000000 ; } }
impl Word for LoopControl {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct FunctionControl : u32 { const NONE = 0x0000 ; const INLINE = 0x0001 ; const DONT_INLINE = 0x0002 ; const PURE = 0x0004 ; const CONST = 0x0008 ; const OPT_NONE_EXT = 0x10000 ; } }
impl Word for FunctionControl {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct MemorySemantics : u32 { const RELAXED = 0x0000 ; const ACQUIRE = 0x0002 ; const RELEASE = 0x0004 ; const ACQUIRE_RELEASE = 0x0008 ; const SEQUENTIALLY_CONSISTENT = 0x0010 ; const UNIFORM_MEMORY = 0x0040 ; const SUBGROUP_MEMORY = 0x0080 ; const WORKGROUP_MEMORY = 0x0100 ; const CROSS_WORKGROUP_MEMORY = 0x0200 ; const ATOMIC_COUNTER_MEMORY = 0x0400 ; const IMAGE_MEMORY = 0x0800 ; const OUTPUT_MEMORY = 0x1000 ; const MAKE_AVAILABLE = 0x2000 ; const MAKE_VISIBLE = 0x4000 ; const VOLATILE = 0x8000 ; } }
impl Word for MemorySemantics {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct MemoryAccess : u32 { const NONE = 0x0000 ; const VOLATILE = 0x0001 ; const ALIGNED = 0x0002 ; const NONTEMPORAL = 0x0004 ; const MAKE_POINTER_AVAILABLE = 0x0008 ; const MAKE_POINTER_VISIBLE = 0x0010 ; const NON_PRIVATE_POINTER = 0x0020 ; const ALIAS_SCOPE_INTELMASK = 0x10000 ; const NO_ALIAS_INTELMASK = 0x20000 ; } }
impl Word for MemoryAccess {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct KernelProfilingInfo : u32 { const NONE = 0x0000 ; const CMD_EXEC_TIME = 0x0001 ; } }
impl Word for KernelProfilingInfo {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct RayFlags : u32 { const NONE_KHR = 0x0000 ; const OPAQUE_KHR = 0x0001 ; const NO_OPAQUE_KHR = 0x0002 ; const TERMINATE_ON_FIRST_HIT_KHR = 0x0004 ; const SKIP_CLOSEST_HIT_SHADER_KHR = 0x0008 ; const CULL_BACK_FACING_TRIANGLES_KHR = 0x0010 ; const CULL_FRONT_FACING_TRIANGLES_KHR = 0x0020 ; const CULL_OPAQUE_KHR = 0x0040 ; const CULL_NO_OPAQUE_KHR = 0x0080 ; const SKIP_TRIANGLES_KHR = 0x0100 ; const SKIP_AABBS_KHR = 0x0200 ; const FORCE_OPACITY_MICROMAP_2_STATE_EXT = 0x0400 ; } }
impl Word for RayFlags {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct FragmentShadingRate : u32 { const VERTICAL_2_PIXELS = 0x0001 ; const VERTICAL_4_PIXELS = 0x0002 ; const HORIZONTAL_2_PIXELS = 0x0004 ; const HORIZONTAL_4_PIXELS = 0x0008 ; } }
impl Word for FragmentShadingRate {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct RawAccessChainOperands : u32 { const NONE = 0x0000 ; const ROBUSTNESS_PER_COMPONENT_NV = 0x0001 ; const ROBUSTNESS_PER_ELEMENT_NV = 0x0002 ; } }
impl Word for RawAccessChainOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLanguage(pub(crate) u32);
impl SourceLanguage {
    pub const UNKNOWN: Self = Self(0u32);
    pub const ESSL: Self = Self(1u32);
    pub const GLSL: Self = Self(2u32);
    pub const OPEN_CL_C: Self = Self(3u32);
    pub const OPEN_CL_CPP: Self = Self(4u32);
    pub const HLSL: Self = Self(5u32);
    pub const CPP_FOR_OPEN_CL: Self = Self(6u32);
    pub const SYCL: Self = Self(7u32);
    pub const HERO_C: Self = Self(8u32);
    pub const NZSL: Self = Self(9u32);
    pub const WGSL: Self = Self(10u32);
    pub const SLANG: Self = Self(11u32);
    pub const ZIG: Self = Self(12u32);
    pub const RUST: Self = Self(13u32);
}
impl Word for SourceLanguage {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for SourceLanguage {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN => Display::fmt(stringify!(UNKNOWN), f),
            Self::ESSL => Display::fmt(stringify!(ESSL), f),
            Self::GLSL => Display::fmt(stringify!(GLSL), f),
            Self::OPEN_CL_C => Display::fmt(stringify!(OPEN_CL_C), f),
            Self::OPEN_CL_CPP => Display::fmt(stringify!(OPEN_CL_CPP), f),
            Self::HLSL => Display::fmt(stringify!(HLSL), f),
            Self::CPP_FOR_OPEN_CL => Display::fmt(stringify!(CPP_FOR_OPEN_CL), f),
            Self::SYCL => Display::fmt(stringify!(SYCL), f),
            Self::HERO_C => Display::fmt(stringify!(HERO_C), f),
            Self::NZSL => Display::fmt(stringify!(NZSL), f),
            Self::WGSL => Display::fmt(stringify!(WGSL), f),
            Self::SLANG => Display::fmt(stringify!(SLANG), f),
            Self::ZIG => Display::fmt(stringify!(ZIG), f),
            Self::RUST => Display::fmt(stringify!(RUST), f),
            x => write!(f, "{}({})", stringify!(SourceLanguage), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionModel(pub(crate) u32);
impl ExecutionModel {
    pub const VERTEX: Self = Self(0u32);
    pub const TESSELLATION_CONTROL: Self = Self(1u32);
    pub const TESSELLATION_EVALUATION: Self = Self(2u32);
    pub const GEOMETRY: Self = Self(3u32);
    pub const FRAGMENT: Self = Self(4u32);
    pub const GLCOMPUTE: Self = Self(5u32);
    pub const KERNEL: Self = Self(6u32);
    pub const TASK_NV: Self = Self(5267u32);
    pub const MESH_NV: Self = Self(5268u32);
    pub const RAY_GENERATION_KHR: Self = Self(5313u32);
    pub const INTERSECTION_KHR: Self = Self(5314u32);
    pub const ANY_HIT_KHR: Self = Self(5315u32);
    pub const CLOSEST_HIT_KHR: Self = Self(5316u32);
    pub const MISS_KHR: Self = Self(5317u32);
    pub const CALLABLE_KHR: Self = Self(5318u32);
    pub const TASK_EXT: Self = Self(5364u32);
    pub const MESH_EXT: Self = Self(5365u32);
}
impl Word for ExecutionModel {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ExecutionModel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VERTEX => Display::fmt(stringify!(VERTEX), f),
            Self::TESSELLATION_CONTROL => Display::fmt(stringify!(TESSELLATION_CONTROL), f),
            Self::TESSELLATION_EVALUATION => Display::fmt(stringify!(TESSELLATION_EVALUATION), f),
            Self::GEOMETRY => Display::fmt(stringify!(GEOMETRY), f),
            Self::FRAGMENT => Display::fmt(stringify!(FRAGMENT), f),
            Self::GLCOMPUTE => Display::fmt(stringify!(GLCOMPUTE), f),
            Self::KERNEL => Display::fmt(stringify!(KERNEL), f),
            Self::TASK_NV => Display::fmt(stringify!(TASK_NV), f),
            Self::MESH_NV => Display::fmt(stringify!(MESH_NV), f),
            Self::RAY_GENERATION_KHR => Display::fmt(stringify!(RAY_GENERATION_KHR), f),
            Self::INTERSECTION_KHR => Display::fmt(stringify!(INTERSECTION_KHR), f),
            Self::ANY_HIT_KHR => Display::fmt(stringify!(ANY_HIT_KHR), f),
            Self::CLOSEST_HIT_KHR => Display::fmt(stringify!(CLOSEST_HIT_KHR), f),
            Self::MISS_KHR => Display::fmt(stringify!(MISS_KHR), f),
            Self::CALLABLE_KHR => Display::fmt(stringify!(CALLABLE_KHR), f),
            Self::TASK_EXT => Display::fmt(stringify!(TASK_EXT), f),
            Self::MESH_EXT => Display::fmt(stringify!(MESH_EXT), f),
            x => write!(f, "{}({})", stringify!(ExecutionModel), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressingModel(pub(crate) u32);
impl AddressingModel {
    pub const LOGICAL: Self = Self(0u32);
    pub const PHYSICAL_32: Self = Self(1u32);
    pub const PHYSICAL_64: Self = Self(2u32);
    pub const PHYSICAL_STORAGE_BUFFER_64: Self = Self(5348u32);
}
impl Word for AddressingModel {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for AddressingModel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LOGICAL => Display::fmt(stringify!(LOGICAL), f),
            Self::PHYSICAL_32 => Display::fmt(stringify!(PHYSICAL_32), f),
            Self::PHYSICAL_64 => Display::fmt(stringify!(PHYSICAL_64), f),
            Self::PHYSICAL_STORAGE_BUFFER_64 => {
                Display::fmt(stringify!(PHYSICAL_STORAGE_BUFFER_64), f)
            }
            x => write!(f, "{}({})", stringify!(AddressingModel), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryModel(pub(crate) u32);
impl MemoryModel {
    pub const SIMPLE: Self = Self(0u32);
    pub const GLSL450: Self = Self(1u32);
    pub const OPEN_CL: Self = Self(2u32);
    pub const VULKAN: Self = Self(3u32);
}
impl Word for MemoryModel {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for MemoryModel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SIMPLE => Display::fmt(stringify!(SIMPLE), f),
            Self::GLSL450 => Display::fmt(stringify!(GLSL450), f),
            Self::OPEN_CL => Display::fmt(stringify!(OPEN_CL), f),
            Self::VULKAN => Display::fmt(stringify!(VULKAN), f),
            x => write!(f, "{}({})", stringify!(MemoryModel), x.0),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ExecutionModeOperands {
    Invocations {
        number_of_invocations: LiteralInteger,
    },
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize {
        x_size: LiteralInteger,
        y_size: LiteralInteger,
        z_size: LiteralInteger,
    },
    LocalSizeHint {
        x_size: LiteralInteger,
        y_size: LiteralInteger,
        z_size: LiteralInteger,
    },
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVertices {
        vertex_count: LiteralInteger,
    },
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint {
        vector_type: LiteralInteger,
    },
    ContractionOff,
    Initializer,
    Finalizer,
    SubgroupSize {
        subgroup_size: LiteralInteger,
    },
    SubgroupsPerWorkgroup {
        subgroups_per_workgroup: LiteralInteger,
    },
    SubgroupsPerWorkgroupId {
        subgroups_per_workgroup: IdRef,
    },
    LocalSizeId {
        x_size: IdRef,
        y_size: IdRef,
        z_size: IdRef,
    },
    LocalSizeHintId {
        x_size_hint: IdRef,
        y_size_hint: IdRef,
        z_size_hint: IdRef,
    },
    NonCoherentColorAttachmentReadEXT,
    NonCoherentDepthAttachmentReadEXT,
    NonCoherentStencilAttachmentReadEXT,
    SubgroupUniformControlFlowKHR,
    PostDepthCoverage,
    DenormPreserve {
        target_width: LiteralInteger,
    },
    DenormFlushToZero {
        target_width: LiteralInteger,
    },
    SignedZeroInfNanPreserve {
        target_width: LiteralInteger,
    },
    RoundingModeRTE {
        target_width: LiteralInteger,
    },
    RoundingModeRTZ {
        target_width: LiteralInteger,
    },
    NonCoherentTileAttachmentReadQCOM,
    TileShadingRateQCOM {
        x_rate: LiteralInteger,
        y_rate: LiteralInteger,
        z_rate: LiteralInteger,
    },
    EarlyAndLateFragmentTestsAMD,
    StencilRefReplacingEXT,
    CoalescingAMDX,
    IsApiEntryAMDX {
        is_entry: IdRef,
    },
    MaxNodeRecursionAMDX {
        number_of_recursions: IdRef,
    },
    StaticNumWorkgroupsAMDX {
        x_size: IdRef,
        y_size: IdRef,
        z_size: IdRef,
    },
    ShaderIndexAMDX {
        shader_index: IdRef,
    },
    MaxNumWorkgroupsAMDX {
        x_size: IdRef,
        y_size: IdRef,
        z_size: IdRef,
    },
    StencilRefUnchangedFrontAMD,
    StencilRefGreaterFrontAMD,
    StencilRefLessFrontAMD,
    StencilRefUnchangedBackAMD,
    StencilRefGreaterBackAMD,
    StencilRefLessBackAMD,
    QuadDerivativesKHR,
    RequireFullQuadsKHR,
    SharesInputWithAMDX {
        node_name: IdRef,
        shader_index: IdRef,
    },
    ArithmeticPoisonKHR,
    OutputLinesEXT,
    OutputPrimitivesEXT {
        primitive_count: LiteralInteger,
    },
    DerivativeGroupQuadsKHR,
    DerivativeGroupLinearKHR,
    OutputTrianglesEXT,
    PixelInterlockOrderedEXT,
    PixelInterlockUnorderedEXT,
    SampleInterlockOrderedEXT,
    SampleInterlockUnorderedEXT,
    ShadingRateInterlockOrderedEXT,
    ShadingRateInterlockUnorderedEXT,
    Shader64BitIndexingEXT,
    SharedLocalMemorySizeINTEL {
        size: LiteralInteger,
    },
    RoundingModeRTPINTEL {
        target_width: LiteralInteger,
    },
    RoundingModeRTNINTEL {
        target_width: LiteralInteger,
    },
    FloatingPointModeALTINTEL {
        target_width: LiteralInteger,
    },
    FloatingPointModeIEEEINTEL {
        target_width: LiteralInteger,
    },
    MaxWorkgroupSizeINTEL {
        max_x_size: LiteralInteger,
        max_y_size: LiteralInteger,
        max_z_size: LiteralInteger,
    },
    MaxWorkDimINTEL {
        max_dimensions: LiteralInteger,
    },
    NoGlobalOffsetINTEL,
    NumSIMDWorkitemsINTEL {
        vector_width: LiteralInteger,
    },
    SchedulerTargetFmaxMhzINTEL {
        target_fmax: LiteralInteger,
    },
    MaximallyReconvergesKHR,
    FPFastMathDefault {
        target_type: IdRef,
        fast_math_mode: IdRef,
    },
    StreamingInterfaceINTEL {
        stall_free_return: LiteralInteger,
    },
    RegisterMapInterfaceINTEL {
        wait_for_done_write: LiteralInteger,
    },
    NamedBarrierCountINTEL {
        barrier_count: LiteralInteger,
    },
    MaximumRegistersINTEL {
        number_of_registers: LiteralInteger,
    },
    MaximumRegistersIdINTEL {
        number_of_registers: IdRef,
    },
    NamedMaximumRegistersINTEL {
        named_maximum_number_of_registers: NamedMaximumNumberOfRegisters,
    },
}
impl<'a> ExecutionModeOperands {
    #[inline]
    pub fn parse_one(stream: &mut InstructionStream<'a>) -> ParseResult<Self> {
        let variant = stream.read()?;
        match variant {
            0u32 => Ok(Self::Invocations {
                number_of_invocations: LiteralInteger::parse_one(stream)?,
            }),
            1u32 => Ok(Self::SpacingEqual),
            2u32 => Ok(Self::SpacingFractionalEven),
            3u32 => Ok(Self::SpacingFractionalOdd),
            4u32 => Ok(Self::VertexOrderCw),
            5u32 => Ok(Self::VertexOrderCcw),
            6u32 => Ok(Self::PixelCenterInteger),
            7u32 => Ok(Self::OriginUpperLeft),
            8u32 => Ok(Self::OriginLowerLeft),
            9u32 => Ok(Self::EarlyFragmentTests),
            10u32 => Ok(Self::PointMode),
            11u32 => Ok(Self::Xfb),
            12u32 => Ok(Self::DepthReplacing),
            14u32 => Ok(Self::DepthGreater),
            15u32 => Ok(Self::DepthLess),
            16u32 => Ok(Self::DepthUnchanged),
            17u32 => Ok(Self::LocalSize {
                x_size: LiteralInteger::parse_one(stream)?,
                y_size: LiteralInteger::parse_one(stream)?,
                z_size: LiteralInteger::parse_one(stream)?,
            }),
            18u32 => Ok(Self::LocalSizeHint {
                x_size: LiteralInteger::parse_one(stream)?,
                y_size: LiteralInteger::parse_one(stream)?,
                z_size: LiteralInteger::parse_one(stream)?,
            }),
            19u32 => Ok(Self::InputPoints),
            20u32 => Ok(Self::InputLines),
            21u32 => Ok(Self::InputLinesAdjacency),
            22u32 => Ok(Self::Triangles),
            23u32 => Ok(Self::InputTrianglesAdjacency),
            24u32 => Ok(Self::Quads),
            25u32 => Ok(Self::Isolines),
            26u32 => Ok(Self::OutputVertices {
                vertex_count: LiteralInteger::parse_one(stream)?,
            }),
            27u32 => Ok(Self::OutputPoints),
            28u32 => Ok(Self::OutputLineStrip),
            29u32 => Ok(Self::OutputTriangleStrip),
            30u32 => Ok(Self::VecTypeHint {
                vector_type: LiteralInteger::parse_one(stream)?,
            }),
            31u32 => Ok(Self::ContractionOff),
            33u32 => Ok(Self::Initializer),
            34u32 => Ok(Self::Finalizer),
            35u32 => Ok(Self::SubgroupSize {
                subgroup_size: LiteralInteger::parse_one(stream)?,
            }),
            36u32 => Ok(Self::SubgroupsPerWorkgroup {
                subgroups_per_workgroup: LiteralInteger::parse_one(stream)?,
            }),
            37u32 => Ok(Self::SubgroupsPerWorkgroupId {
                subgroups_per_workgroup: IdRef::parse_one(stream)?,
            }),
            38u32 => Ok(Self::LocalSizeId {
                x_size: IdRef::parse_one(stream)?,
                y_size: IdRef::parse_one(stream)?,
                z_size: IdRef::parse_one(stream)?,
            }),
            39u32 => Ok(Self::LocalSizeHintId {
                x_size_hint: IdRef::parse_one(stream)?,
                y_size_hint: IdRef::parse_one(stream)?,
                z_size_hint: IdRef::parse_one(stream)?,
            }),
            4169u32 => Ok(Self::NonCoherentColorAttachmentReadEXT),
            4170u32 => Ok(Self::NonCoherentDepthAttachmentReadEXT),
            4171u32 => Ok(Self::NonCoherentStencilAttachmentReadEXT),
            4421u32 => Ok(Self::SubgroupUniformControlFlowKHR),
            4446u32 => Ok(Self::PostDepthCoverage),
            4459u32 => Ok(Self::DenormPreserve {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            4460u32 => Ok(Self::DenormFlushToZero {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            4461u32 => Ok(Self::SignedZeroInfNanPreserve {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            4462u32 => Ok(Self::RoundingModeRTE {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            4463u32 => Ok(Self::RoundingModeRTZ {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            4489u32 => Ok(Self::NonCoherentTileAttachmentReadQCOM),
            4490u32 => Ok(Self::TileShadingRateQCOM {
                x_rate: LiteralInteger::parse_one(stream)?,
                y_rate: LiteralInteger::parse_one(stream)?,
                z_rate: LiteralInteger::parse_one(stream)?,
            }),
            5017u32 => Ok(Self::EarlyAndLateFragmentTestsAMD),
            5027u32 => Ok(Self::StencilRefReplacingEXT),
            5069u32 => Ok(Self::CoalescingAMDX),
            5070u32 => Ok(Self::IsApiEntryAMDX {
                is_entry: IdRef::parse_one(stream)?,
            }),
            5071u32 => Ok(Self::MaxNodeRecursionAMDX {
                number_of_recursions: IdRef::parse_one(stream)?,
            }),
            5072u32 => Ok(Self::StaticNumWorkgroupsAMDX {
                x_size: IdRef::parse_one(stream)?,
                y_size: IdRef::parse_one(stream)?,
                z_size: IdRef::parse_one(stream)?,
            }),
            5073u32 => Ok(Self::ShaderIndexAMDX {
                shader_index: IdRef::parse_one(stream)?,
            }),
            5077u32 => Ok(Self::MaxNumWorkgroupsAMDX {
                x_size: IdRef::parse_one(stream)?,
                y_size: IdRef::parse_one(stream)?,
                z_size: IdRef::parse_one(stream)?,
            }),
            5079u32 => Ok(Self::StencilRefUnchangedFrontAMD),
            5080u32 => Ok(Self::StencilRefGreaterFrontAMD),
            5081u32 => Ok(Self::StencilRefLessFrontAMD),
            5082u32 => Ok(Self::StencilRefUnchangedBackAMD),
            5083u32 => Ok(Self::StencilRefGreaterBackAMD),
            5084u32 => Ok(Self::StencilRefLessBackAMD),
            5088u32 => Ok(Self::QuadDerivativesKHR),
            5089u32 => Ok(Self::RequireFullQuadsKHR),
            5102u32 => Ok(Self::SharesInputWithAMDX {
                node_name: IdRef::parse_one(stream)?,
                shader_index: IdRef::parse_one(stream)?,
            }),
            5157u32 => Ok(Self::ArithmeticPoisonKHR),
            5269u32 => Ok(Self::OutputLinesEXT),
            5270u32 => Ok(Self::OutputPrimitivesEXT {
                primitive_count: LiteralInteger::parse_one(stream)?,
            }),
            5289u32 => Ok(Self::DerivativeGroupQuadsKHR),
            5290u32 => Ok(Self::DerivativeGroupLinearKHR),
            5298u32 => Ok(Self::OutputTrianglesEXT),
            5366u32 => Ok(Self::PixelInterlockOrderedEXT),
            5367u32 => Ok(Self::PixelInterlockUnorderedEXT),
            5368u32 => Ok(Self::SampleInterlockOrderedEXT),
            5369u32 => Ok(Self::SampleInterlockUnorderedEXT),
            5370u32 => Ok(Self::ShadingRateInterlockOrderedEXT),
            5371u32 => Ok(Self::ShadingRateInterlockUnorderedEXT),
            5427u32 => Ok(Self::Shader64BitIndexingEXT),
            5618u32 => Ok(Self::SharedLocalMemorySizeINTEL {
                size: LiteralInteger::parse_one(stream)?,
            }),
            5620u32 => Ok(Self::RoundingModeRTPINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            5621u32 => Ok(Self::RoundingModeRTNINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            5622u32 => Ok(Self::FloatingPointModeALTINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            5623u32 => Ok(Self::FloatingPointModeIEEEINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
            }),
            5893u32 => Ok(Self::MaxWorkgroupSizeINTEL {
                max_x_size: LiteralInteger::parse_one(stream)?,
                max_y_size: LiteralInteger::parse_one(stream)?,
                max_z_size: LiteralInteger::parse_one(stream)?,
            }),
            5894u32 => Ok(Self::MaxWorkDimINTEL {
                max_dimensions: LiteralInteger::parse_one(stream)?,
            }),
            5895u32 => Ok(Self::NoGlobalOffsetINTEL),
            5896u32 => Ok(Self::NumSIMDWorkitemsINTEL {
                vector_width: LiteralInteger::parse_one(stream)?,
            }),
            5903u32 => Ok(Self::SchedulerTargetFmaxMhzINTEL {
                target_fmax: LiteralInteger::parse_one(stream)?,
            }),
            6023u32 => Ok(Self::MaximallyReconvergesKHR),
            6028u32 => Ok(Self::FPFastMathDefault {
                target_type: IdRef::parse_one(stream)?,
                fast_math_mode: IdRef::parse_one(stream)?,
            }),
            6154u32 => Ok(Self::StreamingInterfaceINTEL {
                stall_free_return: LiteralInteger::parse_one(stream)?,
            }),
            6160u32 => Ok(Self::RegisterMapInterfaceINTEL {
                wait_for_done_write: LiteralInteger::parse_one(stream)?,
            }),
            6417u32 => Ok(Self::NamedBarrierCountINTEL {
                barrier_count: LiteralInteger::parse_one(stream)?,
            }),
            6461u32 => Ok(Self::MaximumRegistersINTEL {
                number_of_registers: LiteralInteger::parse_one(stream)?,
            }),
            6462u32 => Ok(Self::MaximumRegistersIdINTEL {
                number_of_registers: IdRef::parse_one(stream)?,
            }),
            6463u32 => Ok(Self::NamedMaximumRegistersINTEL {
                named_maximum_number_of_registers: NamedMaximumNumberOfRegisters::parse_one(
                    stream,
                )?,
            }),
            x => Err(ParseError::UnknownVariant {
                kind: stringify!(ExecutionModeOperands),
                value: x,
            }),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionMode(pub(crate) u32);
impl ExecutionMode {
    pub const INVOCATIONS: Self = Self(0u32);
    pub const SPACING_EQUAL: Self = Self(1u32);
    pub const SPACING_FRACTIONAL_EVEN: Self = Self(2u32);
    pub const SPACING_FRACTIONAL_ODD: Self = Self(3u32);
    pub const VERTEX_ORDER_CW: Self = Self(4u32);
    pub const VERTEX_ORDER_CCW: Self = Self(5u32);
    pub const PIXEL_CENTER_INTEGER: Self = Self(6u32);
    pub const ORIGIN_UPPER_LEFT: Self = Self(7u32);
    pub const ORIGIN_LOWER_LEFT: Self = Self(8u32);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(9u32);
    pub const POINT_MODE: Self = Self(10u32);
    pub const XFB: Self = Self(11u32);
    pub const DEPTH_REPLACING: Self = Self(12u32);
    pub const DEPTH_GREATER: Self = Self(14u32);
    pub const DEPTH_LESS: Self = Self(15u32);
    pub const DEPTH_UNCHANGED: Self = Self(16u32);
    pub const LOCAL_SIZE: Self = Self(17u32);
    pub const LOCAL_SIZE_HINT: Self = Self(18u32);
    pub const INPUT_POINTS: Self = Self(19u32);
    pub const INPUT_LINES: Self = Self(20u32);
    pub const INPUT_LINES_ADJACENCY: Self = Self(21u32);
    pub const TRIANGLES: Self = Self(22u32);
    pub const INPUT_TRIANGLES_ADJACENCY: Self = Self(23u32);
    pub const QUADS: Self = Self(24u32);
    pub const ISOLINES: Self = Self(25u32);
    pub const OUTPUT_VERTICES: Self = Self(26u32);
    pub const OUTPUT_POINTS: Self = Self(27u32);
    pub const OUTPUT_LINE_STRIP: Self = Self(28u32);
    pub const OUTPUT_TRIANGLE_STRIP: Self = Self(29u32);
    pub const VEC_TYPE_HINT: Self = Self(30u32);
    pub const CONTRACTION_OFF: Self = Self(31u32);
    pub const INITIALIZER: Self = Self(33u32);
    pub const FINALIZER: Self = Self(34u32);
    pub const SUBGROUP_SIZE: Self = Self(35u32);
    pub const SUBGROUPS_PER_WORKGROUP: Self = Self(36u32);
    pub const SUBGROUPS_PER_WORKGROUP_ID: Self = Self(37u32);
    pub const LOCAL_SIZE_ID: Self = Self(38u32);
    pub const LOCAL_SIZE_HINT_ID: Self = Self(39u32);
    pub const NON_COHERENT_COLOR_ATTACHMENT_READ_EXT: Self = Self(4169u32);
    pub const NON_COHERENT_DEPTH_ATTACHMENT_READ_EXT: Self = Self(4170u32);
    pub const NON_COHERENT_STENCIL_ATTACHMENT_READ_EXT: Self = Self(4171u32);
    pub const SUBGROUP_UNIFORM_CONTROL_FLOW_KHR: Self = Self(4421u32);
    pub const POST_DEPTH_COVERAGE: Self = Self(4446u32);
    pub const DENORM_PRESERVE: Self = Self(4459u32);
    pub const DENORM_FLUSH_TO_ZERO: Self = Self(4460u32);
    pub const SIGNED_ZERO_INF_NAN_PRESERVE: Self = Self(4461u32);
    pub const ROUNDING_MODE_RTE: Self = Self(4462u32);
    pub const ROUNDING_MODE_RTZ: Self = Self(4463u32);
    pub const NON_COHERENT_TILE_ATTACHMENT_READ_QCOM: Self = Self(4489u32);
    pub const TILE_SHADING_RATE_QCOM: Self = Self(4490u32);
    pub const EARLY_AND_LATE_FRAGMENT_TESTS_AMD: Self = Self(5017u32);
    pub const STENCIL_REF_REPLACING_EXT: Self = Self(5027u32);
    pub const COALESCING_AMDX: Self = Self(5069u32);
    pub const IS_API_ENTRY_AMDX: Self = Self(5070u32);
    pub const MAX_NODE_RECURSION_AMDX: Self = Self(5071u32);
    pub const STATIC_NUM_WORKGROUPS_AMDX: Self = Self(5072u32);
    pub const SHADER_INDEX_AMDX: Self = Self(5073u32);
    pub const MAX_NUM_WORKGROUPS_AMDX: Self = Self(5077u32);
    pub const STENCIL_REF_UNCHANGED_FRONT_AMD: Self = Self(5079u32);
    pub const STENCIL_REF_GREATER_FRONT_AMD: Self = Self(5080u32);
    pub const STENCIL_REF_LESS_FRONT_AMD: Self = Self(5081u32);
    pub const STENCIL_REF_UNCHANGED_BACK_AMD: Self = Self(5082u32);
    pub const STENCIL_REF_GREATER_BACK_AMD: Self = Self(5083u32);
    pub const STENCIL_REF_LESS_BACK_AMD: Self = Self(5084u32);
    pub const QUAD_DERIVATIVES_KHR: Self = Self(5088u32);
    pub const REQUIRE_FULL_QUADS_KHR: Self = Self(5089u32);
    pub const SHARES_INPUT_WITH_AMDX: Self = Self(5102u32);
    pub const ARITHMETIC_POISON_KHR: Self = Self(5157u32);
    pub const OUTPUT_LINES_EXT: Self = Self(5269u32);
    pub const OUTPUT_PRIMITIVES_EXT: Self = Self(5270u32);
    pub const DERIVATIVE_GROUP_QUADS_KHR: Self = Self(5289u32);
    pub const DERIVATIVE_GROUP_LINEAR_KHR: Self = Self(5290u32);
    pub const OUTPUT_TRIANGLES_EXT: Self = Self(5298u32);
    pub const PIXEL_INTERLOCK_ORDERED_EXT: Self = Self(5366u32);
    pub const PIXEL_INTERLOCK_UNORDERED_EXT: Self = Self(5367u32);
    pub const SAMPLE_INTERLOCK_ORDERED_EXT: Self = Self(5368u32);
    pub const SAMPLE_INTERLOCK_UNORDERED_EXT: Self = Self(5369u32);
    pub const SHADING_RATE_INTERLOCK_ORDERED_EXT: Self = Self(5370u32);
    pub const SHADING_RATE_INTERLOCK_UNORDERED_EXT: Self = Self(5371u32);
    pub const SHADER_64_BIT_INDEXING_EXT: Self = Self(5427u32);
    pub const SHARED_LOCAL_MEMORY_SIZE_INTEL: Self = Self(5618u32);
    pub const ROUNDING_MODE_RTPINTEL: Self = Self(5620u32);
    pub const ROUNDING_MODE_RTNINTEL: Self = Self(5621u32);
    pub const FLOATING_POINT_MODE_ALTINTEL: Self = Self(5622u32);
    pub const FLOATING_POINT_MODE_IEEEINTEL: Self = Self(5623u32);
    pub const MAX_WORKGROUP_SIZE_INTEL: Self = Self(5893u32);
    pub const MAX_WORK_DIM_INTEL: Self = Self(5894u32);
    pub const NO_GLOBAL_OFFSET_INTEL: Self = Self(5895u32);
    pub const NUM_SIMDWORKITEMS_INTEL: Self = Self(5896u32);
    pub const SCHEDULER_TARGET_FMAX_MHZ_INTEL: Self = Self(5903u32);
    pub const MAXIMALLY_RECONVERGES_KHR: Self = Self(6023u32);
    pub const FPFAST_MATH_DEFAULT: Self = Self(6028u32);
    pub const STREAMING_INTERFACE_INTEL: Self = Self(6154u32);
    pub const REGISTER_MAP_INTERFACE_INTEL: Self = Self(6160u32);
    pub const NAMED_BARRIER_COUNT_INTEL: Self = Self(6417u32);
    pub const MAXIMUM_REGISTERS_INTEL: Self = Self(6461u32);
    pub const MAXIMUM_REGISTERS_ID_INTEL: Self = Self(6462u32);
    pub const NAMED_MAXIMUM_REGISTERS_INTEL: Self = Self(6463u32);
}
impl Word for ExecutionMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ExecutionMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INVOCATIONS => Display::fmt(stringify!(INVOCATIONS), f),
            Self::SPACING_EQUAL => Display::fmt(stringify!(SPACING_EQUAL), f),
            Self::SPACING_FRACTIONAL_EVEN => Display::fmt(stringify!(SPACING_FRACTIONAL_EVEN), f),
            Self::SPACING_FRACTIONAL_ODD => Display::fmt(stringify!(SPACING_FRACTIONAL_ODD), f),
            Self::VERTEX_ORDER_CW => Display::fmt(stringify!(VERTEX_ORDER_CW), f),
            Self::VERTEX_ORDER_CCW => Display::fmt(stringify!(VERTEX_ORDER_CCW), f),
            Self::PIXEL_CENTER_INTEGER => Display::fmt(stringify!(PIXEL_CENTER_INTEGER), f),
            Self::ORIGIN_UPPER_LEFT => Display::fmt(stringify!(ORIGIN_UPPER_LEFT), f),
            Self::ORIGIN_LOWER_LEFT => Display::fmt(stringify!(ORIGIN_LOWER_LEFT), f),
            Self::EARLY_FRAGMENT_TESTS => Display::fmt(stringify!(EARLY_FRAGMENT_TESTS), f),
            Self::POINT_MODE => Display::fmt(stringify!(POINT_MODE), f),
            Self::XFB => Display::fmt(stringify!(XFB), f),
            Self::DEPTH_REPLACING => Display::fmt(stringify!(DEPTH_REPLACING), f),
            Self::DEPTH_GREATER => Display::fmt(stringify!(DEPTH_GREATER), f),
            Self::DEPTH_LESS => Display::fmt(stringify!(DEPTH_LESS), f),
            Self::DEPTH_UNCHANGED => Display::fmt(stringify!(DEPTH_UNCHANGED), f),
            Self::LOCAL_SIZE => Display::fmt(stringify!(LOCAL_SIZE), f),
            Self::LOCAL_SIZE_HINT => Display::fmt(stringify!(LOCAL_SIZE_HINT), f),
            Self::INPUT_POINTS => Display::fmt(stringify!(INPUT_POINTS), f),
            Self::INPUT_LINES => Display::fmt(stringify!(INPUT_LINES), f),
            Self::INPUT_LINES_ADJACENCY => Display::fmt(stringify!(INPUT_LINES_ADJACENCY), f),
            Self::TRIANGLES => Display::fmt(stringify!(TRIANGLES), f),
            Self::INPUT_TRIANGLES_ADJACENCY => {
                Display::fmt(stringify!(INPUT_TRIANGLES_ADJACENCY), f)
            }
            Self::QUADS => Display::fmt(stringify!(QUADS), f),
            Self::ISOLINES => Display::fmt(stringify!(ISOLINES), f),
            Self::OUTPUT_VERTICES => Display::fmt(stringify!(OUTPUT_VERTICES), f),
            Self::OUTPUT_POINTS => Display::fmt(stringify!(OUTPUT_POINTS), f),
            Self::OUTPUT_LINE_STRIP => Display::fmt(stringify!(OUTPUT_LINE_STRIP), f),
            Self::OUTPUT_TRIANGLE_STRIP => Display::fmt(stringify!(OUTPUT_TRIANGLE_STRIP), f),
            Self::VEC_TYPE_HINT => Display::fmt(stringify!(VEC_TYPE_HINT), f),
            Self::CONTRACTION_OFF => Display::fmt(stringify!(CONTRACTION_OFF), f),
            Self::INITIALIZER => Display::fmt(stringify!(INITIALIZER), f),
            Self::FINALIZER => Display::fmt(stringify!(FINALIZER), f),
            Self::SUBGROUP_SIZE => Display::fmt(stringify!(SUBGROUP_SIZE), f),
            Self::SUBGROUPS_PER_WORKGROUP => Display::fmt(stringify!(SUBGROUPS_PER_WORKGROUP), f),
            Self::SUBGROUPS_PER_WORKGROUP_ID => {
                Display::fmt(stringify!(SUBGROUPS_PER_WORKGROUP_ID), f)
            }
            Self::LOCAL_SIZE_ID => Display::fmt(stringify!(LOCAL_SIZE_ID), f),
            Self::LOCAL_SIZE_HINT_ID => Display::fmt(stringify!(LOCAL_SIZE_HINT_ID), f),
            Self::NON_COHERENT_COLOR_ATTACHMENT_READ_EXT => {
                Display::fmt(stringify!(NON_COHERENT_COLOR_ATTACHMENT_READ_EXT), f)
            }
            Self::NON_COHERENT_DEPTH_ATTACHMENT_READ_EXT => {
                Display::fmt(stringify!(NON_COHERENT_DEPTH_ATTACHMENT_READ_EXT), f)
            }
            Self::NON_COHERENT_STENCIL_ATTACHMENT_READ_EXT => {
                Display::fmt(stringify!(NON_COHERENT_STENCIL_ATTACHMENT_READ_EXT), f)
            }
            Self::SUBGROUP_UNIFORM_CONTROL_FLOW_KHR => {
                Display::fmt(stringify!(SUBGROUP_UNIFORM_CONTROL_FLOW_KHR), f)
            }
            Self::POST_DEPTH_COVERAGE => Display::fmt(stringify!(POST_DEPTH_COVERAGE), f),
            Self::DENORM_PRESERVE => Display::fmt(stringify!(DENORM_PRESERVE), f),
            Self::DENORM_FLUSH_TO_ZERO => Display::fmt(stringify!(DENORM_FLUSH_TO_ZERO), f),
            Self::SIGNED_ZERO_INF_NAN_PRESERVE => {
                Display::fmt(stringify!(SIGNED_ZERO_INF_NAN_PRESERVE), f)
            }
            Self::ROUNDING_MODE_RTE => Display::fmt(stringify!(ROUNDING_MODE_RTE), f),
            Self::ROUNDING_MODE_RTZ => Display::fmt(stringify!(ROUNDING_MODE_RTZ), f),
            Self::NON_COHERENT_TILE_ATTACHMENT_READ_QCOM => {
                Display::fmt(stringify!(NON_COHERENT_TILE_ATTACHMENT_READ_QCOM), f)
            }
            Self::TILE_SHADING_RATE_QCOM => Display::fmt(stringify!(TILE_SHADING_RATE_QCOM), f),
            Self::EARLY_AND_LATE_FRAGMENT_TESTS_AMD => {
                Display::fmt(stringify!(EARLY_AND_LATE_FRAGMENT_TESTS_AMD), f)
            }
            Self::STENCIL_REF_REPLACING_EXT => {
                Display::fmt(stringify!(STENCIL_REF_REPLACING_EXT), f)
            }
            Self::COALESCING_AMDX => Display::fmt(stringify!(COALESCING_AMDX), f),
            Self::IS_API_ENTRY_AMDX => Display::fmt(stringify!(IS_API_ENTRY_AMDX), f),
            Self::MAX_NODE_RECURSION_AMDX => Display::fmt(stringify!(MAX_NODE_RECURSION_AMDX), f),
            Self::STATIC_NUM_WORKGROUPS_AMDX => {
                Display::fmt(stringify!(STATIC_NUM_WORKGROUPS_AMDX), f)
            }
            Self::SHADER_INDEX_AMDX => Display::fmt(stringify!(SHADER_INDEX_AMDX), f),
            Self::MAX_NUM_WORKGROUPS_AMDX => Display::fmt(stringify!(MAX_NUM_WORKGROUPS_AMDX), f),
            Self::STENCIL_REF_UNCHANGED_FRONT_AMD => {
                Display::fmt(stringify!(STENCIL_REF_UNCHANGED_FRONT_AMD), f)
            }
            Self::STENCIL_REF_GREATER_FRONT_AMD => {
                Display::fmt(stringify!(STENCIL_REF_GREATER_FRONT_AMD), f)
            }
            Self::STENCIL_REF_LESS_FRONT_AMD => {
                Display::fmt(stringify!(STENCIL_REF_LESS_FRONT_AMD), f)
            }
            Self::STENCIL_REF_UNCHANGED_BACK_AMD => {
                Display::fmt(stringify!(STENCIL_REF_UNCHANGED_BACK_AMD), f)
            }
            Self::STENCIL_REF_GREATER_BACK_AMD => {
                Display::fmt(stringify!(STENCIL_REF_GREATER_BACK_AMD), f)
            }
            Self::STENCIL_REF_LESS_BACK_AMD => {
                Display::fmt(stringify!(STENCIL_REF_LESS_BACK_AMD), f)
            }
            Self::QUAD_DERIVATIVES_KHR => Display::fmt(stringify!(QUAD_DERIVATIVES_KHR), f),
            Self::REQUIRE_FULL_QUADS_KHR => Display::fmt(stringify!(REQUIRE_FULL_QUADS_KHR), f),
            Self::SHARES_INPUT_WITH_AMDX => Display::fmt(stringify!(SHARES_INPUT_WITH_AMDX), f),
            Self::ARITHMETIC_POISON_KHR => Display::fmt(stringify!(ARITHMETIC_POISON_KHR), f),
            Self::OUTPUT_LINES_EXT => Display::fmt(stringify!(OUTPUT_LINES_EXT), f),
            Self::OUTPUT_PRIMITIVES_EXT => Display::fmt(stringify!(OUTPUT_PRIMITIVES_EXT), f),
            Self::DERIVATIVE_GROUP_QUADS_KHR => {
                Display::fmt(stringify!(DERIVATIVE_GROUP_QUADS_KHR), f)
            }
            Self::DERIVATIVE_GROUP_LINEAR_KHR => {
                Display::fmt(stringify!(DERIVATIVE_GROUP_LINEAR_KHR), f)
            }
            Self::OUTPUT_TRIANGLES_EXT => Display::fmt(stringify!(OUTPUT_TRIANGLES_EXT), f),
            Self::PIXEL_INTERLOCK_ORDERED_EXT => {
                Display::fmt(stringify!(PIXEL_INTERLOCK_ORDERED_EXT), f)
            }
            Self::PIXEL_INTERLOCK_UNORDERED_EXT => {
                Display::fmt(stringify!(PIXEL_INTERLOCK_UNORDERED_EXT), f)
            }
            Self::SAMPLE_INTERLOCK_ORDERED_EXT => {
                Display::fmt(stringify!(SAMPLE_INTERLOCK_ORDERED_EXT), f)
            }
            Self::SAMPLE_INTERLOCK_UNORDERED_EXT => {
                Display::fmt(stringify!(SAMPLE_INTERLOCK_UNORDERED_EXT), f)
            }
            Self::SHADING_RATE_INTERLOCK_ORDERED_EXT => {
                Display::fmt(stringify!(SHADING_RATE_INTERLOCK_ORDERED_EXT), f)
            }
            Self::SHADING_RATE_INTERLOCK_UNORDERED_EXT => {
                Display::fmt(stringify!(SHADING_RATE_INTERLOCK_UNORDERED_EXT), f)
            }
            Self::SHADER_64_BIT_INDEXING_EXT => {
                Display::fmt(stringify!(SHADER_64_BIT_INDEXING_EXT), f)
            }
            Self::SHARED_LOCAL_MEMORY_SIZE_INTEL => {
                Display::fmt(stringify!(SHARED_LOCAL_MEMORY_SIZE_INTEL), f)
            }
            Self::ROUNDING_MODE_RTPINTEL => Display::fmt(stringify!(ROUNDING_MODE_RTPINTEL), f),
            Self::ROUNDING_MODE_RTNINTEL => Display::fmt(stringify!(ROUNDING_MODE_RTNINTEL), f),
            Self::FLOATING_POINT_MODE_ALTINTEL => {
                Display::fmt(stringify!(FLOATING_POINT_MODE_ALTINTEL), f)
            }
            Self::FLOATING_POINT_MODE_IEEEINTEL => {
                Display::fmt(stringify!(FLOATING_POINT_MODE_IEEEINTEL), f)
            }
            Self::MAX_WORKGROUP_SIZE_INTEL => Display::fmt(stringify!(MAX_WORKGROUP_SIZE_INTEL), f),
            Self::MAX_WORK_DIM_INTEL => Display::fmt(stringify!(MAX_WORK_DIM_INTEL), f),
            Self::NO_GLOBAL_OFFSET_INTEL => Display::fmt(stringify!(NO_GLOBAL_OFFSET_INTEL), f),
            Self::NUM_SIMDWORKITEMS_INTEL => Display::fmt(stringify!(NUM_SIMDWORKITEMS_INTEL), f),
            Self::SCHEDULER_TARGET_FMAX_MHZ_INTEL => {
                Display::fmt(stringify!(SCHEDULER_TARGET_FMAX_MHZ_INTEL), f)
            }
            Self::MAXIMALLY_RECONVERGES_KHR => {
                Display::fmt(stringify!(MAXIMALLY_RECONVERGES_KHR), f)
            }
            Self::FPFAST_MATH_DEFAULT => Display::fmt(stringify!(FPFAST_MATH_DEFAULT), f),
            Self::STREAMING_INTERFACE_INTEL => {
                Display::fmt(stringify!(STREAMING_INTERFACE_INTEL), f)
            }
            Self::REGISTER_MAP_INTERFACE_INTEL => {
                Display::fmt(stringify!(REGISTER_MAP_INTERFACE_INTEL), f)
            }
            Self::NAMED_BARRIER_COUNT_INTEL => {
                Display::fmt(stringify!(NAMED_BARRIER_COUNT_INTEL), f)
            }
            Self::MAXIMUM_REGISTERS_INTEL => Display::fmt(stringify!(MAXIMUM_REGISTERS_INTEL), f),
            Self::MAXIMUM_REGISTERS_ID_INTEL => {
                Display::fmt(stringify!(MAXIMUM_REGISTERS_ID_INTEL), f)
            }
            Self::NAMED_MAXIMUM_REGISTERS_INTEL => {
                Display::fmt(stringify!(NAMED_MAXIMUM_REGISTERS_INTEL), f)
            }
            x => write!(f, "{}({})", stringify!(ExecutionMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageClass(pub(crate) u32);
impl StorageClass {
    pub const UNIFORM_CONSTANT: Self = Self(0u32);
    pub const INPUT: Self = Self(1u32);
    pub const UNIFORM: Self = Self(2u32);
    pub const OUTPUT: Self = Self(3u32);
    pub const WORKGROUP: Self = Self(4u32);
    pub const CROSS_WORKGROUP: Self = Self(5u32);
    pub const PRIVATE: Self = Self(6u32);
    pub const FUNCTION: Self = Self(7u32);
    pub const GENERIC: Self = Self(8u32);
    pub const PUSH_CONSTANT: Self = Self(9u32);
    pub const ATOMIC_COUNTER: Self = Self(10u32);
    pub const IMAGE: Self = Self(11u32);
    pub const STORAGE_BUFFER: Self = Self(12u32);
    pub const TILE_IMAGE_EXT: Self = Self(4172u32);
    pub const TILE_ATTACHMENT_QCOM: Self = Self(4491u32);
    pub const NODE_PAYLOAD_AMDX: Self = Self(5068u32);
    pub const CALLABLE_DATA_KHR: Self = Self(5328u32);
    pub const INCOMING_CALLABLE_DATA_KHR: Self = Self(5329u32);
    pub const RAY_PAYLOAD_KHR: Self = Self(5338u32);
    pub const HIT_ATTRIBUTE_KHR: Self = Self(5339u32);
    pub const INCOMING_RAY_PAYLOAD_KHR: Self = Self(5342u32);
    pub const SHADER_RECORD_BUFFER_KHR: Self = Self(5343u32);
    pub const PHYSICAL_STORAGE_BUFFER: Self = Self(5349u32);
    pub const HIT_OBJECT_ATTRIBUTE_NV: Self = Self(5385u32);
    pub const TASK_PAYLOAD_WORKGROUP_EXT: Self = Self(5402u32);
    pub const HIT_OBJECT_ATTRIBUTE_EXT: Self = Self(5411u32);
    pub const CODE_SECTION_INTEL: Self = Self(5605u32);
    pub const DEVICE_ONLY_ALTERA: Self = Self(5936u32);
    pub const HOST_ONLY_ALTERA: Self = Self(5937u32);
}
impl Word for StorageClass {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for StorageClass {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNIFORM_CONSTANT => Display::fmt(stringify!(UNIFORM_CONSTANT), f),
            Self::INPUT => Display::fmt(stringify!(INPUT), f),
            Self::UNIFORM => Display::fmt(stringify!(UNIFORM), f),
            Self::OUTPUT => Display::fmt(stringify!(OUTPUT), f),
            Self::WORKGROUP => Display::fmt(stringify!(WORKGROUP), f),
            Self::CROSS_WORKGROUP => Display::fmt(stringify!(CROSS_WORKGROUP), f),
            Self::PRIVATE => Display::fmt(stringify!(PRIVATE), f),
            Self::FUNCTION => Display::fmt(stringify!(FUNCTION), f),
            Self::GENERIC => Display::fmt(stringify!(GENERIC), f),
            Self::PUSH_CONSTANT => Display::fmt(stringify!(PUSH_CONSTANT), f),
            Self::ATOMIC_COUNTER => Display::fmt(stringify!(ATOMIC_COUNTER), f),
            Self::IMAGE => Display::fmt(stringify!(IMAGE), f),
            Self::STORAGE_BUFFER => Display::fmt(stringify!(STORAGE_BUFFER), f),
            Self::TILE_IMAGE_EXT => Display::fmt(stringify!(TILE_IMAGE_EXT), f),
            Self::TILE_ATTACHMENT_QCOM => Display::fmt(stringify!(TILE_ATTACHMENT_QCOM), f),
            Self::NODE_PAYLOAD_AMDX => Display::fmt(stringify!(NODE_PAYLOAD_AMDX), f),
            Self::CALLABLE_DATA_KHR => Display::fmt(stringify!(CALLABLE_DATA_KHR), f),
            Self::INCOMING_CALLABLE_DATA_KHR => {
                Display::fmt(stringify!(INCOMING_CALLABLE_DATA_KHR), f)
            }
            Self::RAY_PAYLOAD_KHR => Display::fmt(stringify!(RAY_PAYLOAD_KHR), f),
            Self::HIT_ATTRIBUTE_KHR => Display::fmt(stringify!(HIT_ATTRIBUTE_KHR), f),
            Self::INCOMING_RAY_PAYLOAD_KHR => Display::fmt(stringify!(INCOMING_RAY_PAYLOAD_KHR), f),
            Self::SHADER_RECORD_BUFFER_KHR => Display::fmt(stringify!(SHADER_RECORD_BUFFER_KHR), f),
            Self::PHYSICAL_STORAGE_BUFFER => Display::fmt(stringify!(PHYSICAL_STORAGE_BUFFER), f),
            Self::HIT_OBJECT_ATTRIBUTE_NV => Display::fmt(stringify!(HIT_OBJECT_ATTRIBUTE_NV), f),
            Self::TASK_PAYLOAD_WORKGROUP_EXT => {
                Display::fmt(stringify!(TASK_PAYLOAD_WORKGROUP_EXT), f)
            }
            Self::HIT_OBJECT_ATTRIBUTE_EXT => Display::fmt(stringify!(HIT_OBJECT_ATTRIBUTE_EXT), f),
            Self::CODE_SECTION_INTEL => Display::fmt(stringify!(CODE_SECTION_INTEL), f),
            Self::DEVICE_ONLY_ALTERA => Display::fmt(stringify!(DEVICE_ONLY_ALTERA), f),
            Self::HOST_ONLY_ALTERA => Display::fmt(stringify!(HOST_ONLY_ALTERA), f),
            x => write!(f, "{}({})", stringify!(StorageClass), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dim(pub(crate) u32);
impl Dim {
    pub const TYPE_1D: Self = Self(0u32);
    pub const TYPE_2D: Self = Self(1u32);
    pub const TYPE_3D: Self = Self(2u32);
    pub const CUBE: Self = Self(3u32);
    pub const RECT: Self = Self(4u32);
    pub const BUFFER: Self = Self(5u32);
    pub const SUBPASS_DATA: Self = Self(6u32);
    pub const TILE_IMAGE_DATA_EXT: Self = Self(4173u32);
}
impl Word for Dim {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for Dim {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_1D => Display::fmt(stringify!(TYPE_1D), f),
            Self::TYPE_2D => Display::fmt(stringify!(TYPE_2D), f),
            Self::TYPE_3D => Display::fmt(stringify!(TYPE_3D), f),
            Self::CUBE => Display::fmt(stringify!(CUBE), f),
            Self::RECT => Display::fmt(stringify!(RECT), f),
            Self::BUFFER => Display::fmt(stringify!(BUFFER), f),
            Self::SUBPASS_DATA => Display::fmt(stringify!(SUBPASS_DATA), f),
            Self::TILE_IMAGE_DATA_EXT => Display::fmt(stringify!(TILE_IMAGE_DATA_EXT), f),
            x => write!(f, "{}({})", stringify!(Dim), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerAddressingMode(pub(crate) u32);
impl SamplerAddressingMode {
    pub const NONE: Self = Self(0u32);
    pub const CLAMP_TO_EDGE: Self = Self(1u32);
    pub const CLAMP: Self = Self(2u32);
    pub const REPEAT: Self = Self(3u32);
    pub const REPEAT_MIRRORED: Self = Self(4u32);
}
impl Word for SamplerAddressingMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for SamplerAddressingMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE => Display::fmt(stringify!(NONE), f),
            Self::CLAMP_TO_EDGE => Display::fmt(stringify!(CLAMP_TO_EDGE), f),
            Self::CLAMP => Display::fmt(stringify!(CLAMP), f),
            Self::REPEAT => Display::fmt(stringify!(REPEAT), f),
            Self::REPEAT_MIRRORED => Display::fmt(stringify!(REPEAT_MIRRORED), f),
            x => write!(f, "{}({})", stringify!(SamplerAddressingMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerFilterMode(pub(crate) u32);
impl SamplerFilterMode {
    pub const NEAREST: Self = Self(0u32);
    pub const LINEAR: Self = Self(1u32);
}
impl Word for SamplerFilterMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for SamplerFilterMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NEAREST => Display::fmt(stringify!(NEAREST), f),
            Self::LINEAR => Display::fmt(stringify!(LINEAR), f),
            x => write!(f, "{}({})", stringify!(SamplerFilterMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageFormat(pub(crate) u32);
impl ImageFormat {
    pub const UNKNOWN: Self = Self(0u32);
    pub const RGBA_32F: Self = Self(1u32);
    pub const RGBA_16F: Self = Self(2u32);
    pub const R32F: Self = Self(3u32);
    pub const RGBA_8: Self = Self(4u32);
    pub const RGBA_8_SNORM: Self = Self(5u32);
    pub const RG_32F: Self = Self(6u32);
    pub const RG_16F: Self = Self(7u32);
    pub const R11F_G11F_B10F: Self = Self(8u32);
    pub const R16F: Self = Self(9u32);
    pub const RGBA_16: Self = Self(10u32);
    pub const RGB_10_A2: Self = Self(11u32);
    pub const RG_16: Self = Self(12u32);
    pub const RG_8: Self = Self(13u32);
    pub const R16: Self = Self(14u32);
    pub const R8: Self = Self(15u32);
    pub const RGBA_16_SNORM: Self = Self(16u32);
    pub const RG_16_SNORM: Self = Self(17u32);
    pub const RG_8_SNORM: Self = Self(18u32);
    pub const R16SNORM: Self = Self(19u32);
    pub const R8SNORM: Self = Self(20u32);
    pub const RGBA_32I: Self = Self(21u32);
    pub const RGBA_16I: Self = Self(22u32);
    pub const RGBA_8I: Self = Self(23u32);
    pub const R32I: Self = Self(24u32);
    pub const RG_32I: Self = Self(25u32);
    pub const RG_16I: Self = Self(26u32);
    pub const RG_8I: Self = Self(27u32);
    pub const R16I: Self = Self(28u32);
    pub const R8I: Self = Self(29u32);
    pub const RGBA_32UI: Self = Self(30u32);
    pub const RGBA_16UI: Self = Self(31u32);
    pub const RGBA_8UI: Self = Self(32u32);
    pub const R32UI: Self = Self(33u32);
    pub const RGB_10A_2UI: Self = Self(34u32);
    pub const RG_32UI: Self = Self(35u32);
    pub const RG_16UI: Self = Self(36u32);
    pub const RG_8UI: Self = Self(37u32);
    pub const R16UI: Self = Self(38u32);
    pub const R8UI: Self = Self(39u32);
    pub const R64UI: Self = Self(40u32);
    pub const R64I: Self = Self(41u32);
}
impl Word for ImageFormat {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ImageFormat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN => Display::fmt(stringify!(UNKNOWN), f),
            Self::RGBA_32F => Display::fmt(stringify!(RGBA_32F), f),
            Self::RGBA_16F => Display::fmt(stringify!(RGBA_16F), f),
            Self::R32F => Display::fmt(stringify!(R32F), f),
            Self::RGBA_8 => Display::fmt(stringify!(RGBA_8), f),
            Self::RGBA_8_SNORM => Display::fmt(stringify!(RGBA_8_SNORM), f),
            Self::RG_32F => Display::fmt(stringify!(RG_32F), f),
            Self::RG_16F => Display::fmt(stringify!(RG_16F), f),
            Self::R11F_G11F_B10F => Display::fmt(stringify!(R11F_G11F_B10F), f),
            Self::R16F => Display::fmt(stringify!(R16F), f),
            Self::RGBA_16 => Display::fmt(stringify!(RGBA_16), f),
            Self::RGB_10_A2 => Display::fmt(stringify!(RGB_10_A2), f),
            Self::RG_16 => Display::fmt(stringify!(RG_16), f),
            Self::RG_8 => Display::fmt(stringify!(RG_8), f),
            Self::R16 => Display::fmt(stringify!(R16), f),
            Self::R8 => Display::fmt(stringify!(R8), f),
            Self::RGBA_16_SNORM => Display::fmt(stringify!(RGBA_16_SNORM), f),
            Self::RG_16_SNORM => Display::fmt(stringify!(RG_16_SNORM), f),
            Self::RG_8_SNORM => Display::fmt(stringify!(RG_8_SNORM), f),
            Self::R16SNORM => Display::fmt(stringify!(R16SNORM), f),
            Self::R8SNORM => Display::fmt(stringify!(R8SNORM), f),
            Self::RGBA_32I => Display::fmt(stringify!(RGBA_32I), f),
            Self::RGBA_16I => Display::fmt(stringify!(RGBA_16I), f),
            Self::RGBA_8I => Display::fmt(stringify!(RGBA_8I), f),
            Self::R32I => Display::fmt(stringify!(R32I), f),
            Self::RG_32I => Display::fmt(stringify!(RG_32I), f),
            Self::RG_16I => Display::fmt(stringify!(RG_16I), f),
            Self::RG_8I => Display::fmt(stringify!(RG_8I), f),
            Self::R16I => Display::fmt(stringify!(R16I), f),
            Self::R8I => Display::fmt(stringify!(R8I), f),
            Self::RGBA_32UI => Display::fmt(stringify!(RGBA_32UI), f),
            Self::RGBA_16UI => Display::fmt(stringify!(RGBA_16UI), f),
            Self::RGBA_8UI => Display::fmt(stringify!(RGBA_8UI), f),
            Self::R32UI => Display::fmt(stringify!(R32UI), f),
            Self::RGB_10A_2UI => Display::fmt(stringify!(RGB_10A_2UI), f),
            Self::RG_32UI => Display::fmt(stringify!(RG_32UI), f),
            Self::RG_16UI => Display::fmt(stringify!(RG_16UI), f),
            Self::RG_8UI => Display::fmt(stringify!(RG_8UI), f),
            Self::R16UI => Display::fmt(stringify!(R16UI), f),
            Self::R8UI => Display::fmt(stringify!(R8UI), f),
            Self::R64UI => Display::fmt(stringify!(R64UI), f),
            Self::R64I => Display::fmt(stringify!(R64I), f),
            x => write!(f, "{}({})", stringify!(ImageFormat), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageChannelOrder(pub(crate) u32);
impl ImageChannelOrder {
    pub const R: Self = Self(0u32);
    pub const A: Self = Self(1u32);
    pub const RG: Self = Self(2u32);
    pub const RA: Self = Self(3u32);
    pub const RGB: Self = Self(4u32);
    pub const RGBA: Self = Self(5u32);
    pub const BGRA: Self = Self(6u32);
    pub const ARGB: Self = Self(7u32);
    pub const INTENSITY: Self = Self(8u32);
    pub const LUMINANCE: Self = Self(9u32);
    pub const RX: Self = Self(10u32);
    pub const RGX: Self = Self(11u32);
    pub const RGBX: Self = Self(12u32);
    pub const DEPTH: Self = Self(13u32);
    pub const DEPTH_STENCIL: Self = Self(14u32);
    pub const SRGB: Self = Self(15u32);
    pub const SRGBX: Self = Self(16u32);
    pub const SRGBA: Self = Self(17u32);
    pub const SBGRA: Self = Self(18u32);
    pub const ABGR: Self = Self(19u32);
}
impl Word for ImageChannelOrder {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ImageChannelOrder {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::R => Display::fmt(stringify!(R), f),
            Self::A => Display::fmt(stringify!(A), f),
            Self::RG => Display::fmt(stringify!(RG), f),
            Self::RA => Display::fmt(stringify!(RA), f),
            Self::RGB => Display::fmt(stringify!(RGB), f),
            Self::RGBA => Display::fmt(stringify!(RGBA), f),
            Self::BGRA => Display::fmt(stringify!(BGRA), f),
            Self::ARGB => Display::fmt(stringify!(ARGB), f),
            Self::INTENSITY => Display::fmt(stringify!(INTENSITY), f),
            Self::LUMINANCE => Display::fmt(stringify!(LUMINANCE), f),
            Self::RX => Display::fmt(stringify!(RX), f),
            Self::RGX => Display::fmt(stringify!(RGX), f),
            Self::RGBX => Display::fmt(stringify!(RGBX), f),
            Self::DEPTH => Display::fmt(stringify!(DEPTH), f),
            Self::DEPTH_STENCIL => Display::fmt(stringify!(DEPTH_STENCIL), f),
            Self::SRGB => Display::fmt(stringify!(SRGB), f),
            Self::SRGBX => Display::fmt(stringify!(SRGBX), f),
            Self::SRGBA => Display::fmt(stringify!(SRGBA), f),
            Self::SBGRA => Display::fmt(stringify!(SBGRA), f),
            Self::ABGR => Display::fmt(stringify!(ABGR), f),
            x => write!(f, "{}({})", stringify!(ImageChannelOrder), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageChannelDataType(pub(crate) u32);
impl ImageChannelDataType {
    pub const SNORM_INT_8: Self = Self(0u32);
    pub const SNORM_INT_16: Self = Self(1u32);
    pub const UNORM_INT_8: Self = Self(2u32);
    pub const UNORM_INT_16: Self = Self(3u32);
    pub const UNORM_SHORT_565: Self = Self(4u32);
    pub const UNORM_SHORT_555: Self = Self(5u32);
    pub const UNORM_INT_101010: Self = Self(6u32);
    pub const SIGNED_INT_8: Self = Self(7u32);
    pub const SIGNED_INT_16: Self = Self(8u32);
    pub const SIGNED_INT_32: Self = Self(9u32);
    pub const UNSIGNED_INT_8: Self = Self(10u32);
    pub const UNSIGNED_INT_16: Self = Self(11u32);
    pub const UNSIGNED_INT_32: Self = Self(12u32);
    pub const HALF_FLOAT: Self = Self(13u32);
    pub const FLOAT: Self = Self(14u32);
    pub const UNORM_INT_24: Self = Self(15u32);
    pub const UNORM_INT_101010_2: Self = Self(16u32);
    pub const UNORM_INT_10_X6EXT: Self = Self(17u32);
    pub const UNSIGNED_INT_RAW_10_EXT: Self = Self(19u32);
    pub const UNSIGNED_INT_RAW_12_EXT: Self = Self(20u32);
    pub const UNORM_INT_2_101010_EXT: Self = Self(21u32);
    pub const UNSIGNED_INT_10_X6EXT: Self = Self(22u32);
    pub const UNSIGNED_INT_12_X4EXT: Self = Self(23u32);
    pub const UNSIGNED_INT_14_X2EXT: Self = Self(24u32);
    pub const UNORM_INT_12_X4EXT: Self = Self(25u32);
    pub const UNORM_INT_14_X2EXT: Self = Self(26u32);
}
impl Word for ImageChannelDataType {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ImageChannelDataType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SNORM_INT_8 => Display::fmt(stringify!(SNORM_INT_8), f),
            Self::SNORM_INT_16 => Display::fmt(stringify!(SNORM_INT_16), f),
            Self::UNORM_INT_8 => Display::fmt(stringify!(UNORM_INT_8), f),
            Self::UNORM_INT_16 => Display::fmt(stringify!(UNORM_INT_16), f),
            Self::UNORM_SHORT_565 => Display::fmt(stringify!(UNORM_SHORT_565), f),
            Self::UNORM_SHORT_555 => Display::fmt(stringify!(UNORM_SHORT_555), f),
            Self::UNORM_INT_101010 => Display::fmt(stringify!(UNORM_INT_101010), f),
            Self::SIGNED_INT_8 => Display::fmt(stringify!(SIGNED_INT_8), f),
            Self::SIGNED_INT_16 => Display::fmt(stringify!(SIGNED_INT_16), f),
            Self::SIGNED_INT_32 => Display::fmt(stringify!(SIGNED_INT_32), f),
            Self::UNSIGNED_INT_8 => Display::fmt(stringify!(UNSIGNED_INT_8), f),
            Self::UNSIGNED_INT_16 => Display::fmt(stringify!(UNSIGNED_INT_16), f),
            Self::UNSIGNED_INT_32 => Display::fmt(stringify!(UNSIGNED_INT_32), f),
            Self::HALF_FLOAT => Display::fmt(stringify!(HALF_FLOAT), f),
            Self::FLOAT => Display::fmt(stringify!(FLOAT), f),
            Self::UNORM_INT_24 => Display::fmt(stringify!(UNORM_INT_24), f),
            Self::UNORM_INT_101010_2 => Display::fmt(stringify!(UNORM_INT_101010_2), f),
            Self::UNORM_INT_10_X6EXT => Display::fmt(stringify!(UNORM_INT_10_X6EXT), f),
            Self::UNSIGNED_INT_RAW_10_EXT => Display::fmt(stringify!(UNSIGNED_INT_RAW_10_EXT), f),
            Self::UNSIGNED_INT_RAW_12_EXT => Display::fmt(stringify!(UNSIGNED_INT_RAW_12_EXT), f),
            Self::UNORM_INT_2_101010_EXT => Display::fmt(stringify!(UNORM_INT_2_101010_EXT), f),
            Self::UNSIGNED_INT_10_X6EXT => Display::fmt(stringify!(UNSIGNED_INT_10_X6EXT), f),
            Self::UNSIGNED_INT_12_X4EXT => Display::fmt(stringify!(UNSIGNED_INT_12_X4EXT), f),
            Self::UNSIGNED_INT_14_X2EXT => Display::fmt(stringify!(UNSIGNED_INT_14_X2EXT), f),
            Self::UNORM_INT_12_X4EXT => Display::fmt(stringify!(UNORM_INT_12_X4EXT), f),
            Self::UNORM_INT_14_X2EXT => Display::fmt(stringify!(UNORM_INT_14_X2EXT), f),
            x => write!(f, "{}({})", stringify!(ImageChannelDataType), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FPRoundingMode(pub(crate) u32);
impl FPRoundingMode {
    pub const RTE: Self = Self(0u32);
    pub const RTZ: Self = Self(1u32);
    pub const RTP: Self = Self(2u32);
    pub const RTN: Self = Self(3u32);
}
impl Word for FPRoundingMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for FPRoundingMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RTE => Display::fmt(stringify!(RTE), f),
            Self::RTZ => Display::fmt(stringify!(RTZ), f),
            Self::RTP => Display::fmt(stringify!(RTP), f),
            Self::RTN => Display::fmt(stringify!(RTN), f),
            x => write!(f, "{}({})", stringify!(FPRoundingMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FPDenormMode(pub(crate) u32);
impl FPDenormMode {
    pub const PRESERVE: Self = Self(0u32);
    pub const FLUSH_TO_ZERO: Self = Self(1u32);
}
impl Word for FPDenormMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for FPDenormMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::PRESERVE => Display::fmt(stringify!(PRESERVE), f),
            Self::FLUSH_TO_ZERO => Display::fmt(stringify!(FLUSH_TO_ZERO), f),
            x => write!(f, "{}({})", stringify!(FPDenormMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantizationModes(pub(crate) u32);
impl QuantizationModes {
    pub const TRN: Self = Self(0u32);
    pub const TRN_ZERO: Self = Self(1u32);
    pub const RND: Self = Self(2u32);
    pub const RND_ZERO: Self = Self(3u32);
    pub const RND_INF: Self = Self(4u32);
    pub const RND_MIN_INF: Self = Self(5u32);
    pub const RND_CONV: Self = Self(6u32);
    pub const RND_CONV_ODD: Self = Self(7u32);
}
impl Word for QuantizationModes {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for QuantizationModes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TRN => Display::fmt(stringify!(TRN), f),
            Self::TRN_ZERO => Display::fmt(stringify!(TRN_ZERO), f),
            Self::RND => Display::fmt(stringify!(RND), f),
            Self::RND_ZERO => Display::fmt(stringify!(RND_ZERO), f),
            Self::RND_INF => Display::fmt(stringify!(RND_INF), f),
            Self::RND_MIN_INF => Display::fmt(stringify!(RND_MIN_INF), f),
            Self::RND_CONV => Display::fmt(stringify!(RND_CONV), f),
            Self::RND_CONV_ODD => Display::fmt(stringify!(RND_CONV_ODD), f),
            x => write!(f, "{}({})", stringify!(QuantizationModes), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FPOperationMode(pub(crate) u32);
impl FPOperationMode {
    pub const IEEE: Self = Self(0u32);
    pub const ALT: Self = Self(1u32);
}
impl Word for FPOperationMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for FPOperationMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IEEE => Display::fmt(stringify!(IEEE), f),
            Self::ALT => Display::fmt(stringify!(ALT), f),
            x => write!(f, "{}({})", stringify!(FPOperationMode), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverflowModes(pub(crate) u32);
impl OverflowModes {
    pub const WRAP: Self = Self(0u32);
    pub const SAT: Self = Self(1u32);
    pub const SAT_ZERO: Self = Self(2u32);
    pub const SAT_SYM: Self = Self(3u32);
}
impl Word for OverflowModes {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for OverflowModes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WRAP => Display::fmt(stringify!(WRAP), f),
            Self::SAT => Display::fmt(stringify!(SAT), f),
            Self::SAT_ZERO => Display::fmt(stringify!(SAT_ZERO), f),
            Self::SAT_SYM => Display::fmt(stringify!(SAT_SYM), f),
            x => write!(f, "{}({})", stringify!(OverflowModes), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkageType(pub(crate) u32);
impl LinkageType {
    pub const EXPORT: Self = Self(0u32);
    pub const IMPORT: Self = Self(1u32);
    pub const LINK_ONCE_ODR: Self = Self(2u32);
}
impl Word for LinkageType {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for LinkageType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EXPORT => Display::fmt(stringify!(EXPORT), f),
            Self::IMPORT => Display::fmt(stringify!(IMPORT), f),
            Self::LINK_ONCE_ODR => Display::fmt(stringify!(LINK_ONCE_ODR), f),
            x => write!(f, "{}({})", stringify!(LinkageType), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccessQualifier(pub(crate) u32);
impl AccessQualifier {
    pub const READ_ONLY: Self = Self(0u32);
    pub const WRITE_ONLY: Self = Self(1u32);
    pub const READ_WRITE: Self = Self(2u32);
}
impl Word for AccessQualifier {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for AccessQualifier {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::READ_ONLY => Display::fmt(stringify!(READ_ONLY), f),
            Self::WRITE_ONLY => Display::fmt(stringify!(WRITE_ONLY), f),
            Self::READ_WRITE => Display::fmt(stringify!(READ_WRITE), f),
            x => write!(f, "{}({})", stringify!(AccessQualifier), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostAccessQualifier(pub(crate) u32);
impl HostAccessQualifier {
    pub const NONE_INTEL: Self = Self(0u32);
    pub const READ_INTEL: Self = Self(1u32);
    pub const WRITE_INTEL: Self = Self(2u32);
    pub const READ_WRITE_INTEL: Self = Self(3u32);
}
impl Word for HostAccessQualifier {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for HostAccessQualifier {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_INTEL => Display::fmt(stringify!(NONE_INTEL), f),
            Self::READ_INTEL => Display::fmt(stringify!(READ_INTEL), f),
            Self::WRITE_INTEL => Display::fmt(stringify!(WRITE_INTEL), f),
            Self::READ_WRITE_INTEL => Display::fmt(stringify!(READ_WRITE_INTEL), f),
            x => write!(f, "{}({})", stringify!(HostAccessQualifier), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionParameterAttribute(pub(crate) u32);
impl FunctionParameterAttribute {
    pub const ZEXT: Self = Self(0u32);
    pub const SEXT: Self = Self(1u32);
    pub const BY_VAL: Self = Self(2u32);
    pub const SRET: Self = Self(3u32);
    pub const NO_ALIAS: Self = Self(4u32);
    pub const NO_CAPTURE: Self = Self(5u32);
    pub const NO_WRITE: Self = Self(6u32);
    pub const NO_READ_WRITE: Self = Self(7u32);
    pub const RUNTIME_ALIGNED_ALTERA: Self = Self(5940u32);
}
impl Word for FunctionParameterAttribute {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for FunctionParameterAttribute {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ZEXT => Display::fmt(stringify!(ZEXT), f),
            Self::SEXT => Display::fmt(stringify!(SEXT), f),
            Self::BY_VAL => Display::fmt(stringify!(BY_VAL), f),
            Self::SRET => Display::fmt(stringify!(SRET), f),
            Self::NO_ALIAS => Display::fmt(stringify!(NO_ALIAS), f),
            Self::NO_CAPTURE => Display::fmt(stringify!(NO_CAPTURE), f),
            Self::NO_WRITE => Display::fmt(stringify!(NO_WRITE), f),
            Self::NO_READ_WRITE => Display::fmt(stringify!(NO_READ_WRITE), f),
            Self::RUNTIME_ALIGNED_ALTERA => Display::fmt(stringify!(RUNTIME_ALIGNED_ALTERA), f),
            x => write!(f, "{}({})", stringify!(FunctionParameterAttribute), x.0),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum DecorationOperands<'a> {
    RelaxedPrecision,
    SpecId {
        specialization_constant_id: LiteralInteger,
    },
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride {
        array_stride: LiteralInteger,
    },
    MatrixStride {
        matrix_stride: LiteralInteger,
    },
    GLSLShared,
    GLSLPacked,
    CPacked,
    BuiltIn {
        built_in: BuiltIn,
    },
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    UniformId {
        execution: IdScope,
    },
    SaturatedConversion,
    Stream {
        stream_number: LiteralInteger,
    },
    Location {
        location: LiteralInteger,
    },
    Component {
        component: LiteralInteger,
    },
    Index {
        index: LiteralInteger,
    },
    Binding {
        binding_point: LiteralInteger,
    },
    DescriptorSet {
        descriptor_set: LiteralInteger,
    },
    Offset {
        byte_offset: LiteralInteger,
    },
    XfbBuffer {
        xfb_buffer_number: LiteralInteger,
    },
    XfbStride {
        xfb_stride: LiteralInteger,
    },
    FuncParamAttr {
        function_parameter_attribute: FunctionParameterAttribute,
    },
    FPRoundingMode {
        floating_point_rounding_mode: FPRoundingMode,
    },
    FPFastMathMode {
        fast_math_mode: FPFastMathMode,
    },
    LinkageAttributes {
        name: LiteralString<'a>,
        linkage_type: LinkageType,
    },
    NoContraction,
    InputAttachmentIndex {
        attachment_index: LiteralInteger,
    },
    Alignment {
        alignment: LiteralInteger,
    },
    MaxByteOffset {
        max_byte_offset: LiteralInteger,
    },
    AlignmentId {
        alignment: IdRef,
    },
    MaxByteOffsetId {
        max_byte_offset: IdRef,
    },
    SaturatedToLargestFloat8NormalConversionEXT,
    NoSignedWrap,
    NoUnsignedWrap,
    WeightTextureQCOM,
    BlockMatchTextureQCOM,
    BlockMatchSamplerQCOM,
    ExplicitInterpAMD,
    NodeSharesPayloadLimitsWithAMDX {
        payload_type: IdRef,
    },
    NodeMaxPayloadsAMDX {
        max_number_of_payloads: IdRef,
    },
    TrackFinishWritingAMDX,
    PayloadNodeNameAMDX {
        node_name: IdRef,
    },
    PayloadNodeBaseIndexAMDX {
        base_index: IdRef,
    },
    PayloadNodeSparseArrayAMDX,
    PayloadNodeArraySizeAMDX {
        array_size: IdRef,
    },
    PayloadDispatchIndirectAMDX,
    ArrayStrideIdEXT {
        array_stride: IdRef,
    },
    OffsetIdEXT {
        byte_offset: IdRef,
    },
    UTFEncodedKHR,
    OverrideCoverageNV,
    PassthroughNV,
    ViewportRelativeNV,
    SecondaryViewportRelativeNV {
        offset: LiteralInteger,
    },
    PerPrimitiveEXT,
    PerViewNV,
    PerTaskNV,
    PerVertexKHR,
    NonUniform,
    RestrictPointer,
    AliasedPointer,
    MemberOffsetNV {
        member_offset: LiteralInteger,
    },
    HitObjectShaderRecordBufferNV,
    HitObjectShaderRecordBufferEXT,
    BankNV {
        bank: LiteralInteger,
    },
    BindlessSamplerNV,
    BindlessImageNV,
    BoundSamplerNV,
    BoundImageNV,
    SIMTCallINTEL {
        n: LiteralInteger,
    },
    ReferencedIndirectlyINTEL,
    ClobberINTEL {
        register: LiteralString<'a>,
    },
    SideEffectsINTEL,
    VectorComputeVariableINTEL,
    FuncParamIOKindINTEL {
        kind: LiteralInteger,
    },
    VectorComputeFunctionINTEL,
    StackCallINTEL,
    GlobalVariableOffsetINTEL {
        offset: LiteralInteger,
    },
    CounterBuffer {
        counter_buffer: IdRef,
    },
    UserSemantic {
        semantic: LiteralString<'a>,
    },
    UserTypeGOOGLE {
        user_type: LiteralString<'a>,
    },
    FunctionRoundingModeINTEL {
        target_width: LiteralInteger,
        fp_rounding_mode: FPRoundingMode,
    },
    FunctionDenormModeINTEL {
        target_width: LiteralInteger,
        fp_denorm_mode: FPDenormMode,
    },
    RegisterALTERA,
    MemoryALTERA {
        memory_type: LiteralString<'a>,
    },
    NumbanksALTERA {
        banks: LiteralInteger,
    },
    BankwidthALTERA {
        bank_width: LiteralInteger,
    },
    MaxPrivateCopiesALTERA {
        maximum_copies: LiteralInteger,
    },
    SinglepumpALTERA,
    DoublepumpALTERA,
    MaxReplicatesALTERA {
        maximum_replicates: LiteralInteger,
    },
    SimpleDualPortALTERA,
    MergeALTERA {
        merge_key: LiteralString<'a>,
        merge_type: LiteralString<'a>,
    },
    BankBitsALTERA {
        bank_bits: LiteralInteger,
    },
    ForcePow2DepthALTERA {
        force_key: LiteralInteger,
    },
    StridesizeALTERA {
        stride_size: LiteralInteger,
    },
    WordsizeALTERA {
        word_size: LiteralInteger,
    },
    TrueDualPortALTERA,
    BurstCoalesceALTERA,
    CacheSizeALTERA {
        cache_size_in_bytes: LiteralInteger,
    },
    DontStaticallyCoalesceALTERA,
    PrefetchALTERA {
        prefetcher_size_in_bytes: LiteralInteger,
    },
    StallEnableALTERA,
    FuseLoopsInFunctionALTERA,
    MathOpDSPModeALTERA {
        mode: LiteralInteger,
        propagate: LiteralInteger,
    },
    AliasScopeINTEL {
        aliasing_scopes_list: IdRef,
    },
    NoAliasINTEL {
        aliasing_scopes_list: IdRef,
    },
    InitiationIntervalALTERA {
        cycles: LiteralInteger,
    },
    MaxConcurrencyALTERA {
        invocations: LiteralInteger,
    },
    PipelineEnableALTERA {
        enable: LiteralInteger,
    },
    BufferLocationALTERA {
        buffer_location_id: LiteralInteger,
    },
    IOPipeStorageALTERA {
        io_pipe_id: LiteralInteger,
    },
    FunctionFloatingPointModeINTEL {
        target_width: LiteralInteger,
        fp_operation_mode: FPOperationMode,
    },
    SingleElementVectorINTEL,
    VectorComputeCallableFunctionINTEL,
    MediaBlockIOINTEL,
    StallFreeALTERA,
    FPMaxErrorDecorationINTEL {
        max_error: LiteralFloat,
    },
    LatencyControlLabelALTERA {
        latency_label: LiteralInteger,
    },
    LatencyControlConstraintALTERA {
        relative_to: LiteralInteger,
        control_type: LiteralInteger,
        relative_cycle: LiteralInteger,
    },
    ConduitKernelArgumentALTERA,
    RegisterMapKernelArgumentALTERA,
    MMHostInterfaceAddressWidthALTERA {
        address_width: LiteralInteger,
    },
    MMHostInterfaceDataWidthALTERA {
        data_width: LiteralInteger,
    },
    MMHostInterfaceLatencyALTERA {
        latency: LiteralInteger,
    },
    MMHostInterfaceReadWriteModeALTERA {
        read_write_mode: AccessQualifier,
    },
    MMHostInterfaceMaxBurstALTERA {
        max_burst_count: LiteralInteger,
    },
    MMHostInterfaceWaitRequestALTERA {
        waitrequest: LiteralInteger,
    },
    StableKernelArgumentALTERA,
    HostAccessINTEL {
        access: HostAccessQualifier,
        name: LiteralString<'a>,
    },
    InitModeALTERA {
        trigger: InitializationModeQualifier,
    },
    ImplementInRegisterMapALTERA {
        value: LiteralInteger,
    },
    ConditionalINTEL {
        condition: IdRef,
    },
    CacheControlLoadINTEL {
        cache_level: LiteralInteger,
        cache_control: LoadCacheControl,
    },
    CacheControlStoreINTEL {
        cache_level: LiteralInteger,
        cache_control: StoreCacheControl,
    },
}
impl<'a> DecorationOperands<'a> {
    #[inline]
    pub fn parse_one(stream: &mut InstructionStream<'a>) -> ParseResult<Self> {
        let variant = stream.read()?;
        match variant {
            0u32 => Ok(Self::RelaxedPrecision),
            1u32 => Ok(Self::SpecId {
                specialization_constant_id: LiteralInteger::parse_one(stream)?,
            }),
            2u32 => Ok(Self::Block),
            3u32 => Ok(Self::BufferBlock),
            4u32 => Ok(Self::RowMajor),
            5u32 => Ok(Self::ColMajor),
            6u32 => Ok(Self::ArrayStride {
                array_stride: LiteralInteger::parse_one(stream)?,
            }),
            7u32 => Ok(Self::MatrixStride {
                matrix_stride: LiteralInteger::parse_one(stream)?,
            }),
            8u32 => Ok(Self::GLSLShared),
            9u32 => Ok(Self::GLSLPacked),
            10u32 => Ok(Self::CPacked),
            11u32 => Ok(Self::BuiltIn {
                built_in: BuiltIn::parse_one(stream)?,
            }),
            13u32 => Ok(Self::NoPerspective),
            14u32 => Ok(Self::Flat),
            15u32 => Ok(Self::Patch),
            16u32 => Ok(Self::Centroid),
            17u32 => Ok(Self::Sample),
            18u32 => Ok(Self::Invariant),
            19u32 => Ok(Self::Restrict),
            20u32 => Ok(Self::Aliased),
            21u32 => Ok(Self::Volatile),
            22u32 => Ok(Self::Constant),
            23u32 => Ok(Self::Coherent),
            24u32 => Ok(Self::NonWritable),
            25u32 => Ok(Self::NonReadable),
            26u32 => Ok(Self::Uniform),
            27u32 => Ok(Self::UniformId {
                execution: IdScope::parse_one(stream)?,
            }),
            28u32 => Ok(Self::SaturatedConversion),
            29u32 => Ok(Self::Stream {
                stream_number: LiteralInteger::parse_one(stream)?,
            }),
            30u32 => Ok(Self::Location {
                location: LiteralInteger::parse_one(stream)?,
            }),
            31u32 => Ok(Self::Component {
                component: LiteralInteger::parse_one(stream)?,
            }),
            32u32 => Ok(Self::Index {
                index: LiteralInteger::parse_one(stream)?,
            }),
            33u32 => Ok(Self::Binding {
                binding_point: LiteralInteger::parse_one(stream)?,
            }),
            34u32 => Ok(Self::DescriptorSet {
                descriptor_set: LiteralInteger::parse_one(stream)?,
            }),
            35u32 => Ok(Self::Offset {
                byte_offset: LiteralInteger::parse_one(stream)?,
            }),
            36u32 => Ok(Self::XfbBuffer {
                xfb_buffer_number: LiteralInteger::parse_one(stream)?,
            }),
            37u32 => Ok(Self::XfbStride {
                xfb_stride: LiteralInteger::parse_one(stream)?,
            }),
            38u32 => Ok(Self::FuncParamAttr {
                function_parameter_attribute: FunctionParameterAttribute::parse_one(stream)?,
            }),
            39u32 => Ok(Self::FPRoundingMode {
                floating_point_rounding_mode: FPRoundingMode::parse_one(stream)?,
            }),
            40u32 => Ok(Self::FPFastMathMode {
                fast_math_mode: FPFastMathMode::parse_one(stream)?,
            }),
            41u32 => Ok(Self::LinkageAttributes {
                name: LiteralString::parse_one(stream)?,
                linkage_type: LinkageType::parse_one(stream)?,
            }),
            42u32 => Ok(Self::NoContraction),
            43u32 => Ok(Self::InputAttachmentIndex {
                attachment_index: LiteralInteger::parse_one(stream)?,
            }),
            44u32 => Ok(Self::Alignment {
                alignment: LiteralInteger::parse_one(stream)?,
            }),
            45u32 => Ok(Self::MaxByteOffset {
                max_byte_offset: LiteralInteger::parse_one(stream)?,
            }),
            46u32 => Ok(Self::AlignmentId {
                alignment: IdRef::parse_one(stream)?,
            }),
            47u32 => Ok(Self::MaxByteOffsetId {
                max_byte_offset: IdRef::parse_one(stream)?,
            }),
            4216u32 => Ok(Self::SaturatedToLargestFloat8NormalConversionEXT),
            4469u32 => Ok(Self::NoSignedWrap),
            4470u32 => Ok(Self::NoUnsignedWrap),
            4487u32 => Ok(Self::WeightTextureQCOM),
            4488u32 => Ok(Self::BlockMatchTextureQCOM),
            4499u32 => Ok(Self::BlockMatchSamplerQCOM),
            4999u32 => Ok(Self::ExplicitInterpAMD),
            5019u32 => Ok(Self::NodeSharesPayloadLimitsWithAMDX {
                payload_type: IdRef::parse_one(stream)?,
            }),
            5020u32 => Ok(Self::NodeMaxPayloadsAMDX {
                max_number_of_payloads: IdRef::parse_one(stream)?,
            }),
            5078u32 => Ok(Self::TrackFinishWritingAMDX),
            5091u32 => Ok(Self::PayloadNodeNameAMDX {
                node_name: IdRef::parse_one(stream)?,
            }),
            5098u32 => Ok(Self::PayloadNodeBaseIndexAMDX {
                base_index: IdRef::parse_one(stream)?,
            }),
            5099u32 => Ok(Self::PayloadNodeSparseArrayAMDX),
            5100u32 => Ok(Self::PayloadNodeArraySizeAMDX {
                array_size: IdRef::parse_one(stream)?,
            }),
            5105u32 => Ok(Self::PayloadDispatchIndirectAMDX),
            5124u32 => Ok(Self::ArrayStrideIdEXT {
                array_stride: IdRef::parse_one(stream)?,
            }),
            5125u32 => Ok(Self::OffsetIdEXT {
                byte_offset: IdRef::parse_one(stream)?,
            }),
            5145u32 => Ok(Self::UTFEncodedKHR),
            5248u32 => Ok(Self::OverrideCoverageNV),
            5250u32 => Ok(Self::PassthroughNV),
            5252u32 => Ok(Self::ViewportRelativeNV),
            5256u32 => Ok(Self::SecondaryViewportRelativeNV {
                offset: LiteralInteger::parse_one(stream)?,
            }),
            5271u32 => Ok(Self::PerPrimitiveEXT),
            5272u32 => Ok(Self::PerViewNV),
            5273u32 => Ok(Self::PerTaskNV),
            5285u32 => Ok(Self::PerVertexKHR),
            5300u32 => Ok(Self::NonUniform),
            5355u32 => Ok(Self::RestrictPointer),
            5356u32 => Ok(Self::AliasedPointer),
            5358u32 => Ok(Self::MemberOffsetNV {
                member_offset: LiteralInteger::parse_one(stream)?,
            }),
            5386u32 => Ok(Self::HitObjectShaderRecordBufferNV),
            5389u32 => Ok(Self::HitObjectShaderRecordBufferEXT),
            5397u32 => Ok(Self::BankNV {
                bank: LiteralInteger::parse_one(stream)?,
            }),
            5398u32 => Ok(Self::BindlessSamplerNV),
            5399u32 => Ok(Self::BindlessImageNV),
            5400u32 => Ok(Self::BoundSamplerNV),
            5401u32 => Ok(Self::BoundImageNV),
            5599u32 => Ok(Self::SIMTCallINTEL {
                n: LiteralInteger::parse_one(stream)?,
            }),
            5602u32 => Ok(Self::ReferencedIndirectlyINTEL),
            5607u32 => Ok(Self::ClobberINTEL {
                register: LiteralString::parse_one(stream)?,
            }),
            5608u32 => Ok(Self::SideEffectsINTEL),
            5624u32 => Ok(Self::VectorComputeVariableINTEL),
            5625u32 => Ok(Self::FuncParamIOKindINTEL {
                kind: LiteralInteger::parse_one(stream)?,
            }),
            5626u32 => Ok(Self::VectorComputeFunctionINTEL),
            5627u32 => Ok(Self::StackCallINTEL),
            5628u32 => Ok(Self::GlobalVariableOffsetINTEL {
                offset: LiteralInteger::parse_one(stream)?,
            }),
            5634u32 => Ok(Self::CounterBuffer {
                counter_buffer: IdRef::parse_one(stream)?,
            }),
            5635u32 => Ok(Self::UserSemantic {
                semantic: LiteralString::parse_one(stream)?,
            }),
            5636u32 => Ok(Self::UserTypeGOOGLE {
                user_type: LiteralString::parse_one(stream)?,
            }),
            5822u32 => Ok(Self::FunctionRoundingModeINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
                fp_rounding_mode: FPRoundingMode::parse_one(stream)?,
            }),
            5823u32 => Ok(Self::FunctionDenormModeINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
                fp_denorm_mode: FPDenormMode::parse_one(stream)?,
            }),
            5825u32 => Ok(Self::RegisterALTERA),
            5826u32 => Ok(Self::MemoryALTERA {
                memory_type: LiteralString::parse_one(stream)?,
            }),
            5827u32 => Ok(Self::NumbanksALTERA {
                banks: LiteralInteger::parse_one(stream)?,
            }),
            5828u32 => Ok(Self::BankwidthALTERA {
                bank_width: LiteralInteger::parse_one(stream)?,
            }),
            5829u32 => Ok(Self::MaxPrivateCopiesALTERA {
                maximum_copies: LiteralInteger::parse_one(stream)?,
            }),
            5830u32 => Ok(Self::SinglepumpALTERA),
            5831u32 => Ok(Self::DoublepumpALTERA),
            5832u32 => Ok(Self::MaxReplicatesALTERA {
                maximum_replicates: LiteralInteger::parse_one(stream)?,
            }),
            5833u32 => Ok(Self::SimpleDualPortALTERA),
            5834u32 => Ok(Self::MergeALTERA {
                merge_key: LiteralString::parse_one(stream)?,
                merge_type: LiteralString::parse_one(stream)?,
            }),
            5835u32 => Ok(Self::BankBitsALTERA {
                bank_bits: LiteralInteger::parse_one(stream)?,
            }),
            5836u32 => Ok(Self::ForcePow2DepthALTERA {
                force_key: LiteralInteger::parse_one(stream)?,
            }),
            5883u32 => Ok(Self::StridesizeALTERA {
                stride_size: LiteralInteger::parse_one(stream)?,
            }),
            5884u32 => Ok(Self::WordsizeALTERA {
                word_size: LiteralInteger::parse_one(stream)?,
            }),
            5885u32 => Ok(Self::TrueDualPortALTERA),
            5899u32 => Ok(Self::BurstCoalesceALTERA),
            5900u32 => Ok(Self::CacheSizeALTERA {
                cache_size_in_bytes: LiteralInteger::parse_one(stream)?,
            }),
            5901u32 => Ok(Self::DontStaticallyCoalesceALTERA),
            5902u32 => Ok(Self::PrefetchALTERA {
                prefetcher_size_in_bytes: LiteralInteger::parse_one(stream)?,
            }),
            5905u32 => Ok(Self::StallEnableALTERA),
            5907u32 => Ok(Self::FuseLoopsInFunctionALTERA),
            5909u32 => Ok(Self::MathOpDSPModeALTERA {
                mode: LiteralInteger::parse_one(stream)?,
                propagate: LiteralInteger::parse_one(stream)?,
            }),
            5914u32 => Ok(Self::AliasScopeINTEL {
                aliasing_scopes_list: IdRef::parse_one(stream)?,
            }),
            5915u32 => Ok(Self::NoAliasINTEL {
                aliasing_scopes_list: IdRef::parse_one(stream)?,
            }),
            5917u32 => Ok(Self::InitiationIntervalALTERA {
                cycles: LiteralInteger::parse_one(stream)?,
            }),
            5918u32 => Ok(Self::MaxConcurrencyALTERA {
                invocations: LiteralInteger::parse_one(stream)?,
            }),
            5919u32 => Ok(Self::PipelineEnableALTERA {
                enable: LiteralInteger::parse_one(stream)?,
            }),
            5921u32 => Ok(Self::BufferLocationALTERA {
                buffer_location_id: LiteralInteger::parse_one(stream)?,
            }),
            5944u32 => Ok(Self::IOPipeStorageALTERA {
                io_pipe_id: LiteralInteger::parse_one(stream)?,
            }),
            6080u32 => Ok(Self::FunctionFloatingPointModeINTEL {
                target_width: LiteralInteger::parse_one(stream)?,
                fp_operation_mode: FPOperationMode::parse_one(stream)?,
            }),
            6085u32 => Ok(Self::SingleElementVectorINTEL),
            6087u32 => Ok(Self::VectorComputeCallableFunctionINTEL),
            6140u32 => Ok(Self::MediaBlockIOINTEL),
            6151u32 => Ok(Self::StallFreeALTERA),
            6170u32 => Ok(Self::FPMaxErrorDecorationINTEL {
                max_error: LiteralFloat::parse_one(stream)?,
            }),
            6172u32 => Ok(Self::LatencyControlLabelALTERA {
                latency_label: LiteralInteger::parse_one(stream)?,
            }),
            6173u32 => Ok(Self::LatencyControlConstraintALTERA {
                relative_to: LiteralInteger::parse_one(stream)?,
                control_type: LiteralInteger::parse_one(stream)?,
                relative_cycle: LiteralInteger::parse_one(stream)?,
            }),
            6175u32 => Ok(Self::ConduitKernelArgumentALTERA),
            6176u32 => Ok(Self::RegisterMapKernelArgumentALTERA),
            6177u32 => Ok(Self::MMHostInterfaceAddressWidthALTERA {
                address_width: LiteralInteger::parse_one(stream)?,
            }),
            6178u32 => Ok(Self::MMHostInterfaceDataWidthALTERA {
                data_width: LiteralInteger::parse_one(stream)?,
            }),
            6179u32 => Ok(Self::MMHostInterfaceLatencyALTERA {
                latency: LiteralInteger::parse_one(stream)?,
            }),
            6180u32 => Ok(Self::MMHostInterfaceReadWriteModeALTERA {
                read_write_mode: AccessQualifier::parse_one(stream)?,
            }),
            6181u32 => Ok(Self::MMHostInterfaceMaxBurstALTERA {
                max_burst_count: LiteralInteger::parse_one(stream)?,
            }),
            6182u32 => Ok(Self::MMHostInterfaceWaitRequestALTERA {
                waitrequest: LiteralInteger::parse_one(stream)?,
            }),
            6183u32 => Ok(Self::StableKernelArgumentALTERA),
            6188u32 => Ok(Self::HostAccessINTEL {
                access: HostAccessQualifier::parse_one(stream)?,
                name: LiteralString::parse_one(stream)?,
            }),
            6190u32 => Ok(Self::InitModeALTERA {
                trigger: InitializationModeQualifier::parse_one(stream)?,
            }),
            6191u32 => Ok(Self::ImplementInRegisterMapALTERA {
                value: LiteralInteger::parse_one(stream)?,
            }),
            6247u32 => Ok(Self::ConditionalINTEL {
                condition: IdRef::parse_one(stream)?,
            }),
            6442u32 => Ok(Self::CacheControlLoadINTEL {
                cache_level: LiteralInteger::parse_one(stream)?,
                cache_control: LoadCacheControl::parse_one(stream)?,
            }),
            6443u32 => Ok(Self::CacheControlStoreINTEL {
                cache_level: LiteralInteger::parse_one(stream)?,
                cache_control: StoreCacheControl::parse_one(stream)?,
            }),
            x => Err(ParseError::UnknownVariant {
                kind: stringify!(DecorationOperands),
                value: x,
            }),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decoration(pub(crate) u32);
impl Decoration {
    pub const RELAXED_PRECISION: Self = Self(0u32);
    pub const SPEC_ID: Self = Self(1u32);
    pub const BLOCK: Self = Self(2u32);
    pub const BUFFER_BLOCK: Self = Self(3u32);
    pub const ROW_MAJOR: Self = Self(4u32);
    pub const COL_MAJOR: Self = Self(5u32);
    pub const ARRAY_STRIDE: Self = Self(6u32);
    pub const MATRIX_STRIDE: Self = Self(7u32);
    pub const GLSLSHARED: Self = Self(8u32);
    pub const GLSLPACKED: Self = Self(9u32);
    pub const CPACKED: Self = Self(10u32);
    pub const BUILT_IN: Self = Self(11u32);
    pub const NO_PERSPECTIVE: Self = Self(13u32);
    pub const FLAT: Self = Self(14u32);
    pub const PATCH: Self = Self(15u32);
    pub const CENTROID: Self = Self(16u32);
    pub const SAMPLE: Self = Self(17u32);
    pub const INVARIANT: Self = Self(18u32);
    pub const RESTRICT: Self = Self(19u32);
    pub const ALIASED: Self = Self(20u32);
    pub const VOLATILE: Self = Self(21u32);
    pub const CONSTANT: Self = Self(22u32);
    pub const COHERENT: Self = Self(23u32);
    pub const NON_WRITABLE: Self = Self(24u32);
    pub const NON_READABLE: Self = Self(25u32);
    pub const UNIFORM: Self = Self(26u32);
    pub const UNIFORM_ID: Self = Self(27u32);
    pub const SATURATED_CONVERSION: Self = Self(28u32);
    pub const STREAM: Self = Self(29u32);
    pub const LOCATION: Self = Self(30u32);
    pub const COMPONENT: Self = Self(31u32);
    pub const INDEX: Self = Self(32u32);
    pub const BINDING: Self = Self(33u32);
    pub const DESCRIPTOR_SET: Self = Self(34u32);
    pub const OFFSET: Self = Self(35u32);
    pub const XFB_BUFFER: Self = Self(36u32);
    pub const XFB_STRIDE: Self = Self(37u32);
    pub const FUNC_PARAM_ATTR: Self = Self(38u32);
    pub const FPROUNDING_MODE: Self = Self(39u32);
    pub const FPFAST_MATH_MODE: Self = Self(40u32);
    pub const LINKAGE_ATTRIBUTES: Self = Self(41u32);
    pub const NO_CONTRACTION: Self = Self(42u32);
    pub const INPUT_ATTACHMENT_INDEX: Self = Self(43u32);
    pub const ALIGNMENT: Self = Self(44u32);
    pub const MAX_BYTE_OFFSET: Self = Self(45u32);
    pub const ALIGNMENT_ID: Self = Self(46u32);
    pub const MAX_BYTE_OFFSET_ID: Self = Self(47u32);
    pub const SATURATED_TO_LARGEST_FLOAT_8_NORMAL_CONVERSION_EXT: Self = Self(4216u32);
    pub const NO_SIGNED_WRAP: Self = Self(4469u32);
    pub const NO_UNSIGNED_WRAP: Self = Self(4470u32);
    pub const WEIGHT_TEXTURE_QCOM: Self = Self(4487u32);
    pub const BLOCK_MATCH_TEXTURE_QCOM: Self = Self(4488u32);
    pub const BLOCK_MATCH_SAMPLER_QCOM: Self = Self(4499u32);
    pub const EXPLICIT_INTERP_AMD: Self = Self(4999u32);
    pub const NODE_SHARES_PAYLOAD_LIMITS_WITH_AMDX: Self = Self(5019u32);
    pub const NODE_MAX_PAYLOADS_AMDX: Self = Self(5020u32);
    pub const TRACK_FINISH_WRITING_AMDX: Self = Self(5078u32);
    pub const PAYLOAD_NODE_NAME_AMDX: Self = Self(5091u32);
    pub const PAYLOAD_NODE_BASE_INDEX_AMDX: Self = Self(5098u32);
    pub const PAYLOAD_NODE_SPARSE_ARRAY_AMDX: Self = Self(5099u32);
    pub const PAYLOAD_NODE_ARRAY_SIZE_AMDX: Self = Self(5100u32);
    pub const PAYLOAD_DISPATCH_INDIRECT_AMDX: Self = Self(5105u32);
    pub const ARRAY_STRIDE_ID_EXT: Self = Self(5124u32);
    pub const OFFSET_ID_EXT: Self = Self(5125u32);
    pub const UTFENCODED_KHR: Self = Self(5145u32);
    pub const OVERRIDE_COVERAGE_NV: Self = Self(5248u32);
    pub const PASSTHROUGH_NV: Self = Self(5250u32);
    pub const VIEWPORT_RELATIVE_NV: Self = Self(5252u32);
    pub const SECONDARY_VIEWPORT_RELATIVE_NV: Self = Self(5256u32);
    pub const PER_PRIMITIVE_EXT: Self = Self(5271u32);
    pub const PER_VIEW_NV: Self = Self(5272u32);
    pub const PER_TASK_NV: Self = Self(5273u32);
    pub const PER_VERTEX_KHR: Self = Self(5285u32);
    pub const NON_UNIFORM: Self = Self(5300u32);
    pub const RESTRICT_POINTER: Self = Self(5355u32);
    pub const ALIASED_POINTER: Self = Self(5356u32);
    pub const MEMBER_OFFSET_NV: Self = Self(5358u32);
    pub const HIT_OBJECT_SHADER_RECORD_BUFFER_NV: Self = Self(5386u32);
    pub const HIT_OBJECT_SHADER_RECORD_BUFFER_EXT: Self = Self(5389u32);
    pub const BANK_NV: Self = Self(5397u32);
    pub const BINDLESS_SAMPLER_NV: Self = Self(5398u32);
    pub const BINDLESS_IMAGE_NV: Self = Self(5399u32);
    pub const BOUND_SAMPLER_NV: Self = Self(5400u32);
    pub const BOUND_IMAGE_NV: Self = Self(5401u32);
    pub const SIMTCALL_INTEL: Self = Self(5599u32);
    pub const REFERENCED_INDIRECTLY_INTEL: Self = Self(5602u32);
    pub const CLOBBER_INTEL: Self = Self(5607u32);
    pub const SIDE_EFFECTS_INTEL: Self = Self(5608u32);
    pub const VECTOR_COMPUTE_VARIABLE_INTEL: Self = Self(5624u32);
    pub const FUNC_PARAM_IOKIND_INTEL: Self = Self(5625u32);
    pub const VECTOR_COMPUTE_FUNCTION_INTEL: Self = Self(5626u32);
    pub const STACK_CALL_INTEL: Self = Self(5627u32);
    pub const GLOBAL_VARIABLE_OFFSET_INTEL: Self = Self(5628u32);
    pub const COUNTER_BUFFER: Self = Self(5634u32);
    pub const USER_SEMANTIC: Self = Self(5635u32);
    pub const USER_TYPE_GOOGLE: Self = Self(5636u32);
    pub const FUNCTION_ROUNDING_MODE_INTEL: Self = Self(5822u32);
    pub const FUNCTION_DENORM_MODE_INTEL: Self = Self(5823u32);
    pub const REGISTER_ALTERA: Self = Self(5825u32);
    pub const MEMORY_ALTERA: Self = Self(5826u32);
    pub const NUMBANKS_ALTERA: Self = Self(5827u32);
    pub const BANKWIDTH_ALTERA: Self = Self(5828u32);
    pub const MAX_PRIVATE_COPIES_ALTERA: Self = Self(5829u32);
    pub const SINGLEPUMP_ALTERA: Self = Self(5830u32);
    pub const DOUBLEPUMP_ALTERA: Self = Self(5831u32);
    pub const MAX_REPLICATES_ALTERA: Self = Self(5832u32);
    pub const SIMPLE_DUAL_PORT_ALTERA: Self = Self(5833u32);
    pub const MERGE_ALTERA: Self = Self(5834u32);
    pub const BANK_BITS_ALTERA: Self = Self(5835u32);
    pub const FORCE_POW_2DEPTH_ALTERA: Self = Self(5836u32);
    pub const STRIDESIZE_ALTERA: Self = Self(5883u32);
    pub const WORDSIZE_ALTERA: Self = Self(5884u32);
    pub const TRUE_DUAL_PORT_ALTERA: Self = Self(5885u32);
    pub const BURST_COALESCE_ALTERA: Self = Self(5899u32);
    pub const CACHE_SIZE_ALTERA: Self = Self(5900u32);
    pub const DONT_STATICALLY_COALESCE_ALTERA: Self = Self(5901u32);
    pub const PREFETCH_ALTERA: Self = Self(5902u32);
    pub const STALL_ENABLE_ALTERA: Self = Self(5905u32);
    pub const FUSE_LOOPS_IN_FUNCTION_ALTERA: Self = Self(5907u32);
    pub const MATH_OP_DSPMODE_ALTERA: Self = Self(5909u32);
    pub const ALIAS_SCOPE_INTEL: Self = Self(5914u32);
    pub const NO_ALIAS_INTEL: Self = Self(5915u32);
    pub const INITIATION_INTERVAL_ALTERA: Self = Self(5917u32);
    pub const MAX_CONCURRENCY_ALTERA: Self = Self(5918u32);
    pub const PIPELINE_ENABLE_ALTERA: Self = Self(5919u32);
    pub const BUFFER_LOCATION_ALTERA: Self = Self(5921u32);
    pub const IOPIPE_STORAGE_ALTERA: Self = Self(5944u32);
    pub const FUNCTION_FLOATING_POINT_MODE_INTEL: Self = Self(6080u32);
    pub const SINGLE_ELEMENT_VECTOR_INTEL: Self = Self(6085u32);
    pub const VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL: Self = Self(6087u32);
    pub const MEDIA_BLOCK_IOINTEL: Self = Self(6140u32);
    pub const STALL_FREE_ALTERA: Self = Self(6151u32);
    pub const FPMAX_ERROR_DECORATION_INTEL: Self = Self(6170u32);
    pub const LATENCY_CONTROL_LABEL_ALTERA: Self = Self(6172u32);
    pub const LATENCY_CONTROL_CONSTRAINT_ALTERA: Self = Self(6173u32);
    pub const CONDUIT_KERNEL_ARGUMENT_ALTERA: Self = Self(6175u32);
    pub const REGISTER_MAP_KERNEL_ARGUMENT_ALTERA: Self = Self(6176u32);
    pub const MMHOST_INTERFACE_ADDRESS_WIDTH_ALTERA: Self = Self(6177u32);
    pub const MMHOST_INTERFACE_DATA_WIDTH_ALTERA: Self = Self(6178u32);
    pub const MMHOST_INTERFACE_LATENCY_ALTERA: Self = Self(6179u32);
    pub const MMHOST_INTERFACE_READ_WRITE_MODE_ALTERA: Self = Self(6180u32);
    pub const MMHOST_INTERFACE_MAX_BURST_ALTERA: Self = Self(6181u32);
    pub const MMHOST_INTERFACE_WAIT_REQUEST_ALTERA: Self = Self(6182u32);
    pub const STABLE_KERNEL_ARGUMENT_ALTERA: Self = Self(6183u32);
    pub const HOST_ACCESS_INTEL: Self = Self(6188u32);
    pub const INIT_MODE_ALTERA: Self = Self(6190u32);
    pub const IMPLEMENT_IN_REGISTER_MAP_ALTERA: Self = Self(6191u32);
    pub const CONDITIONAL_INTEL: Self = Self(6247u32);
    pub const CACHE_CONTROL_LOAD_INTEL: Self = Self(6442u32);
    pub const CACHE_CONTROL_STORE_INTEL: Self = Self(6443u32);
}
impl Word for Decoration {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for Decoration {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RELAXED_PRECISION => Display::fmt(stringify!(RELAXED_PRECISION), f),
            Self::SPEC_ID => Display::fmt(stringify!(SPEC_ID), f),
            Self::BLOCK => Display::fmt(stringify!(BLOCK), f),
            Self::BUFFER_BLOCK => Display::fmt(stringify!(BUFFER_BLOCK), f),
            Self::ROW_MAJOR => Display::fmt(stringify!(ROW_MAJOR), f),
            Self::COL_MAJOR => Display::fmt(stringify!(COL_MAJOR), f),
            Self::ARRAY_STRIDE => Display::fmt(stringify!(ARRAY_STRIDE), f),
            Self::MATRIX_STRIDE => Display::fmt(stringify!(MATRIX_STRIDE), f),
            Self::GLSLSHARED => Display::fmt(stringify!(GLSLSHARED), f),
            Self::GLSLPACKED => Display::fmt(stringify!(GLSLPACKED), f),
            Self::CPACKED => Display::fmt(stringify!(CPACKED), f),
            Self::BUILT_IN => Display::fmt(stringify!(BUILT_IN), f),
            Self::NO_PERSPECTIVE => Display::fmt(stringify!(NO_PERSPECTIVE), f),
            Self::FLAT => Display::fmt(stringify!(FLAT), f),
            Self::PATCH => Display::fmt(stringify!(PATCH), f),
            Self::CENTROID => Display::fmt(stringify!(CENTROID), f),
            Self::SAMPLE => Display::fmt(stringify!(SAMPLE), f),
            Self::INVARIANT => Display::fmt(stringify!(INVARIANT), f),
            Self::RESTRICT => Display::fmt(stringify!(RESTRICT), f),
            Self::ALIASED => Display::fmt(stringify!(ALIASED), f),
            Self::VOLATILE => Display::fmt(stringify!(VOLATILE), f),
            Self::CONSTANT => Display::fmt(stringify!(CONSTANT), f),
            Self::COHERENT => Display::fmt(stringify!(COHERENT), f),
            Self::NON_WRITABLE => Display::fmt(stringify!(NON_WRITABLE), f),
            Self::NON_READABLE => Display::fmt(stringify!(NON_READABLE), f),
            Self::UNIFORM => Display::fmt(stringify!(UNIFORM), f),
            Self::UNIFORM_ID => Display::fmt(stringify!(UNIFORM_ID), f),
            Self::SATURATED_CONVERSION => Display::fmt(stringify!(SATURATED_CONVERSION), f),
            Self::STREAM => Display::fmt(stringify!(STREAM), f),
            Self::LOCATION => Display::fmt(stringify!(LOCATION), f),
            Self::COMPONENT => Display::fmt(stringify!(COMPONENT), f),
            Self::INDEX => Display::fmt(stringify!(INDEX), f),
            Self::BINDING => Display::fmt(stringify!(BINDING), f),
            Self::DESCRIPTOR_SET => Display::fmt(stringify!(DESCRIPTOR_SET), f),
            Self::OFFSET => Display::fmt(stringify!(OFFSET), f),
            Self::XFB_BUFFER => Display::fmt(stringify!(XFB_BUFFER), f),
            Self::XFB_STRIDE => Display::fmt(stringify!(XFB_STRIDE), f),
            Self::FUNC_PARAM_ATTR => Display::fmt(stringify!(FUNC_PARAM_ATTR), f),
            Self::FPROUNDING_MODE => Display::fmt(stringify!(FPROUNDING_MODE), f),
            Self::FPFAST_MATH_MODE => Display::fmt(stringify!(FPFAST_MATH_MODE), f),
            Self::LINKAGE_ATTRIBUTES => Display::fmt(stringify!(LINKAGE_ATTRIBUTES), f),
            Self::NO_CONTRACTION => Display::fmt(stringify!(NO_CONTRACTION), f),
            Self::INPUT_ATTACHMENT_INDEX => Display::fmt(stringify!(INPUT_ATTACHMENT_INDEX), f),
            Self::ALIGNMENT => Display::fmt(stringify!(ALIGNMENT), f),
            Self::MAX_BYTE_OFFSET => Display::fmt(stringify!(MAX_BYTE_OFFSET), f),
            Self::ALIGNMENT_ID => Display::fmt(stringify!(ALIGNMENT_ID), f),
            Self::MAX_BYTE_OFFSET_ID => Display::fmt(stringify!(MAX_BYTE_OFFSET_ID), f),
            Self::SATURATED_TO_LARGEST_FLOAT_8_NORMAL_CONVERSION_EXT => Display::fmt(
                stringify!(SATURATED_TO_LARGEST_FLOAT_8_NORMAL_CONVERSION_EXT),
                f,
            ),
            Self::NO_SIGNED_WRAP => Display::fmt(stringify!(NO_SIGNED_WRAP), f),
            Self::NO_UNSIGNED_WRAP => Display::fmt(stringify!(NO_UNSIGNED_WRAP), f),
            Self::WEIGHT_TEXTURE_QCOM => Display::fmt(stringify!(WEIGHT_TEXTURE_QCOM), f),
            Self::BLOCK_MATCH_TEXTURE_QCOM => Display::fmt(stringify!(BLOCK_MATCH_TEXTURE_QCOM), f),
            Self::BLOCK_MATCH_SAMPLER_QCOM => Display::fmt(stringify!(BLOCK_MATCH_SAMPLER_QCOM), f),
            Self::EXPLICIT_INTERP_AMD => Display::fmt(stringify!(EXPLICIT_INTERP_AMD), f),
            Self::NODE_SHARES_PAYLOAD_LIMITS_WITH_AMDX => {
                Display::fmt(stringify!(NODE_SHARES_PAYLOAD_LIMITS_WITH_AMDX), f)
            }
            Self::NODE_MAX_PAYLOADS_AMDX => Display::fmt(stringify!(NODE_MAX_PAYLOADS_AMDX), f),
            Self::TRACK_FINISH_WRITING_AMDX => {
                Display::fmt(stringify!(TRACK_FINISH_WRITING_AMDX), f)
            }
            Self::PAYLOAD_NODE_NAME_AMDX => Display::fmt(stringify!(PAYLOAD_NODE_NAME_AMDX), f),
            Self::PAYLOAD_NODE_BASE_INDEX_AMDX => {
                Display::fmt(stringify!(PAYLOAD_NODE_BASE_INDEX_AMDX), f)
            }
            Self::PAYLOAD_NODE_SPARSE_ARRAY_AMDX => {
                Display::fmt(stringify!(PAYLOAD_NODE_SPARSE_ARRAY_AMDX), f)
            }
            Self::PAYLOAD_NODE_ARRAY_SIZE_AMDX => {
                Display::fmt(stringify!(PAYLOAD_NODE_ARRAY_SIZE_AMDX), f)
            }
            Self::PAYLOAD_DISPATCH_INDIRECT_AMDX => {
                Display::fmt(stringify!(PAYLOAD_DISPATCH_INDIRECT_AMDX), f)
            }
            Self::ARRAY_STRIDE_ID_EXT => Display::fmt(stringify!(ARRAY_STRIDE_ID_EXT), f),
            Self::OFFSET_ID_EXT => Display::fmt(stringify!(OFFSET_ID_EXT), f),
            Self::UTFENCODED_KHR => Display::fmt(stringify!(UTFENCODED_KHR), f),
            Self::OVERRIDE_COVERAGE_NV => Display::fmt(stringify!(OVERRIDE_COVERAGE_NV), f),
            Self::PASSTHROUGH_NV => Display::fmt(stringify!(PASSTHROUGH_NV), f),
            Self::VIEWPORT_RELATIVE_NV => Display::fmt(stringify!(VIEWPORT_RELATIVE_NV), f),
            Self::SECONDARY_VIEWPORT_RELATIVE_NV => {
                Display::fmt(stringify!(SECONDARY_VIEWPORT_RELATIVE_NV), f)
            }
            Self::PER_PRIMITIVE_EXT => Display::fmt(stringify!(PER_PRIMITIVE_EXT), f),
            Self::PER_VIEW_NV => Display::fmt(stringify!(PER_VIEW_NV), f),
            Self::PER_TASK_NV => Display::fmt(stringify!(PER_TASK_NV), f),
            Self::PER_VERTEX_KHR => Display::fmt(stringify!(PER_VERTEX_KHR), f),
            Self::NON_UNIFORM => Display::fmt(stringify!(NON_UNIFORM), f),
            Self::RESTRICT_POINTER => Display::fmt(stringify!(RESTRICT_POINTER), f),
            Self::ALIASED_POINTER => Display::fmt(stringify!(ALIASED_POINTER), f),
            Self::MEMBER_OFFSET_NV => Display::fmt(stringify!(MEMBER_OFFSET_NV), f),
            Self::HIT_OBJECT_SHADER_RECORD_BUFFER_NV => {
                Display::fmt(stringify!(HIT_OBJECT_SHADER_RECORD_BUFFER_NV), f)
            }
            Self::HIT_OBJECT_SHADER_RECORD_BUFFER_EXT => {
                Display::fmt(stringify!(HIT_OBJECT_SHADER_RECORD_BUFFER_EXT), f)
            }
            Self::BANK_NV => Display::fmt(stringify!(BANK_NV), f),
            Self::BINDLESS_SAMPLER_NV => Display::fmt(stringify!(BINDLESS_SAMPLER_NV), f),
            Self::BINDLESS_IMAGE_NV => Display::fmt(stringify!(BINDLESS_IMAGE_NV), f),
            Self::BOUND_SAMPLER_NV => Display::fmt(stringify!(BOUND_SAMPLER_NV), f),
            Self::BOUND_IMAGE_NV => Display::fmt(stringify!(BOUND_IMAGE_NV), f),
            Self::SIMTCALL_INTEL => Display::fmt(stringify!(SIMTCALL_INTEL), f),
            Self::REFERENCED_INDIRECTLY_INTEL => {
                Display::fmt(stringify!(REFERENCED_INDIRECTLY_INTEL), f)
            }
            Self::CLOBBER_INTEL => Display::fmt(stringify!(CLOBBER_INTEL), f),
            Self::SIDE_EFFECTS_INTEL => Display::fmt(stringify!(SIDE_EFFECTS_INTEL), f),
            Self::VECTOR_COMPUTE_VARIABLE_INTEL => {
                Display::fmt(stringify!(VECTOR_COMPUTE_VARIABLE_INTEL), f)
            }
            Self::FUNC_PARAM_IOKIND_INTEL => Display::fmt(stringify!(FUNC_PARAM_IOKIND_INTEL), f),
            Self::VECTOR_COMPUTE_FUNCTION_INTEL => {
                Display::fmt(stringify!(VECTOR_COMPUTE_FUNCTION_INTEL), f)
            }
            Self::STACK_CALL_INTEL => Display::fmt(stringify!(STACK_CALL_INTEL), f),
            Self::GLOBAL_VARIABLE_OFFSET_INTEL => {
                Display::fmt(stringify!(GLOBAL_VARIABLE_OFFSET_INTEL), f)
            }
            Self::COUNTER_BUFFER => Display::fmt(stringify!(COUNTER_BUFFER), f),
            Self::USER_SEMANTIC => Display::fmt(stringify!(USER_SEMANTIC), f),
            Self::USER_TYPE_GOOGLE => Display::fmt(stringify!(USER_TYPE_GOOGLE), f),
            Self::FUNCTION_ROUNDING_MODE_INTEL => {
                Display::fmt(stringify!(FUNCTION_ROUNDING_MODE_INTEL), f)
            }
            Self::FUNCTION_DENORM_MODE_INTEL => {
                Display::fmt(stringify!(FUNCTION_DENORM_MODE_INTEL), f)
            }
            Self::REGISTER_ALTERA => Display::fmt(stringify!(REGISTER_ALTERA), f),
            Self::MEMORY_ALTERA => Display::fmt(stringify!(MEMORY_ALTERA), f),
            Self::NUMBANKS_ALTERA => Display::fmt(stringify!(NUMBANKS_ALTERA), f),
            Self::BANKWIDTH_ALTERA => Display::fmt(stringify!(BANKWIDTH_ALTERA), f),
            Self::MAX_PRIVATE_COPIES_ALTERA => {
                Display::fmt(stringify!(MAX_PRIVATE_COPIES_ALTERA), f)
            }
            Self::SINGLEPUMP_ALTERA => Display::fmt(stringify!(SINGLEPUMP_ALTERA), f),
            Self::DOUBLEPUMP_ALTERA => Display::fmt(stringify!(DOUBLEPUMP_ALTERA), f),
            Self::MAX_REPLICATES_ALTERA => Display::fmt(stringify!(MAX_REPLICATES_ALTERA), f),
            Self::SIMPLE_DUAL_PORT_ALTERA => Display::fmt(stringify!(SIMPLE_DUAL_PORT_ALTERA), f),
            Self::MERGE_ALTERA => Display::fmt(stringify!(MERGE_ALTERA), f),
            Self::BANK_BITS_ALTERA => Display::fmt(stringify!(BANK_BITS_ALTERA), f),
            Self::FORCE_POW_2DEPTH_ALTERA => Display::fmt(stringify!(FORCE_POW_2DEPTH_ALTERA), f),
            Self::STRIDESIZE_ALTERA => Display::fmt(stringify!(STRIDESIZE_ALTERA), f),
            Self::WORDSIZE_ALTERA => Display::fmt(stringify!(WORDSIZE_ALTERA), f),
            Self::TRUE_DUAL_PORT_ALTERA => Display::fmt(stringify!(TRUE_DUAL_PORT_ALTERA), f),
            Self::BURST_COALESCE_ALTERA => Display::fmt(stringify!(BURST_COALESCE_ALTERA), f),
            Self::CACHE_SIZE_ALTERA => Display::fmt(stringify!(CACHE_SIZE_ALTERA), f),
            Self::DONT_STATICALLY_COALESCE_ALTERA => {
                Display::fmt(stringify!(DONT_STATICALLY_COALESCE_ALTERA), f)
            }
            Self::PREFETCH_ALTERA => Display::fmt(stringify!(PREFETCH_ALTERA), f),
            Self::STALL_ENABLE_ALTERA => Display::fmt(stringify!(STALL_ENABLE_ALTERA), f),
            Self::FUSE_LOOPS_IN_FUNCTION_ALTERA => {
                Display::fmt(stringify!(FUSE_LOOPS_IN_FUNCTION_ALTERA), f)
            }
            Self::MATH_OP_DSPMODE_ALTERA => Display::fmt(stringify!(MATH_OP_DSPMODE_ALTERA), f),
            Self::ALIAS_SCOPE_INTEL => Display::fmt(stringify!(ALIAS_SCOPE_INTEL), f),
            Self::NO_ALIAS_INTEL => Display::fmt(stringify!(NO_ALIAS_INTEL), f),
            Self::INITIATION_INTERVAL_ALTERA => {
                Display::fmt(stringify!(INITIATION_INTERVAL_ALTERA), f)
            }
            Self::MAX_CONCURRENCY_ALTERA => Display::fmt(stringify!(MAX_CONCURRENCY_ALTERA), f),
            Self::PIPELINE_ENABLE_ALTERA => Display::fmt(stringify!(PIPELINE_ENABLE_ALTERA), f),
            Self::BUFFER_LOCATION_ALTERA => Display::fmt(stringify!(BUFFER_LOCATION_ALTERA), f),
            Self::IOPIPE_STORAGE_ALTERA => Display::fmt(stringify!(IOPIPE_STORAGE_ALTERA), f),
            Self::FUNCTION_FLOATING_POINT_MODE_INTEL => {
                Display::fmt(stringify!(FUNCTION_FLOATING_POINT_MODE_INTEL), f)
            }
            Self::SINGLE_ELEMENT_VECTOR_INTEL => {
                Display::fmt(stringify!(SINGLE_ELEMENT_VECTOR_INTEL), f)
            }
            Self::VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL => {
                Display::fmt(stringify!(VECTOR_COMPUTE_CALLABLE_FUNCTION_INTEL), f)
            }
            Self::MEDIA_BLOCK_IOINTEL => Display::fmt(stringify!(MEDIA_BLOCK_IOINTEL), f),
            Self::STALL_FREE_ALTERA => Display::fmt(stringify!(STALL_FREE_ALTERA), f),
            Self::FPMAX_ERROR_DECORATION_INTEL => {
                Display::fmt(stringify!(FPMAX_ERROR_DECORATION_INTEL), f)
            }
            Self::LATENCY_CONTROL_LABEL_ALTERA => {
                Display::fmt(stringify!(LATENCY_CONTROL_LABEL_ALTERA), f)
            }
            Self::LATENCY_CONTROL_CONSTRAINT_ALTERA => {
                Display::fmt(stringify!(LATENCY_CONTROL_CONSTRAINT_ALTERA), f)
            }
            Self::CONDUIT_KERNEL_ARGUMENT_ALTERA => {
                Display::fmt(stringify!(CONDUIT_KERNEL_ARGUMENT_ALTERA), f)
            }
            Self::REGISTER_MAP_KERNEL_ARGUMENT_ALTERA => {
                Display::fmt(stringify!(REGISTER_MAP_KERNEL_ARGUMENT_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_ADDRESS_WIDTH_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_ADDRESS_WIDTH_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_DATA_WIDTH_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_DATA_WIDTH_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_LATENCY_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_LATENCY_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_READ_WRITE_MODE_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_READ_WRITE_MODE_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_MAX_BURST_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_MAX_BURST_ALTERA), f)
            }
            Self::MMHOST_INTERFACE_WAIT_REQUEST_ALTERA => {
                Display::fmt(stringify!(MMHOST_INTERFACE_WAIT_REQUEST_ALTERA), f)
            }
            Self::STABLE_KERNEL_ARGUMENT_ALTERA => {
                Display::fmt(stringify!(STABLE_KERNEL_ARGUMENT_ALTERA), f)
            }
            Self::HOST_ACCESS_INTEL => Display::fmt(stringify!(HOST_ACCESS_INTEL), f),
            Self::INIT_MODE_ALTERA => Display::fmt(stringify!(INIT_MODE_ALTERA), f),
            Self::IMPLEMENT_IN_REGISTER_MAP_ALTERA => {
                Display::fmt(stringify!(IMPLEMENT_IN_REGISTER_MAP_ALTERA), f)
            }
            Self::CONDITIONAL_INTEL => Display::fmt(stringify!(CONDITIONAL_INTEL), f),
            Self::CACHE_CONTROL_LOAD_INTEL => Display::fmt(stringify!(CACHE_CONTROL_LOAD_INTEL), f),
            Self::CACHE_CONTROL_STORE_INTEL => {
                Display::fmt(stringify!(CACHE_CONTROL_STORE_INTEL), f)
            }
            x => write!(f, "{}({})", stringify!(Decoration), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuiltIn(pub(crate) u32);
impl BuiltIn {
    pub const POSITION: Self = Self(0u32);
    pub const POINT_SIZE: Self = Self(1u32);
    pub const CLIP_DISTANCE: Self = Self(3u32);
    pub const CULL_DISTANCE: Self = Self(4u32);
    pub const VERTEX_ID: Self = Self(5u32);
    pub const INSTANCE_ID: Self = Self(6u32);
    pub const PRIMITIVE_ID: Self = Self(7u32);
    pub const INVOCATION_ID: Self = Self(8u32);
    pub const LAYER: Self = Self(9u32);
    pub const VIEWPORT_INDEX: Self = Self(10u32);
    pub const TESS_LEVEL_OUTER: Self = Self(11u32);
    pub const TESS_LEVEL_INNER: Self = Self(12u32);
    pub const TESS_COORD: Self = Self(13u32);
    pub const PATCH_VERTICES: Self = Self(14u32);
    pub const FRAG_COORD: Self = Self(15u32);
    pub const POINT_COORD: Self = Self(16u32);
    pub const FRONT_FACING: Self = Self(17u32);
    pub const SAMPLE_ID: Self = Self(18u32);
    pub const SAMPLE_POSITION: Self = Self(19u32);
    pub const SAMPLE_MASK: Self = Self(20u32);
    pub const FRAG_DEPTH: Self = Self(22u32);
    pub const HELPER_INVOCATION: Self = Self(23u32);
    pub const NUM_WORKGROUPS: Self = Self(24u32);
    pub const WORKGROUP_SIZE: Self = Self(25u32);
    pub const WORKGROUP_ID: Self = Self(26u32);
    pub const LOCAL_INVOCATION_ID: Self = Self(27u32);
    pub const GLOBAL_INVOCATION_ID: Self = Self(28u32);
    pub const LOCAL_INVOCATION_INDEX: Self = Self(29u32);
    pub const WORK_DIM: Self = Self(30u32);
    pub const GLOBAL_SIZE: Self = Self(31u32);
    pub const ENQUEUED_WORKGROUP_SIZE: Self = Self(32u32);
    pub const GLOBAL_OFFSET: Self = Self(33u32);
    pub const GLOBAL_LINEAR_ID: Self = Self(34u32);
    pub const SUBGROUP_SIZE: Self = Self(36u32);
    pub const SUBGROUP_MAX_SIZE: Self = Self(37u32);
    pub const NUM_SUBGROUPS: Self = Self(38u32);
    pub const NUM_ENQUEUED_SUBGROUPS: Self = Self(39u32);
    pub const SUBGROUP_ID: Self = Self(40u32);
    pub const SUBGROUP_LOCAL_INVOCATION_ID: Self = Self(41u32);
    pub const VERTEX_INDEX: Self = Self(42u32);
    pub const INSTANCE_INDEX: Self = Self(43u32);
    pub const CORE_IDARM: Self = Self(4160u32);
    pub const CORE_COUNT_ARM: Self = Self(4161u32);
    pub const CORE_MAX_IDARM: Self = Self(4162u32);
    pub const WARP_IDARM: Self = Self(4163u32);
    pub const WARP_MAX_IDARM: Self = Self(4164u32);
    pub const SUBGROUP_EQ_MASK: Self = Self(4416u32);
    pub const SUBGROUP_GE_MASK: Self = Self(4417u32);
    pub const SUBGROUP_GT_MASK: Self = Self(4418u32);
    pub const SUBGROUP_LE_MASK: Self = Self(4419u32);
    pub const SUBGROUP_LT_MASK: Self = Self(4420u32);
    pub const BASE_VERTEX: Self = Self(4424u32);
    pub const BASE_INSTANCE: Self = Self(4425u32);
    pub const DRAW_INDEX: Self = Self(4426u32);
    pub const PRIMITIVE_SHADING_RATE_KHR: Self = Self(4432u32);
    pub const DEVICE_INDEX: Self = Self(4438u32);
    pub const VIEW_INDEX: Self = Self(4440u32);
    pub const SHADING_RATE_KHR: Self = Self(4444u32);
    pub const TILE_OFFSET_QCOM: Self = Self(4492u32);
    pub const TILE_DIMENSION_QCOM: Self = Self(4493u32);
    pub const TILE_APRON_SIZE_QCOM: Self = Self(4494u32);
    pub const BARY_COORD_NO_PERSP_AMD: Self = Self(4992u32);
    pub const BARY_COORD_NO_PERSP_CENTROID_AMD: Self = Self(4993u32);
    pub const BARY_COORD_NO_PERSP_SAMPLE_AMD: Self = Self(4994u32);
    pub const BARY_COORD_SMOOTH_AMD: Self = Self(4995u32);
    pub const BARY_COORD_SMOOTH_CENTROID_AMD: Self = Self(4996u32);
    pub const BARY_COORD_SMOOTH_SAMPLE_AMD: Self = Self(4997u32);
    pub const BARY_COORD_PULL_MODEL_AMD: Self = Self(4998u32);
    pub const FRAG_STENCIL_REF_EXT: Self = Self(5014u32);
    pub const REMAINING_RECURSION_LEVELS_AMDX: Self = Self(5021u32);
    pub const SHADER_INDEX_AMDX: Self = Self(5073u32);
    pub const SAMPLER_HEAP_EXT: Self = Self(5122u32);
    pub const RESOURCE_HEAP_EXT: Self = Self(5123u32);
    pub const VIEWPORT_MASK_NV: Self = Self(5253u32);
    pub const SECONDARY_POSITION_NV: Self = Self(5257u32);
    pub const SECONDARY_VIEWPORT_MASK_NV: Self = Self(5258u32);
    pub const POSITION_PER_VIEW_NV: Self = Self(5261u32);
    pub const VIEWPORT_MASK_PER_VIEW_NV: Self = Self(5262u32);
    pub const FULLY_COVERED_EXT: Self = Self(5264u32);
    pub const TASK_COUNT_NV: Self = Self(5274u32);
    pub const PRIMITIVE_COUNT_NV: Self = Self(5275u32);
    pub const PRIMITIVE_INDICES_NV: Self = Self(5276u32);
    pub const CLIP_DISTANCE_PER_VIEW_NV: Self = Self(5277u32);
    pub const CULL_DISTANCE_PER_VIEW_NV: Self = Self(5278u32);
    pub const LAYER_PER_VIEW_NV: Self = Self(5279u32);
    pub const MESH_VIEW_COUNT_NV: Self = Self(5280u32);
    pub const MESH_VIEW_INDICES_NV: Self = Self(5281u32);
    pub const BARY_COORD_KHR: Self = Self(5286u32);
    pub const BARY_COORD_NO_PERSP_KHR: Self = Self(5287u32);
    pub const FRAG_SIZE_EXT: Self = Self(5292u32);
    pub const FRAG_INVOCATION_COUNT_EXT: Self = Self(5293u32);
    pub const PRIMITIVE_POINT_INDICES_EXT: Self = Self(5294u32);
    pub const PRIMITIVE_LINE_INDICES_EXT: Self = Self(5295u32);
    pub const PRIMITIVE_TRIANGLE_INDICES_EXT: Self = Self(5296u32);
    pub const CULL_PRIMITIVE_EXT: Self = Self(5299u32);
    pub const LAUNCH_ID_KHR: Self = Self(5319u32);
    pub const LAUNCH_SIZE_KHR: Self = Self(5320u32);
    pub const WORLD_RAY_ORIGIN_KHR: Self = Self(5321u32);
    pub const WORLD_RAY_DIRECTION_KHR: Self = Self(5322u32);
    pub const OBJECT_RAY_ORIGIN_KHR: Self = Self(5323u32);
    pub const OBJECT_RAY_DIRECTION_KHR: Self = Self(5324u32);
    pub const RAY_TMIN_KHR: Self = Self(5325u32);
    pub const RAY_TMAX_KHR: Self = Self(5326u32);
    pub const INSTANCE_CUSTOM_INDEX_KHR: Self = Self(5327u32);
    pub const OBJECT_TO_WORLD_KHR: Self = Self(5330u32);
    pub const WORLD_TO_OBJECT_KHR: Self = Self(5331u32);
    pub const HIT_TNV: Self = Self(5332u32);
    pub const HIT_KIND_KHR: Self = Self(5333u32);
    pub const CURRENT_RAY_TIME_NV: Self = Self(5334u32);
    pub const HIT_TRIANGLE_VERTEX_POSITIONS_KHR: Self = Self(5335u32);
    pub const HIT_MICRO_TRIANGLE_VERTEX_POSITIONS_NV: Self = Self(5337u32);
    pub const HIT_MICRO_TRIANGLE_VERTEX_BARYCENTRICS_NV: Self = Self(5344u32);
    pub const INCOMING_RAY_FLAGS_KHR: Self = Self(5351u32);
    pub const RAY_GEOMETRY_INDEX_KHR: Self = Self(5352u32);
    pub const HIT_IS_SPHERE_NV: Self = Self(5359u32);
    pub const HIT_IS_LSSNV: Self = Self(5360u32);
    pub const HIT_SPHERE_POSITION_NV: Self = Self(5361u32);
    pub const WARPS_PER_SMNV: Self = Self(5374u32);
    pub const SMCOUNT_NV: Self = Self(5375u32);
    pub const WARP_IDNV: Self = Self(5376u32);
    pub const SMIDNV: Self = Self(5377u32);
    pub const HIT_LSSPOSITIONS_NV: Self = Self(5396u32);
    pub const HIT_KIND_FRONT_FACING_MICRO_TRIANGLE_NV: Self = Self(5405u32);
    pub const HIT_KIND_BACK_FACING_MICRO_TRIANGLE_NV: Self = Self(5406u32);
    pub const HIT_SPHERE_RADIUS_NV: Self = Self(5420u32);
    pub const HIT_LSSRADII_NV: Self = Self(5421u32);
    pub const CLUSTER_IDNV: Self = Self(5436u32);
    pub const CULL_MASK_KHR: Self = Self(6021u32);
}
impl Word for BuiltIn {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for BuiltIn {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::POSITION => Display::fmt(stringify!(POSITION), f),
            Self::POINT_SIZE => Display::fmt(stringify!(POINT_SIZE), f),
            Self::CLIP_DISTANCE => Display::fmt(stringify!(CLIP_DISTANCE), f),
            Self::CULL_DISTANCE => Display::fmt(stringify!(CULL_DISTANCE), f),
            Self::VERTEX_ID => Display::fmt(stringify!(VERTEX_ID), f),
            Self::INSTANCE_ID => Display::fmt(stringify!(INSTANCE_ID), f),
            Self::PRIMITIVE_ID => Display::fmt(stringify!(PRIMITIVE_ID), f),
            Self::INVOCATION_ID => Display::fmt(stringify!(INVOCATION_ID), f),
            Self::LAYER => Display::fmt(stringify!(LAYER), f),
            Self::VIEWPORT_INDEX => Display::fmt(stringify!(VIEWPORT_INDEX), f),
            Self::TESS_LEVEL_OUTER => Display::fmt(stringify!(TESS_LEVEL_OUTER), f),
            Self::TESS_LEVEL_INNER => Display::fmt(stringify!(TESS_LEVEL_INNER), f),
            Self::TESS_COORD => Display::fmt(stringify!(TESS_COORD), f),
            Self::PATCH_VERTICES => Display::fmt(stringify!(PATCH_VERTICES), f),
            Self::FRAG_COORD => Display::fmt(stringify!(FRAG_COORD), f),
            Self::POINT_COORD => Display::fmt(stringify!(POINT_COORD), f),
            Self::FRONT_FACING => Display::fmt(stringify!(FRONT_FACING), f),
            Self::SAMPLE_ID => Display::fmt(stringify!(SAMPLE_ID), f),
            Self::SAMPLE_POSITION => Display::fmt(stringify!(SAMPLE_POSITION), f),
            Self::SAMPLE_MASK => Display::fmt(stringify!(SAMPLE_MASK), f),
            Self::FRAG_DEPTH => Display::fmt(stringify!(FRAG_DEPTH), f),
            Self::HELPER_INVOCATION => Display::fmt(stringify!(HELPER_INVOCATION), f),
            Self::NUM_WORKGROUPS => Display::fmt(stringify!(NUM_WORKGROUPS), f),
            Self::WORKGROUP_SIZE => Display::fmt(stringify!(WORKGROUP_SIZE), f),
            Self::WORKGROUP_ID => Display::fmt(stringify!(WORKGROUP_ID), f),
            Self::LOCAL_INVOCATION_ID => Display::fmt(stringify!(LOCAL_INVOCATION_ID), f),
            Self::GLOBAL_INVOCATION_ID => Display::fmt(stringify!(GLOBAL_INVOCATION_ID), f),
            Self::LOCAL_INVOCATION_INDEX => Display::fmt(stringify!(LOCAL_INVOCATION_INDEX), f),
            Self::WORK_DIM => Display::fmt(stringify!(WORK_DIM), f),
            Self::GLOBAL_SIZE => Display::fmt(stringify!(GLOBAL_SIZE), f),
            Self::ENQUEUED_WORKGROUP_SIZE => Display::fmt(stringify!(ENQUEUED_WORKGROUP_SIZE), f),
            Self::GLOBAL_OFFSET => Display::fmt(stringify!(GLOBAL_OFFSET), f),
            Self::GLOBAL_LINEAR_ID => Display::fmt(stringify!(GLOBAL_LINEAR_ID), f),
            Self::SUBGROUP_SIZE => Display::fmt(stringify!(SUBGROUP_SIZE), f),
            Self::SUBGROUP_MAX_SIZE => Display::fmt(stringify!(SUBGROUP_MAX_SIZE), f),
            Self::NUM_SUBGROUPS => Display::fmt(stringify!(NUM_SUBGROUPS), f),
            Self::NUM_ENQUEUED_SUBGROUPS => Display::fmt(stringify!(NUM_ENQUEUED_SUBGROUPS), f),
            Self::SUBGROUP_ID => Display::fmt(stringify!(SUBGROUP_ID), f),
            Self::SUBGROUP_LOCAL_INVOCATION_ID => {
                Display::fmt(stringify!(SUBGROUP_LOCAL_INVOCATION_ID), f)
            }
            Self::VERTEX_INDEX => Display::fmt(stringify!(VERTEX_INDEX), f),
            Self::INSTANCE_INDEX => Display::fmt(stringify!(INSTANCE_INDEX), f),
            Self::CORE_IDARM => Display::fmt(stringify!(CORE_IDARM), f),
            Self::CORE_COUNT_ARM => Display::fmt(stringify!(CORE_COUNT_ARM), f),
            Self::CORE_MAX_IDARM => Display::fmt(stringify!(CORE_MAX_IDARM), f),
            Self::WARP_IDARM => Display::fmt(stringify!(WARP_IDARM), f),
            Self::WARP_MAX_IDARM => Display::fmt(stringify!(WARP_MAX_IDARM), f),
            Self::SUBGROUP_EQ_MASK => Display::fmt(stringify!(SUBGROUP_EQ_MASK), f),
            Self::SUBGROUP_GE_MASK => Display::fmt(stringify!(SUBGROUP_GE_MASK), f),
            Self::SUBGROUP_GT_MASK => Display::fmt(stringify!(SUBGROUP_GT_MASK), f),
            Self::SUBGROUP_LE_MASK => Display::fmt(stringify!(SUBGROUP_LE_MASK), f),
            Self::SUBGROUP_LT_MASK => Display::fmt(stringify!(SUBGROUP_LT_MASK), f),
            Self::BASE_VERTEX => Display::fmt(stringify!(BASE_VERTEX), f),
            Self::BASE_INSTANCE => Display::fmt(stringify!(BASE_INSTANCE), f),
            Self::DRAW_INDEX => Display::fmt(stringify!(DRAW_INDEX), f),
            Self::PRIMITIVE_SHADING_RATE_KHR => {
                Display::fmt(stringify!(PRIMITIVE_SHADING_RATE_KHR), f)
            }
            Self::DEVICE_INDEX => Display::fmt(stringify!(DEVICE_INDEX), f),
            Self::VIEW_INDEX => Display::fmt(stringify!(VIEW_INDEX), f),
            Self::SHADING_RATE_KHR => Display::fmt(stringify!(SHADING_RATE_KHR), f),
            Self::TILE_OFFSET_QCOM => Display::fmt(stringify!(TILE_OFFSET_QCOM), f),
            Self::TILE_DIMENSION_QCOM => Display::fmt(stringify!(TILE_DIMENSION_QCOM), f),
            Self::TILE_APRON_SIZE_QCOM => Display::fmt(stringify!(TILE_APRON_SIZE_QCOM), f),
            Self::BARY_COORD_NO_PERSP_AMD => Display::fmt(stringify!(BARY_COORD_NO_PERSP_AMD), f),
            Self::BARY_COORD_NO_PERSP_CENTROID_AMD => {
                Display::fmt(stringify!(BARY_COORD_NO_PERSP_CENTROID_AMD), f)
            }
            Self::BARY_COORD_NO_PERSP_SAMPLE_AMD => {
                Display::fmt(stringify!(BARY_COORD_NO_PERSP_SAMPLE_AMD), f)
            }
            Self::BARY_COORD_SMOOTH_AMD => Display::fmt(stringify!(BARY_COORD_SMOOTH_AMD), f),
            Self::BARY_COORD_SMOOTH_CENTROID_AMD => {
                Display::fmt(stringify!(BARY_COORD_SMOOTH_CENTROID_AMD), f)
            }
            Self::BARY_COORD_SMOOTH_SAMPLE_AMD => {
                Display::fmt(stringify!(BARY_COORD_SMOOTH_SAMPLE_AMD), f)
            }
            Self::BARY_COORD_PULL_MODEL_AMD => {
                Display::fmt(stringify!(BARY_COORD_PULL_MODEL_AMD), f)
            }
            Self::FRAG_STENCIL_REF_EXT => Display::fmt(stringify!(FRAG_STENCIL_REF_EXT), f),
            Self::REMAINING_RECURSION_LEVELS_AMDX => {
                Display::fmt(stringify!(REMAINING_RECURSION_LEVELS_AMDX), f)
            }
            Self::SHADER_INDEX_AMDX => Display::fmt(stringify!(SHADER_INDEX_AMDX), f),
            Self::SAMPLER_HEAP_EXT => Display::fmt(stringify!(SAMPLER_HEAP_EXT), f),
            Self::RESOURCE_HEAP_EXT => Display::fmt(stringify!(RESOURCE_HEAP_EXT), f),
            Self::VIEWPORT_MASK_NV => Display::fmt(stringify!(VIEWPORT_MASK_NV), f),
            Self::SECONDARY_POSITION_NV => Display::fmt(stringify!(SECONDARY_POSITION_NV), f),
            Self::SECONDARY_VIEWPORT_MASK_NV => {
                Display::fmt(stringify!(SECONDARY_VIEWPORT_MASK_NV), f)
            }
            Self::POSITION_PER_VIEW_NV => Display::fmt(stringify!(POSITION_PER_VIEW_NV), f),
            Self::VIEWPORT_MASK_PER_VIEW_NV => {
                Display::fmt(stringify!(VIEWPORT_MASK_PER_VIEW_NV), f)
            }
            Self::FULLY_COVERED_EXT => Display::fmt(stringify!(FULLY_COVERED_EXT), f),
            Self::TASK_COUNT_NV => Display::fmt(stringify!(TASK_COUNT_NV), f),
            Self::PRIMITIVE_COUNT_NV => Display::fmt(stringify!(PRIMITIVE_COUNT_NV), f),
            Self::PRIMITIVE_INDICES_NV => Display::fmt(stringify!(PRIMITIVE_INDICES_NV), f),
            Self::CLIP_DISTANCE_PER_VIEW_NV => {
                Display::fmt(stringify!(CLIP_DISTANCE_PER_VIEW_NV), f)
            }
            Self::CULL_DISTANCE_PER_VIEW_NV => {
                Display::fmt(stringify!(CULL_DISTANCE_PER_VIEW_NV), f)
            }
            Self::LAYER_PER_VIEW_NV => Display::fmt(stringify!(LAYER_PER_VIEW_NV), f),
            Self::MESH_VIEW_COUNT_NV => Display::fmt(stringify!(MESH_VIEW_COUNT_NV), f),
            Self::MESH_VIEW_INDICES_NV => Display::fmt(stringify!(MESH_VIEW_INDICES_NV), f),
            Self::BARY_COORD_KHR => Display::fmt(stringify!(BARY_COORD_KHR), f),
            Self::BARY_COORD_NO_PERSP_KHR => Display::fmt(stringify!(BARY_COORD_NO_PERSP_KHR), f),
            Self::FRAG_SIZE_EXT => Display::fmt(stringify!(FRAG_SIZE_EXT), f),
            Self::FRAG_INVOCATION_COUNT_EXT => {
                Display::fmt(stringify!(FRAG_INVOCATION_COUNT_EXT), f)
            }
            Self::PRIMITIVE_POINT_INDICES_EXT => {
                Display::fmt(stringify!(PRIMITIVE_POINT_INDICES_EXT), f)
            }
            Self::PRIMITIVE_LINE_INDICES_EXT => {
                Display::fmt(stringify!(PRIMITIVE_LINE_INDICES_EXT), f)
            }
            Self::PRIMITIVE_TRIANGLE_INDICES_EXT => {
                Display::fmt(stringify!(PRIMITIVE_TRIANGLE_INDICES_EXT), f)
            }
            Self::CULL_PRIMITIVE_EXT => Display::fmt(stringify!(CULL_PRIMITIVE_EXT), f),
            Self::LAUNCH_ID_KHR => Display::fmt(stringify!(LAUNCH_ID_KHR), f),
            Self::LAUNCH_SIZE_KHR => Display::fmt(stringify!(LAUNCH_SIZE_KHR), f),
            Self::WORLD_RAY_ORIGIN_KHR => Display::fmt(stringify!(WORLD_RAY_ORIGIN_KHR), f),
            Self::WORLD_RAY_DIRECTION_KHR => Display::fmt(stringify!(WORLD_RAY_DIRECTION_KHR), f),
            Self::OBJECT_RAY_ORIGIN_KHR => Display::fmt(stringify!(OBJECT_RAY_ORIGIN_KHR), f),
            Self::OBJECT_RAY_DIRECTION_KHR => Display::fmt(stringify!(OBJECT_RAY_DIRECTION_KHR), f),
            Self::RAY_TMIN_KHR => Display::fmt(stringify!(RAY_TMIN_KHR), f),
            Self::RAY_TMAX_KHR => Display::fmt(stringify!(RAY_TMAX_KHR), f),
            Self::INSTANCE_CUSTOM_INDEX_KHR => {
                Display::fmt(stringify!(INSTANCE_CUSTOM_INDEX_KHR), f)
            }
            Self::OBJECT_TO_WORLD_KHR => Display::fmt(stringify!(OBJECT_TO_WORLD_KHR), f),
            Self::WORLD_TO_OBJECT_KHR => Display::fmt(stringify!(WORLD_TO_OBJECT_KHR), f),
            Self::HIT_TNV => Display::fmt(stringify!(HIT_TNV), f),
            Self::HIT_KIND_KHR => Display::fmt(stringify!(HIT_KIND_KHR), f),
            Self::CURRENT_RAY_TIME_NV => Display::fmt(stringify!(CURRENT_RAY_TIME_NV), f),
            Self::HIT_TRIANGLE_VERTEX_POSITIONS_KHR => {
                Display::fmt(stringify!(HIT_TRIANGLE_VERTEX_POSITIONS_KHR), f)
            }
            Self::HIT_MICRO_TRIANGLE_VERTEX_POSITIONS_NV => {
                Display::fmt(stringify!(HIT_MICRO_TRIANGLE_VERTEX_POSITIONS_NV), f)
            }
            Self::HIT_MICRO_TRIANGLE_VERTEX_BARYCENTRICS_NV => {
                Display::fmt(stringify!(HIT_MICRO_TRIANGLE_VERTEX_BARYCENTRICS_NV), f)
            }
            Self::INCOMING_RAY_FLAGS_KHR => Display::fmt(stringify!(INCOMING_RAY_FLAGS_KHR), f),
            Self::RAY_GEOMETRY_INDEX_KHR => Display::fmt(stringify!(RAY_GEOMETRY_INDEX_KHR), f),
            Self::HIT_IS_SPHERE_NV => Display::fmt(stringify!(HIT_IS_SPHERE_NV), f),
            Self::HIT_IS_LSSNV => Display::fmt(stringify!(HIT_IS_LSSNV), f),
            Self::HIT_SPHERE_POSITION_NV => Display::fmt(stringify!(HIT_SPHERE_POSITION_NV), f),
            Self::WARPS_PER_SMNV => Display::fmt(stringify!(WARPS_PER_SMNV), f),
            Self::SMCOUNT_NV => Display::fmt(stringify!(SMCOUNT_NV), f),
            Self::WARP_IDNV => Display::fmt(stringify!(WARP_IDNV), f),
            Self::SMIDNV => Display::fmt(stringify!(SMIDNV), f),
            Self::HIT_LSSPOSITIONS_NV => Display::fmt(stringify!(HIT_LSSPOSITIONS_NV), f),
            Self::HIT_KIND_FRONT_FACING_MICRO_TRIANGLE_NV => {
                Display::fmt(stringify!(HIT_KIND_FRONT_FACING_MICRO_TRIANGLE_NV), f)
            }
            Self::HIT_KIND_BACK_FACING_MICRO_TRIANGLE_NV => {
                Display::fmt(stringify!(HIT_KIND_BACK_FACING_MICRO_TRIANGLE_NV), f)
            }
            Self::HIT_SPHERE_RADIUS_NV => Display::fmt(stringify!(HIT_SPHERE_RADIUS_NV), f),
            Self::HIT_LSSRADII_NV => Display::fmt(stringify!(HIT_LSSRADII_NV), f),
            Self::CLUSTER_IDNV => Display::fmt(stringify!(CLUSTER_IDNV), f),
            Self::CULL_MASK_KHR => Display::fmt(stringify!(CULL_MASK_KHR), f),
            x => write!(f, "{}({})", stringify!(BuiltIn), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scope(pub(crate) u32);
impl Scope {
    pub const CROSS_DEVICE: Self = Self(0u32);
    pub const DEVICE: Self = Self(1u32);
    pub const WORKGROUP: Self = Self(2u32);
    pub const SUBGROUP: Self = Self(3u32);
    pub const INVOCATION: Self = Self(4u32);
    pub const QUEUE_FAMILY: Self = Self(5u32);
    pub const SHADER_CALL_KHR: Self = Self(6u32);
}
impl Word for Scope {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for Scope {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CROSS_DEVICE => Display::fmt(stringify!(CROSS_DEVICE), f),
            Self::DEVICE => Display::fmt(stringify!(DEVICE), f),
            Self::WORKGROUP => Display::fmt(stringify!(WORKGROUP), f),
            Self::SUBGROUP => Display::fmt(stringify!(SUBGROUP), f),
            Self::INVOCATION => Display::fmt(stringify!(INVOCATION), f),
            Self::QUEUE_FAMILY => Display::fmt(stringify!(QUEUE_FAMILY), f),
            Self::SHADER_CALL_KHR => Display::fmt(stringify!(SHADER_CALL_KHR), f),
            x => write!(f, "{}({})", stringify!(Scope), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupOperation(pub(crate) u32);
impl GroupOperation {
    pub const REDUCE: Self = Self(0u32);
    pub const INCLUSIVE_SCAN: Self = Self(1u32);
    pub const EXCLUSIVE_SCAN: Self = Self(2u32);
    pub const CLUSTERED_REDUCE: Self = Self(3u32);
    pub const PARTITIONED_REDUCE_EXT: Self = Self(6u32);
    pub const PARTITIONED_INCLUSIVE_SCAN_EXT: Self = Self(7u32);
    pub const PARTITIONED_EXCLUSIVE_SCAN_EXT: Self = Self(8u32);
}
impl Word for GroupOperation {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for GroupOperation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::REDUCE => Display::fmt(stringify!(REDUCE), f),
            Self::INCLUSIVE_SCAN => Display::fmt(stringify!(INCLUSIVE_SCAN), f),
            Self::EXCLUSIVE_SCAN => Display::fmt(stringify!(EXCLUSIVE_SCAN), f),
            Self::CLUSTERED_REDUCE => Display::fmt(stringify!(CLUSTERED_REDUCE), f),
            Self::PARTITIONED_REDUCE_EXT => Display::fmt(stringify!(PARTITIONED_REDUCE_EXT), f),
            Self::PARTITIONED_INCLUSIVE_SCAN_EXT => {
                Display::fmt(stringify!(PARTITIONED_INCLUSIVE_SCAN_EXT), f)
            }
            Self::PARTITIONED_EXCLUSIVE_SCAN_EXT => {
                Display::fmt(stringify!(PARTITIONED_EXCLUSIVE_SCAN_EXT), f)
            }
            x => write!(f, "{}({})", stringify!(GroupOperation), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelEnqueueFlags(pub(crate) u32);
impl KernelEnqueueFlags {
    pub const NO_WAIT: Self = Self(0u32);
    pub const WAIT_KERNEL: Self = Self(1u32);
    pub const WAIT_WORK_GROUP: Self = Self(2u32);
}
impl Word for KernelEnqueueFlags {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for KernelEnqueueFlags {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NO_WAIT => Display::fmt(stringify!(NO_WAIT), f),
            Self::WAIT_KERNEL => Display::fmt(stringify!(WAIT_KERNEL), f),
            Self::WAIT_WORK_GROUP => Display::fmt(stringify!(WAIT_WORK_GROUP), f),
            x => write!(f, "{}({})", stringify!(KernelEnqueueFlags), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Capability(pub(crate) u32);
impl Capability {
    pub const MATRIX: Self = Self(0u32);
    pub const SHADER: Self = Self(1u32);
    pub const GEOMETRY: Self = Self(2u32);
    pub const TESSELLATION: Self = Self(3u32);
    pub const ADDRESSES: Self = Self(4u32);
    pub const LINKAGE: Self = Self(5u32);
    pub const KERNEL: Self = Self(6u32);
    pub const VECTOR_16: Self = Self(7u32);
    pub const FLOAT_16_BUFFER: Self = Self(8u32);
    pub const FLOAT_16: Self = Self(9u32);
    pub const FLOAT_64: Self = Self(10u32);
    pub const INT_64: Self = Self(11u32);
    pub const INT_64_ATOMICS: Self = Self(12u32);
    pub const IMAGE_BASIC: Self = Self(13u32);
    pub const IMAGE_READ_WRITE: Self = Self(14u32);
    pub const IMAGE_MIPMAP: Self = Self(15u32);
    pub const PIPES: Self = Self(17u32);
    pub const GROUPS: Self = Self(18u32);
    pub const DEVICE_ENQUEUE: Self = Self(19u32);
    pub const LITERAL_SAMPLER: Self = Self(20u32);
    pub const ATOMIC_STORAGE: Self = Self(21u32);
    pub const INT_16: Self = Self(22u32);
    pub const TESSELLATION_POINT_SIZE: Self = Self(23u32);
    pub const GEOMETRY_POINT_SIZE: Self = Self(24u32);
    pub const IMAGE_GATHER_EXTENDED: Self = Self(25u32);
    pub const STORAGE_IMAGE_MULTISAMPLE: Self = Self(27u32);
    pub const UNIFORM_BUFFER_ARRAY_DYNAMIC_INDEXING: Self = Self(28u32);
    pub const SAMPLED_IMAGE_ARRAY_DYNAMIC_INDEXING: Self = Self(29u32);
    pub const STORAGE_BUFFER_ARRAY_DYNAMIC_INDEXING: Self = Self(30u32);
    pub const STORAGE_IMAGE_ARRAY_DYNAMIC_INDEXING: Self = Self(31u32);
    pub const CLIP_DISTANCE: Self = Self(32u32);
    pub const CULL_DISTANCE: Self = Self(33u32);
    pub const IMAGE_CUBE_ARRAY: Self = Self(34u32);
    pub const SAMPLE_RATE_SHADING: Self = Self(35u32);
    pub const IMAGE_RECT: Self = Self(36u32);
    pub const SAMPLED_RECT: Self = Self(37u32);
    pub const GENERIC_POINTER: Self = Self(38u32);
    pub const INT_8: Self = Self(39u32);
    pub const INPUT_ATTACHMENT: Self = Self(40u32);
    pub const SPARSE_RESIDENCY: Self = Self(41u32);
    pub const MIN_LOD: Self = Self(42u32);
    pub const SAMPLED_1D: Self = Self(43u32);
    pub const IMAGE_1D: Self = Self(44u32);
    pub const SAMPLED_CUBE_ARRAY: Self = Self(45u32);
    pub const SAMPLED_BUFFER: Self = Self(46u32);
    pub const IMAGE_BUFFER: Self = Self(47u32);
    pub const IMAGE_MSARRAY: Self = Self(48u32);
    pub const STORAGE_IMAGE_EXTENDED_FORMATS: Self = Self(49u32);
    pub const IMAGE_QUERY: Self = Self(50u32);
    pub const DERIVATIVE_CONTROL: Self = Self(51u32);
    pub const INTERPOLATION_FUNCTION: Self = Self(52u32);
    pub const TRANSFORM_FEEDBACK: Self = Self(53u32);
    pub const GEOMETRY_STREAMS: Self = Self(54u32);
    pub const STORAGE_IMAGE_READ_WITHOUT_FORMAT: Self = Self(55u32);
    pub const STORAGE_IMAGE_WRITE_WITHOUT_FORMAT: Self = Self(56u32);
    pub const MULTI_VIEWPORT: Self = Self(57u32);
    pub const SUBGROUP_DISPATCH: Self = Self(58u32);
    pub const NAMED_BARRIER: Self = Self(59u32);
    pub const PIPE_STORAGE: Self = Self(60u32);
    pub const GROUP_NON_UNIFORM: Self = Self(61u32);
    pub const GROUP_NON_UNIFORM_VOTE: Self = Self(62u32);
    pub const GROUP_NON_UNIFORM_ARITHMETIC: Self = Self(63u32);
    pub const GROUP_NON_UNIFORM_BALLOT: Self = Self(64u32);
    pub const GROUP_NON_UNIFORM_SHUFFLE: Self = Self(65u32);
    pub const GROUP_NON_UNIFORM_SHUFFLE_RELATIVE: Self = Self(66u32);
    pub const GROUP_NON_UNIFORM_CLUSTERED: Self = Self(67u32);
    pub const GROUP_NON_UNIFORM_QUAD: Self = Self(68u32);
    pub const SHADER_LAYER: Self = Self(69u32);
    pub const SHADER_VIEWPORT_INDEX: Self = Self(70u32);
    pub const UNIFORM_DECORATION: Self = Self(71u32);
    pub const CORE_BUILTINS_ARM: Self = Self(4165u32);
    pub const TILE_IMAGE_COLOR_READ_ACCESS_EXT: Self = Self(4166u32);
    pub const TILE_IMAGE_DEPTH_READ_ACCESS_EXT: Self = Self(4167u32);
    pub const TILE_IMAGE_STENCIL_READ_ACCESS_EXT: Self = Self(4168u32);
    pub const TENSORS_ARM: Self = Self(4174u32);
    pub const STORAGE_TENSOR_ARRAY_DYNAMIC_INDEXING_ARM: Self = Self(4175u32);
    pub const STORAGE_TENSOR_ARRAY_NON_UNIFORM_INDEXING_ARM: Self = Self(4176u32);
    pub const GRAPH_ARM: Self = Self(4191u32);
    pub const COOPERATIVE_MATRIX_LAYOUTS_ARM: Self = Self(4201u32);
    pub const FLOAT_8_EXT: Self = Self(4212u32);
    pub const FLOAT_8_COOPERATIVE_MATRIX_EXT: Self = Self(4213u32);
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(4422u32);
    pub const SUBGROUP_BALLOT_KHR: Self = Self(4423u32);
    pub const DRAW_PARAMETERS: Self = Self(4427u32);
    pub const WORKGROUP_MEMORY_EXPLICIT_LAYOUT_KHR: Self = Self(4428u32);
    pub const WORKGROUP_MEMORY_EXPLICIT_LAYOUT_8_BIT_ACCESS_KHR: Self = Self(4429u32);
    pub const WORKGROUP_MEMORY_EXPLICIT_LAYOUT_16_BIT_ACCESS_KHR: Self = Self(4430u32);
    pub const SUBGROUP_VOTE_KHR: Self = Self(4431u32);
    pub const STORAGE_BUFFER_16_BIT_ACCESS: Self = Self(4433u32);
    pub const UNIFORM_AND_STORAGE_BUFFER_16_BIT_ACCESS: Self = Self(4434u32);
    pub const STORAGE_PUSH_CONSTANT_16: Self = Self(4435u32);
    pub const STORAGE_INPUT_OUTPUT_16: Self = Self(4436u32);
    pub const DEVICE_GROUP: Self = Self(4437u32);
    pub const MULTI_VIEW: Self = Self(4439u32);
    pub const VARIABLE_POINTERS_STORAGE_BUFFER: Self = Self(4441u32);
    pub const VARIABLE_POINTERS: Self = Self(4442u32);
    pub const ATOMIC_STORAGE_OPS: Self = Self(4445u32);
    pub const SAMPLE_MASK_POST_DEPTH_COVERAGE: Self = Self(4447u32);
    pub const STORAGE_BUFFER_8_BIT_ACCESS: Self = Self(4448u32);
    pub const UNIFORM_AND_STORAGE_BUFFER_8_BIT_ACCESS: Self = Self(4449u32);
    pub const STORAGE_PUSH_CONSTANT_8: Self = Self(4450u32);
    pub const DENORM_PRESERVE: Self = Self(4464u32);
    pub const DENORM_FLUSH_TO_ZERO: Self = Self(4465u32);
    pub const SIGNED_ZERO_INF_NAN_PRESERVE: Self = Self(4466u32);
    pub const ROUNDING_MODE_RTE: Self = Self(4467u32);
    pub const ROUNDING_MODE_RTZ: Self = Self(4468u32);
    pub const RAY_QUERY_PROVISIONAL_KHR: Self = Self(4471u32);
    pub const RAY_QUERY_KHR: Self = Self(4472u32);
    pub const UNTYPED_POINTERS_KHR: Self = Self(4473u32);
    pub const RAY_TRAVERSAL_PRIMITIVE_CULLING_KHR: Self = Self(4478u32);
    pub const RAY_TRACING_KHR: Self = Self(4479u32);
    pub const TEXTURE_SAMPLE_WEIGHTED_QCOM: Self = Self(4484u32);
    pub const TEXTURE_BOX_FILTER_QCOM: Self = Self(4485u32);
    pub const TEXTURE_BLOCK_MATCH_QCOM: Self = Self(4486u32);
    pub const TILE_SHADING_QCOM: Self = Self(4495u32);
    pub const COOPERATIVE_MATRIX_CONVERSION_QCOM: Self = Self(4496u32);
    pub const TEXTURE_BLOCK_MATCH_2_QCOM: Self = Self(4498u32);
    pub const FLOAT_16_IMAGE_AMD: Self = Self(5008u32);
    pub const IMAGE_GATHER_BIAS_LOD_AMD: Self = Self(5009u32);
    pub const FRAGMENT_MASK_AMD: Self = Self(5010u32);
    pub const STENCIL_EXPORT_EXT: Self = Self(5013u32);
    pub const IMAGE_READ_WRITE_LOD_AMD: Self = Self(5015u32);
    pub const INT_64_IMAGE_EXT: Self = Self(5016u32);
    pub const SHADER_CLOCK_KHR: Self = Self(5055u32);
    pub const SHADER_ENQUEUE_AMDX: Self = Self(5067u32);
    pub const QUAD_CONTROL_KHR: Self = Self(5087u32);
    pub const INT_4_TYPE_INTEL: Self = Self(5112u32);
    pub const INT_4_COOPERATIVE_MATRIX_INTEL: Self = Self(5114u32);
    pub const BFLOAT_16_TYPE_KHR: Self = Self(5116u32);
    pub const BFLOAT_16DOT_PRODUCT_KHR: Self = Self(5117u32);
    pub const BFLOAT_16_COOPERATIVE_MATRIX_KHR: Self = Self(5118u32);
    pub const ABORT_KHR: Self = Self(5120u32);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(5128u32);
    pub const CONSTANT_DATA_KHR: Self = Self(5146u32);
    pub const POISON_FREEZE_KHR: Self = Self(5156u32);
    pub const SAMPLE_MASK_OVERRIDE_COVERAGE_NV: Self = Self(5249u32);
    pub const GEOMETRY_SHADER_PASSTHROUGH_NV: Self = Self(5251u32);
    pub const SHADER_VIEWPORT_INDEX_LAYER_EXT: Self = Self(5254u32);
    pub const SHADER_VIEWPORT_MASK_NV: Self = Self(5255u32);
    pub const SHADER_STEREO_VIEW_NV: Self = Self(5259u32);
    pub const PER_VIEW_ATTRIBUTES_NV: Self = Self(5260u32);
    pub const FRAGMENT_FULLY_COVERED_EXT: Self = Self(5265u32);
    pub const MESH_SHADING_NV: Self = Self(5266u32);
    pub const IMAGE_FOOTPRINT_NV: Self = Self(5282u32);
    pub const MESH_SHADING_EXT: Self = Self(5283u32);
    pub const FRAGMENT_BARYCENTRIC_KHR: Self = Self(5284u32);
    pub const COMPUTE_DERIVATIVE_GROUP_QUADS_KHR: Self = Self(5288u32);
    pub const FRAGMENT_DENSITY_EXT: Self = Self(5291u32);
    pub const GROUP_NON_UNIFORM_PARTITIONED_EXT: Self = Self(5297u32);
    pub const SHADER_NON_UNIFORM: Self = Self(5301u32);
    pub const RUNTIME_DESCRIPTOR_ARRAY: Self = Self(5302u32);
    pub const INPUT_ATTACHMENT_ARRAY_DYNAMIC_INDEXING: Self = Self(5303u32);
    pub const UNIFORM_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING: Self = Self(5304u32);
    pub const STORAGE_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING: Self = Self(5305u32);
    pub const UNIFORM_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5306u32);
    pub const SAMPLED_IMAGE_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5307u32);
    pub const STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5308u32);
    pub const STORAGE_IMAGE_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5309u32);
    pub const INPUT_ATTACHMENT_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5310u32);
    pub const UNIFORM_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5311u32);
    pub const STORAGE_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING: Self = Self(5312u32);
    pub const RAY_TRACING_POSITION_FETCH_KHR: Self = Self(5336u32);
    pub const RAY_TRACING_NV: Self = Self(5340u32);
    pub const RAY_TRACING_MOTION_BLUR_NV: Self = Self(5341u32);
    pub const VULKAN_MEMORY_MODEL: Self = Self(5345u32);
    pub const VULKAN_MEMORY_MODEL_DEVICE_SCOPE: Self = Self(5346u32);
    pub const PHYSICAL_STORAGE_BUFFER_ADDRESSES: Self = Self(5347u32);
    pub const COMPUTE_DERIVATIVE_GROUP_LINEAR_KHR: Self = Self(5350u32);
    pub const RAY_TRACING_PROVISIONAL_KHR: Self = Self(5353u32);
    pub const COOPERATIVE_MATRIX_NV: Self = Self(5357u32);
    pub const FRAGMENT_SHADER_SAMPLE_INTERLOCK_EXT: Self = Self(5363u32);
    pub const FRAGMENT_SHADER_SHADING_RATE_INTERLOCK_EXT: Self = Self(5372u32);
    pub const SHADER_SMBUILTINS_NV: Self = Self(5373u32);
    pub const FRAGMENT_SHADER_PIXEL_INTERLOCK_EXT: Self = Self(5378u32);
    pub const DEMOTE_TO_HELPER_INVOCATION: Self = Self(5379u32);
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(5380u32);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(5381u32);
    pub const SHADER_INVOCATION_REORDER_NV: Self = Self(5383u32);
    pub const SHADER_INVOCATION_REORDER_EXT: Self = Self(5388u32);
    pub const BINDLESS_TEXTURE_NV: Self = Self(5390u32);
    pub const RAY_QUERY_POSITION_FETCH_KHR: Self = Self(5391u32);
    pub const COOPERATIVE_VECTOR_NV: Self = Self(5394u32);
    pub const ATOMIC_FLOAT_16_VECTOR_NV: Self = Self(5404u32);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(5409u32);
    pub const RAW_ACCESS_CHAINS_NV: Self = Self(5414u32);
    pub const RAY_TRACING_SPHERES_GEOMETRY_NV: Self = Self(5418u32);
    pub const RAY_TRACING_LINEAR_SWEPT_SPHERES_GEOMETRY_NV: Self = Self(5419u32);
    pub const PUSH_CONSTANT_BANKS_NV: Self = Self(5423u32);
    pub const LONG_VECTOR_EXT: Self = Self(5425u32);
    pub const SHADER_64_BIT_INDEXING_EXT: Self = Self(5426u32);
    pub const COOPERATIVE_MATRIX_REDUCTIONS_NV: Self = Self(5430u32);
    pub const COOPERATIVE_MATRIX_CONVERSIONS_NV: Self = Self(5431u32);
    pub const COOPERATIVE_MATRIX_PER_ELEMENT_OPERATIONS_NV: Self = Self(5432u32);
    pub const COOPERATIVE_MATRIX_TENSOR_ADDRESSING_NV: Self = Self(5433u32);
    pub const COOPERATIVE_MATRIX_BLOCK_LOADS_NV: Self = Self(5434u32);
    pub const COOPERATIVE_VECTOR_TRAINING_NV: Self = Self(5435u32);
    pub const RAY_TRACING_CLUSTER_ACCELERATION_STRUCTURE_NV: Self = Self(5437u32);
    pub const TENSOR_ADDRESSING_NV: Self = Self(5439u32);
    pub const SUBGROUP_SHUFFLE_INTEL: Self = Self(5568u32);
    pub const SUBGROUP_BUFFER_BLOCK_IOINTEL: Self = Self(5569u32);
    pub const SUBGROUP_IMAGE_BLOCK_IOINTEL: Self = Self(5570u32);
    pub const SUBGROUP_IMAGE_MEDIA_BLOCK_IOINTEL: Self = Self(5579u32);
    pub const ROUND_TO_INFINITY_INTEL: Self = Self(5582u32);
    pub const FLOATING_POINT_MODE_INTEL: Self = Self(5583u32);
    pub const INTEGER_FUNCTIONS_2_INTEL: Self = Self(5584u32);
    pub const FUNCTION_POINTERS_INTEL: Self = Self(5603u32);
    pub const INDIRECT_REFERENCES_INTEL: Self = Self(5604u32);
    pub const ASM_INTEL: Self = Self(5606u32);
    pub const ATOMIC_FLOAT_32_MIN_MAX_EXT: Self = Self(5612u32);
    pub const ATOMIC_FLOAT_64_MIN_MAX_EXT: Self = Self(5613u32);
    pub const ATOMIC_FLOAT_16_MIN_MAX_EXT: Self = Self(5616u32);
    pub const VECTOR_COMPUTE_INTEL: Self = Self(5617u32);
    pub const VECTOR_ANY_INTEL: Self = Self(5619u32);
    pub const EXPECT_ASSUME_KHR: Self = Self(5629u32);
    pub const SUBGROUP_AVC_MOTION_ESTIMATION_INTEL: Self = Self(5696u32);
    pub const SUBGROUP_AVC_MOTION_ESTIMATION_INTRA_INTEL: Self = Self(5697u32);
    pub const SUBGROUP_AVC_MOTION_ESTIMATION_CHROMA_INTEL: Self = Self(5698u32);
    pub const VARIABLE_LENGTH_ARRAY_INTEL: Self = Self(5817u32);
    pub const FUNCTION_FLOAT_CONTROL_INTEL: Self = Self(5821u32);
    pub const FPGAMEMORY_ATTRIBUTES_ALTERA: Self = Self(5824u32);
    pub const FPFAST_MATH_MODE_INTEL: Self = Self(5837u32);
    pub const ARBITRARY_PRECISION_INTEGERS_ALTERA: Self = Self(5844u32);
    pub const ARBITRARY_PRECISION_FLOATING_POINT_ALTERA: Self = Self(5845u32);
    pub const UNSTRUCTURED_LOOP_CONTROLS_INTEL: Self = Self(5886u32);
    pub const FPGALOOP_CONTROLS_ALTERA: Self = Self(5888u32);
    pub const KERNEL_ATTRIBUTES_INTEL: Self = Self(5892u32);
    pub const FPGAKERNEL_ATTRIBUTES_INTEL: Self = Self(5897u32);
    pub const FPGAMEMORY_ACCESSES_ALTERA: Self = Self(5898u32);
    pub const FPGACLUSTER_ATTRIBUTES_ALTERA: Self = Self(5904u32);
    pub const LOOP_FUSE_ALTERA: Self = Self(5906u32);
    pub const FPGADSPCONTROL_ALTERA: Self = Self(5908u32);
    pub const MEMORY_ACCESS_ALIASING_INTEL: Self = Self(5910u32);
    pub const FPGAINVOCATION_PIPELINING_ATTRIBUTES_ALTERA: Self = Self(5916u32);
    pub const FPGABUFFER_LOCATION_ALTERA: Self = Self(5920u32);
    pub const ARBITRARY_PRECISION_FIXED_POINT_ALTERA: Self = Self(5922u32);
    pub const USMSTORAGE_CLASSES_ALTERA: Self = Self(5935u32);
    pub const RUNTIME_ALIGNED_ATTRIBUTE_ALTERA: Self = Self(5939u32);
    pub const IOPIPES_ALTERA: Self = Self(5943u32);
    pub const BLOCKING_PIPES_ALTERA: Self = Self(5945u32);
    pub const FPGAREG_ALTERA: Self = Self(5948u32);
    pub const DOT_PRODUCT_INPUT_ALL: Self = Self(6016u32);
    pub const DOT_PRODUCT_INPUT_4X_8_BIT: Self = Self(6017u32);
    pub const DOT_PRODUCT_INPUT_4X_8_BIT_PACKED: Self = Self(6018u32);
    pub const DOT_PRODUCT: Self = Self(6019u32);
    pub const RAY_CULL_MASK_KHR: Self = Self(6020u32);
    pub const COOPERATIVE_MATRIX_KHR: Self = Self(6022u32);
    pub const REPLICATED_COMPOSITES_EXT: Self = Self(6024u32);
    pub const BIT_INSTRUCTIONS: Self = Self(6025u32);
    pub const GROUP_NON_UNIFORM_ROTATE_KHR: Self = Self(6026u32);
    pub const FLOAT_CONTROLS_2: Self = Self(6029u32);
    pub const FMAKHR: Self = Self(6030u32);
    pub const ATOMIC_FLOAT_32_ADD_EXT: Self = Self(6033u32);
    pub const ATOMIC_FLOAT_64_ADD_EXT: Self = Self(6034u32);
    pub const LONG_COMPOSITES_INTEL: Self = Self(6089u32);
    pub const OPT_NONE_EXT: Self = Self(6094u32);
    pub const ATOMIC_FLOAT_16_ADD_EXT: Self = Self(6095u32);
    pub const DEBUG_INFO_MODULE_INTEL: Self = Self(6114u32);
    pub const BFLOAT_16_CONVERSION_INTEL: Self = Self(6115u32);
    pub const SPLIT_BARRIER_INTEL: Self = Self(6141u32);
    pub const ARITHMETIC_FENCE_EXT: Self = Self(6144u32);
    pub const FPGACLUSTER_ATTRIBUTES_V2ALTERA: Self = Self(6150u32);
    pub const FPGAKERNEL_ATTRIBUTESV_2_INTEL: Self = Self(6161u32);
    pub const TASK_SEQUENCE_ALTERA: Self = Self(6162u32);
    pub const FPMAX_ERROR_INTEL: Self = Self(6169u32);
    pub const FPGALATENCY_CONTROL_ALTERA: Self = Self(6171u32);
    pub const FPGAARGUMENT_INTERFACES_ALTERA: Self = Self(6174u32);
    pub const GLOBAL_VARIABLE_HOST_ACCESS_INTEL: Self = Self(6187u32);
    pub const GLOBAL_VARIABLE_FPGADECORATIONS_ALTERA: Self = Self(6189u32);
    pub const SUBGROUP_BUFFER_PREFETCH_INTEL: Self = Self(6220u32);
    pub const SUBGROUP_2D_BLOCK_IOINTEL: Self = Self(6228u32);
    pub const SUBGROUP_2D_BLOCK_TRANSFORM_INTEL: Self = Self(6229u32);
    pub const SUBGROUP_2D_BLOCK_TRANSPOSE_INTEL: Self = Self(6230u32);
    pub const SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE_INTEL: Self = Self(6236u32);
    pub const TERNARY_BITWISE_FUNCTION_INTEL: Self = Self(6241u32);
    pub const UNTYPED_VARIABLE_LENGTH_ARRAY_INTEL: Self = Self(6243u32);
    pub const SPEC_CONDITIONAL_INTEL: Self = Self(6245u32);
    pub const FUNCTION_VARIANTS_INTEL: Self = Self(6246u32);
    pub const GROUP_UNIFORM_ARITHMETIC_KHR: Self = Self(6400u32);
    pub const TENSOR_FLOAT_32_ROUNDING_INTEL: Self = Self(6425u32);
    pub const MASKED_GATHER_SCATTER_INTEL: Self = Self(6427u32);
    pub const CACHE_CONTROLS_INTEL: Self = Self(6441u32);
    pub const REGISTER_LIMITS_INTEL: Self = Self(6460u32);
    pub const BINDLESS_IMAGES_INTEL: Self = Self(6528u32);
    pub const DOT_PRODUCT_FLOAT_16_ACC_FLOAT_32_VALVE: Self = Self(6912u32);
    pub const DOT_PRODUCT_FLOAT_16_ACC_FLOAT_16_VALVE: Self = Self(6913u32);
    pub const DOT_PRODUCT_BFLOAT_16_ACC_VALVE: Self = Self(6914u32);
    pub const DOT_PRODUCT_FLOAT_8_ACC_FLOAT_32_VALVE: Self = Self(6915u32);
}
impl Word for Capability {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for Capability {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MATRIX => Display::fmt(stringify!(MATRIX), f),
            Self::SHADER => Display::fmt(stringify!(SHADER), f),
            Self::GEOMETRY => Display::fmt(stringify!(GEOMETRY), f),
            Self::TESSELLATION => Display::fmt(stringify!(TESSELLATION), f),
            Self::ADDRESSES => Display::fmt(stringify!(ADDRESSES), f),
            Self::LINKAGE => Display::fmt(stringify!(LINKAGE), f),
            Self::KERNEL => Display::fmt(stringify!(KERNEL), f),
            Self::VECTOR_16 => Display::fmt(stringify!(VECTOR_16), f),
            Self::FLOAT_16_BUFFER => Display::fmt(stringify!(FLOAT_16_BUFFER), f),
            Self::FLOAT_16 => Display::fmt(stringify!(FLOAT_16), f),
            Self::FLOAT_64 => Display::fmt(stringify!(FLOAT_64), f),
            Self::INT_64 => Display::fmt(stringify!(INT_64), f),
            Self::INT_64_ATOMICS => Display::fmt(stringify!(INT_64_ATOMICS), f),
            Self::IMAGE_BASIC => Display::fmt(stringify!(IMAGE_BASIC), f),
            Self::IMAGE_READ_WRITE => Display::fmt(stringify!(IMAGE_READ_WRITE), f),
            Self::IMAGE_MIPMAP => Display::fmt(stringify!(IMAGE_MIPMAP), f),
            Self::PIPES => Display::fmt(stringify!(PIPES), f),
            Self::GROUPS => Display::fmt(stringify!(GROUPS), f),
            Self::DEVICE_ENQUEUE => Display::fmt(stringify!(DEVICE_ENQUEUE), f),
            Self::LITERAL_SAMPLER => Display::fmt(stringify!(LITERAL_SAMPLER), f),
            Self::ATOMIC_STORAGE => Display::fmt(stringify!(ATOMIC_STORAGE), f),
            Self::INT_16 => Display::fmt(stringify!(INT_16), f),
            Self::TESSELLATION_POINT_SIZE => Display::fmt(stringify!(TESSELLATION_POINT_SIZE), f),
            Self::GEOMETRY_POINT_SIZE => Display::fmt(stringify!(GEOMETRY_POINT_SIZE), f),
            Self::IMAGE_GATHER_EXTENDED => Display::fmt(stringify!(IMAGE_GATHER_EXTENDED), f),
            Self::STORAGE_IMAGE_MULTISAMPLE => {
                Display::fmt(stringify!(STORAGE_IMAGE_MULTISAMPLE), f)
            }
            Self::UNIFORM_BUFFER_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(UNIFORM_BUFFER_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::SAMPLED_IMAGE_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(SAMPLED_IMAGE_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::STORAGE_BUFFER_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(STORAGE_BUFFER_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::STORAGE_IMAGE_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(STORAGE_IMAGE_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::CLIP_DISTANCE => Display::fmt(stringify!(CLIP_DISTANCE), f),
            Self::CULL_DISTANCE => Display::fmt(stringify!(CULL_DISTANCE), f),
            Self::IMAGE_CUBE_ARRAY => Display::fmt(stringify!(IMAGE_CUBE_ARRAY), f),
            Self::SAMPLE_RATE_SHADING => Display::fmt(stringify!(SAMPLE_RATE_SHADING), f),
            Self::IMAGE_RECT => Display::fmt(stringify!(IMAGE_RECT), f),
            Self::SAMPLED_RECT => Display::fmt(stringify!(SAMPLED_RECT), f),
            Self::GENERIC_POINTER => Display::fmt(stringify!(GENERIC_POINTER), f),
            Self::INT_8 => Display::fmt(stringify!(INT_8), f),
            Self::INPUT_ATTACHMENT => Display::fmt(stringify!(INPUT_ATTACHMENT), f),
            Self::SPARSE_RESIDENCY => Display::fmt(stringify!(SPARSE_RESIDENCY), f),
            Self::MIN_LOD => Display::fmt(stringify!(MIN_LOD), f),
            Self::SAMPLED_1D => Display::fmt(stringify!(SAMPLED_1D), f),
            Self::IMAGE_1D => Display::fmt(stringify!(IMAGE_1D), f),
            Self::SAMPLED_CUBE_ARRAY => Display::fmt(stringify!(SAMPLED_CUBE_ARRAY), f),
            Self::SAMPLED_BUFFER => Display::fmt(stringify!(SAMPLED_BUFFER), f),
            Self::IMAGE_BUFFER => Display::fmt(stringify!(IMAGE_BUFFER), f),
            Self::IMAGE_MSARRAY => Display::fmt(stringify!(IMAGE_MSARRAY), f),
            Self::STORAGE_IMAGE_EXTENDED_FORMATS => {
                Display::fmt(stringify!(STORAGE_IMAGE_EXTENDED_FORMATS), f)
            }
            Self::IMAGE_QUERY => Display::fmt(stringify!(IMAGE_QUERY), f),
            Self::DERIVATIVE_CONTROL => Display::fmt(stringify!(DERIVATIVE_CONTROL), f),
            Self::INTERPOLATION_FUNCTION => Display::fmt(stringify!(INTERPOLATION_FUNCTION), f),
            Self::TRANSFORM_FEEDBACK => Display::fmt(stringify!(TRANSFORM_FEEDBACK), f),
            Self::GEOMETRY_STREAMS => Display::fmt(stringify!(GEOMETRY_STREAMS), f),
            Self::STORAGE_IMAGE_READ_WITHOUT_FORMAT => {
                Display::fmt(stringify!(STORAGE_IMAGE_READ_WITHOUT_FORMAT), f)
            }
            Self::STORAGE_IMAGE_WRITE_WITHOUT_FORMAT => {
                Display::fmt(stringify!(STORAGE_IMAGE_WRITE_WITHOUT_FORMAT), f)
            }
            Self::MULTI_VIEWPORT => Display::fmt(stringify!(MULTI_VIEWPORT), f),
            Self::SUBGROUP_DISPATCH => Display::fmt(stringify!(SUBGROUP_DISPATCH), f),
            Self::NAMED_BARRIER => Display::fmt(stringify!(NAMED_BARRIER), f),
            Self::PIPE_STORAGE => Display::fmt(stringify!(PIPE_STORAGE), f),
            Self::GROUP_NON_UNIFORM => Display::fmt(stringify!(GROUP_NON_UNIFORM), f),
            Self::GROUP_NON_UNIFORM_VOTE => Display::fmt(stringify!(GROUP_NON_UNIFORM_VOTE), f),
            Self::GROUP_NON_UNIFORM_ARITHMETIC => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_ARITHMETIC), f)
            }
            Self::GROUP_NON_UNIFORM_BALLOT => Display::fmt(stringify!(GROUP_NON_UNIFORM_BALLOT), f),
            Self::GROUP_NON_UNIFORM_SHUFFLE => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_SHUFFLE), f)
            }
            Self::GROUP_NON_UNIFORM_SHUFFLE_RELATIVE => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_SHUFFLE_RELATIVE), f)
            }
            Self::GROUP_NON_UNIFORM_CLUSTERED => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_CLUSTERED), f)
            }
            Self::GROUP_NON_UNIFORM_QUAD => Display::fmt(stringify!(GROUP_NON_UNIFORM_QUAD), f),
            Self::SHADER_LAYER => Display::fmt(stringify!(SHADER_LAYER), f),
            Self::SHADER_VIEWPORT_INDEX => Display::fmt(stringify!(SHADER_VIEWPORT_INDEX), f),
            Self::UNIFORM_DECORATION => Display::fmt(stringify!(UNIFORM_DECORATION), f),
            Self::CORE_BUILTINS_ARM => Display::fmt(stringify!(CORE_BUILTINS_ARM), f),
            Self::TILE_IMAGE_COLOR_READ_ACCESS_EXT => {
                Display::fmt(stringify!(TILE_IMAGE_COLOR_READ_ACCESS_EXT), f)
            }
            Self::TILE_IMAGE_DEPTH_READ_ACCESS_EXT => {
                Display::fmt(stringify!(TILE_IMAGE_DEPTH_READ_ACCESS_EXT), f)
            }
            Self::TILE_IMAGE_STENCIL_READ_ACCESS_EXT => {
                Display::fmt(stringify!(TILE_IMAGE_STENCIL_READ_ACCESS_EXT), f)
            }
            Self::TENSORS_ARM => Display::fmt(stringify!(TENSORS_ARM), f),
            Self::STORAGE_TENSOR_ARRAY_DYNAMIC_INDEXING_ARM => {
                Display::fmt(stringify!(STORAGE_TENSOR_ARRAY_DYNAMIC_INDEXING_ARM), f)
            }
            Self::STORAGE_TENSOR_ARRAY_NON_UNIFORM_INDEXING_ARM => {
                Display::fmt(stringify!(STORAGE_TENSOR_ARRAY_NON_UNIFORM_INDEXING_ARM), f)
            }
            Self::GRAPH_ARM => Display::fmt(stringify!(GRAPH_ARM), f),
            Self::COOPERATIVE_MATRIX_LAYOUTS_ARM => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_LAYOUTS_ARM), f)
            }
            Self::FLOAT_8_EXT => Display::fmt(stringify!(FLOAT_8_EXT), f),
            Self::FLOAT_8_COOPERATIVE_MATRIX_EXT => {
                Display::fmt(stringify!(FLOAT_8_COOPERATIVE_MATRIX_EXT), f)
            }
            Self::FRAGMENT_SHADING_RATE_KHR => {
                Display::fmt(stringify!(FRAGMENT_SHADING_RATE_KHR), f)
            }
            Self::SUBGROUP_BALLOT_KHR => Display::fmt(stringify!(SUBGROUP_BALLOT_KHR), f),
            Self::DRAW_PARAMETERS => Display::fmt(stringify!(DRAW_PARAMETERS), f),
            Self::WORKGROUP_MEMORY_EXPLICIT_LAYOUT_KHR => {
                Display::fmt(stringify!(WORKGROUP_MEMORY_EXPLICIT_LAYOUT_KHR), f)
            }
            Self::WORKGROUP_MEMORY_EXPLICIT_LAYOUT_8_BIT_ACCESS_KHR => Display::fmt(
                stringify!(WORKGROUP_MEMORY_EXPLICIT_LAYOUT_8_BIT_ACCESS_KHR),
                f,
            ),
            Self::WORKGROUP_MEMORY_EXPLICIT_LAYOUT_16_BIT_ACCESS_KHR => Display::fmt(
                stringify!(WORKGROUP_MEMORY_EXPLICIT_LAYOUT_16_BIT_ACCESS_KHR),
                f,
            ),
            Self::SUBGROUP_VOTE_KHR => Display::fmt(stringify!(SUBGROUP_VOTE_KHR), f),
            Self::STORAGE_BUFFER_16_BIT_ACCESS => {
                Display::fmt(stringify!(STORAGE_BUFFER_16_BIT_ACCESS), f)
            }
            Self::UNIFORM_AND_STORAGE_BUFFER_16_BIT_ACCESS => {
                Display::fmt(stringify!(UNIFORM_AND_STORAGE_BUFFER_16_BIT_ACCESS), f)
            }
            Self::STORAGE_PUSH_CONSTANT_16 => Display::fmt(stringify!(STORAGE_PUSH_CONSTANT_16), f),
            Self::STORAGE_INPUT_OUTPUT_16 => Display::fmt(stringify!(STORAGE_INPUT_OUTPUT_16), f),
            Self::DEVICE_GROUP => Display::fmt(stringify!(DEVICE_GROUP), f),
            Self::MULTI_VIEW => Display::fmt(stringify!(MULTI_VIEW), f),
            Self::VARIABLE_POINTERS_STORAGE_BUFFER => {
                Display::fmt(stringify!(VARIABLE_POINTERS_STORAGE_BUFFER), f)
            }
            Self::VARIABLE_POINTERS => Display::fmt(stringify!(VARIABLE_POINTERS), f),
            Self::ATOMIC_STORAGE_OPS => Display::fmt(stringify!(ATOMIC_STORAGE_OPS), f),
            Self::SAMPLE_MASK_POST_DEPTH_COVERAGE => {
                Display::fmt(stringify!(SAMPLE_MASK_POST_DEPTH_COVERAGE), f)
            }
            Self::STORAGE_BUFFER_8_BIT_ACCESS => {
                Display::fmt(stringify!(STORAGE_BUFFER_8_BIT_ACCESS), f)
            }
            Self::UNIFORM_AND_STORAGE_BUFFER_8_BIT_ACCESS => {
                Display::fmt(stringify!(UNIFORM_AND_STORAGE_BUFFER_8_BIT_ACCESS), f)
            }
            Self::STORAGE_PUSH_CONSTANT_8 => Display::fmt(stringify!(STORAGE_PUSH_CONSTANT_8), f),
            Self::DENORM_PRESERVE => Display::fmt(stringify!(DENORM_PRESERVE), f),
            Self::DENORM_FLUSH_TO_ZERO => Display::fmt(stringify!(DENORM_FLUSH_TO_ZERO), f),
            Self::SIGNED_ZERO_INF_NAN_PRESERVE => {
                Display::fmt(stringify!(SIGNED_ZERO_INF_NAN_PRESERVE), f)
            }
            Self::ROUNDING_MODE_RTE => Display::fmt(stringify!(ROUNDING_MODE_RTE), f),
            Self::ROUNDING_MODE_RTZ => Display::fmt(stringify!(ROUNDING_MODE_RTZ), f),
            Self::RAY_QUERY_PROVISIONAL_KHR => {
                Display::fmt(stringify!(RAY_QUERY_PROVISIONAL_KHR), f)
            }
            Self::RAY_QUERY_KHR => Display::fmt(stringify!(RAY_QUERY_KHR), f),
            Self::UNTYPED_POINTERS_KHR => Display::fmt(stringify!(UNTYPED_POINTERS_KHR), f),
            Self::RAY_TRAVERSAL_PRIMITIVE_CULLING_KHR => {
                Display::fmt(stringify!(RAY_TRAVERSAL_PRIMITIVE_CULLING_KHR), f)
            }
            Self::RAY_TRACING_KHR => Display::fmt(stringify!(RAY_TRACING_KHR), f),
            Self::TEXTURE_SAMPLE_WEIGHTED_QCOM => {
                Display::fmt(stringify!(TEXTURE_SAMPLE_WEIGHTED_QCOM), f)
            }
            Self::TEXTURE_BOX_FILTER_QCOM => Display::fmt(stringify!(TEXTURE_BOX_FILTER_QCOM), f),
            Self::TEXTURE_BLOCK_MATCH_QCOM => Display::fmt(stringify!(TEXTURE_BLOCK_MATCH_QCOM), f),
            Self::TILE_SHADING_QCOM => Display::fmt(stringify!(TILE_SHADING_QCOM), f),
            Self::COOPERATIVE_MATRIX_CONVERSION_QCOM => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_CONVERSION_QCOM), f)
            }
            Self::TEXTURE_BLOCK_MATCH_2_QCOM => {
                Display::fmt(stringify!(TEXTURE_BLOCK_MATCH_2_QCOM), f)
            }
            Self::FLOAT_16_IMAGE_AMD => Display::fmt(stringify!(FLOAT_16_IMAGE_AMD), f),
            Self::IMAGE_GATHER_BIAS_LOD_AMD => {
                Display::fmt(stringify!(IMAGE_GATHER_BIAS_LOD_AMD), f)
            }
            Self::FRAGMENT_MASK_AMD => Display::fmt(stringify!(FRAGMENT_MASK_AMD), f),
            Self::STENCIL_EXPORT_EXT => Display::fmt(stringify!(STENCIL_EXPORT_EXT), f),
            Self::IMAGE_READ_WRITE_LOD_AMD => Display::fmt(stringify!(IMAGE_READ_WRITE_LOD_AMD), f),
            Self::INT_64_IMAGE_EXT => Display::fmt(stringify!(INT_64_IMAGE_EXT), f),
            Self::SHADER_CLOCK_KHR => Display::fmt(stringify!(SHADER_CLOCK_KHR), f),
            Self::SHADER_ENQUEUE_AMDX => Display::fmt(stringify!(SHADER_ENQUEUE_AMDX), f),
            Self::QUAD_CONTROL_KHR => Display::fmt(stringify!(QUAD_CONTROL_KHR), f),
            Self::INT_4_TYPE_INTEL => Display::fmt(stringify!(INT_4_TYPE_INTEL), f),
            Self::INT_4_COOPERATIVE_MATRIX_INTEL => {
                Display::fmt(stringify!(INT_4_COOPERATIVE_MATRIX_INTEL), f)
            }
            Self::BFLOAT_16_TYPE_KHR => Display::fmt(stringify!(BFLOAT_16_TYPE_KHR), f),
            Self::BFLOAT_16DOT_PRODUCT_KHR => Display::fmt(stringify!(BFLOAT_16DOT_PRODUCT_KHR), f),
            Self::BFLOAT_16_COOPERATIVE_MATRIX_KHR => {
                Display::fmt(stringify!(BFLOAT_16_COOPERATIVE_MATRIX_KHR), f)
            }
            Self::ABORT_KHR => Display::fmt(stringify!(ABORT_KHR), f),
            Self::DESCRIPTOR_HEAP_EXT => Display::fmt(stringify!(DESCRIPTOR_HEAP_EXT), f),
            Self::CONSTANT_DATA_KHR => Display::fmt(stringify!(CONSTANT_DATA_KHR), f),
            Self::POISON_FREEZE_KHR => Display::fmt(stringify!(POISON_FREEZE_KHR), f),
            Self::SAMPLE_MASK_OVERRIDE_COVERAGE_NV => {
                Display::fmt(stringify!(SAMPLE_MASK_OVERRIDE_COVERAGE_NV), f)
            }
            Self::GEOMETRY_SHADER_PASSTHROUGH_NV => {
                Display::fmt(stringify!(GEOMETRY_SHADER_PASSTHROUGH_NV), f)
            }
            Self::SHADER_VIEWPORT_INDEX_LAYER_EXT => {
                Display::fmt(stringify!(SHADER_VIEWPORT_INDEX_LAYER_EXT), f)
            }
            Self::SHADER_VIEWPORT_MASK_NV => Display::fmt(stringify!(SHADER_VIEWPORT_MASK_NV), f),
            Self::SHADER_STEREO_VIEW_NV => Display::fmt(stringify!(SHADER_STEREO_VIEW_NV), f),
            Self::PER_VIEW_ATTRIBUTES_NV => Display::fmt(stringify!(PER_VIEW_ATTRIBUTES_NV), f),
            Self::FRAGMENT_FULLY_COVERED_EXT => {
                Display::fmt(stringify!(FRAGMENT_FULLY_COVERED_EXT), f)
            }
            Self::MESH_SHADING_NV => Display::fmt(stringify!(MESH_SHADING_NV), f),
            Self::IMAGE_FOOTPRINT_NV => Display::fmt(stringify!(IMAGE_FOOTPRINT_NV), f),
            Self::MESH_SHADING_EXT => Display::fmt(stringify!(MESH_SHADING_EXT), f),
            Self::FRAGMENT_BARYCENTRIC_KHR => Display::fmt(stringify!(FRAGMENT_BARYCENTRIC_KHR), f),
            Self::COMPUTE_DERIVATIVE_GROUP_QUADS_KHR => {
                Display::fmt(stringify!(COMPUTE_DERIVATIVE_GROUP_QUADS_KHR), f)
            }
            Self::FRAGMENT_DENSITY_EXT => Display::fmt(stringify!(FRAGMENT_DENSITY_EXT), f),
            Self::GROUP_NON_UNIFORM_PARTITIONED_EXT => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_PARTITIONED_EXT), f)
            }
            Self::SHADER_NON_UNIFORM => Display::fmt(stringify!(SHADER_NON_UNIFORM), f),
            Self::RUNTIME_DESCRIPTOR_ARRAY => Display::fmt(stringify!(RUNTIME_DESCRIPTOR_ARRAY), f),
            Self::INPUT_ATTACHMENT_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(INPUT_ATTACHMENT_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::UNIFORM_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(UNIFORM_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::STORAGE_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING => {
                Display::fmt(stringify!(STORAGE_TEXEL_BUFFER_ARRAY_DYNAMIC_INDEXING), f)
            }
            Self::UNIFORM_BUFFER_ARRAY_NON_UNIFORM_INDEXING => {
                Display::fmt(stringify!(UNIFORM_BUFFER_ARRAY_NON_UNIFORM_INDEXING), f)
            }
            Self::SAMPLED_IMAGE_ARRAY_NON_UNIFORM_INDEXING => {
                Display::fmt(stringify!(SAMPLED_IMAGE_ARRAY_NON_UNIFORM_INDEXING), f)
            }
            Self::STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING => {
                Display::fmt(stringify!(STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING), f)
            }
            Self::STORAGE_IMAGE_ARRAY_NON_UNIFORM_INDEXING => {
                Display::fmt(stringify!(STORAGE_IMAGE_ARRAY_NON_UNIFORM_INDEXING), f)
            }
            Self::INPUT_ATTACHMENT_ARRAY_NON_UNIFORM_INDEXING => {
                Display::fmt(stringify!(INPUT_ATTACHMENT_ARRAY_NON_UNIFORM_INDEXING), f)
            }
            Self::UNIFORM_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING => Display::fmt(
                stringify!(UNIFORM_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
                f,
            ),
            Self::STORAGE_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING => Display::fmt(
                stringify!(STORAGE_TEXEL_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
                f,
            ),
            Self::RAY_TRACING_POSITION_FETCH_KHR => {
                Display::fmt(stringify!(RAY_TRACING_POSITION_FETCH_KHR), f)
            }
            Self::RAY_TRACING_NV => Display::fmt(stringify!(RAY_TRACING_NV), f),
            Self::RAY_TRACING_MOTION_BLUR_NV => {
                Display::fmt(stringify!(RAY_TRACING_MOTION_BLUR_NV), f)
            }
            Self::VULKAN_MEMORY_MODEL => Display::fmt(stringify!(VULKAN_MEMORY_MODEL), f),
            Self::VULKAN_MEMORY_MODEL_DEVICE_SCOPE => {
                Display::fmt(stringify!(VULKAN_MEMORY_MODEL_DEVICE_SCOPE), f)
            }
            Self::PHYSICAL_STORAGE_BUFFER_ADDRESSES => {
                Display::fmt(stringify!(PHYSICAL_STORAGE_BUFFER_ADDRESSES), f)
            }
            Self::COMPUTE_DERIVATIVE_GROUP_LINEAR_KHR => {
                Display::fmt(stringify!(COMPUTE_DERIVATIVE_GROUP_LINEAR_KHR), f)
            }
            Self::RAY_TRACING_PROVISIONAL_KHR => {
                Display::fmt(stringify!(RAY_TRACING_PROVISIONAL_KHR), f)
            }
            Self::COOPERATIVE_MATRIX_NV => Display::fmt(stringify!(COOPERATIVE_MATRIX_NV), f),
            Self::FRAGMENT_SHADER_SAMPLE_INTERLOCK_EXT => {
                Display::fmt(stringify!(FRAGMENT_SHADER_SAMPLE_INTERLOCK_EXT), f)
            }
            Self::FRAGMENT_SHADER_SHADING_RATE_INTERLOCK_EXT => {
                Display::fmt(stringify!(FRAGMENT_SHADER_SHADING_RATE_INTERLOCK_EXT), f)
            }
            Self::SHADER_SMBUILTINS_NV => Display::fmt(stringify!(SHADER_SMBUILTINS_NV), f),
            Self::FRAGMENT_SHADER_PIXEL_INTERLOCK_EXT => {
                Display::fmt(stringify!(FRAGMENT_SHADER_PIXEL_INTERLOCK_EXT), f)
            }
            Self::DEMOTE_TO_HELPER_INVOCATION => {
                Display::fmt(stringify!(DEMOTE_TO_HELPER_INVOCATION), f)
            }
            Self::DISPLACEMENT_MICROMAP_NV => Display::fmt(stringify!(DISPLACEMENT_MICROMAP_NV), f),
            Self::RAY_TRACING_OPACITY_MICROMAP_EXT => {
                Display::fmt(stringify!(RAY_TRACING_OPACITY_MICROMAP_EXT), f)
            }
            Self::SHADER_INVOCATION_REORDER_NV => {
                Display::fmt(stringify!(SHADER_INVOCATION_REORDER_NV), f)
            }
            Self::SHADER_INVOCATION_REORDER_EXT => {
                Display::fmt(stringify!(SHADER_INVOCATION_REORDER_EXT), f)
            }
            Self::BINDLESS_TEXTURE_NV => Display::fmt(stringify!(BINDLESS_TEXTURE_NV), f),
            Self::RAY_QUERY_POSITION_FETCH_KHR => {
                Display::fmt(stringify!(RAY_QUERY_POSITION_FETCH_KHR), f)
            }
            Self::COOPERATIVE_VECTOR_NV => Display::fmt(stringify!(COOPERATIVE_VECTOR_NV), f),
            Self::ATOMIC_FLOAT_16_VECTOR_NV => {
                Display::fmt(stringify!(ATOMIC_FLOAT_16_VECTOR_NV), f)
            }
            Self::RAY_TRACING_DISPLACEMENT_MICROMAP_NV => {
                Display::fmt(stringify!(RAY_TRACING_DISPLACEMENT_MICROMAP_NV), f)
            }
            Self::RAW_ACCESS_CHAINS_NV => Display::fmt(stringify!(RAW_ACCESS_CHAINS_NV), f),
            Self::RAY_TRACING_SPHERES_GEOMETRY_NV => {
                Display::fmt(stringify!(RAY_TRACING_SPHERES_GEOMETRY_NV), f)
            }
            Self::RAY_TRACING_LINEAR_SWEPT_SPHERES_GEOMETRY_NV => {
                Display::fmt(stringify!(RAY_TRACING_LINEAR_SWEPT_SPHERES_GEOMETRY_NV), f)
            }
            Self::PUSH_CONSTANT_BANKS_NV => Display::fmt(stringify!(PUSH_CONSTANT_BANKS_NV), f),
            Self::LONG_VECTOR_EXT => Display::fmt(stringify!(LONG_VECTOR_EXT), f),
            Self::SHADER_64_BIT_INDEXING_EXT => {
                Display::fmt(stringify!(SHADER_64_BIT_INDEXING_EXT), f)
            }
            Self::COOPERATIVE_MATRIX_REDUCTIONS_NV => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_REDUCTIONS_NV), f)
            }
            Self::COOPERATIVE_MATRIX_CONVERSIONS_NV => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_CONVERSIONS_NV), f)
            }
            Self::COOPERATIVE_MATRIX_PER_ELEMENT_OPERATIONS_NV => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_PER_ELEMENT_OPERATIONS_NV), f)
            }
            Self::COOPERATIVE_MATRIX_TENSOR_ADDRESSING_NV => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_TENSOR_ADDRESSING_NV), f)
            }
            Self::COOPERATIVE_MATRIX_BLOCK_LOADS_NV => {
                Display::fmt(stringify!(COOPERATIVE_MATRIX_BLOCK_LOADS_NV), f)
            }
            Self::COOPERATIVE_VECTOR_TRAINING_NV => {
                Display::fmt(stringify!(COOPERATIVE_VECTOR_TRAINING_NV), f)
            }
            Self::RAY_TRACING_CLUSTER_ACCELERATION_STRUCTURE_NV => {
                Display::fmt(stringify!(RAY_TRACING_CLUSTER_ACCELERATION_STRUCTURE_NV), f)
            }
            Self::TENSOR_ADDRESSING_NV => Display::fmt(stringify!(TENSOR_ADDRESSING_NV), f),
            Self::SUBGROUP_SHUFFLE_INTEL => Display::fmt(stringify!(SUBGROUP_SHUFFLE_INTEL), f),
            Self::SUBGROUP_BUFFER_BLOCK_IOINTEL => {
                Display::fmt(stringify!(SUBGROUP_BUFFER_BLOCK_IOINTEL), f)
            }
            Self::SUBGROUP_IMAGE_BLOCK_IOINTEL => {
                Display::fmt(stringify!(SUBGROUP_IMAGE_BLOCK_IOINTEL), f)
            }
            Self::SUBGROUP_IMAGE_MEDIA_BLOCK_IOINTEL => {
                Display::fmt(stringify!(SUBGROUP_IMAGE_MEDIA_BLOCK_IOINTEL), f)
            }
            Self::ROUND_TO_INFINITY_INTEL => Display::fmt(stringify!(ROUND_TO_INFINITY_INTEL), f),
            Self::FLOATING_POINT_MODE_INTEL => {
                Display::fmt(stringify!(FLOATING_POINT_MODE_INTEL), f)
            }
            Self::INTEGER_FUNCTIONS_2_INTEL => {
                Display::fmt(stringify!(INTEGER_FUNCTIONS_2_INTEL), f)
            }
            Self::FUNCTION_POINTERS_INTEL => Display::fmt(stringify!(FUNCTION_POINTERS_INTEL), f),
            Self::INDIRECT_REFERENCES_INTEL => {
                Display::fmt(stringify!(INDIRECT_REFERENCES_INTEL), f)
            }
            Self::ASM_INTEL => Display::fmt(stringify!(ASM_INTEL), f),
            Self::ATOMIC_FLOAT_32_MIN_MAX_EXT => {
                Display::fmt(stringify!(ATOMIC_FLOAT_32_MIN_MAX_EXT), f)
            }
            Self::ATOMIC_FLOAT_64_MIN_MAX_EXT => {
                Display::fmt(stringify!(ATOMIC_FLOAT_64_MIN_MAX_EXT), f)
            }
            Self::ATOMIC_FLOAT_16_MIN_MAX_EXT => {
                Display::fmt(stringify!(ATOMIC_FLOAT_16_MIN_MAX_EXT), f)
            }
            Self::VECTOR_COMPUTE_INTEL => Display::fmt(stringify!(VECTOR_COMPUTE_INTEL), f),
            Self::VECTOR_ANY_INTEL => Display::fmt(stringify!(VECTOR_ANY_INTEL), f),
            Self::EXPECT_ASSUME_KHR => Display::fmt(stringify!(EXPECT_ASSUME_KHR), f),
            Self::SUBGROUP_AVC_MOTION_ESTIMATION_INTEL => {
                Display::fmt(stringify!(SUBGROUP_AVC_MOTION_ESTIMATION_INTEL), f)
            }
            Self::SUBGROUP_AVC_MOTION_ESTIMATION_INTRA_INTEL => {
                Display::fmt(stringify!(SUBGROUP_AVC_MOTION_ESTIMATION_INTRA_INTEL), f)
            }
            Self::SUBGROUP_AVC_MOTION_ESTIMATION_CHROMA_INTEL => {
                Display::fmt(stringify!(SUBGROUP_AVC_MOTION_ESTIMATION_CHROMA_INTEL), f)
            }
            Self::VARIABLE_LENGTH_ARRAY_INTEL => {
                Display::fmt(stringify!(VARIABLE_LENGTH_ARRAY_INTEL), f)
            }
            Self::FUNCTION_FLOAT_CONTROL_INTEL => {
                Display::fmt(stringify!(FUNCTION_FLOAT_CONTROL_INTEL), f)
            }
            Self::FPGAMEMORY_ATTRIBUTES_ALTERA => {
                Display::fmt(stringify!(FPGAMEMORY_ATTRIBUTES_ALTERA), f)
            }
            Self::FPFAST_MATH_MODE_INTEL => Display::fmt(stringify!(FPFAST_MATH_MODE_INTEL), f),
            Self::ARBITRARY_PRECISION_INTEGERS_ALTERA => {
                Display::fmt(stringify!(ARBITRARY_PRECISION_INTEGERS_ALTERA), f)
            }
            Self::ARBITRARY_PRECISION_FLOATING_POINT_ALTERA => {
                Display::fmt(stringify!(ARBITRARY_PRECISION_FLOATING_POINT_ALTERA), f)
            }
            Self::UNSTRUCTURED_LOOP_CONTROLS_INTEL => {
                Display::fmt(stringify!(UNSTRUCTURED_LOOP_CONTROLS_INTEL), f)
            }
            Self::FPGALOOP_CONTROLS_ALTERA => Display::fmt(stringify!(FPGALOOP_CONTROLS_ALTERA), f),
            Self::KERNEL_ATTRIBUTES_INTEL => Display::fmt(stringify!(KERNEL_ATTRIBUTES_INTEL), f),
            Self::FPGAKERNEL_ATTRIBUTES_INTEL => {
                Display::fmt(stringify!(FPGAKERNEL_ATTRIBUTES_INTEL), f)
            }
            Self::FPGAMEMORY_ACCESSES_ALTERA => {
                Display::fmt(stringify!(FPGAMEMORY_ACCESSES_ALTERA), f)
            }
            Self::FPGACLUSTER_ATTRIBUTES_ALTERA => {
                Display::fmt(stringify!(FPGACLUSTER_ATTRIBUTES_ALTERA), f)
            }
            Self::LOOP_FUSE_ALTERA => Display::fmt(stringify!(LOOP_FUSE_ALTERA), f),
            Self::FPGADSPCONTROL_ALTERA => Display::fmt(stringify!(FPGADSPCONTROL_ALTERA), f),
            Self::MEMORY_ACCESS_ALIASING_INTEL => {
                Display::fmt(stringify!(MEMORY_ACCESS_ALIASING_INTEL), f)
            }
            Self::FPGAINVOCATION_PIPELINING_ATTRIBUTES_ALTERA => {
                Display::fmt(stringify!(FPGAINVOCATION_PIPELINING_ATTRIBUTES_ALTERA), f)
            }
            Self::FPGABUFFER_LOCATION_ALTERA => {
                Display::fmt(stringify!(FPGABUFFER_LOCATION_ALTERA), f)
            }
            Self::ARBITRARY_PRECISION_FIXED_POINT_ALTERA => {
                Display::fmt(stringify!(ARBITRARY_PRECISION_FIXED_POINT_ALTERA), f)
            }
            Self::USMSTORAGE_CLASSES_ALTERA => {
                Display::fmt(stringify!(USMSTORAGE_CLASSES_ALTERA), f)
            }
            Self::RUNTIME_ALIGNED_ATTRIBUTE_ALTERA => {
                Display::fmt(stringify!(RUNTIME_ALIGNED_ATTRIBUTE_ALTERA), f)
            }
            Self::IOPIPES_ALTERA => Display::fmt(stringify!(IOPIPES_ALTERA), f),
            Self::BLOCKING_PIPES_ALTERA => Display::fmt(stringify!(BLOCKING_PIPES_ALTERA), f),
            Self::FPGAREG_ALTERA => Display::fmt(stringify!(FPGAREG_ALTERA), f),
            Self::DOT_PRODUCT_INPUT_ALL => Display::fmt(stringify!(DOT_PRODUCT_INPUT_ALL), f),
            Self::DOT_PRODUCT_INPUT_4X_8_BIT => {
                Display::fmt(stringify!(DOT_PRODUCT_INPUT_4X_8_BIT), f)
            }
            Self::DOT_PRODUCT_INPUT_4X_8_BIT_PACKED => {
                Display::fmt(stringify!(DOT_PRODUCT_INPUT_4X_8_BIT_PACKED), f)
            }
            Self::DOT_PRODUCT => Display::fmt(stringify!(DOT_PRODUCT), f),
            Self::RAY_CULL_MASK_KHR => Display::fmt(stringify!(RAY_CULL_MASK_KHR), f),
            Self::COOPERATIVE_MATRIX_KHR => Display::fmt(stringify!(COOPERATIVE_MATRIX_KHR), f),
            Self::REPLICATED_COMPOSITES_EXT => {
                Display::fmt(stringify!(REPLICATED_COMPOSITES_EXT), f)
            }
            Self::BIT_INSTRUCTIONS => Display::fmt(stringify!(BIT_INSTRUCTIONS), f),
            Self::GROUP_NON_UNIFORM_ROTATE_KHR => {
                Display::fmt(stringify!(GROUP_NON_UNIFORM_ROTATE_KHR), f)
            }
            Self::FLOAT_CONTROLS_2 => Display::fmt(stringify!(FLOAT_CONTROLS_2), f),
            Self::FMAKHR => Display::fmt(stringify!(FMAKHR), f),
            Self::ATOMIC_FLOAT_32_ADD_EXT => Display::fmt(stringify!(ATOMIC_FLOAT_32_ADD_EXT), f),
            Self::ATOMIC_FLOAT_64_ADD_EXT => Display::fmt(stringify!(ATOMIC_FLOAT_64_ADD_EXT), f),
            Self::LONG_COMPOSITES_INTEL => Display::fmt(stringify!(LONG_COMPOSITES_INTEL), f),
            Self::OPT_NONE_EXT => Display::fmt(stringify!(OPT_NONE_EXT), f),
            Self::ATOMIC_FLOAT_16_ADD_EXT => Display::fmt(stringify!(ATOMIC_FLOAT_16_ADD_EXT), f),
            Self::DEBUG_INFO_MODULE_INTEL => Display::fmt(stringify!(DEBUG_INFO_MODULE_INTEL), f),
            Self::BFLOAT_16_CONVERSION_INTEL => {
                Display::fmt(stringify!(BFLOAT_16_CONVERSION_INTEL), f)
            }
            Self::SPLIT_BARRIER_INTEL => Display::fmt(stringify!(SPLIT_BARRIER_INTEL), f),
            Self::ARITHMETIC_FENCE_EXT => Display::fmt(stringify!(ARITHMETIC_FENCE_EXT), f),
            Self::FPGACLUSTER_ATTRIBUTES_V2ALTERA => {
                Display::fmt(stringify!(FPGACLUSTER_ATTRIBUTES_V2ALTERA), f)
            }
            Self::FPGAKERNEL_ATTRIBUTESV_2_INTEL => {
                Display::fmt(stringify!(FPGAKERNEL_ATTRIBUTESV_2_INTEL), f)
            }
            Self::TASK_SEQUENCE_ALTERA => Display::fmt(stringify!(TASK_SEQUENCE_ALTERA), f),
            Self::FPMAX_ERROR_INTEL => Display::fmt(stringify!(FPMAX_ERROR_INTEL), f),
            Self::FPGALATENCY_CONTROL_ALTERA => {
                Display::fmt(stringify!(FPGALATENCY_CONTROL_ALTERA), f)
            }
            Self::FPGAARGUMENT_INTERFACES_ALTERA => {
                Display::fmt(stringify!(FPGAARGUMENT_INTERFACES_ALTERA), f)
            }
            Self::GLOBAL_VARIABLE_HOST_ACCESS_INTEL => {
                Display::fmt(stringify!(GLOBAL_VARIABLE_HOST_ACCESS_INTEL), f)
            }
            Self::GLOBAL_VARIABLE_FPGADECORATIONS_ALTERA => {
                Display::fmt(stringify!(GLOBAL_VARIABLE_FPGADECORATIONS_ALTERA), f)
            }
            Self::SUBGROUP_BUFFER_PREFETCH_INTEL => {
                Display::fmt(stringify!(SUBGROUP_BUFFER_PREFETCH_INTEL), f)
            }
            Self::SUBGROUP_2D_BLOCK_IOINTEL => {
                Display::fmt(stringify!(SUBGROUP_2D_BLOCK_IOINTEL), f)
            }
            Self::SUBGROUP_2D_BLOCK_TRANSFORM_INTEL => {
                Display::fmt(stringify!(SUBGROUP_2D_BLOCK_TRANSFORM_INTEL), f)
            }
            Self::SUBGROUP_2D_BLOCK_TRANSPOSE_INTEL => {
                Display::fmt(stringify!(SUBGROUP_2D_BLOCK_TRANSPOSE_INTEL), f)
            }
            Self::SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE_INTEL => {
                Display::fmt(stringify!(SUBGROUP_MATRIX_MULTIPLY_ACCUMULATE_INTEL), f)
            }
            Self::TERNARY_BITWISE_FUNCTION_INTEL => {
                Display::fmt(stringify!(TERNARY_BITWISE_FUNCTION_INTEL), f)
            }
            Self::UNTYPED_VARIABLE_LENGTH_ARRAY_INTEL => {
                Display::fmt(stringify!(UNTYPED_VARIABLE_LENGTH_ARRAY_INTEL), f)
            }
            Self::SPEC_CONDITIONAL_INTEL => Display::fmt(stringify!(SPEC_CONDITIONAL_INTEL), f),
            Self::FUNCTION_VARIANTS_INTEL => Display::fmt(stringify!(FUNCTION_VARIANTS_INTEL), f),
            Self::GROUP_UNIFORM_ARITHMETIC_KHR => {
                Display::fmt(stringify!(GROUP_UNIFORM_ARITHMETIC_KHR), f)
            }
            Self::TENSOR_FLOAT_32_ROUNDING_INTEL => {
                Display::fmt(stringify!(TENSOR_FLOAT_32_ROUNDING_INTEL), f)
            }
            Self::MASKED_GATHER_SCATTER_INTEL => {
                Display::fmt(stringify!(MASKED_GATHER_SCATTER_INTEL), f)
            }
            Self::CACHE_CONTROLS_INTEL => Display::fmt(stringify!(CACHE_CONTROLS_INTEL), f),
            Self::REGISTER_LIMITS_INTEL => Display::fmt(stringify!(REGISTER_LIMITS_INTEL), f),
            Self::BINDLESS_IMAGES_INTEL => Display::fmt(stringify!(BINDLESS_IMAGES_INTEL), f),
            Self::DOT_PRODUCT_FLOAT_16_ACC_FLOAT_32_VALVE => {
                Display::fmt(stringify!(DOT_PRODUCT_FLOAT_16_ACC_FLOAT_32_VALVE), f)
            }
            Self::DOT_PRODUCT_FLOAT_16_ACC_FLOAT_16_VALVE => {
                Display::fmt(stringify!(DOT_PRODUCT_FLOAT_16_ACC_FLOAT_16_VALVE), f)
            }
            Self::DOT_PRODUCT_BFLOAT_16_ACC_VALVE => {
                Display::fmt(stringify!(DOT_PRODUCT_BFLOAT_16_ACC_VALVE), f)
            }
            Self::DOT_PRODUCT_FLOAT_8_ACC_FLOAT_32_VALVE => {
                Display::fmt(stringify!(DOT_PRODUCT_FLOAT_8_ACC_FLOAT_32_VALVE), f)
            }
            x => write!(f, "{}({})", stringify!(Capability), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RayQueryIntersection(pub(crate) u32);
impl RayQueryIntersection {
    pub const RAY_QUERY_CANDIDATE_INTERSECTION_KHR: Self = Self(0u32);
    pub const RAY_QUERY_COMMITTED_INTERSECTION_KHR: Self = Self(1u32);
}
impl Word for RayQueryIntersection {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for RayQueryIntersection {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RAY_QUERY_CANDIDATE_INTERSECTION_KHR => {
                Display::fmt(stringify!(RAY_QUERY_CANDIDATE_INTERSECTION_KHR), f)
            }
            Self::RAY_QUERY_COMMITTED_INTERSECTION_KHR => {
                Display::fmt(stringify!(RAY_QUERY_COMMITTED_INTERSECTION_KHR), f)
            }
            x => write!(f, "{}({})", stringify!(RayQueryIntersection), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RayQueryCommittedIntersectionType(pub(crate) u32);
impl RayQueryCommittedIntersectionType {
    pub const RAY_QUERY_COMMITTED_INTERSECTION_NONE_KHR: Self = Self(0u32);
    pub const RAY_QUERY_COMMITTED_INTERSECTION_TRIANGLE_KHR: Self = Self(1u32);
    pub const RAY_QUERY_COMMITTED_INTERSECTION_GENERATED_KHR: Self = Self(2u32);
}
impl Word for RayQueryCommittedIntersectionType {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for RayQueryCommittedIntersectionType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RAY_QUERY_COMMITTED_INTERSECTION_NONE_KHR => {
                Display::fmt(stringify!(RAY_QUERY_COMMITTED_INTERSECTION_NONE_KHR), f)
            }
            Self::RAY_QUERY_COMMITTED_INTERSECTION_TRIANGLE_KHR => {
                Display::fmt(stringify!(RAY_QUERY_COMMITTED_INTERSECTION_TRIANGLE_KHR), f)
            }
            Self::RAY_QUERY_COMMITTED_INTERSECTION_GENERATED_KHR => Display::fmt(
                stringify!(RAY_QUERY_COMMITTED_INTERSECTION_GENERATED_KHR),
                f,
            ),
            x => write!(
                f,
                "{}({})",
                stringify!(RayQueryCommittedIntersectionType),
                x.0
            ),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RayQueryCandidateIntersectionType(pub(crate) u32);
impl RayQueryCandidateIntersectionType {
    pub const RAY_QUERY_CANDIDATE_INTERSECTION_TRIANGLE_KHR: Self = Self(0u32);
    pub const RAY_QUERY_CANDIDATE_INTERSECTION_AABBKHR: Self = Self(1u32);
}
impl Word for RayQueryCandidateIntersectionType {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for RayQueryCandidateIntersectionType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RAY_QUERY_CANDIDATE_INTERSECTION_TRIANGLE_KHR => {
                Display::fmt(stringify!(RAY_QUERY_CANDIDATE_INTERSECTION_TRIANGLE_KHR), f)
            }
            Self::RAY_QUERY_CANDIDATE_INTERSECTION_AABBKHR => {
                Display::fmt(stringify!(RAY_QUERY_CANDIDATE_INTERSECTION_AABBKHR), f)
            }
            x => write!(
                f,
                "{}({})",
                stringify!(RayQueryCandidateIntersectionType),
                x.0
            ),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PackedVectorFormat(pub(crate) u32);
impl PackedVectorFormat {
    pub const PACKED_VECTOR_FORMAT_4X_8_BIT: Self = Self(0u32);
}
impl Word for PackedVectorFormat {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for PackedVectorFormat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::PACKED_VECTOR_FORMAT_4X_8_BIT => {
                Display::fmt(stringify!(PACKED_VECTOR_FORMAT_4X_8_BIT), f)
            }
            x => write!(f, "{}({})", stringify!(PackedVectorFormat), x.0),
        }
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct CooperativeMatrixOperands : u32 { const NONE_KHR = 0x0000 ; const MATRIX_ASIGNED_COMPONENTS_KHR = 0x0001 ; const MATRIX_BSIGNED_COMPONENTS_KHR = 0x0002 ; const MATRIX_CSIGNED_COMPONENTS_KHR = 0x0004 ; const MATRIX_RESULT_SIGNED_COMPONENTS_KHR = 0x0008 ; const SATURATING_ACCUMULATION_KHR = 0x0010 ; } }
impl Word for CooperativeMatrixOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CooperativeMatrixLayout(pub(crate) u32);
impl CooperativeMatrixLayout {
    pub const ROW_MAJOR_KHR: Self = Self(0u32);
    pub const COLUMN_MAJOR_KHR: Self = Self(1u32);
    pub const ROW_BLOCKED_INTERLEAVED_ARM: Self = Self(4202u32);
    pub const COLUMN_BLOCKED_INTERLEAVED_ARM: Self = Self(4203u32);
}
impl Word for CooperativeMatrixLayout {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for CooperativeMatrixLayout {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ROW_MAJOR_KHR => Display::fmt(stringify!(ROW_MAJOR_KHR), f),
            Self::COLUMN_MAJOR_KHR => Display::fmt(stringify!(COLUMN_MAJOR_KHR), f),
            Self::ROW_BLOCKED_INTERLEAVED_ARM => {
                Display::fmt(stringify!(ROW_BLOCKED_INTERLEAVED_ARM), f)
            }
            Self::COLUMN_BLOCKED_INTERLEAVED_ARM => {
                Display::fmt(stringify!(COLUMN_BLOCKED_INTERLEAVED_ARM), f)
            }
            x => write!(f, "{}({})", stringify!(CooperativeMatrixLayout), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CooperativeMatrixUse(pub(crate) u32);
impl CooperativeMatrixUse {
    pub const MATRIX_AKHR: Self = Self(0u32);
    pub const MATRIX_BKHR: Self = Self(1u32);
    pub const MATRIX_ACCUMULATOR_KHR: Self = Self(2u32);
}
impl Word for CooperativeMatrixUse {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for CooperativeMatrixUse {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MATRIX_AKHR => Display::fmt(stringify!(MATRIX_AKHR), f),
            Self::MATRIX_BKHR => Display::fmt(stringify!(MATRIX_BKHR), f),
            Self::MATRIX_ACCUMULATOR_KHR => Display::fmt(stringify!(MATRIX_ACCUMULATOR_KHR), f),
            x => write!(f, "{}({})", stringify!(CooperativeMatrixUse), x.0),
        }
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct CooperativeMatrixReduce : u32 { const ROW = 0x0001 ; const COLUMN = 0x0002 ; const TYPE_2X_2 = 0x0004 ; } }
impl Word for CooperativeMatrixReduce {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TensorClampMode(pub(crate) u32);
impl TensorClampMode {
    pub const UNDEFINED: Self = Self(0u32);
    pub const CONSTANT: Self = Self(1u32);
    pub const CLAMP_TO_EDGE: Self = Self(2u32);
    pub const REPEAT: Self = Self(3u32);
    pub const REPEAT_MIRRORED: Self = Self(4u32);
}
impl Word for TensorClampMode {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for TensorClampMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNDEFINED => Display::fmt(stringify!(UNDEFINED), f),
            Self::CONSTANT => Display::fmt(stringify!(CONSTANT), f),
            Self::CLAMP_TO_EDGE => Display::fmt(stringify!(CLAMP_TO_EDGE), f),
            Self::REPEAT => Display::fmt(stringify!(REPEAT), f),
            Self::REPEAT_MIRRORED => Display::fmt(stringify!(REPEAT_MIRRORED), f),
            x => write!(f, "{}({})", stringify!(TensorClampMode), x.0),
        }
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct TensorAddressingOperands : u32 { const NONE = 0x0000 ; const TENSOR_VIEW = 0x0001 ; const DECODE_FUNC = 0x0002 ; } }
impl Word for TensorAddressingOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InitializationModeQualifier(pub(crate) u32);
impl InitializationModeQualifier {
    pub const INIT_ON_DEVICE_REPROGRAM_ALTERA: Self = Self(0u32);
    pub const INIT_ON_DEVICE_RESET_ALTERA: Self = Self(1u32);
}
impl Word for InitializationModeQualifier {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for InitializationModeQualifier {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INIT_ON_DEVICE_REPROGRAM_ALTERA => {
                Display::fmt(stringify!(INIT_ON_DEVICE_REPROGRAM_ALTERA), f)
            }
            Self::INIT_ON_DEVICE_RESET_ALTERA => {
                Display::fmt(stringify!(INIT_ON_DEVICE_RESET_ALTERA), f)
            }
            x => write!(f, "{}({})", stringify!(InitializationModeQualifier), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadCacheControl(pub(crate) u32);
impl LoadCacheControl {
    pub const UNCACHED_INTEL: Self = Self(0u32);
    pub const CACHED_INTEL: Self = Self(1u32);
    pub const STREAMING_INTEL: Self = Self(2u32);
    pub const INVALIDATE_AFTER_READ_INTEL: Self = Self(3u32);
    pub const CONST_CACHED_INTEL: Self = Self(4u32);
}
impl Word for LoadCacheControl {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for LoadCacheControl {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNCACHED_INTEL => Display::fmt(stringify!(UNCACHED_INTEL), f),
            Self::CACHED_INTEL => Display::fmt(stringify!(CACHED_INTEL), f),
            Self::STREAMING_INTEL => Display::fmt(stringify!(STREAMING_INTEL), f),
            Self::INVALIDATE_AFTER_READ_INTEL => {
                Display::fmt(stringify!(INVALIDATE_AFTER_READ_INTEL), f)
            }
            Self::CONST_CACHED_INTEL => Display::fmt(stringify!(CONST_CACHED_INTEL), f),
            x => write!(f, "{}({})", stringify!(LoadCacheControl), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreCacheControl(pub(crate) u32);
impl StoreCacheControl {
    pub const UNCACHED_INTEL: Self = Self(0u32);
    pub const WRITE_THROUGH_INTEL: Self = Self(1u32);
    pub const WRITE_BACK_INTEL: Self = Self(2u32);
    pub const STREAMING_INTEL: Self = Self(3u32);
}
impl Word for StoreCacheControl {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for StoreCacheControl {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNCACHED_INTEL => Display::fmt(stringify!(UNCACHED_INTEL), f),
            Self::WRITE_THROUGH_INTEL => Display::fmt(stringify!(WRITE_THROUGH_INTEL), f),
            Self::WRITE_BACK_INTEL => Display::fmt(stringify!(WRITE_BACK_INTEL), f),
            Self::STREAMING_INTEL => Display::fmt(stringify!(STREAMING_INTEL), f),
            x => write!(f, "{}({})", stringify!(StoreCacheControl), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedMaximumNumberOfRegisters(pub(crate) u32);
impl NamedMaximumNumberOfRegisters {
    pub const AUTO_INTEL: Self = Self(0u32);
}
impl Word for NamedMaximumNumberOfRegisters {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for NamedMaximumNumberOfRegisters {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AUTO_INTEL => Display::fmt(stringify!(AUTO_INTEL), f),
            x => write!(f, "{}({})", stringify!(NamedMaximumNumberOfRegisters), x.0),
        }
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct MatrixMultiplyAccumulateOperands : u32 { const NONE = 0x0 ; const MATRIX_ASIGNED_COMPONENTS_INTEL = 0x1 ; const MATRIX_BSIGNED_COMPONENTS_INTEL = 0x2 ; const MATRIX_CBFLOAT_16_INTEL = 0x4 ; const MATRIX_RESULT_BFLOAT_16_INTEL = 0x8 ; const MATRIX_APACKED_INT_8_INTEL = 0x10 ; const MATRIX_BPACKED_INT_8_INTEL = 0x20 ; const MATRIX_APACKED_INT_4_INTEL = 0x40 ; const MATRIX_BPACKED_INT_4_INTEL = 0x80 ; const MATRIX_ATF32INTEL = 0x100 ; const MATRIX_BTF32INTEL = 0x200 ; const MATRIX_APACKED_FLOAT_16_INTEL = 0x400 ; const MATRIX_BPACKED_FLOAT_16_INTEL = 0x800 ; const MATRIX_APACKED_BFLOAT_16_INTEL = 0x1000 ; const MATRIX_BPACKED_BFLOAT_16_INTEL = 0x2000 ; } }
impl Word for MatrixMultiplyAccumulateOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FPEncoding(pub(crate) u32);
impl FPEncoding {
    pub const BFLOAT_16_KHR: Self = Self(0u32);
    pub const FLOAT_8_E4M3EXT: Self = Self(4214u32);
    pub const FLOAT_8_E5M2EXT: Self = Self(4215u32);
}
impl Word for FPEncoding {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for FPEncoding {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BFLOAT_16_KHR => Display::fmt(stringify!(BFLOAT_16_KHR), f),
            Self::FLOAT_8_E4M3EXT => Display::fmt(stringify!(FLOAT_8_E4M3EXT), f),
            Self::FLOAT_8_E5M2EXT => Display::fmt(stringify!(FLOAT_8_E5M2EXT), f),
            x => write!(f, "{}({})", stringify!(FPEncoding), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CooperativeVectorMatrixLayout(pub(crate) u32);
impl CooperativeVectorMatrixLayout {
    pub const ROW_MAJOR_NV: Self = Self(0u32);
    pub const COLUMN_MAJOR_NV: Self = Self(1u32);
    pub const INFERENCING_OPTIMAL_NV: Self = Self(2u32);
    pub const TRAINING_OPTIMAL_NV: Self = Self(3u32);
}
impl Word for CooperativeVectorMatrixLayout {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for CooperativeVectorMatrixLayout {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ROW_MAJOR_NV => Display::fmt(stringify!(ROW_MAJOR_NV), f),
            Self::COLUMN_MAJOR_NV => Display::fmt(stringify!(COLUMN_MAJOR_NV), f),
            Self::INFERENCING_OPTIMAL_NV => Display::fmt(stringify!(INFERENCING_OPTIMAL_NV), f),
            Self::TRAINING_OPTIMAL_NV => Display::fmt(stringify!(TRAINING_OPTIMAL_NV), f),
            x => write!(f, "{}({})", stringify!(CooperativeVectorMatrixLayout), x.0),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentType(pub(crate) u32);
impl ComponentType {
    pub const FLOAT_16_NV: Self = Self(0u32);
    pub const FLOAT_32_NV: Self = Self(1u32);
    pub const FLOAT_64_NV: Self = Self(2u32);
    pub const SIGNED_INT_8_NV: Self = Self(3u32);
    pub const SIGNED_INT_16_NV: Self = Self(4u32);
    pub const SIGNED_INT_32_NV: Self = Self(5u32);
    pub const SIGNED_INT_64_NV: Self = Self(6u32);
    pub const UNSIGNED_INT_8_NV: Self = Self(7u32);
    pub const UNSIGNED_INT_16_NV: Self = Self(8u32);
    pub const UNSIGNED_INT_32_NV: Self = Self(9u32);
    pub const UNSIGNED_INT_64_NV: Self = Self(10u32);
    pub const SIGNED_INT_8_PACKED_NV: Self = Self(1000491000u32);
    pub const UNSIGNED_INT_8_PACKED_NV: Self = Self(1000491001u32);
    pub const FLOAT_E4M3NV: Self = Self(1000491002u32);
    pub const FLOAT_E5M2NV: Self = Self(1000491003u32);
}
impl Word for ComponentType {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self(word)
    }
}
impl Debug for ComponentType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FLOAT_16_NV => Display::fmt(stringify!(FLOAT_16_NV), f),
            Self::FLOAT_32_NV => Display::fmt(stringify!(FLOAT_32_NV), f),
            Self::FLOAT_64_NV => Display::fmt(stringify!(FLOAT_64_NV), f),
            Self::SIGNED_INT_8_NV => Display::fmt(stringify!(SIGNED_INT_8_NV), f),
            Self::SIGNED_INT_16_NV => Display::fmt(stringify!(SIGNED_INT_16_NV), f),
            Self::SIGNED_INT_32_NV => Display::fmt(stringify!(SIGNED_INT_32_NV), f),
            Self::SIGNED_INT_64_NV => Display::fmt(stringify!(SIGNED_INT_64_NV), f),
            Self::UNSIGNED_INT_8_NV => Display::fmt(stringify!(UNSIGNED_INT_8_NV), f),
            Self::UNSIGNED_INT_16_NV => Display::fmt(stringify!(UNSIGNED_INT_16_NV), f),
            Self::UNSIGNED_INT_32_NV => Display::fmt(stringify!(UNSIGNED_INT_32_NV), f),
            Self::UNSIGNED_INT_64_NV => Display::fmt(stringify!(UNSIGNED_INT_64_NV), f),
            Self::SIGNED_INT_8_PACKED_NV => Display::fmt(stringify!(SIGNED_INT_8_PACKED_NV), f),
            Self::UNSIGNED_INT_8_PACKED_NV => Display::fmt(stringify!(UNSIGNED_INT_8_PACKED_NV), f),
            Self::FLOAT_E4M3NV => Display::fmt(stringify!(FLOAT_E4M3NV), f),
            Self::FLOAT_E5M2NV => Display::fmt(stringify!(FLOAT_E5M2NV), f),
            x => write!(f, "{}({})", stringify!(ComponentType), x.0),
        }
    }
}
bitflags! { # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct TensorOperands : u32 { const NONE_ARM = 0x0000 ; const NONTEMPORAL_ARM = 0x0001 ; const OUT_OF_BOUNDS_VALUE_ARM = 0x0002 ; const MAKE_ELEMENT_AVAILABLE_ARM = 0x0004 ; const MAKE_ELEMENT_VISIBLE_ARM = 0x0008 ; const NON_PRIVATE_ELEMENT_ARM = 0x0010 ; } }
impl Word for TensorOperands {
    #[inline]
    fn from_native(word: u32) -> Self {
        Self::from_bits_retain(word)
    }
}
