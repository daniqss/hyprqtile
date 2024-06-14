# Maintainer: daniqss <danielqueijo14@gmail.com>
pkgname=hyprqtile
pkgver=0.1.0
pkgrel=1
pkgdesc="Qtile-like workspaces and monitors management for the Hyprland compositor"
arch=('x86_64')
url="https://github.com/daniqss/hyprqtile"
license=('GPL-3.0 license')
depends=('gcc-libs' 'glibc')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/daniqss/$pkgname/archive/$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/hyprqtile" "$pkgdir/usr/bin/hyprqtile"
}
