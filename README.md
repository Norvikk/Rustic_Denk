# Rustic_Denk Rewrite

Rustic_Denk is an encryption and decryption algorithm featuring key-based encryption, key-buffering with shorter keys, binding of buffer-keys, and decentralization to obfuscate keys and bricked files. The algorithm is optimized, achieving mostly O(n^1) efficiency. It requires Rust 1.67.0 ^ and Windows 10 & 11 SDK (MSVC). To install, clone the repository, navigate to the directory, build the binary with cargo build --release, and run the program with /target/release/rustic_denk_algo.exe. Testing core functionality can be done with cargo test. Note: Some inputs may not be handled.

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

