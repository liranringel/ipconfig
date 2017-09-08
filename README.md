# Ipconfig

**Get network adapters information for windows.**

[![Build Status](https://travis-ci.org/liranringel/ipconfig.svg?branch=master)](https://travis-ci.org/liranringel/ipconfig)
[![Build status](https://ci.appveyor.com/api/projects/status/tiwjo6q4eete0nmh/branch/master?svg=true)](https://ci.appveyor.com/project/liran-ringel/ipconfig/branch/master)
[![Crates.io](https://img.shields.io/crates/v/ipconfig.svg)](https://crates.io/crates/ipconfig)

[Documentation](https://docs.rs/ipconfig)

## Examples

```rust
// Print the ip addresses and dns servers of all adapters:
for adapter in ipconfig::get_adapters()? {
    println!("Ip addresses: {:#?}", adapter.ip_addresses());
    println!("Dns servers: {:#?}", adapter.dns_servers());
}
```
