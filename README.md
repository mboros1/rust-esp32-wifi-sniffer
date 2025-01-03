## An ESP32 based WiFi Sniffer, Written in Rust

Making this project as a test bed for developing microcontrollers with Rust, as the Espressif ESP32 as the platform.

To build, first install the ESP toolchain manager:

```{bash}
cargo install espup
```

 Then to install the necessary toolchains and dependencies.  
 
 ```{bash}
 espup install
```

Then, clone the repo and in the cloned repo run:

```{bash}
cargo build
```

Resources I'm using:

* [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/index.html), the basic documentation for ESP-IDF platform, which packages useful libraries with FreeRTOS, allowing you to use `std` Rust in an embedded environment.
* [Crate esp_idf_svc](https://docs.esp-rs.org/esp-idf-svc/esp_idf_svc/index.html), documentation for the crate that allows you to program using the ESP-IDF SDK with Rust. Works quite well, sometimes have to access the lower level unsafe C bindings, but still better then programming in C IMO
* [The Rust on ESP Book](https://docs.esp-rs.org/book/), a useful guide to get started
* [Espressif IoT Development Framework](https://github.com/espressif/esp-idf), the C SDK that the `esp_idf_svc` crate uses under the hood. Sometimes useful when you want to build a C/C++ program to compare against the Rust version, at least so far for me because all my application code is written in C++ atm

