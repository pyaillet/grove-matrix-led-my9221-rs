#![feature(destructuring_assignment)]

use embedded_hal::blocking::i2c::{Read, Write};

pub const DEFAULT_ADDRESS: u8 = 0x65;

#[derive(Debug, Clone, Copy)]
pub enum I2cCmd {
    GetDevID = 0x00,           // This command gets device ID information
    DispBar = 0x01,            // This command displays LED bar
    DispEmoji = 0x02,          // This command displays emoji
    DispNum = 0x03,            // This command displays number
    DispStr = 0x04,            // This command displays string
    DispCustom = 0x05,         // TODO: This command displays user-defined pictures
    DispOff = 0x06,            // This command cleans the display
    DispAscii = 0x07,          // not use
    DispFlash = 0x08,          // TODO: This command displays pictures which are stored in flash
    DispColorBar = 0x09,       // This command displays colorful led bar
    DispColorWave = 0x0a,      // This command displays built-in wave animation
    DispColorClockWise = 0x0b, // This command displays built-in clockwise animation
    DispColorAnimation = 0x0c, // This command displays other built-in animation
    DispColorBlock = 0x0d,     // This command displays an user-defined color

    ContinueData = 0x81,

    StoreFlash = 0xa0,  // TODO: This command stores frames in flash
    DeleteFlash = 0xa1, // TODO: This command deletes all the frames in flash

    LedOn = 0xb0,        // This command turns on the indicator LED flash mode
    LedOff = 0xb1,       // This command turns off the indicator LED flash mode
    AutoSleepOn = 0xb2,  // This command enable device auto sleep mode
    AutoSleepOff = 0xb3, // This command disable device auto sleep mode (default mode)

    DispRotate = 0xb4, // This command setting the display orientation
    DispOffset = 0xb5, // This command setting the display offset

    SetAddress = 0xc0,     // TODO: This command sets device i2c address
    ResetAddress = 0xc1,   // TODO: This command resets device i2c address
    TestTXRXOn = 0xe0,     // This command enable TX RX pin test mode
    TestTXRXOff = 0xe1,    // This command disable TX RX pin test mode
    TestGetVersion = 0xe2, // This command use to get software version
    GetDeviceUID = 0xf1,   // This command use to get chip id
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayRotate {
    Deg0 = 0,
    Deg90 = 1,
    Deg180 = 2,
    Deg270 = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum Colors {
    Red = 0x00,
    Orange = 0x12,
    Yellow = 0x18,
    Green = 0x52,
    Cyan = 0x7f,
    Blue = 0xaa,
    Purple = 0xc3,
    Pink = 0xdc,
    White = 0xfe,
    Black = 0xff,
}

pub struct My9221LedMatrix<I2C: Write> {
    address: u8,
    i2c: I2C,
}

#[cfg_attr(feature = "std", derive(Debug))]
pub enum My9221LedMatrixError {
    I2CError,
}

#[cfg(feature = "std")]
impl std::fmt::Display for My9221LedMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            My9221LedMatrixError::I2CError => write!(f, "I2C error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for My9221LedMatrixError {}

impl<I2C> My9221LedMatrix<I2C>
where
    I2C: Write + Read,
{
    pub fn new(address: u8, i2c: I2C) -> Self {
        Self { address, i2c }
    }

    pub fn set_led_matrix_rotate(
        &mut self,
        rotate: DisplayRotate,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 2];
        buf[0] = I2cCmd::DispRotate as u8;
        buf[1] = rotate as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn stop_displat(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::DispOff as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn set_led_matrix_offset(&mut self, offset: (u8, u8)) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 3];
        buf[0] = I2cCmd::DispOffset as u8;
        buf[1] = offset.0;
        buf[2] = offset.1;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn turn_on_led_flash(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::LedOn as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn turn_off_led_flash(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::LedOff as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn enable_auto_sleep(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::AutoSleepOn as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn disable_auto_sleep(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 1];
        buf[0] = I2cCmd::AutoSleepOff as u8;
        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_bar(
        &mut self,
        bar: u8,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 6];
        buf[0] = I2cCmd::DispBar as u8;
        buf[1] = if bar <= 32 { bar } else { 32 };
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 1 } else { 0 };
        buf[5] = color as u8;

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_emoji(
        &mut self,
        emoji: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 5];
        buf[0] = I2cCmd::DispEmoji as u8;
        buf[1] = emoji;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 1 } else { 0 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_number(
        &mut self,
        number: u16,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf = [0; 7];
        buf[0] = I2cCmd::DispNum as u8;
        buf[1] = (number & 0xff) as u8;
        buf[2] = ((number >> 8) & 0xff) as u8;
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff & 0xff) as u8;
        buf[5] = if forever_flag { 1 } else { 0 };
        buf[6] = color as u8;

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_string(
        &mut self,
        string: &str,
        duration_time: u16,
        forever_flag: bool,
        color: Colors,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 36] = [0; 36];
        let len = if string.len() > 25 {
            25u8
        } else {
            string.len() as u8
        };

        for i in 0..len {
            buf[(i + 6) as usize] = string.chars().nth(i as usize).unwrap() as u8;
        }

        buf[0] = I2cCmd::DispStr as u8;
        buf[1] = if forever_flag { 1 } else { 0 };
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = len;
        buf[5] = color as u8;

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_color_block(
        &mut self,
        rgb: u32,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 7] = [0; 7];

        buf[0] = I2cCmd::DispColorBlock as u8;
        buf[1] = ((rgb >> 16) & 0xff) as u8;
        buf[2] = ((rgb >> 8) & 0xff) as u8;
        buf[3] = (rgb & 0xff) as u8;
        buf[4] = (duration_time & 0xff) as u8;
        buf[5] = ((duration_time >> 8) & 0xff) as u8;
        buf[6] = if forever_flag { 0 } else { 1 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_color_bar(
        &mut self,
        bar: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 5] = [0; 5];

        buf[0] = I2cCmd::DispColorBar as u8;
        buf[1] = bar;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 0 } else { 1 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_color_wave(
        &mut self,
        wave: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 5] = [0; 5];

        buf[0] = I2cCmd::DispColorWave as u8;
        buf[1] = wave;
        buf[2] = (duration_time & 0xff) as u8;
        buf[3] = ((duration_time >> 8) & 0xff) as u8;
        buf[4] = if forever_flag { 0 } else { 1 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_color_clockwise(
        &mut self,
        clockwise: bool,
        big: bool,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 6] = [0; 6];

        buf[0] = I2cCmd::DispColorClockWise as u8;
        buf[1] = if clockwise { 0 } else { 1 };
        buf[2] = if big { 0 } else { 1 };
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff) as u8;
        buf[5] = if forever_flag { 0 } else { 1 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn display_color_animation(
        &mut self,
        animation_index: u8,
        duration_time: u16,
        forever_flag: bool,
    ) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 6] = [0; 6];

        buf[0] = I2cCmd::DispColorAnimation as u8;
        (buf[1], buf[2]) = match animation_index {
            0 => (0, 28),    // big clockwise
            1 => (29, 41),   // small clockwise
            2 => (255, 255), // rainbow cycle
            3 => (254, 254), // fire
            4 => (42, 43),   // walking
            5 => (44, 52),   // broken heart
            _ => (0, 0),     // None
        };
        buf[3] = (duration_time & 0xff) as u8;
        buf[4] = ((duration_time >> 8) & 0xff) as u8;
        buf[5] = if forever_flag { 0 } else { 1 };

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn enable_test_mode(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 1] = [0; 1];

        buf[0] = I2cCmd::TestTXRXOn as u8;

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn disable_test_mode(&mut self) -> Result<(), My9221LedMatrixError> {
        let mut buf: [u8; 1] = [0; 1];

        buf[0] = I2cCmd::TestTXRXOff as u8;

        self.i2c
            .write(self.address, &buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        Ok(())
    }

    pub fn test_get_version(&mut self) -> Result<u32, My9221LedMatrixError> {
        let mut cmd: [u8; 1] = [0; 1];
        let mut buf: [u8; 4] = [0; 4];

        cmd[0] = I2cCmd::TestGetVersion as u8;

        self.i2c
            .write(self.address, &cmd)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        self.i2c
            .read(self.address, &mut buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;

        Ok((buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32))
    }

    pub fn get_device_id(&mut self) -> Result<u8, My9221LedMatrixError> {
        let mut cmd: [u8; 1] = [0; 1];
        let mut buf: [u8; 1] = [0; 1];

        cmd[0] = I2cCmd::GetDeviceUID as u8;

        self.i2c
            .write(self.address, &cmd)
            .map_err(|_| My9221LedMatrixError::I2CError)?;
        self.i2c
            .read(self.address, &mut buf)
            .map_err(|_| My9221LedMatrixError::I2CError)?;

        Ok(buf[0])
    }
}
