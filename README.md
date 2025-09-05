# Rustlet 🦀


![Build Status](https://github.com/mjrgr/rustlet/actions/workflows/build.yml/badge.svg?branch=main)
![Tests](https://img.shields.io/github/actions/workflow/status/mjrgr/rustlet/build.yml?label=tests&branch=main)
![Latest Release](https://img.shields.io/github/v/release/mjrgr/rustlet)
![Docker Pulls](https://img.shields.io/docker/pulls/mehdijrgr/rustlet?logo=docker)
![Docker Image Size](https://img.shields.io/docker/image-size/mehdijrgr/rustlet/latest?label=image%20size)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-lightgrey)
![License](https://img.shields.io/github/license/mjrgr/rustlet)

Rustlet is a lightweight, blazing-fast **init container** tool built in [Rust](https://www.rust-lang.org/).  
It helps Kubernetes pods prepare their runtime environment by running initialization tasks before the main containers start.

## ✨ Features
- ⚡ Written in Rust for speed and reliability
- 🛠️ Designed as an **init container** for Kubernetes pods
- 🔒 Safe, minimal, and resource-efficient
- 📦 Easy to integrate with existing Kubernetes manifests

## 🚀 Use Cases
Rustlet is ideal for:
- Running pre-checks or health validations

## 📦 Installation

### Using Cargo

Clone the repository and build with Cargo:

```bash
git clone https://github.com/mjrgr/rustlet.git
cd rustlet
cargo build --release
```

The compiled binary will be available in `target/release/rustlet`.

### Using Podman/Docker

Clone the repository and build with Podman/Docker CLI:

```bash
git clone https://github.com/mjrgr/rustlet.git
cd rustlet
podman build . -t rustlet
#docker build . -t rustlet
```

The built image will be locally available as `rustlet:latest`.

## 🛠️ Usage

Add Rustlet as an init container in your Kubernetes Pod spec:

```yaml
initContainers:
  - name: rustlet
    image: registry/rustlet:latest
    args: ["--url", "https://test.com"]
```

## 🗺️ Roadmap
- [ ] Add common built-in init tasks
- [ ] Bootstrapping external dependencies before the app starts
- [ ] Preparing configuration files or secrets
- [ ] Setting up directories, permissions, or storage volumes

## 🤝 Contributing
Contributions are welcome!  
Please open an issue or submit a PR to discuss improvements or new features.
