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

1. Clone the Repository:
   git clone https://github.com/Psymfonius/Kavosh-Cloudflare-Scanner.git
   cd Kavosh-Cloudflare-Scanner

2. Install Dependencies:
   npm install

3. Run in Development Mode:
   npm run tauri dev

4. Build Production Binaries:
   - Windows: npm run tauri build
   - Android: npm run tauri android build (APK release coming soon)

---

## 🇮🇷 فارسی

**اسکنر آی‌پی کلادفلر کاوش** یک نرم‌افزار حرفه‌ای، فوق‌العاده سریع، سبک و چندپلتفرمی است که با فریم‌ورک **Tauri**، زبان **Rust** و **Vue.js 3** توسعه یافته است تا تمیزترین و کم‌تاخیرترین آی‌پی‌ها و پورتهای کلادفلر را شناسایی و تست کند.

### ✨ قابلیت‌های اصلی

- **کارایی و سرعت فوق‌العاده:** اسکن شبکه‌ای هم‌زمان و چندنخی (Multi-threaded) بر پایه هسته غیرهم‌زمان Rust بدون کوچک‌ترین افت سرعت یا هنگ کردن محیط کاربری.
- **پشتیبانی از چند پلتفرم:** ارائه نسخه نیتیو اختصاصی برای **ویندوز** (دسکتاپ) و **اندروید** (به‌زودی در بخش ریلیزها قرار می‌گیرد).
- **رابط کاربری حرفه‌ای:** داشبورد مدرن طراحی‌شده با Tailwind CSS و DaisyUI به همراه پنجره‌های بدون فریم نیتیو.
- **پشتیبانی کامل از دو زبان:** امکان جابه‌جایی کامل و آنی بین دو زبان انگلیسی (چپ‌به‌راست) و فارسی (راست‌به‌چپ).
- **انتقال‌های بصری نرم:** انیمیشن‌های روان هنگام تغییر زبان یا تم برنامه (حالت تاریک و روشن).
- **دیتابیس گسترده رنج‌ها:** شامل زیرشبکه‌های رسمی کلادفلر، چین موبایل، AS13335، AS209242 و لیست‌های جامعه IRCF.
- **فیلترهای پیشرفته دقیق:** تنظیمات سفارشی برای تعداد تست، حد بالایی نخ‌ها (Concurrency)، حداکثر پینگ و فیلتر پورتهای HTTPS (پورتهای 443، 2053، 2083 و...).
- **خروجی‌گرفتن حرفه‌ای:** قابلیت کپی در حافظه (Clipboard) یا ذخیره مستقیم فایل‌های خروجی متنی (.txt) و فشرده اکسل (.csv).

### 🛠️ تکنولوژی‌های مورد استفاده

- **هسته و بک‌اند:** Rust (Tauri Core)
- **فرانت‌اند و رابط کاربری:** Vue.js 3 (TypeScript), Vite
- **استایل و تم:** Tailwind CSS, DaisyUI
- **فونت‌ها:** وزیرمتن (تایپوگرافی بهینه‌شده برای چیدمان فارسی)

### 💻 راهنمای نصب و اجرا در محیط توسعه

#### پیش‌نیازها
از نصب بودن Node.js و ابزار کامپایلر Rust روی سیستم خود اطمینان حاصل کنید.

۱. کلون کردن ریپوزیتوری:
   git clone https://github.com/Psymfonius/Kavosh-Cloudflare-Scanner.git
   cd Kavosh-Cloudflare-Scanner

۲. نصب وابسته‌ها:
   npm install

۳. اجرا در حالت توسعه:
   npm run tauri dev

۴. خروجی گرفتن برای تولید (Release):
   - نسخه ویندوز: npm run tauri build
   - نسخه اندروید: npm run tauri android build (فایل APK به‌زودی اضافه می‌شود)

---

## 📄 License / پروانه

This project is open-source software licensed under the **GNU General Public License v3.0 (GPL-3.0)**.  
این پروژه یک نرم‌افزار متن‌باز است که تحت پروانه **GPL-3.0** منتشر شده است