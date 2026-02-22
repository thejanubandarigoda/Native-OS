# 🧠 AI-OS (Bare-Metal Rust Operating System)

![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Target](https://img.shields.io/badge/Target-x86__64--unknown--none-blue.svg)
![Environment](https://img.shields.io/badge/Environment-QEMU-green.svg)

**AI-OS** is an experimental, intent-driven, bare-metal operating system built entirely from scratch using Rust. Designed to be lightweight and futuristic, this OS explores the possibilities of integrating AI operations at the kernel level. 

Currently, it successfully boots on x86_64 architectures using QEMU, featuring a custom panic handler and direct VGA buffer text rendering.

## ✨ Current Features
- **Bare-Metal Rust Engine:** Built in a `no_std` and `no_main` environment, running directly on hardware without an underlying OS.
- **Custom Bootloader Integration:** Uses the Rust `bootloader` crate to safely map memory and load the kernel.
- **Direct VGA Text Rendering:** Interacts directly with the `0xb8000` memory address to render colored text on the screen.
- **Safe Error Handling:** Implements a custom hardware-level `panic_handler`.

## 🛠️ Prerequisites
To build and run this OS, you need the following tools installed:

1. **Rust (Nightly Toolchain):** Specifically, the `2023-10-31` build for compatibility.
2. **QEMU:** For hardware emulation and testing.
3. **Bootimage:** To compile the kernel and bootloader into a bootable disk image.

## 🚀 Getting Started

### 1. Clone the Repository
```bash
git clone [https://github.com/thejanubandarigoda/ai_os.git](https://github.com/thejanubandarigoda/ai_os.git)
cd ai_os