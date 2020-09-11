import React from "react"
import { Canvas, useThree } from "react-three-fiber"
import { OrbitControls, Box, Stars } from "drei"

import { CityTile, CornerTile, CenterTile } from "../components/Tile.js"
import WS from "../components/WS.js"
// import Color from "../utils/colors.js"

import "./App.css"

export default class App extends React.Component {
  ws = new WS( `ws://91.231.24.247:3000/` )
  cameraRef = React.createRef()

  state = {
    loadedBoard: { size:0, tiles:[] },
    active: false,
    hovered: false,
    gameBoard: {},
    spacingBetweenTiles: 0.1,
  }

  performEnum( enumObj ) {
    if (typeof enumObj === `string`) return { name:enumObj, values:[] }

    const name = Object.keys( enumObj )[ 0 ]
    let values = enumObj[ name ]

    if (!Array.isArray( values )) values = [ values ]

    return { name, values }
  }

  componentDidMount() {
    this.ws.on( `founded game`, gameBoard => {
      const { name, values } = this.performEnum( gameBoard.boardType )

      switch (name) {
        case `square`: this.loadSquareMap( gameBoard.tiles, values[ 0 ] ); break
        default: break
      }
    } )

    this.ws.emit( `searchGame`, { square:9 } )

    this.ws.on( `pong`, console.log )
    setInterval( () => this.ws.emit( `ping` ), 1000 )
  }

  loadSquareMap( tiles, size ) {
    const performedTiles = []

    for (const { typeEnum } of tiles) {
      const { name, values } = this.performEnum( typeEnum )

      switch (name) {
        case `start`:
        case `jail`:
        case `parking`:
        case `goToJail`: performedTiles.push( { type:name } ); break
        case `city`:
          performedTiles.push( {
            type:`city`, id:values[ 0 ], color:`#${values[ 1 ].toString( 16 )}`, cost:values[ 2 ], name:values[ 3 ]
          } )
          break

        default: break
      }
    }

    console.log( { size, tiles:performedTiles } )

    this.setState( { loadedBoard: { size, tiles:performedTiles } } )
  }

  render() {
    /** @type {Tile[]} */
    const boxes = []
    const { loadedBoard, spacingBetweenTiles } = this.state
    const { size, tiles } = loadedBoard
    const positionMultiplier = 1 + spacingBetweenTiles

    let lastCorner = 0
    for (let i = 0, x = 1, z = 1; i < (size ** 2 - (size - 2) ** 2); ++i) {
      // const { type, id, color, cost, name } = tiles[ i ]
      const { color, cost, name } = tiles[ i ]
      let rotate
      let isCorner = false
      let position = [
        (x - (size + 1) / 2) * positionMultiplier,
        0,
        (z - (size + 1) / 2) * positionMultiplier,
      ]

      // const paintedTiles = {}
      // let color = 0x33333333
      // if (id) {
      //   if (!(id in paintedTiles)) {
      //     let i = 0
      //     let color = Color.getPaletteRandom()

      //     while (i++ < 99 && Object.values( paintedTiles ).includes( color )) {
      //       color = Color.getPaletteRandom()
      //     }

      //     paintedTiles[ id ] = color
      //   }

      //   color = paintedTiles[ id ]
      // }

      if (true || i % (size - 1) !== 0) {
        if (x === 1)    position[ 0 ] -= .5
        if (x === size) position[ 0 ] += .5
        if (z === 1)    position[ 2 ] -= .5
        if (z === size) position[ 2 ] += .5
      }

      if (i % (size - 1) === 0) {
        isCorner = true
        lastCorner++
        rotate = lastCorner
      } else if (x !== size && z === 1) {
        rotate = 180
      } else if (x === size && z !== size) {
        rotate = 90
      } else if (x !== 1 && z === size) {
        rotate = 0
      } else if (x === 1 && z !== 1) {
        rotate = 270
      }

      if (isCorner) {
        boxes.push( <CornerTile
          isCorner={isCorner}
          key={`${x};${z}`}
          rotate={rotate}
          color={color}
          position={position}
        /> )
      } else {
        boxes.push( <CityTile
          name={name}
          cost={cost}
          isCorner={isCorner}
          key={`${x};${z}`}
          rotate={rotate}
          color={color}
          position={position}
        /> )
      }

      if (x !== size && z === 1) x++
      else if (x === size && z !== size) z++
      else if (x !== 1 && z === size) x--
      else if (x === 1 && z !== 1) z--
    }

    return <>
      <Canvas className="game_canvas">
        <Camera />
        <ambientLight />
        <pointLight position={[ 6, 2, 6 ]} />
        {boxes}
        <CenterTile args={[ 7, 1, 7 ]} position={[ 0, -.5, 0 ]} />
        <Stars />
      </Canvas>
    </>
  }
}

function Camera() {
  const { camera } = useThree()
  const isMobile = /mobile/i.test( navigator.userAgent )

  camera.position.set( 0, 10, 0 )

  return <OrbitControls rotateSpeed={isMobile ? 1.1 : 0.7} />
}