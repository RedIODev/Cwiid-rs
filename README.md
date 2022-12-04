# Setup
## Dependencies
(sudo apt) install libcwiid-dev<br>
(sudo apt) install libbluetooth-dev<br>
(cargo) install bindgen-cli<br>

## Bindgen
bindgen include/cwiid.h -o src/bindings.rs

## Optional
uses cross compiler if CC_armv5te_unknown_linux_gnueabi is detected.