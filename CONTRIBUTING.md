# Contributing to PWAsForAllLinux

¡Gracias por tu interés en contribuir a PWAsForAllLinux! 🎉

## Formas de Contribuir

### 🐛 Reportar Bugs

Si encuentras un error, por favor [abre un issue](https://github.com/pwasforalllinux/pwasforalllinux/issues/new?labels=bug) incluyendo:

1. **Descripción clara del problema**
2. **Pasos para reproducirlo**
3. **Comportamiento esperado vs actual**
4. **Información del sistema:**
   - Distribución Linux
   - Entorno de escritorio
   - Versión de PWAsForAllLinux
   - Versión de GTK (`gtk4-launch --version`)

### 💡 Sugerir Funcionalidades

¿Tienes una idea para mejorar PWAsForAllLinux? [Abre un issue](https://github.com/pwasforalllinux/pwasforalllinux/issues/new?labels=enhancement) con:

1. **Descripción clara de la funcionalidad**
2. **Por qué sería útil**
3. **Ejemplos de uso** (si aplica)

### 📝 Mejorar Documentación

La documentación siempre puede ser mejorada:

- Corregir errores tipográficos
- Añadir ejemplos
- Traducir a otros idiomas
- Mejorar explicaciones

### 🔧 Contribuir Código

#### Configurar el Entorno de Desarrollo

```bash
# 1. Clonar el repositorio
git clone https://github.com/pwasforalllinux/pwasforalllinux.git
cd pwasforalllinux

# 2. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Instalar dependencias (Ubuntu/Debian)
sudo apt install build-essential libgtk-4-dev libwebkit2gtk-4.1-dev

# 4. Compilar
cargo build

# 5. Ejecutar tests
cargo test
```

#### Flujo de Trabajo

1. **Fork** el repositorio
2. **Crea una rama** para tu cambio:
   ```bash
   git checkout -b feature/mi-nueva-funcionalidad
   ```
3. **Haz tus cambios** siguiendo las convenciones del código
4. **Ejecuta los tests**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
5. **Commit** con mensajes descriptivos:
   ```bash
   git commit -m "feat: añadir soporte para notificaciones"
   ```
6. **Push** a tu fork:
   ```bash
   git push origin feature/mi-nueva-funcionalidad
   ```
7. **Abre un Pull Request**

#### Convenciones de Código

- **Rust**: Seguir [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Commits**: Usar [Conventional Commits](https://www.conventionalcommits.org/)
  - `feat:` nueva funcionalidad
  - `fix:` corrección de bug
  - `docs:` documentación
  - `refactor:` refactorización
  - `test:` tests
  - `chore:` mantenimiento

#### Estructura del Proyecto

```
src/
├── main.rs      # Punto de entrada, inicialización GTK
├── app.rs       # Interfaz gráfica principal
├── config.rs    # Gestión de configuración
├── pwa.rs       # Lógica de PWAs
├── utils.rs     # Funciones de utilidad
└── launcher.rs  # Lanzador de PWAs individuales
```

### 🌍 Traducir

Ayuda a traducir PWAsForAllLinux a tu idioma:

1. Únete a nuestro proyecto en [Crowdin](https://crowdin.com/project/pwasforalllinux)
2. O crea un archivo de traducción en `locales/{idioma}/LC_MESSAGES/`

## Código de Conducta

- Sé respetuoso e inclusivo
- Acepta críticas constructivas
- Enfócate en lo mejor para la comunidad
- Muestra empatía hacia otros miembros

## Preguntas

¿Dudas? Abre un issue con la etiqueta `question` o únete a nuestras discusiones.

---

¡Gracias por contribuir! 💜
