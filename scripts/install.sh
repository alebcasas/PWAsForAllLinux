#!/bin/bash
#
# PWAsForAllLinux - Universal Installer for Linux
# Compatible with all Linux distributions and desktop environments
#
# Usage: ./install.sh [--uninstall] [--help]
#
# This script will:
# 1. Check system requirements
# 2. Install dependencies
# 3. Download or build PWAsForAllLinux
# 4. Install the application system-wide
# 5. Create desktop entries and icons

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Version
VERSION="1.0.0"
APP_NAME="PWAsForAllLinux"
APP_ID="com.pwasforalllinux.PWAsForAllLinux"

# Directories
INSTALL_DIR="/opt/pwasforalllinux"
BIN_DIR="/usr/local/bin"
DESKTOP_DIR="/usr/share/applications"
ICON_DIR="/usr/share/icons/hicolor"

# Temporary directory
TEMP_DIR=$(mktemp -d)

# Cleanup on exit
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

# Print functions
print_header() {
    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                                                              ║"
    echo "║             PWAsForAllLinux - Universal Installer            ║"
    echo "║                                                              ║"
    echo "║     Install Progressive Web Apps on any Linux desktop        ║"
    echo "║                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_step() {
    echo -e "\n${CYAN}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "This script must be run as root (use sudo)"
        exit 1
    fi
}

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        DISTRO_LIKE=$ID_LIKE
        DISTRO_VERSION=$VERSION_ID
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
        DISTRO_LIKE="debian"
    elif [ -f /etc/redhat-release ]; then
        DISTRO="rhel"
        DISTRO_LIKE="rhel"
    else
        DISTRO="unknown"
        DISTRO_LIKE="unknown"
    fi
    print_success "Detected distribution: $DISTRO ($DISTRO_LIKE)"
}

# Check system architecture
check_architecture() {
    ARCH=$(uname -m)
    if [[ "$ARCH" != "x86_64" && "$ARCH" != "aarch64" ]]; then
        print_error "Unsupported architecture: $ARCH"
        print_info "This application requires x86_64 or aarch64 (ARM64)"
        exit 1
    fi
    print_success "Architecture: $ARCH"
}

# Detect desktop environment
detect_desktop() {
    DESKTOP=$(echo $XDG_CURRENT_DESKTOP | tr '[:upper:]' '[:lower:]')
    if [ -z "$DESKTOP" ]; then
        DESKTOP="unknown"
    fi
    print_success "Desktop environment: $DESKTOP"
}

# Check system requirements
check_requirements() {
    print_step "Checking system requirements..."
    
    # Check for required commands
    local missing_commands=()
    
    for cmd in curl wget tar; do
        if ! command -v $cmd &> /dev/null; then
            missing_commands+=($cmd)
        fi
    done
    
    if [ ${#missing_commands[@]} -gt 0 ]; then
        print_warning "Missing commands: ${missing_commands[*]}"
        print_info "Attempting to install missing dependencies..."
        install_dependencies "${missing_commands[@]}"
    fi
    
    print_success "All requirements met"
}

# Install dependencies based on distribution
install_dependencies() {
    local extra_packages=("$@")
    
    print_step "Installing dependencies..."
    
    case $DISTRO_LIKE in
        debian|ubuntu)
            apt-get update -qq
            apt-get install -y -qq \
                build-essential \
                curl \
                wget \
                pkg-config \
                libgtk-4-dev \
                libwebkit2gtk-4.1-dev \
                libssl-dev \
                librsvg2-dev \
                git \
                "${extra_packages[@]}" 2>/dev/null || true
            
            # Verify pkg-config is installed
            if ! command -v pkg-config &> /dev/null; then
                print_warning "pkg-config not found, installing explicitly..."
                apt-get install -y -qq pkg-config
            fi
            ;;
        fedora|rhel|centos)
            dnf install -y -q \
                @development-tools \
                curl \
                wget \
                pkgconfig \
                gtk4-devel \
                webkit2gtk4.1-devel \
                openssl-devel \
                librsvg2-devel \
                "${extra_packages[@]}" 2>/dev/null || true
            ;;
        arch|manjaro)
            pacman -Sy --noconfirm --quiet \
                base-devel \
                curl \
                wget \
                pkg-config \
                gtk4 \
                webkit2gtk-4.1 \
                openssl \
                librsvg \
                "${extra_packages[@]}" 2>/dev/null || true
            ;;
        opensuse*)
            zypper -q install -y \
                -t pattern devel_basis \
                curl \
                wget \
                pkg-config \
                gtk4-devel \
                webkit2gtk3-devel \
                libopenssl-devel \
                librsvg-devel \
                "${extra_packages[@]}" 2>/dev/null || true
            ;;
        *)
            print_warning "Unknown distribution. Please install dependencies manually:"
            echo "  - GTK4 development libraries"
            echo "  - WebKitGTK development libraries"
            echo "  - OpenSSL development libraries"
            echo "  - Rust toolchain"
            read -p "Press Enter to continue or Ctrl+C to exit..."
            ;;
    esac
    
    # Install Rust if not present
    if ! command -v rustc &> /dev/null; then
        print_info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || source "/root/.cargo/env" 2>/dev/null || true
    fi
    
    print_success "Dependencies installed"
}

# Download pre-built binaries
download_binaries() {
    print_step "Downloading PWAsForAllLinux..."
    
    local download_url="https://github.com/alebcasas/PWAsForAllLinux/releases/download/v${VERSION}/pwasforalllinux-${ARCH}.tar.gz"
    
    # Try to download pre-built binaries
    if curl -fsSL "$download_url" -o "$TEMP_DIR/pwasforalllinux.tar.gz" 2>/dev/null; then
        print_success "Downloaded pre-built binaries"
        tar -xzf "$TEMP_DIR/pwasforalllinux.tar.gz" -C "$TEMP_DIR"
        return 0
    else
        print_warning "Pre-built binaries not available, will build from source"
        return 1
    fi
}

# Build from source
build_from_source() {
    print_step "Building PWAsForAllLinux from source..."
    
    # Ensure Rust is installed and in PATH
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        export PATH="$HOME/.cargo/bin:/root/.cargo/bin:$PATH"
        source "$HOME/.cargo/env" 2>/dev/null || source "/root/.cargo/env" 2>/dev/null || true
        
        # Verify installation
        if ! command -v cargo &> /dev/null; then
            print_error "Failed to install Rust. Please install manually: https://rustup.rs"
            exit 1
        fi
    fi
    
    # Try to clone repository, fall back to local source if it fails
    if ! git clone --depth 1 https://github.com/alebcasas/PWAsForAllLinux.git "$TEMP_DIR/src" 2>/dev/null; then
        print_warning "Could not clone repository. Using local source..."
        # Use local source (script is in scripts/ directory, source is in parent)
        local script_dir="$(cd "$(dirname "$0")" && pwd)"
        local source_dir="$(dirname "$script_dir")"
        
        if [ -f "$source_dir/Cargo.toml" ]; then
            cp -r "$source_dir" "$TEMP_DIR/src"
        else
            print_error "Could not find source code"
            print_error "Please ensure you have internet connection or run from the repository directory"
            exit 1
        fi
    fi
    
    cd "$TEMP_DIR/src"
    
    # Verify Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found in source directory"
        exit 1
    fi
    
    # Build release
    cargo build --release
    
    # Create package directory
    mkdir -p "$TEMP_DIR/pwasforalllinux"
    cp target/release/pwasforalllinux "$TEMP_DIR/pwasforalllinux/"
    cp target/release/pwa-launcher "$TEMP_DIR/pwasforalllinux/"
    
    print_success "Build completed"
}

# Install the application
install_application() {
    print_step "Installing PWAsForAllLinux..."
    
    # Create directories
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"
    mkdir -p "$DESKTOP_DIR"
    mkdir -p "$ICON_DIR/scalable/apps"
    mkdir -p "$ICON_DIR/128x128/apps"
    mkdir -p "$ICON_DIR/64x64/apps"
    mkdir -p "$ICON_DIR/48x48/apps"
    
    # Copy binaries
    cp "$TEMP_DIR/pwasforalllinux/pwasforalllinux" "$INSTALL_DIR/"
    cp "$TEMP_DIR/pwasforalllinux/pwa-launcher" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/pwasforalllinux"
    chmod +x "$INSTALL_DIR/pwa-launcher"
    
    # Create symlinks
    ln -sf "$INSTALL_DIR/pwasforalllinux" "$BIN_DIR/pwasforalllinux"
    ln -sf "$INSTALL_DIR/pwa-launcher" "$BIN_DIR/pwa-launcher"
    
    # Create desktop entry
    cat > "$DESKTOP_DIR/$APP_ID.desktop" << EOF
[Desktop Entry]
Version=1.0
Name=PWAsForAllLinux
Comment=Install and manage Progressive Web Apps on Linux
Exec=pwasforalllinux
Icon=pwasforalllinux
Terminal=false
Type=Application
Categories=Utility;Network;WebApp;
StartupNotify=true
StartupWMClass=$APP_ID
Keywords=pwa;webapp;install;manage;
X-GNOME-Autostart-enabled=true
EOF
    
    # Copy icons
    if [ -f "$TEMP_DIR/pwasforalllinux/icon.svg" ]; then
        cp "$TEMP_DIR/pwasforalllinux/icon.svg" "$ICON_DIR/scalable/apps/pwasforalllinux.svg"
    fi
    if [ -f "$TEMP_DIR/pwasforalllinux/icon-128.png" ]; then
        cp "$TEMP_DIR/pwasforalllinux/icon-128.png" "$ICON_DIR/128x128/apps/pwasforalllinux.png"
    fi
    if [ -f "$TEMP_DIR/pwasforalllinux/icon-64.png" ]; then
        cp "$TEMP_DIR/pwasforalllinux/icon-64.png" "$ICON_DIR/64x64/apps/pwasforalllinux.png"
    fi
    if [ -f "$TEMP_DIR/pwasforalllinux/icon-48.png" ]; then
        cp "$TEMP_DIR/pwasforalllinux/icon-48.png" "$ICON_DIR/48x48/apps/pwasforalllinux.png"
    fi
    
    # Update icon cache
    gtk4-update-icon-cache -f "$ICON_DIR" 2>/dev/null || true
    update-icon-caches /usr/share/icons/* 2>/dev/null || true
    
    # Update desktop database
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    
    print_success "Installation completed"
}

# Uninstall the application
uninstall_application() {
    print_step "Uninstalling PWAsForAllLinux..."
    
    # Remove files
    rm -rf "$INSTALL_DIR"
    rm -f "$BIN_DIR/pwasforalllinux"
    rm -f "$BIN_DIR/pwa-launcher"
    rm -f "$DESKTOP_DIR/$APP_ID.desktop"
    rm -f "$ICON_DIR/scalable/apps/pwasforalllinux.svg"
    rm -f "$ICON_DIR/128x128/apps/pwasforalllinux.png"
    rm -f "$ICON_DIR/64x64/apps/pwasforalllinux.png"
    rm -f "$ICON_DIR/48x48/apps/pwasforalllinux.png"
    
    # Remove user data (optional)
    read -p "Remove user data and installed PWAs? [y/N]: " remove_data
    if [[ "$remove_data" =~ ^[Yy]$ ]]; then
        rm -rf "$HOME/.config/pwasforalllinux"
        rm -rf "$HOME/.local/share/pwasforalllinux"
        rm -rf "$HOME/.cache/pwasforalllinux"
        rm -f "$HOME/.local/share/applications/pwasforalllinux-"*.desktop
        print_success "User data removed"
    fi
    
    # Update caches
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    update-icon-caches /usr/share/icons/* 2>/dev/null || true
    
    print_success "Uninstallation completed"
}

# Show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --install       Install PWAsForAllLinux (default)"
    echo "  --uninstall     Uninstall PWAsForAllLinux"
    echo "  --build         Build from source instead of downloading"
    echo "  --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  sudo $0              # Install PWAsForAllLinux"
    echo "  sudo $0 --uninstall  # Uninstall PWAsForAllLinux"
    echo "  sudo $0 --build      # Build from source and install"
}

# Main function
main() {
    local action="install"
    local build_from_src=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --install)
                action="install"
                shift
                ;;
            --uninstall)
                action="uninstall"
                shift
                ;;
            --build)
                build_from_src=true
                shift
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    print_header
    
    # Check root
    check_root
    
    # Detect system
    detect_distro
    check_architecture
    detect_desktop
    
    case $action in
        install)
            check_requirements
            
            # Always install build dependencies before attempting to compile
            if [ "$build_from_src" = true ] || ! download_binaries; then
                print_step "Installing build dependencies..."
                install_dependencies
                build_from_source
            fi
            
            install_application
            
            echo ""
            echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
            echo -e "${GREEN}║               Installation successful!                       ║${NC}"
            echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
            echo ""
            echo "You can now launch PWAsForAllLinux from your applications menu"
            echo "or by running 'pwasforalllinux' in a terminal."
            echo ""
            ;;
        uninstall)
            uninstall_application
            ;;
    esac
}

# Run main
main "$@"
