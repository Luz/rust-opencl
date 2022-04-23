# rust-opencl
A template for OpenCL projects in Rust.

## Dependencies
- OpenCL's ICD loader

## Building

### Nixos
Create default.nix
```Shell
with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    ocl-icd
  ];
}
```

Then build with cargo
```Shell
nix-shell . --command "cargo build"
```

## Example Output
File "output.png" will be created:
![output](https://raw.githubusercontent.com/Luz/rust-opencl/master/output.png)
