//! Types related to the recompiler's execution context.
//!
//! This module defines the types used to represent the state of a MIPS64 CPU
//! during recompilation, including register sets and floating-point representations.

/// Type alias for General Purpose Registers (64-bit)
pub type Gpr = u64;

/// Representation of a MIPS floating-point register
///
/// This union allows access to the register in different formats:
/// - As a 64-bit floating point value (double)
/// - As a pair of 32-bit floating point values
/// - As a pair of 32-bit unsigned integer values
/// - As a single 64-bit unsigned integer
#[repr(C)]
#[derive(Copy, Clone)]
pub union Fpr {
    /// 64-bit floating point view (double)
    pub d: f64,
    /// Two 32-bit floating point values
    pub f: FprFloat,
    /// Two 32-bit unsigned integers
    pub u: FprUint,
    /// Raw 64-bit unsigned integer view
    pub u64: u64,
}

/// Representation of a floating-point register as two 32-bit floats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FprFloat {
    /// Lower 32-bit float (f32)
    pub fl: f32,
    /// Higher 32-bit float (f32)
    pub fh: f32,
}

/// Representation of a floating-point register as two 32-bit unsigned integers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FprUint {
    /// Lower 32-bit unsigned integer
    pub u32l: u32,
    /// Higher 32-bit unsigned integer
    pub u32h: u32,
}

/// Complete state of a MIPS64 CPU for recompilation
///
/// This structure contains all CPU registers and states needed for
/// accurate emulation and recompilation of MIPS64 code.
#[repr(C)]
pub struct RecompContext {
    // =====================================================
    // General Purpose Registers (GPRs) - 32 x 64-bit
    // =====================================================

    // Zero register (always 0)
    r0: Gpr,

    // Assembler temporary
    r1: Gpr,

    // Function return values
    pub r2: Gpr,
    pub r3: Gpr,

    // Function arguments (a0-a3)
    r4: Gpr,
    r5: Gpr,
    r6: Gpr,
    r7: Gpr,

    // Temporary registers (t0-t7)
    r8: Gpr,
    r9: Gpr,
    r10: Gpr,
    r11: Gpr,
    r12: Gpr,
    r13: Gpr,
    r14: Gpr,
    r15: Gpr,

    // Saved temporary registers (s0-s7)
    r16: Gpr,
    r17: Gpr,
    r18: Gpr,
    r19: Gpr,
    r20: Gpr,
    r21: Gpr,
    r22: Gpr,
    r23: Gpr,

    // Temporary registers (t8-t9)
    r24: Gpr,
    r25: Gpr,

    // Reserved for OS kernel
    r26: Gpr,
    r27: Gpr,

    // Global pointer
    r28: Gpr,

    // Stack pointer
    r29: Gpr,

    // Frame pointer
    r30: Gpr,

    // Return address
    r31: Gpr,

    // =====================================================
    // Floating Point Registers (FPRs) - 32 x 64-bit
    // =====================================================

    // FPRs for function return values and function arguments
    pub f0: Fpr,
    pub f1: Fpr,
    pub f2: Fpr,
    pub f3: Fpr,
    pub f4: Fpr,
    pub f5: Fpr,
    pub f6: Fpr,
    pub f7: Fpr,
    pub f8: Fpr,
    pub f9: Fpr,
    pub f10: Fpr,
    pub f11: Fpr,
    pub f12: Fpr,
    pub f13: Fpr,
    pub f14: Fpr,
    pub f15: Fpr,

    // FPRs for temporary values and saved values
    pub f16: Fpr,
    pub f17: Fpr,
    pub f18: Fpr,
    pub f19: Fpr,
    pub f20: Fpr,
    pub f21: Fpr,
    pub f22: Fpr,
    pub f23: Fpr,
    pub f24: Fpr,
    pub f25: Fpr,
    pub f26: Fpr,
    pub f27: Fpr,
    pub f28: Fpr,
    pub f29: Fpr,
    pub f30: Fpr,
    pub f31: Fpr,

    // =====================================================
    // Special Registers and Control Flags
    // =====================================================
    /// HI register for multiply and divide operations
    pub hi: u64,

    /// LO register for multiply and divide operations
    pub lo: u64,

    /// Pointer to odd-numbered floating point registers (MIPS32 compatibility)
    pub f_odd: *mut u32,

    /// Processor status register
    pub status_reg: u32,

    /// Flag indicating MIPS3 floating-point mode
    pub mips3_float_mode: u8,
}

// Useful register aliases
impl RecompContext {
    // GPR aliases according to MIPS convention

    /// Zero register (always zero)
    #[inline(always)]
    pub fn zero(&self) -> u64 {
        self.r0
    }

    /// Assembler temporary
    #[inline(always)]
    pub fn at(&self) -> u64 {
        self.r1
    }

    /// Function return values
    #[inline(always)]
    pub fn v0(&self) -> u64 {
        self.r2
    }
    #[inline(always)]
    pub fn v1(&self) -> u64 {
        self.r3
    }

    /// Function arguments
    #[inline(always)]
    pub fn a0(&self) -> u64 {
        self.r4
    }
    #[inline(always)]
    pub fn a1(&self) -> u64 {
        self.r5
    }
    #[inline(always)]
    pub fn a2(&self) -> u64 {
        self.r6
    }
    #[inline(always)]
    pub fn a3(&self) -> u64 {
        self.r7
    }

    /// Temporary registers
    #[inline(always)]
    pub fn t0(&self) -> u64 {
        self.r8
    }
    #[inline(always)]
    pub fn t1(&self) -> u64 {
        self.r9
    }
    #[inline(always)]
    pub fn t2(&self) -> u64 {
        self.r10
    }
    #[inline(always)]
    pub fn t3(&self) -> u64 {
        self.r11
    }
    #[inline(always)]
    pub fn t4(&self) -> u64 {
        self.r12
    }
    #[inline(always)]
    pub fn t5(&self) -> u64 {
        self.r13
    }
    #[inline(always)]
    pub fn t6(&self) -> u64 {
        self.r14
    }
    #[inline(always)]
    pub fn t7(&self) -> u64 {
        self.r15
    }

    /// Saved temporary registers
    #[inline(always)]
    pub fn s0(&self) -> u64 {
        self.r16
    }
    #[inline(always)]
    pub fn s1(&self) -> u64 {
        self.r17
    }
    #[inline(always)]
    pub fn s2(&self) -> u64 {
        self.r18
    }
    #[inline(always)]
    pub fn s3(&self) -> u64 {
        self.r19
    }
    #[inline(always)]
    pub fn s4(&self) -> u64 {
        self.r20
    }
    #[inline(always)]
    pub fn s5(&self) -> u64 {
        self.r21
    }
    #[inline(always)]
    pub fn s6(&self) -> u64 {
        self.r22
    }
    #[inline(always)]
    pub fn s7(&self) -> u64 {
        self.r23
    }

    /// Additional temporary registers
    #[inline(always)]
    pub fn t8(&self) -> u64 {
        self.r24
    }
    #[inline(always)]
    pub fn t9(&self) -> u64 {
        self.r25
    }

    /// Kernel registers
    #[inline(always)]
    pub fn k0(&self) -> u64 {
        self.r26
    }
    #[inline(always)]
    pub fn k1(&self) -> u64 {
        self.r27
    }

    /// Global pointer
    #[inline(always)]
    pub fn gp(&self) -> u64 {
        self.r28
    }

    /// Stack pointer
    #[inline(always)]
    pub fn sp(&self) -> u64 {
        self.r29
    }

    /// Frame pointer
    #[inline(always)]
    pub fn fp(&self) -> u64 {
        self.r30
    }

    /// Return address
    #[inline(always)]
    pub fn ra(&self) -> u64 {
        self.r31
    }
}
