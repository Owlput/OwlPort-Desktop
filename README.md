# OwlPort-Desktop  

This is the GUI interface for [OwlNest](https://github.com/Owlput/OwlNest), powered by [Tauri](https://tauri.app) and [Vue](https://vuejs.org).

## Work In Progress
The GUI and the library it is using(OwlNest) is still in early stage of development, you will experience bugs and even memory leaks. Feel free to file an issue if something is wrong! It is also a way to keep the project churning.


## Build from source

You will need Rust toolchain installed on your machine to compile the code. Use [`rustup`](https://www.rust-lang.org/tools/install) to bootstrap the Rust toolchain.

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- If you want to share your experience when with other IDE setups, please file a PR.

### For Windows

#### Prerequisite: vcpkg for linking openssl

Note: You can also use `vendored` feature of crate `openssl` with crate `openssl-src` to bundle an openssl library along with the application. This will increase the size of the produced binary and build time.

 - Clone `https://github.com/Microsoft/vcpkg` and execute `bootstrap-vcpkg.bat` inside the cloned repository. 
 - Execute `./vcpkg.exe integrate install` to integrate this copy of vcpkg with your shell.
 - Execute `./vcpkg.exe install openssl:x64-windows-static-md` to install openssl library that can be statically linked.

Note: You can also use dynamically linked openssl. Please refer to crate `openssl` [documentation](https://docs.rs/openssl/0.10.66/openssl/) for more info.

#### Prerequetie: NodeJS for frontend build

 - Get the latest LTS version of NodeJS from its [official website](https://nodejs.org). Non-LTS or older version may work, but there is no guarantee.
 - Alternatively, install `nvm` for managing all NodeJS installations on your machine using [Chocolatey](https://community.chocolatey.org/) using `choco install nvm`. Note that normal NodeJS will ask if you want to use Chocolately as your package manager for installing node modules with native code(C/C++).

#### Prerequestie: WebView2  

  - Tauri needs WebView2 to render the UI properly. Windows 10 Update 1809 and later should have WebView2 installed by default. If you are building on an older OS you can get WebView 2 [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)(English page).

### For Linux

#### Prerequesties: `webkit2gtk` and common build tools

 - Similar to Windows, Tauri needs a browser engine to render the UI properly, which is `webkit2gtk` in this case.
 - Please refer to [official documentation](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux) of Tauri for more information to setup for developing Tauri applications.
