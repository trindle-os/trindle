//! Keeping Trindle's own programs small in memory.


pub const TRIM_SECS: u32 = 30;

pub fn few_heaps() {
    // SAFETY: a setting of the C allocator, made before anything else runs.
    unsafe { libc::mallopt(libc::M_ARENA_MAX, 2) };
}

pub fn give_back() {
    // SAFETY: asks the allocator to release free memory; nothing of ours is

    unsafe { libc::malloc_trim(0) };
}
