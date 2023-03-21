# Maintainer: Kleidi Bujari <dev at kleidi dot ca>
pkgname=rorrim
pkgver=0.1.0
pkgrel=1
pkgdesc="Mirrorlist generator for Arch Linux"
arch=('x86_64')
url="https://github.com/kbzt/rorrim"
license=('MIT')
makedepends=('cargo' 'git')
source=("${pkgname}::git+https://github.com/kbzt/rorrim.git")
sha256sums=('SKIP')

pkgver() {
  sed -n '/^version/p' ${pkgname}/Cargo.toml | cut -d "\"" -f 2
}

build(){
  cd "${pkgname}"

  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target

  cargo update
  cargo fetch --locked --target "${CARCH}-unknown-linux-gnu"
  cargo build --release
}

package(){
  cd "${pkgname}"

  install -Dm 755 "target/release/rorrim" -t "${pkgdir}/usr/bin/"
  install -Dm 644 "README.md" -t "${pkgdir}/usr/share/doc/${pkgname}"
  install -Dm 644 "LICENSE" -t "${pkgdir}/usr/share/licenses/${pkgname}"
}
