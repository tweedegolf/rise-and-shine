# Rise and Shine: Putting the nRF52840 to sleep, and waking back up

This code puts the nRF52840 into System OFF mode, then uses a button with SENSE enabled to send a DETECT signal, waking the system back up.

This project (at the time of writing) requires a nightly version of rust. To use nightly for this project (but not globally), run the command below in the `rise-and-shine` directory.

    rustup override set nightly

Then, with the nRF52840 plugged in, simply run

    cargo run
    
Led 1 will turn on. When button 1 is pressed, the led will turn off and System OFF mode is entered. Pressing button 2 will wake the system back up and the led will turn on again.
