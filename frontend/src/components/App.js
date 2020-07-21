import React from "react"
import { Canvas, useThree } from "react-three-fiber"
import { OrbitControls, Box } from "drei"

import { TileToBuy, CornerTile } from "./Tile.js"
import WS from "../utils/WS.js"
import Color from "../utils/colors.js"

import "./App.css"

export default class App extends React.Component {
  ws = new WS( `ws://localhost:8080` )
  cameraRef = React.createRef()

  state = {
    loadedBoard: { size:0, tiles:[] },
    active: false,
    hovered: false,
    gameBoard: {},
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
        case `square`: this.loadSquareMap( gameBoard.tiles, values[ 0 ] )
      }
    } )

    this.ws.emit( `searchGame`, { square:5 } )
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
        case `city`: performedTiles.push( {
          type:`city`, id:values[ 0 ], color:values[ 1 ], name:values[ 2 ]
        } ); break
      }
    }

    console.log( { size, tiles:performedTiles } )

    this.setState( { loadedBoard: { size, tiles:performedTiles } } )
  }

  render() {
    /** @type {Tile[]} */
    const boxes = []
    const { loadedBoard } = this.state
    const { size, tiles } = loadedBoard
    const paintedTiles = {}
    const positionMultiplier = 1.1

    let lastCorner = 0
    for (let i = 0, x = 1, z = 1; i < (size ** 2 - (size - 2) ** 2); ++i) {
      const { id, color, type, name } = tiles[ i ]
      let rotate
      let isCorner = false
      let position = [
        (x - (size + 1) / 2) * positionMultiplier,
        0,
        (z - (size + 1) / 2) * positionMultiplier,
      ]

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

      if (true || i % 4 !== 0) {
        if (x === 1) position[ 0 ] -= .5
        if (x === size) position[ 0 ] += .5
        if (z === 1) position[ 2 ] -= .5
        if (z === size) position[ 2 ] += .5
      }

      if (i % 4 === 0) {
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
        boxes.push( <TileToBuy
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
        <ambientLight />
        <Camera />
        <pointLight position={[ 6, 2, 6 ]} />
        {boxes}
        {/* <Box args={[ 7.5, 0.1, 7.5 ]} position={[ 0, -0.1, 0 ]}> */}
        <Box
          args={[ 20, 0.1, 20 ]}
          position={[ 0, -0.05, 0 ]}
          onPointerOver={e => e.stopPropagation()}
          onPointerOut={e => e.stopPropagation()}
        >
          <lineBasicMaterial attach="material" color={0x202020} />
        </Box>
      </Canvas>
    </>
  }
}

function Camera() {
  const { camera } = useThree()

  camera.position.set( 0, 10, 0 )

  return <OrbitControls />
}