## OpenSandbot
OpenSandbot is an open-source robotic kinetic art table. It has four components: parts, hardware, firmware, and app.

### Firmware
This firmware is written in rust for the $4 Raspberry Pi Pico (RP2040) and provides a UART API on [UART0](https://pico.pinout.xyz/) to control the sandbot.

### API
The API expects messages to be formatted as `METHOD[ ARG0][ ARG1]\n`. All messages (rx and tx) expect the new line character (`\n`) at the end of each message.

- Movement: Ex. `MOVE 1.2 0.5` would move to theta 1.2 rho 0.5.

The API also emits the status messages
- `STATUS IDLE`: the sandbot is idle and not currently moving
- `STATUS MOVING` the sandbot is currently moving to a target position
- `ERROR INVALID_UTF8`: command contains invalid UTF-8
- `ERROR MISSING_ARGS`: command is missing required arguments
- `ERROR INVALID_NUMBERS`: arguments could not be parsed as numbers

### Testing
```bash
# Install just (one time)
cargo install just

# Run tests
just test-host     # Run integration tests
just test-embedded # Check embedded build
just test-all      # Run all tests
```
