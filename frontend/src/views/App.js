import React from "react"
import { Canvas, useThree } from "react-three-fiber"
import { OrbitControls, Stars } from "drei"

import { CityTile, CornerTile, CenterTile } from "../components/Tile.js"
import WS from "../components/WS.js"
// import Color from "../utils/colors.js"

import "./App.css"

export default class App extends React.Component {
  ws = new WS( `ws://91.231.24.247:3000/` )
  cameraRef = React.createRef()

  state = {
    loadedBoard: { size:0, tiles:[] },
    gameBoard: {},
    spacingBetweenTiles: 0.1,
  }
  tiles = []
  players = []
  player = null

  performEnum( enumObj ) {
    if (typeof enumObj === `string`) return { name:enumObj, values:[] }

    const name = Object.keys( enumObj )[ 0 ]
    let values = enumObj[ name ]

    if (!Array.isArray( values )) values = [ values ]

    return { name, values }
  }

  componentDidMount() {
    this.ws.on( `founded game`, data => {
      const { playerId, boardData } = data
      const { boardType, players, tiles } = boardData
      const { name, values } = this.performEnum( boardType )

      this.setState( { player:players.find( p => p.id === playerId ), players } )

      switch (name) {
        case `square`: this.loadSquareMap( tiles, values[ 0 ] ); break
        default: break
      }
    } )

    this.ws.on( `move`, newTileIndex => {
      if (typeof newTileIndex != `number`) return console.log( newTileIndex )
      const { player } = this.state
      const tileFrom = this.tiles[ player.tileIndex ]
      const tileTo = this.tiles[ newTileIndex ]

      tileFrom.removePlayer( player )
      tileTo.addPlayer( player )

      player.tileIndex = newTileIndex
    } )

    this.ws.emit( `searchGame`, { square:9 } )
    setInterval( () => this.ws.emit( `move`, this.state.player.boardId ), 1000 )

    window.game = this
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

    this.setState( { loadedBoard: { size, tiles:performedTiles } } )
  }

  render() {
    /** @type {Tile[]} */
    const boxes = []
    const { loadedBoard, spacingBetweenTiles, players } = this.state
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

      const commonData = {
        key: i, //`${x};${z}`,
        ref: ref => this.tiles[ i ] = ref,
        players: players.filter( p => p.tileIndex === i ),
        color,
        position,
        rotate,
        isCorner,
      }

      if (isCorner) {
        boxes.push( <CornerTile {...commonData} /> )
      } else {
        boxes.push( <CityTile {...commonData} name={name} cost={cost} /> )
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