pkgname=sleepcount-git
pkgver=0.1.0
pkgrel=1
pkgdesc="A reimplementation of sleep with countdown printout"
arch=('x86_64')
url="https://github.com/jcj83429/sleepcount"
license=('GPLv3')
depends=(
  'gcc-libs'
  'glibc'
)
makedepends=('rust')
source=("git+https://github.com/jcj83429/sleepcount#branch=main")
sha512sums=('SKIP')

build() {
  cd sleepcount

  cargo build --release --locked
}

check() {
  cd sleepcount

  cargo test --release --locked
}

package() {
  cd sleepcount

  install -vDm755 -t "$pkgdir/usr/bin" target/release/sleepcount
}
