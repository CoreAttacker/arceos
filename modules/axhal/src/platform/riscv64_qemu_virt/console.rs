/// Writes a byte to the console.
pub fn putchar(c: u8, color: &str) {
    let color_code = match color {
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        _ => "\x1b[0m", // Reset to default
    };
    for &byte in color_code.as_bytes() {
        #[allow(deprecated)]
        sbi_rt::legacy::console_putchar(byte as usize);
    }
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c as usize);
    for &byte in "\x1b[0m".as_bytes() { // Reset color after character
        #[allow(deprecated)]
        sbi_rt::legacy::console_putchar(byte as usize);
    }
}


/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn getchar() -> Option<u8> {
    #[allow(deprecated)]
    match sbi_rt::legacy::console_getchar() as isize {
        -1 => None,
        c => Some(c as u8),
    }
}
