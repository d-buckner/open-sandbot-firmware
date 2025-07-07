use core::{
    fmt,
    str::{from_utf8, Utf8Error},
};

const COMMAND_BUFFER_SIZE: usize = 32;

#[derive(Debug)]
pub enum CommandError {
    EOLReached,
    BufferOverflow,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::EOLReached => {
                write!(f, "cannot append to buffer after command is complete")
            }
            CommandError::BufferOverflow => write!(f, "command too long for buffer"),
        }
    }
}

pub struct Command {
    buf: [u8; COMMAND_BUFFER_SIZE],
    current_buf_idx: usize,
    is_eol: bool,
}

impl Command {
    pub fn new() -> Self {
        Command {
            buf: [0u8; COMMAND_BUFFER_SIZE],
            current_buf_idx: 0,
            is_eol: false,
        }
    }

    pub fn add_char_buf(&mut self, char_buf: &[u8; 1]) -> Result<(), CommandError> {
        if self.is_eol {
            return Err(CommandError::EOLReached);
        }
        let current_byte = char_buf[0];
        if current_byte == b'\n' {
            // check if we have room for the terminating space
            if self.current_buf_idx >= self.buf.len() {
                return Err(CommandError::BufferOverflow);
            }
            // str split treats last split different, add extra split char to avoid
            self.buf[self.current_buf_idx] = b' ';
            self.is_eol = true;
            return Ok(());
        }
        // check bounds before writing
        if self.current_buf_idx >= self.buf.len() {
            return Err(CommandError::BufferOverflow);
        }
        self.buf[self.current_buf_idx] = current_byte;
        self.current_buf_idx += 1;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.is_eol
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.buf[..self.current_buf_idx])
    }
}
