////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Imports                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Constants                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

const PB: u64 = 1 << 50;
const TB: u64 = 1 << 40;
const GB: u64 = 1 << 30;
const MB: u64 = 1 << 20;
const KB: u64 = 1 << 10;

const PB_F: f64 = PB as f64;
const TB_F: f64 = TB as f64;
const GB_F: f64 = GB as f64;
const MB_F: f64 = MB as f64;
const KB_F: f64 = KB as f64;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Main Struct                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The size of a file, consisting of counts for data units.
#[derive(Debug, Clone)]
pub struct FileSize {
    /// Pebibytes (2<sup>50</sup> bytes) component of the file size
    pub pb: u16,

    /// Tebibytes (2<sup>40</sup> bytes) component of the file size
    pub tb: u16,

    /// Gibibytes (2<sup>30</sup> bytes) component of the file size
    pub gb: u16,

    /// Mebibytes (2<sup>20</sup> bytes) component of the file size
    pub mb: u16,

    /// Kibibytes (2<sup>10</sup> bytes) component of the file size
    pub kb: u16,

    /// Bytes component of the file size
    pub b: u16,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                      Main Implementation                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

impl FileSize {
    /// Returns a new `FileSize` with representing given number of bytes
    ///
    /// This produces a `FileSize` with optimal values for each count
    /// field. In other words, the largest possible units are filled 
    /// first, rather than just setting the bytes field to the input.
    ///
    /// # Arguments
    ///
    /// * `total_bytes` - The number of bytes the new `FileSize` should represent
    ///
    /// # Examples
    ///
    /// ```
    /// use zeus::zeuslib::utils::fs::FileSize;
    ///
    /// const b: u64 = 1048726;
    ///
    /// let fs = FileSize::from_total_bytes(b);
    ///
    /// assert_eq!(fs.pb, 0);
    /// assert_eq!(fs.tb, 0);
    /// assert_eq!(fs.gb, 0);
    /// assert_eq!(fs.mb, 1);
    /// assert_eq!(fs.kb, 0);
    /// assert_eq!(fs.b, 150);
    /// ```
    ///
    pub fn from_total_bytes(total_bytes: u64) -> Self {
        let b = total_bytes;

        let pb = b / PB;
        let b = b - pb * PB;

        let tb = b / TB;
        let b = b - tb * TB;
        let gb = b / GB;
        let b = b - gb * GB;

        let mb = b / MB;
        let b = b - mb * MB;
        let kb = b / KB;
        let b = b - kb * KB;

        Self {
            pb: pb as u16,
            tb: tb as u16,
            gb: gb as u16,
            mb: mb as u16,
            kb: kb as u16,
            b: b as u16,
        }
    }

    /// Get the total number of bytes in this `FileSize`
    ///
    /// # Examples
    ///
    /// ```
    /// use zeus::zeuslib::utils::fs::FileSize;
    ///
    /// const b: u64 = 1048576;
    /// let fsize = FileSize::from_total_bytes(b); // Create a `FileSize`
    /// let total_bytes = fsize.get_total_bytes(); // Get the total number of bytes
    ///
    /// assert_eq!(total_bytes, b);
    /// ```
    pub fn get_total_bytes(&self) -> u64 {
        let pb: u64 = (self.pb as u64) * PB;
        let tb: u64 = (self.tb as u64) * TB;
        let gb: u64 = (self.gb as u64) * GB;
        let mb: u64 = (self.mb as u64) * MB;
        let kb: u64 = (self.kb as u64) * KB;
        let b: u64 = self.b as u64;
        b + kb + mb + gb + tb + pb
    }

    /// Get the total number of Pebibytes (2<sup>50</sup> bytes) this `FileSize` represents as a
    /// floating-point number.
    pub fn get_fractional_pb(&self) -> f64 {
        (self.get_total_bytes() as f64) / PB_F
    }
    
    /// Get the total number of Tebibytes (2<sup>40</sup> bytes) this `FileSize` represents as a
    /// floating-point number.
    pub fn get_fractional_tb(&self) -> f64 {
        (self.get_total_bytes() as f64) / TB_F
    }
    
    /// Get the total number of Gibibytes (2<sup>30</sup> bytes) this `FileSize` represents as a
    /// floating-point number.
    pub fn get_fractional_gb(&self) -> f64 {
        (self.get_total_bytes() as f64) / GB_F
    }
    
    /// Get the total number of Mebibytes (2<sup>20</sup> bytes) this `FileSize` represents as a
    /// floating-point number.
    pub fn get_fractional_mb(&self) -> f64 {
        (self.get_total_bytes() as f64) / MB_F
    }
    
    /// Get the total number of Kibibytes (2<sup>10</sup> bytes) this `FileSize` represents as a
    /// floating-point number.
    pub fn get_fractional_kb(&self) -> f64 {
        (self.get_total_bytes() as f64) / KB_F
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Traits                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

// Allow `FileSize` to be sorted
impl PartialOrd for FileSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Allow `FileSize` to be compared
impl Ord for FileSize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_total_bytes().cmp(&other.get_total_bytes())
    }
}

// Allow `FileSize` to be equality-tested
impl PartialEq for FileSize {
    fn eq(&self, other: &Self) -> bool {
        self.get_total_bytes() == other.get_total_bytes()
    }
}

// Strict equality
impl Eq for FileSize {}

// Allow `FileSize` to be used in string formatting
impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: (f64, &str) = if self.pb > 0 {
            (self.get_fractional_pb(), "PiB")
        } else if self.tb > 0 {
            (self.get_fractional_tb(), "TiB")
        } else if self.gb > 0 {
            (self.get_fractional_gb(), "GiB")
        } else if self.mb > 0 {
            (self.get_fractional_mb(), "MiB")
        } else if self.kb > 0 {
            (self.get_fractional_kb(), "KiB")
        } else {
            return write!(f, "{}B", self.get_total_bytes());
        };
        write!(f, "{:.2}{}", data.0, data.1)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Tests                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filesize_frombytes_1048576() {
        let fs = FileSize::from_total_bytes(1048576);
        assert_eq!(fs.pb, 0);
        assert_eq!(fs.tb, 0);
        assert_eq!(fs.gb, 0);
        assert_eq!(fs.mb, 1);
        assert_eq!(fs.kb, 0);
        assert_eq!(fs.b, 0);
    }
    #[test]
    fn filesize_frombytes_1048726() {
        let fs = FileSize::from_total_bytes(1048726);
        assert_eq!(fs.pb, 0);
        assert_eq!(fs.tb, 0);
        assert_eq!(fs.gb, 0);
        assert_eq!(fs.mb, 1);
        assert_eq!(fs.kb, 0);
        assert_eq!(fs.b, 150);
    }
}
