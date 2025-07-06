# Rust Dioxus Web Application

> **Language Options**: 🇺🇸 English | [🇹🇭 Thai Version](#thai-version)

A modern, interactive web application built with Rust and Dioxus framework, featuring real-time server functions, routing, and responsive design with Tailwind CSS.

## Features

- 🚀 **Modern Web Framework**: Built with Dioxus 0.6.0 for blazing-fast performance
- 🔄 **Server Functions**: Real-time echo functionality demonstrating fullstack capabilities
- 🧭 **Dynamic Routing**: Blog system with URL parameters and navigation
- 🎨 **Tailwind CSS**: Beautiful, responsive styling with modern design
- 📱 **Multi-Platform**: Supports web, desktop, and mobile platforms
- ⚡ **Hot Reload**: Live development server with instant updates
- 🦀 **Rust Performance**: Memory-safe, high-performance web applications

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust** (latest stable version): [Install Rust](https://rustup.rs/)
- **Node.js and npm**: [Install Node.js](https://nodejs.org/)
- **Dioxus CLI**: Install with `cargo install dioxus-cli`

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/rust-dioxus-v0.git
   cd rust-dioxus-v0
   ```

2. **Install Rust dependencies**:
   ```bash
   cargo build
   ```

3. **Install Tailwind CSS CLI**:
   ```bash
   npm install -g tailwindcss
   ```

4. **Build Tailwind CSS**:
   ```bash
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```
   > Keep this command running in a separate terminal for live CSS updates.

## Usage

### Running the Application

1. **Start the development server**:
   ```bash
   dx serve --platform web
   ```

2. **Open your browser** and navigate to `http://localhost:8080`

### Available Platforms

- **Web Application** (default):
  ```bash
  dx serve --platform web
  ```

- **Desktop Application**:
  ```bash
  dx serve --platform desktop
  ```

- **Mobile Development**:
  ```bash
  dx serve --platform mobile
  ```

### Features Overview

- **Home Page**: Interactive hero section with useful links and echo functionality
- **Blog System**: Navigate through blog posts with dynamic routing (`/blog/1`, `/blog/2`, etc.)
- **Server Functions**: Type in the echo input to see real-time server communication
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile devices

### Project Structure

```
rust-dioxus-v0/
├── assets/           # Static assets (CSS, images, icons)
├── src/
│   └── main.rs      # Main application entry point
├── Cargo.toml       # Rust dependencies and configuration
├── Dioxus.toml      # Dioxus framework configuration
├── tailwind.config.js # Tailwind CSS configuration
└── README.md        # This file
```

## Contributing

We welcome contributions to improve this project! Here's how you can help:

1. **Fork the repository** on GitHub
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** and test thoroughly
4. **Commit your changes**: `git commit -m "Add amazing feature"`
5. **Push to your branch**: `git push origin feature/amazing-feature`
6. **Open a Pull Request** with a clear description of your changes

### Development Guidelines

- Follow Rust coding conventions and use `cargo fmt`
- Run `cargo clippy` to catch common mistakes
- Test your changes across different platforms
- Update documentation for new features

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- **Author**: TaiChi
- **Email**: anothai.0978452316@gmail.com
- **GitHub**: [Your GitHub Profile](https://github.com/yourusername)

For questions, suggestions, or support, please:
- Open an issue on GitHub
- Join the [Dioxus Community Discord](https://discord.gg/XgGxMSkvUM)
- Check out the [Dioxus Documentation](https://dioxuslabs.com/learn/0.6/)

---

## Thai Version

> **ตัวเลือกภาษา**: [🇺🇸 English Version](#rust-dioxus-web-application) | 🇹🇭 ไทย

# แอปพลิเคชัน Rust Dioxus Web Application

แอปพลิเคชันเว็บที่ทันสมัยและโต้ตอบได้ ที่สร้างด้วย Rust และ Dioxus framework มีฟีเจอร์ server functions แบบเรียลไทม์, routing, และการออกแบบที่ตอบสนองด้วย Tailwind CSS

## ฟีเจอร์หลัก

- 🚀 **เฟรมเวิร์กเว็บสมัยใหม่**: สร้างด้วย Dioxus 0.6.0 เพื่อประสิทธิภาพที่รวดเร็วเหมือนฟ้าแลบ
- 🔄 **Server Functions**: ฟังก์ชันการสะท้อนเสียงแบบเรียลไทม์ที่แสดงความสามารถแบบ fullstack
- 🧭 **Dynamic Routing**: ระบบบล็อกพร้อม URL parameters และการนำทาง
- 🎨 **Tailwind CSS**: การจัดแต่งที่สวยงามและตอบสนองด้วยการออกแบบที่ทันสมัย
- 📱 **หลายแพลตฟอร์ม**: รองรับเว็บ, เดสก์ท็อป, และมือถือ
- ⚡ **Hot Reload**: เซิร์ฟเวอร์พัฒนาสดพร้อมการอัปเดตทันที
- 🦀 **ประสิทธิภาพ Rust**: แอปพลิเคชันเว็บที่ปลอดภัยต่อหน่วยความจำและประสิทธิภาพสูง

## เริ่มต้นใช้งาน

### ข้อกำหนดเบื้องต้น

ก่อนเริ่มต้น ให้แน่ใจว่าคุณได้ติดตั้งสิ่งต่อไปนี้ในระบบของคุณแล้ว:

- **Rust** (เวอร์ชันเสถียรล่าสุด): [ติดตั้ง Rust](https://rustup.rs/)
- **Node.js และ npm**: [ติดตั้ง Node.js](https://nodejs.org/)
- **Dioxus CLI**: ติดตั้งด้วย `cargo install dioxus-cli`

### การติดตั้ง

1. **โคลน repository**:
   ```bash
   git clone https://github.com/yourusername/rust-dioxus-v0.git
   cd rust-dioxus-v0
   ```

2. **ติดตั้ง dependencies ของ Rust**:
   ```bash
   cargo build
   ```

3. **ติดตั้ง Tailwind CSS CLI**:
   ```bash
   npm install -g tailwindcss
   ```

4. **สร้าง Tailwind CSS**:
   ```bash
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```
   > ให้คำสั่งนี้ทำงานต่อไปในเทอร์มินัลแยกต่างหากเพื่อการอัปเดต CSS แบบสด

## การใช้งาน

### เรียกใช้แอปพลิเคชัน

1. **เริ่มเซิร์ฟเวอร์พัฒนา**:
   ```bash
   dx serve --platform web
   ```

2. **เปิดเบราว์เซอร์** และไปที่ `http://localhost:8080`

### แพลตฟอร์มที่มีให้ใช้งาน

- **แอปพลิเคชันเว็บ** (ค่าเริ่มต้น):
  ```bash
  dx serve --platform web
  ```

- **แอปพลิเคชันเดสก์ท็อป**:
  ```bash
  dx serve --platform desktop
  ```

- **การพัฒนามือถือ**:
  ```bash
  dx serve --platform mobile
  ```

### ภาพรวมฟีเจอร์

- **หน้าแรก**: ส่วน hero แบบโต้ตอบพร้อมลิงก์ที่มีประโยชน์และฟังก์ชันการสะท้อนเสียง
- **ระบบบล็อก**: นำทางผ่านโพสต์บล็อกด้วย dynamic routing (`/blog/1`, `/blog/2`, ฯลฯ)
- **Server Functions**: พิมพ์ในช่องป้อนข้อมูลเพื่อดูการสื่อสารเซิร์ฟเวอร์แบบเรียลไทม์
- **การออกแบบที่ตอบสนอง**: ทำงานได้อย่างราบรื่นบนเดสก์ท็อป, แท็บเล็ต, และมือถือ

### โครงสร้างโปรเจกต์

```
rust-dioxus-v0/
├── assets/           # Static assets (CSS, รูปภาพ, ไอคอน)
├── src/
│   └── main.rs      # จุดเข้าหลักของแอปพลิเคชัน
├── Cargo.toml       # dependencies และการกำหนดค่า Rust
├── Dioxus.toml      # การกำหนดค่า Dioxus framework
├── tailwind.config.js # การกำหนดค่า Tailwind CSS
└── README.md        # ไฟล์นี้
```

## การมีส่วนร่วม

เรายินดีรับการมีส่วนร่วมเพื่อปรับปรุงโปรเจกต์นี้! นี่คือวิธีที่คุณสามารถช่วยได้:

1. **Fork repository** บน GitHub
2. **สร้าง feature branch**: `git checkout -b feature/amazing-feature`
3. **ทำการเปลี่ยนแปลง** และทดสอบอย่างละเอียด
4. **Commit การเปลี่ยนแปลงของคุณ**: `git commit -m "Add amazing feature"`
5. **Push ไปยัง branch ของคุณ**: `git push origin feature/amazing-feature`
6. **เปิด Pull Request** พร้อมคำอธิบายที่ชัดเจนเกี่ยวกับการเปลี่ยนแปลงของคุณ

### แนวทางการพัฒนา

- ปฏิบัติตามการแปลง Rust และใช้ `cargo fmt`
- เรียกใช้ `cargo clippy` เพื่อตรวจจับข้อผิดพลาดทั่วไป
- ทดสอบการเปลี่ยนแปลงของคุณในแพลตฟอร์มต่างๆ
- อัปเดตเอกสารสำหรับฟีเจอร์ใหม่

## สัญญาอนุญาต

โปรเจกต์นี้ได้รับอนุญาตภายใต้ MIT License - ดูไฟล์ [LICENSE](LICENSE) สำหรับรายละเอียด

## ติดต่อ

- **ผู้เขียน**: TaiChi
- **อีเมล**: anothai.0978452316@gmail.com
- **GitHub**: [โปรไฟล์ GitHub ของคุณ](https://github.com/yourusername)

สำหรับคำถาม, ข้อเสนอแนะ, หรือการสนับสนุน โปรด:
- เปิด issue บน GitHub
- เข้าร่วม [Dioxus Community Discord](https://discord.gg/XgGxMSkvUM)
- ดู [เอกสาร Dioxus](https://dioxuslabs.com/learn/0.6/)


