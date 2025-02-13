use crate::table::crc64_table_slice_16;
use crate::{Algorithm, Crc, Digest};

use super::{finalize, init, update_slice16};

impl Crc<u64> {
    pub const fn new(algorithm: &'static Algorithm<u64>) -> Self {
        let table = crc64_table_slice_16(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u64 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u64, bytes: &[u8]) -> u64 {
        update_slice16(crc, self.algorithm.refin, &self.table, bytes)
    }

    pub const fn digest(&self) -> Digest<u64> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u64) -> Digest<u64> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, u64> {
    const fn new(crc: &'a Crc<u64>, value: u64) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u64 {
        finalize(self.crc.algorithm, self.value)
    }
}
