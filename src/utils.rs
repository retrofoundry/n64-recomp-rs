use crate::types::RecompContext;

/// Read a byte from memory using the appropriate byte swapping pattern.
/// This is equivalent to the MEM_B macro in C++.
///
/// # Safety
/// This function is unsafe because it accesses raw memory.
#[inline]
pub unsafe fn mem_b(rdram: *mut u8, addr: u64, offset: usize) -> i8 {
    let byte_addr = addr.wrapping_add(offset as u64);
    let rdram_offset = (byte_addr ^ 3).wrapping_sub(0xFFFFFFFF80000000);
    *rdram.add(rdram_offset as usize) as i8
}

impl RecompContext {
    /// Converts a virtual address to a pointer in the RDRAM
    ///
    /// # Safety
    /// This function is unsafe because it returns a raw pointer
    /// which must be properly aligned and within bounds of the RDRAM.
    pub unsafe fn to_ptr<T>(&self, rdram: *mut u8, addr: u64) -> *mut T {
        let offset = addr.wrapping_sub(0xFFFFFFFF80000000);
        rdram.add(offset as usize) as *mut T
    }

    /// Get a u32 argument from registers a0-a3 (r4-r7)
    #[inline]
    pub fn get_arg_u32(&self, index: usize) -> u32 {
        assert!(index < 4, "Only args 0 through 3 supported");
        let reg = match index {
            0 => self.a0(),
            1 => self.a1(),
            2 => self.a2(),
            3 => self.a3(),
            _ => unreachable!(),
        };
        reg as u32
    }

    /// Get a u64 argument from registers a0-a3 (r4-r7)
    #[inline]
    pub fn get_arg_u64(&self, index: usize) -> u64 {
        assert!(index < 4, "Only args 0 through 3 supported");
        match index {
            0 => self.a0(),
            1 => self.a1(),
            2 => self.a2(),
            3 => self.a3(),
            _ => unreachable!(),
        }
    }

    /// Get a float argument (only supported for a0/index 0)
    #[inline]
    pub fn get_arg_f32(&self, index: usize) -> f32 {
        assert!(index == 0, "Floats only supported in arg 0 (a0)");
        unsafe { self.f12.f.fl }
    }

    /// Get a pointer argument, converting virtual address to RDRAM offset
    ///
    /// # Safety
    /// This function is unsafe because it returns a raw pointer
    /// which must be properly aligned and within bounds of the RDRAM.
    #[inline]
    pub unsafe fn get_arg_ptr<T>(&self, rdram: *mut u8, index: usize) -> *mut T {
        let addr = self.get_arg_u64(index);
        self.to_ptr(rdram, addr)
    }

    /// Extract a string argument from the context, with proper byte swapping
    ///
    /// # Safety
    /// This function is unsafe because it accesses raw memory and expects
    /// a valid null-terminated string at the specified address.
    pub unsafe fn get_arg_string(&self, rdram: *mut u8, index: usize) -> String {
        let str_ptr_addr = self.get_arg_u64(index);

        // Get the length of the string
        let mut len = 0;
        loop {
            let byte = mem_b(rdram, str_ptr_addr, len);
            if byte == 0 {
                break;
            }
            len += 1;
        }

        // Create a properly byte-ordered string
        let mut result = String::with_capacity(len);

        for i in 0..len {
            let byte = mem_b(rdram, str_ptr_addr, i);
            result.push(byte as u8 as char);
        }

        result
    }

    /// Set the return value in the appropriate register based on type
    ///
    /// # Type Parameters
    /// * `T`: The type of value to return (must be 32-bit or smaller)
    ///
    /// # Arguments
    /// * `val`: The value to return
    ///
    /// # Examples
    /// ```
    /// ctx.set_return(1); // Set integer return value
    /// ctx.set_return(3.14f32); // Set float return value
    /// ```
    pub fn set_return<T>(&mut self, val: T)
    where
        T: Copy,
    {
        // Use a compile-time check for type and size (Rust equivalent of static_assert)
        let type_name = std::any::type_name::<T>();

        if type_name == "f32" {
            // For float types, set the f0 register
            let float_val = unsafe { std::mem::transmute_copy::<T, f32>(&val) };
            self.f0.f.fl = float_val;
        } else if type_name == "i32"
            || type_name == "u32"
            || type_name == "i16"
            || type_name == "u16"
            || type_name == "i8"
            || type_name == "u8"
            || type_name == "bool"
        {
            // For integer or boolean types, set the r2 register (v0)
            // First cast to i32 and then to u64
            let int_val = unsafe { std::mem::transmute_copy::<T, i32>(&val) };
            self.r2 = int_val as u64;
        } else {
            panic!("Unsupported return type: {}", type_name);
        }
    }
}
