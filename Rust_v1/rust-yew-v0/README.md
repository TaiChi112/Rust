# Yew Counter Web Application

**🌐 Language Options / ตัวเลือกภาษา**
- **English** (Current) - You are reading the English version
- **[ไทย (Thai)](#thai-version)** - สำหรับเอกสารภาษาไทย คลิกที่นี่

---

A modern, interactive web application built with **Yew** (Rust's powerful frontend framework) and **WebAssembly**. This project demonstrates the power of Rust for web development, featuring a responsive counter application with dynamic styling.

## 🚀 Features

- **🦀 Rust-Powered**: Built entirely with Rust, compiled to WebAssembly for near-native performance
- **⚡ Real-time Updates**: Instant UI updates with Yew's reactive component system
- **🎨 Dynamic Styling**: Color-changing interface based on counter value (even/odd)
- **📱 Responsive Design**: Works seamlessly across desktop and mobile devices
- **🔥 Hot Reload**: Automatic rebuilding and reloading during development
- **🌐 Modern Web Standards**: Leverages the latest web technologies and best practices
- **🚀 Production Ready**: Optimized builds for deployment

## 🛠️ Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust** (latest stable version)
  - Install from: https://www.rust-lang.org/tools/install
  - Verify installation: `rustc --version`
- **Git** for cloning the repository
- **A modern web browser** (Chrome, Firefox, Safari, Edge)

### Installation

Follow these step-by-step instructions to set up the project locally:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/rust-yew-v0.git
   cd rust-yew-v0
   ```

2. **Install the WebAssembly target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Trunk** (the build tool for Rust WebAssembly applications):
   ```bash
   cargo install trunk wasm-bindgen-cli
   ```

4. **Verify installation**:
   ```bash
   trunk --version
   ```

That's it! Your development environment is ready.

## 🎯 Usage

### Development Mode

Start the development server with hot reload:

```bash
trunk serve
```

This will:
- Compile your Rust code to WebAssembly
- Start a local development server (usually at `http://127.0.0.1:8080`)
- Watch for file changes and automatically rebuild
- Open your browser to the application

### Building for Production

Create an optimized production build:

```bash
trunk build --release
```

The compiled application will be available in the `dist/` directory, ready for deployment to any web server.

### Watch Mode (Build Only)

If you want to build without serving:

```bash
trunk watch
```

### Application Features

Once running, you can:
- **Click the button** to increment the counter
- **Watch the color change** - the button alternates between green (even numbers) and red (odd numbers)
- **View real-time updates** as the counter value changes instantly

## 🧩 Project Structure

```
rust-yew-v0/
├── src/
│   ├── main.rs          # Application entry point
│   └── app.rs           # Main component with counter logic
├── index.html           # HTML template
├── index.scss           # Styling (Sass)
├── Cargo.toml           # Rust dependencies and metadata
├── Trunk.toml           # Trunk configuration
└── README.md            # This file
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/rust-yew-v0.git
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
4. **Make your changes** and commit them:
   ```bash
   git commit -m "Add some amazing feature"
   ```
5. **Push to your branch**:
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request** on GitHub

### Development Guidelines

- Follow Rust best practices and formatting (`cargo fmt`)
- Run tests before submitting (`cargo test`)
- Update documentation for any new features
- Keep commits atomic and well-described

## 📝 License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## 📧 Contact

**Project Maintainer**: TaiChi  
📧 **Email**: anothai.0978452316@gmail.com  
🔗 **GitHub**: [Your GitHub Profile](https://github.com/your-username)

For questions, bug reports, or feature requests:
- 🐛 **Issues**: [GitHub Issues](https://github.com/your-username/rust-yew-v0/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/your-username/rust-yew-v0/discussions)

---

## Thai Version

**🌐 Language Options / ตัวเลือกภาษา**
- **[English](#yew-counter-web-application)** - สำหรับเอกสารภาษาอังกฤษ คลิกที่นี่
- **ไทย (Thai)** (ปัจจุบัน) - คุณกำลังอ่านเวอร์ชันภาษาไทย

---

# แอปพลิเคชัน Yew Counter

แอปพลิเคชันเว็บที่ทันสมัยและโต้ตอบได้ ที่สร้างด้วย **Yew** (เฟรมเวิร์กส่วนหน้าที่ทรงพลังของ Rust) และ **WebAssembly** โปรเจกต์นี้แสดงให้เห็นถึงพลังของ Rust สำหรับการพัฒนาเว็บ โดยมีแอปพลิเคชันเคาน์เตอร์ที่ตอบสนองพร้อมการจัดรูปแบบแบบไดนามิก

## 🚀 คุณสมบัติ

- **🦀 ขับเคลื่อนด้วย Rust**: สร้างทั้งหมดด้วย Rust คอมไพล์เป็น WebAssembly เพื่อประสิทธิภาพใกล้เคียงกับระบบพื้นฐาน
- **⚡ อัปเดตแบบเรียลไทม์**: การอัปเดต UI ทันทีด้วยระบบคอมโพเนนต์แบบรีแอคทีฟของ Yew
- **🎨 การจัดรูปแบบแบบไดนามิก**: อินเทอร์เฟซที่เปลี่ยนสีตามค่าเคาน์เตอร์ (คู่/คี่)
- **📱 การออกแบบที่ตอบสนอง**: ทำงานได้อย่างราบรื่นทั้งบนเดสก์ท็อปและมือถือ
- **🔥 Hot Reload**: การสร้างใหม่และโหลดใหม่อัตโนมัติระหว่างการพัฒนา
- **🌐 มาตรฐานเว็บสมัยใหม่**: ใช้ประโยชน์จากเทคโนโลยีเว็บล่าสุดและแนวปฏิบัติที่ดีที่สุด
- **🚀 พร้อมสำหรับการใช้งานจริง**: บิลด์ที่เหมาะสำหรับการปรับใช้

## 🛠️ เริ่มต้น

### ข้อกำหนดเบื้องต้น

ก่อนเริ่มต้น ตรวจสอบให้แน่ใจว่าคุณได้ติดตั้งสิ่งต่อไปนี้ในระบบของคุณแล้ว:

- **Rust** (เวอร์ชันเสถียรล่าสุด)
  - ติดตั้งจาก: https://www.rust-lang.org/tools/install
  - ตรวจสอบการติดตั้ง: `rustc --version`
- **Git** สำหรับโคลนรีพอซิทอรี
- **เว็บเบราว์เซอร์ที่ทันสมัย** (Chrome, Firefox, Safari, Edge)

### การติดตั้ง

ทำตามคำแนะนำทีละขั้นตอนเหล่านี้เพื่อตั้งค่าโปรเจกต์ในเครื่อง:

1. **โคลนรีพอซิทอรี**:
   ```bash
   git clone https://github.com/your-username/rust-yew-v0.git
   cd rust-yew-v0
   ```

2. **ติดตั้งเป้าหมาย WebAssembly**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **ติดตั้ง Trunk** (เครื่องมือสร้างสำหรับแอปพลิเคชัน Rust WebAssembly):
   ```bash
   cargo install trunk wasm-bindgen-cli
   ```

4. **ตรวจสอบการติดตั้ง**:
   ```bash
   trunk --version
   ```

เท่านี้ก็เรียบร้อย! สภาพแวดล้อมการพัฒนาของคุณพร้อมแล้ว

## 🎯 การใช้งาน

### โหมดการพัฒนา

เริ่มเซิร์ฟเวอร์การพัฒนาพร้อม hot reload:

```bash
trunk serve
```

นี่จะ:
- คอมไพล์โค้ด Rust ของคุณเป็น WebAssembly
- เริ่มเซิร์ฟเวอร์การพัฒนาในเครื่อง (มักจะที่ `http://127.0.0.1:8080`)
- เฝ้าดูการเปลี่ยนแปลงไฟล์และสร้างใหม่อัตโนมัติ
- เปิดเบราว์เซอร์ของคุณไปยังแอปพลิเคชัน

### การสร้างสำหรับการใช้งานจริง

สร้างบิลด์ที่เหมาะสำหรับการใช้งานจริง:

```bash
trunk build --release
```

แอปพลิเคชันที่คอมไพล์แล้วจะอยู่ในไดเรกทอรี `dist/` พร้อมสำหรับการปรับใช้บนเว็บเซิร์ฟเวอร์ใดๆ

### โหมดเฝ้าดู (สร้างเท่านั้น)

หากคุณต้องการสร้างโดยไม่ต้องให้บริการ:

```bash
trunk watch
```

### คุณสมบัติของแอปพลิเคชัน

เมื่อเรียกใช้แล้ว คุณสามารถ:
- **คลิกปุ่ม** เพื่อเพิ่มเคาน์เตอร์
- **ดูการเปลี่ยนสี** - ปุ่มจะสลับระหว่างสีเขียว (เลขคู่) และสีแดง (เลขคี่)
- **ดูการอัปเดตแบบเรียลไทม์** เมื่อค่าเคาน์เตอร์เปลี่ยนแปลงทันที

## 🧩 โครงสร้างโปรเจกต์

```
rust-yew-v0/
├── src/
│   ├── main.rs          # จุดเริ่มต้นของแอปพลิเคชัน
│   └── app.rs           # คอมโพเนนต์หลักพร้อมตรรกะเคาน์เตอร์
├── index.html           # เทมเพลต HTML
├── index.scss           # การจัดรูปแบบ (Sass)
├── Cargo.toml           # ข้อมูลและการพึ่งพา Rust
├── Trunk.toml           # การกำหนดค่า Trunk
└── README.md            # ไฟล์นี้
```

## 🤝 การมีส่วนร่วม

เรายินดีรับการมีส่วนร่วม! นี่คือวิธีที่คุณสามารถช่วยได้:

1. **ฟอร์กรีพอซิทอรี** บน GitHub
2. **โคลนฟอร์กของคุณ** ในเครื่อง:
   ```bash
   git clone https://github.com/your-username/rust-yew-v0.git
   ```
3. **สร้างสาขาฟีเจอร์**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
4. **ทำการเปลี่ยนแปลง** และคอมมิต:
   ```bash
   git commit -m "Add some amazing feature"
   ```
5. **พุชไปยังสาขาของคุณ**:
   ```bash
   git push origin feature/amazing-feature
   ```
6. **เปิด Pull Request** บน GitHub

### แนวทางการพัฒนา

- ปฏิบัติตามแนวปฏิบัติที่ดีของ Rust และการจัดรูปแบบ (`cargo fmt`)
- รันการทดสอบก่อนส่ง (`cargo test`)
- อัปเดตเอกสารสำหรับฟีเจอร์ใหม่ๆ
- รักษาคอมมิตให้เป็นอะตอมและอธิบายได้ดี

## 📝 ใบอนุญาต

โปรเจกต์นี้ได้รับอนุญาตแบบคู่ภายใต้:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

คุณสามารถเลือกใบอนุญาตใดก็ได้สำหรับการใช้งานของคุณ

## 📧 ติดต่อ

**ผู้ดูแลโปรเจกต์**: TaiChi  
📧 **อีเมล**: anothai.0978452316@gmail.com  
🔗 **GitHub**: [Your GitHub Profile](https://github.com/your-username)

สำหรับคำถาม รายงานข้อผิดพลาด หรือคำขอฟีเจอร์:
- 🐛 **Issues**: [GitHub Issues](https://github.com/your-username/rust-yew-v0/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/your-username/rust-yew-v0/discussions)