import React, { useRef, useEffect } from "react"
import { Canvas, useThree } from "react-three-fiber"

import { Box } from "./models.js"
import WS from "../WS.js"

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
        case `city`: performedTiles.push( { type:`city`, id:values[ 0 ], name:values[ 1 ] } ); break
      }
    }

    console.log( { size, tiles:performedTiles } )

    this.setState( { loadedBoard: { size, tiles:performedTiles } } )
  }

  render() {
    /** @type {Box[]} */
    const boxes = []
    const { loadedBoard } = this.state
    const { size, tiles } = loadedBoard
    const paintedTiles = {}

    for (let i = 0, x = 1, z = 1; i < (size ** 2 - (size - 2) ** 2); ++i) {
      const { id, type, name } = tiles[ i ]
      let color = 0xffffff

      console.log( { x, z, type, id, name } )

      if (id) {
        if (!(id in paintedTiles)) paintedTiles[ id ] = Math.floor( Math.random() * 0xffffff )

        color = paintedTiles[ id ]
      }

      boxes.push( <Box
        key={`${x};${z}`}
        color={color}
        position={[
          -size / 2 + x,
          -(x + z) * 0 - 3,
          -size / 2 + z,
        ]}
      /> )

      if (x !== size && z === 1) x++
      else if (x === size && z !== size) z++
      else if (x !== 1 && z === size) x--
      else if (x === 1 && z !== 1) z--
    }

    return <>
      <Canvas className="game_canvas">
        <ambientLight />
        <Camera />
        <pointLight position={[ 0, 2, 6 ]} />
        {boxes}
      </Canvas>
    </>
  }
}

function Camera() {
  const cameraRef = useRef()
  const { setDefaultCamera } = useThree()

  useEffect( () => {
    cameraRef.current.lookAt( 0, 0, 0 )

    setDefaultCamera( cameraRef.current )
  } )

  return <perspectiveCamera ref={cameraRef}
    position={[ 0, 5, 10 ]}
  />
}