use crate::rendering::framebuffer::Framebuffer;

/// Fast non-cryptographic FNV-1a hash of the entire framebuffer pixel buffer.
///
/// Deterministic for identical pixel content. Useful for visual regression:
/// if the rendered output changes, the hash changes.
pub fn framebuffer_hash(fb: &Framebuffer) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
    for &byte in fb.pixels.iter() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3); // FNV prime
    }
    hash
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn identical_framebuffers_produce_same_hash() {
        let fb1 = Framebuffer::new(8, 8);
        let fb2 = Framebuffer::new(8, 8);
        assert_eq!(framebuffer_hash(&fb1), framebuffer_hash(&fb2));
    }

    #[test]
    fn different_pixels_produce_different_hash() {
        let fb1 = Framebuffer::new(8, 8);
        let mut fb2 = Framebuffer::new(8, 8);
        fb2.pixels[0] = 42;
        assert_ne!(framebuffer_hash(&fb1), framebuffer_hash(&fb2));
    }

    #[test]
    fn hash_is_deterministic() {
        let fb = Framebuffer::new(16, 16);
        let h1 = framebuffer_hash(&fb);
        let h2 = framebuffer_hash(&fb);
        assert_eq!(h1, h2);
    }
}
