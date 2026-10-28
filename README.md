# rustbot
A robot arm built around and esp32 and rust

## Firmware development

The firmware project lives directly in this repository's root and targets the
ESP32-WROOM-32 (`xtensa-esp32-none-elf`). It was generated with `esp-generate`
using Embassy async, serial logging, and panic backtraces. Wi-Fi and BLE are not
enabled yet.

Install the Espressif Rust toolchain using `espup install` and load its environment
with `source ~/export-esp.sh`. Install `espflash` with `cargo install espflash`
if it is not already available.

From the repository root:

```sh
cargo build
cargo run
```

`cargo run` builds, flashes the connected ESP32 over serial, and opens the serial
monitor. The generated starter firmware logs a message once per second; servo
and joystick control are not implemented yet.

The project is based on this:
https://howtomechatronics.com/tutorials/arduino/diy-arduino-robot-arm-with-smartphone-control/

This is a project for learning embedded Rust 
I will use the same set up with six servos and their schematic for 3d printing the arm. 
However, the arduino will be switched to an ESP32 instead. This is because I want to to controll the arm with two joysticks instead of an app to make inte more interactiv. I also want the built in BLE/WIFI in the ESP32 incase I want to add some app/cloud functions later on. 
I will also add "save" buttons to be able to program the arm in different positions and make it automated. This will also be in separate tasks. 

### The basics of how it will work: 
- The left and right motion on the left joystick will control the rotation on the waist.
- The up and down motion on the left joystick will control the tilt on the shoulder and the elbow. 
- The left and right motion on the joystick will control the roll on the wrist
- The up and down motion on the right joystick will control the tilt on the wrist
- The button on the right joystick will control the grip, tightening the grip when the button i held down. Releasing when pushed once. 

- One button, when pressed, will add the postion of all servos to a struct. 
- The struct will be able to contain 5 separate postions (Might be increased or decreased after testing)
- A counter will be added to keep track of which index in the struct the position will be saved to. 

- One button, when pressed, will toggle through the struct and and position the servos att the saved angles at that index. Starting over att index 0 when it reaches the end of the struct. 
- The servos will be changed in ascending order with the waist being servo 1 and the grip servo 6. This as to not drop/grip before the arm is in the correct position. 

- Might also need to add a "reset" button to clear the struct. 

### Materials used:

- 1x  ESP32                   
- 3x  SG90 servos            
- 3x  MG996R servos           
- 2x  Joysticks            
- 1x  Power supply module  
- 1x  3d printed arm        



## The project and readme will be updated accordingly
