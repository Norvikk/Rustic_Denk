# Rustic_Denk

Rustic_Denk encrypts and decrypts text using a key. When text is encrypted, it outputs keys.dnk and bricked.dnk files that are obfuscated. The program was initially written in C#, then rewritten in C++ for added features but was unstable. This final version was rewritten in Rust and is stable with all the features.

# Table Of Contents

- [Rustic\_Denk](#rustic_denk)
- [Table Of Contents](#table-of-contents)
  - [Features](#features)
  - [Usage/Examples](#usageexamples)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [To-Dos](#to-dos)
  - [Limitations](#limitations)


## Features

- Encryption (simple key-base)
- Key-Buffering (re-encrypted with shorter keys)
- Binding (unites two buffer-keys into one)
- Decentralization (obfuscates the keys and bricked file)

## Usage/Examples

<div>
<img src="https://user-images.githubusercontent.com/105376497/225599548-e935aa76-36ad-4deb-8f45-a586920354c2.png"" width="49%"/> <img src="https://user-images.githubusercontent.com/105376497/225600293-ddd74b8a-ff80-4889-a2b6-a235c12c6ad1.png"" width="49%"/>
</div>

<div >
<br />
<img src="https://user-images.githubusercontent.com/105376497/225601652-8db1c9ab-9b10-4a27-be6c-755bd047f409.png"" width="49%"/> <img src="https://user-images.githubusercontent.com/105376497/225601911-dca109d5-e61d-41c9-8500-c3e3ba1a3578.png"" width="49%"/>
</div>



## Requirements

- Rust 1.67.0 ^ (https://www.rust-lang.org/tools/install)
- Windows 10 & 11 SDK (depending on OS) - MSVC (https://visualstudio.microsoft.com/vs/community/)

## Installation

  
1. Clone the Rustic_Denk repository using Git:

``` 
git clone https://github.com/Norvikk/Rustic_Denk.git
``` 

2. Navigate to the cloned repository directory:

``` 
cd Rustic_Denk
```

3. Build the Rustic_Denk binary using the following command:

``` 
cargo build --release
```

This will compile the Rustic_Denk source code and create the binary in the `target/release` directory.

4. Run Rustic_Denk by executing the following command:
```
/target/release/rustic_denk_algo.exe
```

This will launch the Rustic_Denk program, and you can use it to encrypt and decrypt your text using a key.



## To-Dos

- [ ] HashMap addition
- [x] Case tests
- [ ] Fixing CLI misinputs
- [ ] Fixing Decentralization not liking symbols





## Limitations

- "€" is broken. Everything else seems to work
