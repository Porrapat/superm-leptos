# SuperM – Leptos (CSR)

SuperM Leptos is a **client-side rendered (CSR) web application** built with **Leptos + Rust + WASM**.  
This project is part of the *SuperM* online shopping demo, focusing on **clean UI logic, reactive state management, and modern Rust frontend architecture**.

💡 This project is inspired by the **SuperM Final Project** from  
https://react-tutorial.app/ by **Jad Joubran**.

You can explore the original React-based example here:  
- https://react-tutorial.app/  
- https://superm-react.porrapat.com

👉 Live demo (Leptos version):  
https://superm-leptos.porrapat.com

---

## 🤝 Companion Repository

This project has a companion implementation built with **React**, created to compare architecture, state management, and developer experience between **React** and **Leptos (Rust + WASM)**.

- **SuperM – React version**  
  👉 https://www.github.com/Porrapat/superm-react

- **SuperM API – Axum version**  
  👉 https://www.github.com/Porrapat/superm-api
---

## ✨ Features

- ⚡ **Leptos CSR (WASM)** – no SSR, no backend required
- 🧭 Client-side routing with `leptos_router`
- 🎨 Light / Dark theme toggle
- 🛒 Shopping cart state (reactive)
- 🔐 Mock login flow (for UI & UX testing)
- 🧠 Clean component & state design (no DOM manipulation)
- 🌍 Deployed with **Nginx + Cloudflare**

---

## 🛠 Tech Stack

- **Rust**
- **Leptos**
- **Leptos Router**
- **Trunk**
- **WebAssembly (WASM)**
- **Nginx** (static hosting)
- **Cloudflare** (SSL & CDN)

---

## 📁 Project Structure

```
superm-leptos/
├── src/
│   ├── components/
│   ├── api.rs
│   ├── models.rs
│   └── main.rs
├── assets/
├── index.html
├── Trunk.toml
└── Cargo.toml
```

---

## 🚀 Getting Started

### Prerequisites

- Rust (stable)
- Trunk

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

---

### Run in Development Mode

```bash
trunk serve
```

Open:
👉 http://localhost:3000

---

### Build for Production

```bash
trunk build --release
```

Output will be generated in:

```
dist/
```

---

## 🔐 Mock Login Behavior

- Email: `test@example.com`
- Password: any non-empty value

---

## 👤 Author

**Porrapat Petchdamrongskul**
Rust Backend & Frontend (Leptos) Developer

---

## 📄 License

MIT License
