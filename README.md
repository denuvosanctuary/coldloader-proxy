# ColdLoader Proxy

This project is made to be used with [coldloader](https://github.com/denuvosanctuary/coldloader). It's a simple proxy DLL that can load multiple DLLs. Works with both `version.dll` and `winmm.dll`, just rename the compiled DLL to the desired library.

## Usage

1. **Build the DLL:**

    ```sh
    cargo build --release
    ```

    The resulting DLL will be located in `target/release/version.dll`.

2. **Add the coldloader files to the game files:**

    Compile or download [coldloader](https://github.com/denuvosanctuary/coldloader/releases) and get `coldloader.dll`, create a `coldloader.EXAMPLE.ini` file if needed. Place them in the same directory as the game executable.

3. **Add the compiled version.dll:**
    
    Rename the compiled DLL to `version.dll` (or `winmm.dll` if you prefer) and place it in the same directory as the game executable.

4. **Run the game:**
    
    Start the game as you normally would.

## Builds

Builds are available in the [releases](https://github.com/denuvosanctuary/coldloader-proxy/releases) section of the repository. Nighly builds are also available in the [actions](https://github.com/denuvosanctuary/coldloader-proxy/actions) section.

## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.

### Credits

- @notgitgit for the help in making this
