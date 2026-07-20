# Kavosh Cloudflare IP Scanner 🚀

[English](#-english) | [فارسی](#-فارسی)

[![License: GPL v3](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/vue-3.x-green.svg)](https://vuejs.org/)
[![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20Android%20(Soon)-brightgreen.svg)]()

---

## 🌐 English

A premium, blazingly fast, and lightweight cross-platform native application built with **Tauri**, **Rust**, and **Vue.js 3** to scan, test, and find the cleanest and lowest-latency Cloudflare IP addresses and ports.

### ✨ Features

- **Blazingly Fast Performance:** Asynchronous multi-threaded TCP network scanning powered by Rust background workers. No UI freezing or hanging.
- **Cross-Platform Support:** Native binary support for **Windows** (Desktop) and **Android** (Mobile support in progress).
- **Premium User Interface:** Modern dashboard stylized with Tailwind CSS and DaisyUI, featuring native custom frameless window dragging/controls.
- **Bi-directional Bilingual Support:** Full localization supporting English (LTR) and Persian (RTL) out of the box.
- **Smooth Visual Transitions:** Completely fluent fade in/out transitions during language toggles and theme switches (Dark / Light modes).
- **Extended Subnet Databases:** Includes official Cloudflare subnets, China Mobile, AS13335, AS209242, and IRCF community lists.
- **Granular Advanced Filters:** Custom settings for test counts, thread concurrency limits, maximum pings, and specific HTTPS port filters (443, 2053, 2083, etc.).
- **Professional Exporters:** Export results directly with native OS dialogs into clipboard, `.txt` raw format, or structured Excel-friendly `.csv` files.

### 🛠️ Tech Stack

- **Backend / Core Engine:** Rust (Tauri Core)
- **Frontend / Interface:** Vue.js 3 (TypeScript), Vite
- **Styling & Theme:** Tailwind CSS, DaisyUI
- **Fonts:** Vazirmatn (Optimized typography for Persian layouts)

### 💻 Getting Started (Development & Build)

#### Prerequisites
Make sure you have Node.js and the Rust compiler toolchain installed on your machine.

1. **Clone the Repository:**
   ```bash
   git clone [https://github.com/Psymfonius/Kavosh-Cloudflare-Scanner.git](https://github.com/Psymfonius/Kavosh-Cloudflare-Scanner.git)
   cd Kavosh-Cloudflare-Scanner

2. **Install Dependencies:**
   ```bash
   npm install

3. **Run in Development Mode:**
   ```bash
   npm run tauri dev

4. **Build Production Binaries:**
   ```bash
   Windows: npm run tauri build
   Android: npm run tauri android build (APK release coming soon)

