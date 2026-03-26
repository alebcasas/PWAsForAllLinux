<p align="center">
  <img src="assets/icon.svg" alt="PWAsForAllLinux Logo" width="128" height="128">
</p>

<h1 align="center">PWAsForAllLinux</h1>

<p align="center">
  <strong>🎨 Convierte cualquier sitio web en una aplicación nativa de Linux</strong>
</p>

<p align="center">
  <a href="#-qué-es-pwasforalllinux">¿Qué es?</a> •
  <a href="#-instalación">Instalación</a> •
  <a href="#-uso">Uso</a> •
  <a href="#-preguntas-frecuentes">FAQ</a> •
  <a href="#-contribuir">Contribuir</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/platform-Linux-orange" alt="Platform">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  <img src="https://img.shields.io/badge/arch-x86__64%20|%20ARM64-red" alt="Architecture">
</p>

---

## 🌟 ¿Qué es PWAsForAllLinux?

**PWAsForAllLinux** es una aplicación que te permite instalar y usar **Progressive Web Apps (PWAs)** como si fueran aplicaciones nativas en tu sistema Linux.

### ¿Qué es una PWA?

Una **Progressive Web App** es un sitio web que se comporta como una aplicación:
- ✅ Se abre en su propia ventana (sin pestañas ni barra de direcciones)
- ✅ Tiene su propio icono en el menú de aplicaciones
- ✅ Se puede anclar al dock/panel
- ✅ Funciona de forma aislada de tu navegador principal

### ¿Por qué PWAsForAllLinux?

| Característica | PWAsForAllLinux | Otros |
|----------------|-----------------|-------|
| Compatible con TODAS las distribuciones Linux | ✅ | ❌ |
| Compatible con TODOS los entornos de escritorio | ✅ | ❌ |
| No depende de un navegador específico | ✅ | ❌ |
| Código abierto y gratuito | ✅ | ❌ |
| Perfiles aislados por PWA | ✅ | ❌ |

---

## 📦 Instalación

### ✅ Requisitos

| Requisito | Detalle |
|-----------|---------|
| **Sistema Operativo** | Cualquier distribución Linux de 64 bits |
| **Arquitectura** | x86_64 o ARM64 |
| **RAM** | Mínimo 2 GB (recomendado 4 GB) |
| **Espacio en disco** | Mínimo 100 MB |
| **Entorno de escritorio** | GNOME, KDE, XFCE, MATE, Cinnamon, LXQt, Budgie, i3, Sway, o cualquier otro |

---

### 🚀 Instalación (3 Pasos Simples)

#### Paso 1: Abrir Terminal

Abre una terminal en tu sistema:
- **GNOME/KDE/XFCE:** Presiona `Ctrl + Alt + T`
- **O busca:** "Terminal" en el menú de aplicaciones

#### Paso 2: Descargar el Instalador

Copia y pega este comando en la terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/alebcasas/PWAsForAllLinux/main/scripts/install.sh -o install.sh
```

> 💡 Este comando descarga el archivo de instalación automáticamente.

#### Paso 3: Ejecutar la Instalación

Ejecuta estos dos comandos:

```bash
chmod +x install.sh
sudo ./install.sh
```

> ⚠️ Se te pedirá tu contraseña de administrador. Es normal y necesario para instalar la aplicación.

#### ✅ ¡Listo!

**PWAsForAllLinux** ya está instalado. Búscalo en el menú de aplicaciones de tu sistema.

---

### 🔄 Actualizar

Para actualizar a una nueva versión, simplemente ejecuta el instalador de nuevo:

```bash
sudo ./install.sh
```

> 💡 Tus PWAs instaladas se conservarán automáticamente.

---

### 🗑️ Desinstalar

Para eliminar PWAsForAllLinux de tu sistema:

```bash
sudo ./install.sh --uninstall
```

---

## 🎮 Uso

### Abrir la Aplicación

1. Abre el **menú de aplicaciones** de tu sistema
2. Busca **"PWAsForAllLinux"**
3. Haz clic en el icono

### Instalar tu Primera PWA

#### Método 1: Detección Automática (Recomendado)

1. En la ventana principal, haz clic en **"Add PWA"** en el panel lateral
2. Ingresa la **URL** del sitio web (ejemplo: `https://web.whatsapp.com`)
3. Haz clic en **"Auto-detect from URL"**
4. La aplicación automáticamente:
   - 📋 Obtiene el nombre del sitio
   - 🖼️ Descarga el icono
   - ⚙️ Configura los ajustes óptimos
5. Haz clic en **"Install PWA"**
6. ¡La aplicación aparecerá en tu menú de aplicaciones!

#### Método 2: Configuración Manual

1. Haz clic en **"Add PWA"**
2. Completa los campos manualmente:
   - **Name:** El nombre que aparecerá en el menú
   - **URL:** La dirección del sitio web
   - **Width/Height:** Tamaño de la ventana
   - **Display Mode:** 
     - `Standalone` (recomendado): Ventana sin interfaz de navegador
     - `Minimal UI`: Ventana con botones mínimos
     - `Fullscreen`: Pantalla completa

### Ejecutar una PWA Instalada

Hay tres formas de ejecutar tus PWAs:

1. **Desde el menú de aplicaciones:** Busca el nombre de la PWA
2. **Desde PWAsForAllLinux:** Haz clic en "Launch" junto a la PWA
3. **Desde la terminal:** `pwa-launcher <id-de-tu-pwa>`

### Gestionar PWAs

Desde la ventana principal puedes:

| Acción | Descripción |
|--------|-------------|
| 🚀 **Launch** | Abrir la PWA |
| 🗑️ **Delete** | Eliminar la PWA del sistema (con confirmación) |

> 📝 **Nota:** La funcionalidad de edición de PWAs estará disponible en futuras versiones.

### Configuración

En la pestaña **"Settings"** puedes ajustar:

- **Browser Engine:** Motor de renderizado (WebKitGTK es el predeterminado)
- **Hardware Acceleration:** Aceleración por GPU (mejor rendimiento)

> 📝 **Nota:** Las funcionalidades de notificaciones y temas personalizados estarán disponibles en futuras versiones.

---

## 🌐 PWAs Populares

Aquí tienes algunos sitios web que funcionan excelentemente como PWAs:

| Aplicación | URL | Categoría |
|------------|-----|-----------|
| WhatsApp Web | `https://web.whatsapp.com` | Mensajería |
| Telegram Web | `https://web.telegram.org` | Mensajería |
| Discord Web | `https://discord.com/app` | Comunicación |
| Spotify Web | `https://open.spotify.com` | Música |
| YouTube Music | `https://music.youtube.com` | Música |
| Netflix | `https://www.netflix.com` | Video |
| Google Docs | `https://docs.google.com` | Productividad |
| Notion | `https://notion.so` | Productividad |
| Figma | `https://figma.com` | Diseño |
| Canva | `https://canva.com` | Diseño |
| Twitter/X | `https://twitter.com` | Redes Sociales |
| Reddit | `https://reddit.com` | Redes Sociales |
| GitHub | `https://github.com` | Desarrollo |

---

## ❓ Preguntas Frecuentes

### General

<details>
<summary><strong>¿En qué se diferencia de instalar PWAs en Chrome/Firefox?</strong></summary>

PWAsForAllLinux es **independiente del navegador**. Las ventajas son:

- Si cambias o eliminas tu navegador, tus PWAs siguen funcionando
- Cada PWA tiene su propio perfil aislado
- No dependes de las políticas de un navegador específico
- Funciona con cualquier entorno de escritorio

</details>

<details>
<summary><strong>¿Puedo usar PWAsForAllLinux sin saber programación?</strong></summary>

¡Sí! PWAsForAllLinux está diseñado para usuarios de todos los niveles. Solo necesitas:

1. Saber copiar y pegar comandos en la terminal (para instalar)
2. Saber hacer clic en botones (para usar la aplicación)

Todo tiene una interfaz gráfica intuitiva.

</details>

<details>
<summary><strong>¿Es seguro instalar PWAs de cualquier sitio web?</strong></summary>

PWAsForAllLinux aísla cada aplicación en su propio perfil. Esto significa:

- Cada PWA tiene su propio almacenamiento separado
- Las cookies y datos no se comparten entre PWAs
- Puedes eliminar una PWA sin afectar a las demás

Siempre instala PWAs de sitios web en los que confíes.

</details>

### Problemas Comunes

<details>
<summary><strong>La aplicación no se abre después de instalar</strong></summary>

1. Verifica que las dependencias estén instaladas:

```bash
# Ubuntu/Debian
sudo apt install libgtk-4-1 libwebkit2gtk-4.1-0

# Fedora
sudo dnf install gtk4 webkit2gtk4.1
```

2. Intenta ejecutar desde la terminal para ver errores:

```bash
pwasforalllinux
```

</details>

<details>
<summary><strong>El icono de la PWA no aparece en el menú</strong></summary>

Ejecuta este comando para actualizar la caché del menú:

```bash
update-desktop-database ~/.local/share/applications
```

Si el problema persiste, cierra sesión y vuelve a iniciar.

</details>

<details>
<summary><strong>¿Cómo desinstalo PWAsForAllLinux?</strong></summary>

```bash
sudo ./install.sh --uninstall
```

O manualmente:

```bash
sudo rm -rf /opt/pwasforalllinux
sudo rm /usr/local/bin/pwasforalllinux
sudo rm /usr/local/bin/pwa-launcher
sudo rm /usr/share/applications/com.pwasforalllinux.PWAsForAllLinux.desktop
```

</details>

<details>
<summary><strong>¿Cómo actualizo a una nueva versión?</strong></summary>

Simplemente ejecuta el instalador de nuevo:

```bash
sudo ./install.sh
```

Tus PWAs instaladas se conservarán.

</details>

---

## 🛠️ Compilar desde el Código Fuente

Si prefieres compilar la aplicación tú mismo:

### Requisitos

- **Rust** 1.70 o superior
- **GTK4** development libraries (versión 4.8+)
- **WebKitGTK 6.0** development libraries

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/alebcasas/PWAsForAllLinux.git
cd PWAsForAllLinux

# 2. Instalar Rust (si no está instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Instalar dependencias según tu distribución
# Ubuntu/Debian:
sudo apt install build-essential libgtk-4-dev libwebkitgtk-6.0-dev

# Fedora:
sudo dnf install @development-tools gtk4-devel webkitgtk6.0-devel

# Arch Linux:
sudo pacman -S base-devel gtk4 webkitgtk-6.0

# 4. Compilar
cargo build --release

# 5. Instalar
sudo ./scripts/install.sh --build
```

### Notas Técnicas

Este proyecto utiliza:
- **GTK4** (versión 0.8) para la interfaz gráfica
- **WebKit6** (versión 0.3) para el renderizado web
- **glib/gio** (versión 0.18) para funcionalidades del sistema

La migración de WebKit2GTK (GTK3) a WebKit6 (GTK4) garantiza compatibilidad total con las últimas versiones de GTK y mejor rendimiento.

---

## 🤝 Contribuir

¡Nos encantaría tu ayuda! Hay muchas formas de contribuir:

### Reportar Problemas

¿Encontraste un error? [Abre un issue](https://github.com/alebcasas/PWAsForAllLinux/issues)

### Sugerir Funcionalidades

¿Tienes una idea genial? [Compártela](https://github.com/alebcasas/PWAsForAllLinux/issues/new?labels=enhancement)

### Contribuir Código

1. Haz un fork del repositorio
2. Crea una rama: `git checkout -b mi-funcionalidad`
3. Haz tus cambios
4. Envía un pull request

Consulta nuestra [Guía de Contribución](CONTRIBUTING.md) para más detalles.

### Traducir

Ayuda a traducir PWAsForAllLinux a tu idioma.

---

## 📄 Licencia

PWAsForAllLinux es software libre bajo la **Licencia MIT**. Puedes:

- ✅ Usarlo libremente
- ✅ Modificarlo
- ✅ Distribuirlo
- ✅ Usarlo comercialmente

Ver el archivo [LICENSE](LICENSE) para más detalles.

---

## 🙏 Agradecimientos

- [GTK Project](https://gtk.org/) - Framework de interfaz gráfica
- [WebKitGTK](https://webkitgtk.org/) - Motor de renderizado web
- [Rust](https://rust-lang.org/) - Lenguaje de programación
- Todos los [contribuidores](https://github.com/alebcasas/PWAsForAllLinux/graphs/contributors)

---

<p align="center">
  <strong>Hecho con ❤️ para la comunidad Linux</strong>
</p>

<!-- Updated: Links corrected to alebcasas/PWAsForAllLinux -->

<p align="center">
  <a href="https://github.com/alebcasas/PWAsForAllLinux">GitHub</a> •
  <a href="https://github.com/alebcasas/PWAsForAllLinux/wiki">Documentación</a> •
  <a href="https://github.com/alebcasas/PWAsForAllLinux/releases">Descargas</a>
</p>