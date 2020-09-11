import React from "react"
import { Text, Box } from "drei"
import * as THREE from "three"
import imgTile from "../images/test.png"
import imgTileSide from "../images/tile-side.png"
import imgFrame from "../images/frame.png"
import imgSand from "../images/sand.png"
import imgGrass from "../images/grass.png"

class Tile extends React.Component {
  height = 1
  // This reference will give us direct access to the mesh
  mesh = React.createRef()

  state = {
    color: this.props.color || 0xaaaaaa,
    rotate: this.props.rotate || 0,
  }

  active = false

  /** @type {THREE.MeshBasicMaterial[]} */
  materialArray = [
    new THREE.MeshBasicMaterial( { color:`#dacfd7` } ),
    new THREE.MeshBasicMaterial( { color:`#dacfd7` } ),
    new THREE.MeshBasicMaterial( { color:`#ffffff` } ),
    new THREE.MeshBasicMaterial( { color:`#000` } ),
    new THREE.MeshBasicMaterial( { color:`#dacfd7` } ),
    new THREE.MeshBasicMaterial( { color:`#dacfd7` } ),
  ]

  events = {
    onPointerDown: e => {
      e.stopPropagation()

      if (!this.active) {
        this.active = true

        e.eventObject.position.y = -0.02
      }
    },
    onPointerUp: e => {
      e.stopPropagation()

      if (this.active) {
        this.active = false
        e.eventObject.position.y = 0.03
      }
    },
    onPointerOver: e => {
      e.stopPropagation()
      e.eventObject.position.y = 0.03
    },
    onPointerOut: e => {
      e.stopPropagation()
      e.eventObject.position.y = 0

      this.active = false
    }
  }

  componentDidMount() {
    // requestAnimationFrame( this.update )
  }

  update = () => {
    requestAnimationFrame( this.update )
  }

  getRandomColor() {
    return Math.floor( Math.random() * 0xffffff )
  }
}

export class CityTile extends Tile {
  render() {
    const { position, isCorner, name, cost } = this.props
    const { rotate, color } = this.state
    const loader = new THREE.TextureLoader()
    // const material = new THREE.MeshF
    const texTile = loader.load( imgTile )
    const texTileSide = loader.load( imgTileSide )
    const texFrame = loader.load( imgFrame )
    const textAttrs = {
      scale: [ 1, -1, 1 ],
      rotation: [ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ],
      color: `black`,
      letterSpacing: 0.1,
      font: `https://fonts.gstatic.com/s/raleway/v14/1Ptrg8zYS_SKggPNwK4vaqI.woff`,
      fontSize: name.length > 19
        ? 0.05
        : name.length < 8
        ? 0.15
        : 0.1,
    }

    this.materialArray[ 0 ].map = texTileSide
    this.materialArray[ 1 ].map = texTileSide
    this.materialArray[ 2 ].map = texTile
    this.materialArray[ 4 ].map = texTileSide
    this.materialArray[ 5 ].map = texTileSide

    return <group
      {...this.events}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={[...position]}
    >
      <Text {...textAttrs} position={[ 0, this.height / 2 + 0.01, -0.5 ]}>{name}</Text>
      <Text {...textAttrs} fontSize={0.15} position={[ 0, this.height / 2 + 0.01, 0.6 ]}>{cost.toString()}$</Text>
      <Box material={this.materialArray} args={[ 1, this.height, 1.6 ]} position={[ 0, 0, 0.2 ]} />
      <Box args={[ 1, this.height, 0.4 ]} position={[ 0, 0, -0.8 ]}>
        <meshBasicMaterial attach="material" map={texFrame} color={color} />
      </Box>
    </group>
  }
}

export class CornerTile extends Tile {
  render() {
    const { position, isCorner } = this.props
    const { rotate } = this.state
    const loader = new THREE.TextureLoader()
    const texSand = loader.load( imgSand )
    const texTileSide = loader.load( imgTileSide )
    const textAttrs = {
      scale: [ 1, -1, 1 ],
      rotation: [ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ],
      color: `black`,
      fontSize: 0.5,
    }
    this.materialArray[ 0 ].map = texTileSide
    this.materialArray[ 1 ].map = texTileSide
    this.materialArray[ 2 ].map = texSand
    this.materialArray[ 4 ].map = texTileSide
    this.materialArray[ 5 ].map = texTileSide

    return <group
      {...this.events}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={position}
    >
      <Text {...textAttrs} position={[ 0, this.height / 2 + 0.1, 0 ]}>Pustynia</Text>
      <Box args={[ 2, this.height, 2 ]} material={this.materialArray} />
    </group>
  }
}

export class CenterTile extends Tile {
  render() {
    const { position, args } = this.props
    const { rotate } = this.state
    const loader = new THREE.TextureLoader()
    const texGrass = loader.load( imgGrass )
    const texTileSide = loader.load( imgTileSide )
    this.materialArray[ 0 ].map = texTileSide
    this.materialArray[ 1 ].map = texTileSide
    this.materialArray[ 2 ].map = texGrass
    this.materialArray[ 4 ].map = texTileSide
    this.materialArray[ 5 ].map = texTileSide

    return <group
      position={position}
    >
      <Box
        args={args}
        material={this.materialArray}
      />
    </group>
  }
}