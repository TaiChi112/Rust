# Rust Dioxus v1 - Modern Cross-Platform Web Application

> **🌐 Language Selection | เลือกภาษา**  
> **English**: You are currently reading the English version  
> **ไทย**: สำหรับเอกสารภาษาไทย โปรดคลิก [ที่นี่](#thai-version--เวอร์ชันภาษาไทย)

A modern, fast, and cross-platform web application built with **Dioxus** - a React-like framework for Rust. This project demonstrates a full-stack web application with routing, server functions, and beautiful styling using Tailwind CSS.

## ✨ Features

- 🚀 **Cross-Platform**: Build for web, desktop, and mobile from the same codebase
- ⚡ **High Performance**: Written in Rust with minimal runtime overhead
- 🧭 **Client-Side Routing**: Dynamic navigation with the Dioxus router
- 🎨 **Modern UI**: Styled with Tailwind CSS for a responsive design
- 🔄 **Server Functions**: Full-stack capabilities with server-side functionality
- 📱 **Responsive Design**: Works seamlessly across different screen sizes
- 🛠️ **Component-Based**: Modular architecture with reusable components

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust** (1.75.0 or later): [Install Rust](https://rustup.rs/)
- **Node.js and npm**: [Install Node.js](https://nodejs.org/)
- **Dioxus CLI**: Install with `cargo install dioxus-cli`
- **Git**: [Install Git](https://git-scm.com/)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/rust-dioxus-v1.git
   cd rust-dioxus-v1
   ```

2. **Install Rust dependencies**
   ```bash
   cargo build
   ```

3. **Install Tailwind CSS CLI**
   ```bash
   npm install -g tailwindcss
   ```

4. **Set up Tailwind CSS** (in a separate terminal)
   ```bash
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```

## 📖 Usage

### Development Server

Start the development server for web platform:

```bash
dx serve --platform web
```

The application will be available at `http://localhost:8080`

### Building for Different Platforms

**Web Application:**
```bash
dx build --platform web --release
```

**Desktop Application:**
```bash
dx serve --platform desktop
```

**Mobile Application:**
```bash
dx serve --platform mobile
```

### Project Structure

```
rust-dioxus-v1/
├─ assets/                 # Static assets (images, CSS, etc.)
│  ├─ favicon.ico
│  ├─ header.svg
│  ├─ tailwind.css        # Compiled Tailwind CSS
│  └─ styling/            # Custom CSS files
├─ src/
│  ├─ main.rs            # Application entry point and routing
│  ├─ components/        # Reusable UI components
│  │  ├─ mod.rs
│  │  ├─ hero.rs         # Hero section component
│  │  └─ echo.rs         # Server function demo component
│  └─ views/             # Page components for routes
│     ├─ mod.rs
│     ├─ home.rs         # Home page
│     ├─ blog.rs         # Blog post page
│     └─ navbar.rs       # Navigation component
├─ Cargo.toml            # Rust dependencies and features
├─ Dioxus.toml           # Dioxus configuration
└─ tailwind.config.js    # Tailwind CSS configuration
```

### Available Routes

- `/` - Home page with hero section and echo component
- `/blog/:id` - Dynamic blog post pages

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes** and add tests if applicable
4. **Commit your changes**
   ```bash
   git commit -m "Add amazing feature"
   ```
5. **Push to your branch**
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request**

### Development Guidelines

- Follow Rust coding conventions
- Add comments for complex logic
- Test your changes thoroughly
- Update documentation as needed

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

**TaiChi** - anothai.0978452316@gmail.com

- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/rust-dioxus-v1/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/yourusername/rust-dioxus-v1/discussions)
- 🌐 **Dioxus Community**: [Discord Server](https://discord.gg/XgGxMSkvUM)

---

## Thai Version | เวอร์ชันภาษาไทย

> **🌐 เลือกภาษา | Language Selection**  
> **ไทย**: คุณกำลังอ่านเอกสารภาษาไทย  
> **English**: For English version, please click [here](#rust-dioxus-v1---modern-cross-platform-web-application)

# Rust Dioxus v1 - แอปพลิเคชันเว็บข้ามแพลตฟอร์มสมัยใหม่

แอปพลิเคชันเว็บที่ทันสมัย รวดเร็ว และรองรับหลายแพลตฟอร์ม สร้างด้วย **Dioxus** - เฟรมเวิร์กสำหรับ Rust ที่คล้าย React โปรเจกต์นี้สาธิตการพัฒนาแอปพลิเคชันเว็บแบบ full-stack พร้อมระบบ routing, server functions และการจัดแต่งที่สวยงามด้วย Tailwind CSS

## ✨ คุณสมบัติ

- 🚀 **ข้ามแพลตฟอร์ม**: สร้างแอปสำหรับเว็บ เดสก์ท็อป และมือถือจากโค้ดเดียวกัน
- ⚡ **ประสิทธิภาพสูง**: เขียนด้วย Rust มีโอเวอร์เฮดขณะทำงานน้อยที่สุด
- 🧭 **การนำทางแบบไคลเอนต์**: การนำทางแบบไดนามิกด้วย Dioxus router
- 🎨 **UI สมัยใหม่**: จัดแต่งด้วย Tailwind CSS สำหรับการออกแบบที่ตอบสนอง
- 🔄 **Server Functions**: ความสามารถแบบ full-stack พร้อมฟังก์ชันฝั่งเซิร์ฟเวอร์
- 📱 **การออกแบบตอบสนอง**: ทำงานได้อย่างราบรื่นบนหน้าจอขนาดต่างๆ
- 🛠️ **ระบบคอมโพเนนต์**: สถาปัตยกรรมแบบโมดูลาร์พร้อมคอมโพเนนต์ที่ใช้ซ้ำได้

## 🚀 เริ่มต้นใช้งาน

### ข้อกำหนดเบื้องต้น

ก่อนเริ่มต้น โปรดตรวจสอบให้แน่ใจว่าได้ติดตั้งสิ่งต่อไปนี้ในระบบของคุณแล้ว:

- **Rust** (1.75.0 หรือใหม่กว่า): [ติดตั้ง Rust](https://rustup.rs/)
- **Node.js และ npm**: [ติดตั้ง Node.js](https://nodejs.org/)
- **Dioxus CLI**: ติดตั้งด้วย `cargo install dioxus-cli`
- **Git**: [ติดตั้ง Git](https://git-scm.com/)

### การติดตั้ง

1. **โคลนโปรเจกต์**
   ```bash
   git clone https://github.com/yourusername/rust-dioxus-v1.git
   cd rust-dioxus-v1
   ```

2. **ติดตั้ง dependencies ของ Rust**
   ```bash
   cargo build
   ```

3. **ติดตั้ง Tailwind CSS CLI**
   ```bash
   npm install -g tailwindcss
   ```

4. **ตั้งค่า Tailwind CSS** (ในเทอร์มินัลแยกต่างหาก)
   ```bash
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```

## 📖 การใช้งาน

### เซิร์ฟเวอร์พัฒนา

เริ่มเซิร์ฟเวอร์พัฒนาสำหรับแพลตฟอร์มเว็บ:

```bash
dx serve --platform web
```

แอปพลิเคชันจะพร้อมใช้งานที่ `http://localhost:8080`

### การสร้างสำหรับแพลตฟอร์มต่างๆ

**แอปพลิเคชันเว็บ:**
```bash
dx build --platform web --release
```

**แอปพลิเคชันเดสก์ท็อป:**
```bash
dx serve --platform desktop
```

**แอปพลิเคชันมือถือ:**
```bash
dx serve --platform mobile
```

### โครงสร้างโปรเจกต์

```
rust-dioxus-v1/
├─ assets/                 # ไฟล์ทรัพยากรคงที่ (รูปภาพ, CSS ฯลฯ)
│  ├─ favicon.ico
│  ├─ header.svg
│  ├─ tailwind.css        # Tailwind CSS ที่คอมไพล์แล้ว
│  └─ styling/            # ไฟล์ CSS กำหนดเอง
├─ src/
│  ├─ main.rs            # จุดเริ่มต้นแอปพลิเคชันและ routing
│  ├─ components/        # คอมโพเนนต์ UI ที่ใช้ซ้ำได้
│  │  ├─ mod.rs
│  │  ├─ hero.rs         # คอมโพเนนต์ส่วน hero
│  │  └─ echo.rs         # คอมโพเนนต์สาธิต server function
│  └─ views/             # คอมโพเนนต์หน้าเว็บสำหรับ routes
│     ├─ mod.rs
│     ├─ home.rs         # หน้าแรก
│     ├─ blog.rs         # หน้าโพสต์บล็อก
│     └─ navbar.rs       # คอมโพเนนต์การนำทาง
├─ Cargo.toml            # dependencies และ features ของ Rust
├─ Dioxus.toml           # การกำหนดค่า Dioxus
└─ tailwind.config.js    # การกำหนดค่า Tailwind CSS
```

### Routes ที่มีอยู่

- `/` - หน้าแรกพร้อมส่วน hero และคอมโพเนนต์ echo
- `/blog/:id` - หน้าโพสต์บล็อกแบบไดนามิก

## 🤝 การมีส่วนร่วม

เรายินดีรับการมีส่วนร่วม! วิธีการที่คุณสามารถช่วยได้:

1. **Fork โปรเจกต์**
2. **สร้าง feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **ทำการแก้ไข** และเพิ่มการทดสอบหากจำเป็น
4. **Commit การเปลี่ยนแปลง**
   ```bash
   git commit -m "Add amazing feature"
   ```
5. **Push ไปที่ branch ของคุณ**
   ```bash
   git push origin feature/amazing-feature
   ```
6. **เปิด Pull Request**

### แนวทางการพัฒนา

- ปฏิบัติตามหลักการเขียนโค้ด Rust
- เพิ่มคอมเมนต์สำหรับลอจิกที่ซับซ้อน
- ทดสอบการเปลี่ยนแปลงของคุณอย่างละเอียด
- อัปเดตเอกสารตามความจำเป็น

## 📄 ใบอนุญาต

โปรเจกต์นี้ได้รับอนุญาตภายใต้ MIT License - ดูไฟล์ [LICENSE](LICENSE) สำหรับรายละเอียด

## 📞 ติดต่อ

**TaiChi** - anothai.0978452316@gmail.com

- 🐛 **ปัญหา**: [GitHub Issues](https://github.com/yourusername/rust-dioxus-v1/issues)
- 💬 **การสนทนา**: [GitHub Discussions](https://github.com/yourusername/rust-dioxus-v1/discussions)
- 🌐 **ชุมชน Dioxus**: [Discord Server](https://discord.gg/XgGxMSkvUM)


