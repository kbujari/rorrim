# Maintainer: Kleidi Bujari <arch@kleidi.ca>
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
  cargo build --release --locked
}

package(){
  install -Dm755 "${pkgname}/target/release/rorrim" "${pkdir}/usr/bin/rorrim"

  install -Dm644 "README.md" "${pkgdir}/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE" "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
