export default class Color {
  static palette = [
    `#d84e9c`, // ~ping
    `#fb9942`, // ~orange
    `#fc3e3f`, // ~red
    `#f7f844`, // ~yellow
    `#42a65e`, // ~green
    `#0171ae`, // ~blue
    `#8e4a34`, // ~brown
    `#afdfea`, // ~cyan
  ]

  static getRandom() {
    return Math.floor( Math.random() * (0xffffff - 1) + 1 )
  }

  static getPaletteRandom() {
    return this.palette[ Math.floor( Math.random() * this.palette.length ) ]
  }
}