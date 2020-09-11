import React from "react"
import { Text, Box } from "drei"
import * as THREE from "three"
import imgTile from "./test.png"
import imgFrame from "./frame.png"
import imgSand from "./sand.png"

class Tile extends React.Component {
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
    new THREE.MeshBasicMaterial( { color:`#dacfd7` } ),
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

    this.materialArray[ 2 ].map = texTile

    return <group
      {...this.events}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={[...position]}
    >
      <Text {...textAttrs} position={[ 0, 0.06, -0.5 ]}>{name}</Text>
      <Text {...textAttrs} fontSize={0.15} position={[ 0, 0.06, 0.6 ]}>{cost.toString()}$</Text>
      <Box material={this.materialArray} args={[ 1, .1, 1.6 ]} position={[ 0, 0, 0.2 ]} />
      <Box args={[ 1, .1, 0.4 ]} position={[ 0, 0, -0.8 ]}>
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
    const textAttrs = {
      scale: [ 1, -1, 1 ],
      rotation: [ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ],
      color: `black`,
      fontSize: 0.5,
    }

    return <group
      {...this.events}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={position}
    >
      <Text {...textAttrs} position={[ 0, 0.06, 0 ]}>Pustynia</Text>
      <Box args={[ 2, .1, 2 ]}>
        <meshStandardMaterial attach="material" map={texSand} />
      </Box>
    </group>
  }
}