#!/bin/bash
#
# Build packages for PWAsForAllLinux
# Creates DEB, RPM, and AppImage packages
#

set -e

VERSION="1.0.0"
ARCH=$(uname -m)
BUILD_DIR="$(pwd)/build"
PACKAGE_DIR="$(pwd)/packages"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Prepare build directory
prepare_build() {
    print_step "Preparing build environment..."
    mkdir -p "$BUILD_DIR"
    mkdir -p "$PACKAGE_DIR"
    
    # Build release binary
    cargo build --release
    
    print_success "Build environment ready"
}

# Build DEB package
build_deb() {
    print_step "Building DEB package..."
    
    local DEB_DIR="$BUILD_DIR/deb"
    local DEB_PACKAGE="pwasforalllinux_${VERSION}_${ARCH}"
    
    mkdir -p "$DEB_DIR/DEBIAN"
    mkdir -p "$DEB_DIR/opt/pwasforalllinux"
    mkdir -p "$DEB_DIR/usr/bin"
    mkdir -p "$DEB_DIR/usr/share/applications"
    mkdir -p "$DEB_DIR/usr/share/icons/hicolor/scalable/apps"
    mkdir -p "$DEB_DIR/usr/share/doc/pwasforalllinux"
    
    # Copy binaries
    cp target/release/pwasforalllinux "$DEB_DIR/opt/pwasforalllinux/"
    cp target/release/pwa-launcher "$DEB_DIR/opt/pwasforalllinux/"
    
    # Create symlinks
    ln -sf /opt/pwasforalllinux/pwasforalllinux "$DEB_DIR/usr/bin/pwasforalllinux"
    ln -sf /opt/pwasforalllinux/pwa-launcher "$DEB_DIR/usr/bin/pwa-launcher"
    
    # Create control file
    cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: pwasforalllinux
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libgtk-4-1, libwebkit2gtk-4.1-0, libc6
Maintainer: PWAsForAllLinux Team <team@pwasforalllinux.org>
Description: Progressive Web Apps manager for Linux
 A tool to install, manage and use Progressive Web Apps (PWAs)
 on any Linux distribution and desktop environment.
 .
 Features:
  - Install any website as a standalone web app
  - Native desktop integration (menu entries, icons)
  - Isolated profiles for each PWA
  - Compatible with GNOME, KDE, XFCE, and more
Homepage: https://pwasforalllinux.org
EOF
    
    # Create postinst script
    cat > "$DEB_DIR/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e
update-desktop-database /usr/share/applications 2>/dev/null || true
gtk4-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true
EOF
    chmod 755 "$DEB_DIR/DEBIAN/postinst"
    
    # Create prerm script
    cat > "$DEB_DIR/DEBIAN/prerm" << 'EOF'
#!/bin/bash
set -e
# Nothing to do before removal
EOF
    chmod 755 "$DEB_DIR/DEBIAN/prerm"
    
    # Create desktop entry
    cat > "$DEB_DIR/usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop" << EOF
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
EOF
    
    # Copy icon
    cp assets/icon.svg "$DEB_DIR/usr/share/icons/hicolor/scalable/apps/pwasforalllinux.svg" 2>/dev/null || true
    
    # Build package
    dpkg-deb --build "$DEB_DIR" "$PACKAGE_DIR/${DEB_PACKAGE}.deb"
    
    print_success "DEB package created: $PACKAGE_DIR/${DEB_PACKAGE}.deb"
}

# Build RPM package
build_rpm() {
    print_step "Building RPM package..."
    
    local RPM_DIR="$BUILD_DIR/rpm"
    local RPM_SPEC="$BUILD_DIR/pwasforalllinux.spec"
    
    mkdir -p "$RPM_DIR/BUILD"
    mkdir -p "$RPM_DIR/RPMS"
    mkdir -p "$RPM_DIR/SOURCES"
    mkdir -p "$RPM_DIR/SPECS"
    mkdir -p "$RPM_DIR/SRPMS"
    
    # Create spec file
    cat > "$RPM_SPEC" << EOF
Name:           pwasforalllinux
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Progressive Web Apps manager for Linux

License:        MIT
URL:            https://pwasforalllinux.org
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust, cargo, gtk4-devel, webkit2gtk4.1-devel
Requires:       gtk4, webkit2gtk4.1

%description
A tool to install, manage and use Progressive Web Apps (PWAs)
on any Linux distribution and desktop environment.

%prep
%setup -q

%build
cargo build --release

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/opt/pwasforalllinux
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps

install -m 755 target/release/pwasforalllinux %{buildroot}/opt/pwasforalllinux/
install -m 755 target/release/pwa-launcher %{buildroot}/opt/pwasforalllinux/

ln -sf /opt/pwasforalllinux/pwasforalllinux %{buildroot}/usr/bin/pwasforalllinux
ln -sf /opt/pwasforalllinux/pwa-launcher %{buildroot}/usr/bin/pwa-launcher

install -m 644 assets/icon.svg %{buildroot}/usr/share/icons/hicolor/scalable/apps/pwasforalllinux.svg

cat > %{buildroot}/usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop << 'DESKTOP'
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
DESKTOP

%post
update-desktop-database /usr/share/applications 2>/dev/null || true
gtk4-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true

%postun
update-desktop-database /usr/share/applications 2>/dev/null || true

%files
/opt/pwasforalllinux/
/usr/bin/pwasforalllinux
/usr/bin/pwa-launcher
/usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop
/usr/share/icons/hicolor/scalable/apps/pwasforalllinux.svg

%changelog
* $(date '+%a %b %d %Y') PWAsForAllLinux Team <team@pwasforalllinux.org> - ${VERSION}-1
- Initial package release

EOF
    
    # Build RPM (requires rpmbuild)
    if command -v rpmbuild &> /dev/null; then
        rpmbuild --define "_topdir $RPM_DIR" -bb "$RPM_SPEC"
        cp "$RPM_DIR/RPMS/${ARCH}/"*.rpm "$PACKAGE_DIR/"
        print_success "RPM package created"
    else
        echo "Warning: rpmbuild not found, skipping RPM package"
    fi
}

# Build AppImage
build_appimage() {
    print_step "Building AppImage..."
    
    local APPDIR="$BUILD_DIR/AppDir"
    
    mkdir -p "$APPDIR/usr/bin"
    mkdir -p "$APPDIR/usr/lib"
    mkdir -p "$APPDIR/usr/share/applications"
    mkdir -p "$APPDIR/usr/share/icons/hicolor/scalable/apps"
    
    # Copy binaries
    cp target/release/pwasforalllinux "$APPDIR/usr/bin/"
    cp target/release/pwa-launcher "$APPDIR/usr/bin/"
    
    # Create desktop entry
    cat > "$APPDIR/usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop" << EOF
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
EOF
    
    # Copy icon
    cp assets/icon.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/pwasforalllinux.svg" 2>/dev/null || true
    
    # Create AppRun
    cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/pwasforalllinux" "$@"
EOF
    chmod +x "$APPDIR/AppRun"
    
    # Create desktop entry in AppDir root
    cp "$APPDIR/usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop" "$APPDIR/pwasforalllinux.desktop"
    
    # Download appimagetool
    if ! command -v appimagetool &> /dev/null; then
        wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-${ARCH}.AppImage" -O "$BUILD_DIR/appimagetool"
        chmod +x "$BUILD_DIR/appimagetool"
        APPIMAGETOOL="$BUILD_DIR/appimagetool"
    else
        APPIMAGETOOL="appimagetool"
    fi
    
    # Build AppImage
    ARCH=$ARCH $APPIMAGETOOL "$APPDIR" "$PACKAGE_DIR/pwasforalllinux-${VERSION}-${ARCH}.AppImage"
    
    print_success "AppImage created: $PACKAGE_DIR/pwasforalllinux-${VERSION}-${ARCH}.AppImage"
}

# Main
main() {
    print_step "Building all packages..."
    
    prepare_build
    
    # Build DEB
    build_deb
    
    # Build RPM (if available)
    build_rpm
    
    # Build AppImage
    build_appimage
    
    echo ""
    print_success "All packages built successfully!"
    echo ""
    echo "Packages are available in: $PACKAGE_DIR"
    ls -la "$PACKAGE_DIR"
}

main "$@"
