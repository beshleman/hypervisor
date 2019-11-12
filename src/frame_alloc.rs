use crate::lpae::PAGE_SIZE;

const MEMORY_START: u64 = 0x40000000;
const MEMORY_SIZE: u64 =   0x8000000;
const MEMORY_END: u64 =   MEMORY_START + MEMORY_SIZE;

pub struct FrameAllocator {
    bottom: u64,
}

impl FrameAllocator {
    pub fn new(bottom: u64) -> FrameAllocator {
        FrameAllocator { bottom: bottom }
    }

    pub fn alloc_frame(&mut self) -> u64 {
        let tmp = self.bottom;
        self.bottom += PAGE_SIZE as u64;
        
        tmp
    }
}
