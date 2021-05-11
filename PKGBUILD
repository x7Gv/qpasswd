# Maintainer: Oskari J. Manninen <svn.gv@protonmail.com>
pkgname=qpasswd
pkgver=0.4.0
pkgrel=1
pkgdesc="Official qpasswd pakage"
arch=(x86_64)
url="https://gitlab.com/x7Gv/qpasswd.git"
license=('MIT')
depends=('gcc-libs')
makedepends=('git' 'cargo')
checkdepends=()
optdepends=()
source=("git+$url")
noextract=()
md5sums=('SKIP')

build() {
    RUSTUP_TOOLCHAIN=stable cargo build --release --locked --all-features --target-dir=target
}

package() {
    install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
