//! This crate provides a simple driver for the
//! [Grove Matrix LED my-9221](https://www.seeedstudio.com/Grove-RGB-LED-Matrix-w-Driver.html)
//!
//! # Example
//!
//! ```rust
//!    use grove_matrix_led_my9221_rs::GroveMatrixLedMy9221;
//!
//!    fn main() {
//!        let mut led_matrix = grove_matrix_led_my9221_rs::My9221LedMatrix::default();
//!
//!        let mut emoji_num = 0;
//!
//!        const DELAY: u16 = 5_000u16;
//!
//!        loop {
//!            led_matrix.display_emoji(&mut i2c, emoji_num, DELAY, true);
//!            delay.delay_ms(DELAY);
//!            emoji_num = (emoji_num + 1) % 32;
//!        }
//!    }
//!
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(destructuring_assignment)]

use embedded_hal::blocking::{
    delay::DelayMs,
    i2c::{Read, Write},
};

mod emojis;

pub use emojis::*;

/// Default I2C Address for the grove matrix LED driver
const DEFAULT_ADDRESS: u8 = 0x65;

/// A struct representing a frame to display
pub struct Frame {
    pub data: [u8; 64],
}

enum I2cCmd {
    /// This command gets device ID information
    GetDevID = 0x00,
    /// This command displays LED bar
    DispBar = 0x01,
    /// This command displays emoji
    DispEmoji = 0x02,
    /// This command displays number
    DispNum = 0x03,
    /// This command displays string
    DispStr = 0x04,
    /// This command displays user-defined pictures
    DispCustom = 0x05,
    /// This command cleans the display
    DispOff = 0x06,
    /// not use
    DispAscii = 0x07,
    /// This command displays pictures which are stored in flash
    DispFlash = 0x08,
    /// This command displays colorful led bar
    DispColorBar = 0x09,
    /// This command displays built-in wave animation
    DispColorWave = 0x0a,
    /// This command displays built-in clockwise animation
    DispColorClockWise = 0x0b,
    /// This command displays other built-in animation
    DispColorAnimation = 0x0c,
    /// This command displays an user-defined color
    DispColorBlock = 0x0d,

    ContinueData = 0x81,

    /// This command stores frames in flash
    StoreFlash = 0xa0,
    /// This command deletes all the frames in flash
    DeleteFlash = 0xa1,

    /// This command turns on the indicator LED flash mode
    LedOn = 0xb0,
    /// This command turns off the indicator LED flash mode
    LedOff = 0xb1,
    /// This command enable device auto sleep mode
    AutoSleepOn = 0xb2,
    /// This command disable device auto sleep mode (default mode)
    AutoSleepOff = 0xb3,

    /// This command setting the display orientation
    DispRotate = 0xb4,
    /// This command setting the display offset
    DispOffset = 0xb5,

    /// This command sets device i2c address
    SetAddress = 0xc0,
    /// This command resets device i2c address
    ResetAddress = 0xc1,
    /// This command enable TX RX pin test mode
    TestTXRXOn = 0xe0,
    /// This command disable TX RX pin test mode
    TestTXRXOff = 0xe1,
    /// This command use to get software version
    TestGetVersion = 0xe2,
    /// This command use to get chip id
    GetDeviceUID = 0xf1,
}

/// An enum representing the possible rotations of the display
#[derive(Debug, Clone, Copy)]
pub enum DisplayRotate {
    /// No rotation
    Deg0 = 0,
    /// Rotate 90 degrees
    Deg90 = 1,
    /// Rotate 180 degrees
    Deg180 = 2,
    /// Rotate 270 degrees
    Deg270 = 3,
}

/// An enum representing the animations available in the module
#[derive(Debug, Clone, Copy)]
pub enum ColorAnimation {
    BigClockWise = 0,
    SmallClockWise = 1,
    RainbowCycle = 2,
    Fire = 3,
    Walking = 4,
    BrokenHeart = 5,
}

/// An enum representing the colors available in the module and their
/// corresponding values
#[derive(Debug, Clone, Copy)]
pub enum Colors {
    /// Red
    Red = 0x00,
    /// Orange
    Orange = 0x12,
    /// Yellow
    Yellow = 0x18,
    /// Green
    Green = 0x52,
    /// Cyan
    Cyan = 0x7f,
    /// Blue
    Blue = 0xaa,
    /// Purple
    Purple = 0xc3,
    /// Pink
    Pink = 0xdc,
    /// White
    White = 0xfe,
    /// Black
    Black = 0xff,
}

/// The grove matrix LED driver
pub struct My9221LedMatrix {
    address: u8,
}

/// The specific errors that can occur when communicating with the device
/// or when using the driver
#[cfg_attr(feature = "std", derive(Debug))]
pub enum My9221LedMatrixError {
    I2CError,
    InvalidArgument,
}

#[cfg(feature = "std")]
impl std::fmt::Display for My9221LedMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            My9221LedMatrixError::I2CError => write!(f, "I2C error"),
            My9221LedMatrixError::InvalidArgument => write!(f, "Invalid argument"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for My9221LedMatrixError {}

impl Default for My9221LedMatrix {
    /// Create a new instance of the grove matrix LED driver using the default
    /// I2C address
    fn default() -> Self {
        Self {
            address: DEFAULT_ADDRESS,
        }
    }
}

/// All the methods available to the user to interact with the device
impl My9221LedMatrix {
    /// Create a new instance of the grove matrix LED driver
    ///
    /// # Arguments
    ///
    /// * `address` - The I2C address to use (default is 0x65)
    ///
    #[cfg(not(feature = "std"))]
    pub fn new(address: u8) -> Self {
        Self { address }
    }

    /// Get the device ID information
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    /// # Returns
    ///
    /// * `Result<u8, My9221LedMatrixError>` - Returns the device ID
    ///
    pub fn get_device_id<I2C, E>(&self, i2c: &mut I2C) -> Result<u8, My9221LedMatrixError>
    where
        I2C: Write + Read,
    {
        let mut buf = [0u8; 1];
        i2c.write(self.address, &[I2cCmd::GetDevID as u8])
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        i2c.read(self.address, &mut buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(buf[0])
    }

    /// Rotate the display
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `rotate` - The display orientation
    ///
    pub fn set_led_matrix_rotate<I2C>(
        &self,
        i2c: &mut I2C,
        rotate: DisplayRotate,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 2];
        buf[0] = I2cCmd::DispRotate as u8;
        buf[1] = rotate as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Stop the display
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn stop_display<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::DispOff as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Set the display offset
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `offset` - The display offset (x, y)
    ///
    pub fn set_led_matrix_offset<I2C>(
        &self,
        i2c: &mut I2C,
        offset: (u8, u8),
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 3];
        buf[0] = I2cCmd::DispOffset as u8;
        buf[1] = offset.0;
        buf[2] = offset.1;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Turn on the display
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn turn_on_led_flash<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::LedOn as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Turn off the display
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn turn_off_led_flash<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::LedOff as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Enable auto sleep mode
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn enable_auto_sleep<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::AutoSleepOn as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Disable auto sleep mode
    ///
    /// # Arguments
    ///     
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn disable_auto_sleep<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::AutoSleepOff as u8;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a bar
    ///
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `bar` - The bar to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    /// * `color` - The color of the bar
    ///
    pub fn display_bar<I2C>(
        &self,
        i2c: &mut I2C,
        bar: u8,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 6];
        buf[0] = I2cCmd::DispBar as u8;
        buf[1] = if bar <= 32 { bar } else { 32 };
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 1 } else { 0 };
        buf[5] = color as u8;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display an Emoji
    ///
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `emoji` - The emoji to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_emoji<I2C>(
        &self,
        i2c: &mut I2C,
        emoji: Emojis,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 5];
        buf[0] = I2cCmd::DispEmoji as u8;
        buf[1] = emoji as u8;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 1 } else { 0 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display an number
    ///
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `number` - The number to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    /// * `color` - The color of the number
    ///
    pub fn display_number<I2C>(
        &self,
        i2c: &mut I2C,
        number: u16,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf = [0; 7];
        buf[0] = I2cCmd::DispNum as u8;
        buf[1] = (number & 0xff) as u8;
        buf[2] = ((number >> 8) & 0xff) as u8;
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff & 0xff) as u8;
        buf[5] = if forever_flag { 1 } else { 0 };
        buf[6] = color as u8;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a string
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `delay` - A delay provider
    /// * `string` - The string to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    /// * `color` - The color of the string
    ///
    pub fn display_string<I2C, D, T>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
        string: &str,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
        D: DelayMs<u32>,
    {
        let mut buf: [u8; 36] = [0; 36];
        let len = if string.len() >= 28 {
            28u8
        } else {
            string.len() as u8
        };

        for i in 0..len {
            buf[(i + 6) as usize] = match string.chars().nth(i as usize) {
                Some(c) => c as u8,
                None => 0,
            };
        }

        buf[0] = I2cCmd::DispStr as u8;
        buf[1] = if forever_flag { 1 } else { 0 };
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = len;
        buf[5] = color as u8;

        if len > 25 {
            i2c.write(self.address, &buf[0..31])
                .map_err(|_| My9221LedMatrixError::I2CError)?;
            delay.delay_ms(1);
            let mut buf2: [u8; 6] = [0; 6];
            buf2[0] = I2cCmd::ContinueData as u8;
            for i in 31..36 {
                buf2[(i - 30) as usize] = buf[i as usize];
            }
            i2c.write(self.address, &buf2)
                .map_err(|_| My9221LedMatrixError::I2CError)?;
        } else {
            i2c.write(self.address, &buf[0..(len + 6) as usize])
                .map_err(|_| My9221LedMatrixError::I2CError)?;
        }
        Ok(())
    }

    /// Display a color block
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `rgb` - The color to display in RGB format (0x00RRGGBB)
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_color_block<I2C>(
        &self,
        i2c: &mut I2C,
        rgb: u32,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 7] = [0; 7];

        buf[0] = I2cCmd::DispColorBlock as u8;
        buf[1] = ((rgb >> 16) & 0xff) as u8;
        buf[2] = ((rgb >> 8) & 0xff) as u8;
        buf[3] = (rgb & 0xff) as u8;
        buf[4] = (duration_time & 0xff) as u8;
        buf[5] = ((duration_time >> 8) & 0xff) as u8;
        buf[6] = if forever_flag { 0 } else { 1 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a color bar
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `bar` - the color bar to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_color_bar<I2C>(
        &self,
        i2c: &mut I2C,
        bar: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 5] = [0; 5];

        buf[0] = I2cCmd::DispColorBar as u8;
        buf[1] = bar;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 0 } else { 1 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a color wave
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `wave` - the color wave to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_color_wave<I2C>(
        &self,
        i2c: &mut I2C,
        wave: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 5] = [0; 5];

        buf[0] = I2cCmd::DispColorWave as u8;
        buf[1] = wave;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 0 } else { 1 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a color clockwise
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `clockwise` - If true, the color will be displayed clockwise, if false, anti-clockwise
    /// * `big` - If true, the color clockwise will be displayed in big size, if false, small size
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_color_clockwise<I2C>(
        &self,
        i2c: &mut I2C,
        clockwise: bool,
        big: bool,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 6] = [0; 6];

        buf[0] = I2cCmd::DispColorClockWise as u8;
        buf[1] = if clockwise { 0 } else { 1 };
        buf[2] = if big { 0 } else { 1 };
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff) as u8;
        buf[5] = if forever_flag { 0 } else { 1 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display a color animation
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `animation` - The animation to display
    ///   - `ColorAnimation::BigClockWise`
    ///   - `ColorAnimation::SmallClockWise`
    ///   - `ColorAnimation::RainbowCycle`
    ///   - `ColorAnimation::Fire`
    ///   - `ColorAnimation::Walking`
    ///   - `ColorAnimation::BrokenHeart`
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    ///
    pub fn display_color_animation<I2C>(
        &self,
        i2c: &mut I2C,
        animation_index: ColorAnimation,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 6] = [0; 6];

        buf[0] = I2cCmd::DispColorAnimation as u8;
        (buf[1], buf[2]) = match animation_index {
            ColorAnimation::BigClockWise => (0, 28),    // big clockwise
            ColorAnimation::SmallClockWise => (29, 41), // small clockwise
            ColorAnimation::RainbowCycle => (255, 255), // rainbow cycle
            ColorAnimation::Fire => (254, 254),         // fire
            ColorAnimation::Walking => (42, 43),        // walking
            ColorAnimation::BrokenHeart => (44, 52),    // broken heart
        };
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff) as u8;
        buf[5] = if forever_flag { 0 } else { 1 };

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Display the frame
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `delay` - A delay provider
    /// * `frames` - The frames to display
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    /// * `frame_number` - The total number of frames
    ///
    pub fn display_frames<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
        frames: &[Frame],
        duration_time: u16,
        forever_flag: bool,
        frames_number: u8,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
        D: DelayMs<u32>,
    {
        let mut buf: [u8; 72] = [0; 72];

        let frames_number = if frames_number > 5 {
            5
        } else if frames_number == 0 {
            return Err(My9221LedMatrixError::InvalidArgument);
        } else {
            frames_number
        };

        buf[0] = I2cCmd::DispCustom as u8;
        buf[1] = 0;
        buf[2] = 0;
        buf[3] = 0;
        buf[4] = frames_number;

        for i in (0..frames_number).rev() {
            buf[5] = i;
            buf[8..(64 + 8)].copy_from_slice(&frames[i as usize].data[..64]);
            if i == 0 {
                buf[1] = (duration_time & 0xff) as u8;
                buf[2] = ((duration_time >> 8) & 0xff) as u8;
                buf[3] = if forever_flag { 0 } else { 1 };
            }
            i2c.write(self.address, &buf[0..24])
                .map_err(|_| My9221LedMatrixError::I2CError)?;
            delay.delay_ms(10);
            let mut buf2: [u8; 25] = [0; 25];
            buf2[0] = I2cCmd::ContinueData as u8;
            buf2[1..(24 + 1)].copy_from_slice(&buf[24..(24 + 24)]);
            i2c.write(self.address, &buf2)
                .map_err(|_| My9221LedMatrixError::I2CError)?;
            let mut buf2: [u8; 25] = [0; 25];
            buf2[0] = I2cCmd::ContinueData as u8;
            buf2[1..(24 + 1)].copy_from_slice(&buf[48..(24 + 48)]);
            i2c.write(self.address, &buf2)
                .map_err(|_| My9221LedMatrixError::I2CError)?;
        }
        Ok(())
    }

    /// Store frames to the internal buffer
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `delay` - A delay provider
    ///
    pub fn store_frames<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
        D: DelayMs<u32>,
    {
        i2c.write(self.address, &[I2cCmd::StoreFlash as u8])
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        delay.delay_ms(200);
        Ok(())
    }

    /// Delete frames from the internal buffer
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `delay` - A delay provider
    ///
    pub fn delete_frames<I2C, D>(
        &self,
        i2c: &mut I2C,
        delay: &mut D,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
        D: DelayMs<u32>,
    {
        i2c.write(self.address, &[I2cCmd::DeleteFlash as u8])
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        delay.delay_ms(200);
        Ok(())
    }

    /// Display frames from the internal buffer
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `duration_time` - The duration time of the bar
    /// * `forever_flag` - If true, the bar will be displayed forever
    /// * `from_idx` - The index of the first frame to display
    /// * `to_idx` - The index of the last frame to display
    ///
    pub fn display_frames_from_flash<I2C>(
        &self,
        i2c: &mut I2C,
        duration_time: u16,
        forever_flag: bool,
        from_idx: u8,
        to_idx: u8,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let from_idx = if from_idx > 5 {
            5
        } else if from_idx < 1 {
            1
        } else {
            from_idx
        };
        let to_idx = if to_idx > 5 {
            5
        } else if to_idx < 1 {
            1
        } else {
            to_idx
        };
        let (from_idx, to_idx) = if from_idx > to_idx {
            (to_idx, from_idx)
        } else {
            (from_idx, to_idx)
        };
        let mut buf: [u8; 6] = [0; 6];
        buf[0] = I2cCmd::DispFlash as u8;
        buf[1] = (duration_time & 0xff) as u8;
        buf[2] = ((duration_time >> 8) & 0xff) as u8;
        buf[3] = if forever_flag { 0 } else { 1 };
        buf[4] = from_idx;
        buf[5] = to_idx;
        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Enable test mode
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn enable_test_mode<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 1] = [0; 1];

        buf[0] = I2cCmd::TestTXRXOn as u8;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Disable test mode
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn disable_test_mode<I2C>(&self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 1] = [0; 1];

        buf[0] = I2cCmd::TestTXRXOff as u8;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    /// Test getting the version
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    /// # Returns
    ///
    /// * `Result<u8, My9221LedMatrixError>` - Returns the device version
    ///
    pub fn test_get_version<I2C>(&self, i2c: &mut I2C) -> Result<u32, My9221LedMatrixError>
    where
        I2C: Write + Read,
    {
        let mut cmd: [u8; 1] = [0; 1];
        let mut buf: [u8; 4] = [0; 4];

        cmd[0] = I2cCmd::TestGetVersion as u8;

        i2c.write(self.address, &cmd)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        i2c.read(self.address, &mut buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;

        Ok((buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32))
    }

    /// Get the device UID
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    /// # Returns
    ///
    /// * `Result<u8, My9221LedMatrixError>` - Returns the device UID
    ///
    pub fn get_device_uid<I2C>(&self, i2c: &mut I2C) -> Result<u8, My9221LedMatrixError>
    where
        I2C: Write + Read,
    {
        let mut cmd: [u8; 1] = [0; 1];
        let mut buf: [u8; 1] = [0; 1];

        cmd[0] = I2cCmd::GetDeviceUID as u8;

        i2c.write(self.address, &cmd)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        i2c.read(self.address, &mut buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;

        Ok(buf[0])
    }

    /// Set the address of the device
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    /// * `address` - The new address of the device
    ///
    pub fn set_address<I2C>(
        &mut self,
        i2c: &mut I2C,
        address: u8,
    ) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 2] = [0; 2];

        buf[0] = I2cCmd::SetAddress as u8;
        buf[1] = address;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        self.address = address;
        Ok(())
    }

    /// Reset the address of the device
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral to use
    ///
    pub fn reset_address<I2C>(&mut self, i2c: &mut I2C) -> Result<(), My9221LedMatrixError>
    where
        I2C: Write,
    {
        let mut buf: [u8; 1] = [0; 1];

        buf[0] = I2cCmd::ResetAddress as u8;

        i2c.write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        self.address = DEFAULT_ADDRESS;
        Ok(())
    }
}
