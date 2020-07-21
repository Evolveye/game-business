import React from "react"
import { Text, Box } from "drei"
import * as THREE from "three"
import imgTile from "./test.png"
import imgSand from "./sand.png"

class Tile extends React.Component {
  // This reference will give us direct access to the mesh
  mesh = React.createRef()

  state = {
    color: this.props.color || 0xaaaaaa,
    rotate: this.props.rotate || 0,
    y: 0,
  }

  active = false

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

export class TileToBuy extends Tile {
  render() {
    const { position, isCorner } = this.props
    const { rotate, color, y } = this.state
    const loader = new THREE.TextureLoader()
    const texTile = loader.load( imgTile )
    const textAttrs = {
      scale: [ 1, -1, 1 ],
      rotation: [ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ],
      color: `black`,
      fontSize: 0.2,
    }

    position[ 1 ] += y
    console.log( position )

    return <group
      {...this.events}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={[...position]}
    >
      <Text {...textAttrs} position={[ 0, 0.06, -0.5 ]}>Miasto</Text>
      <Text {...textAttrs} position={[ 0, 0.06, 0.8 ]}>Cena $</Text>
      <Box args={[ 1, .1, 1.6 ]} position={[ 0, 0, 0.2 ]}>
        <meshMatcapMaterial attach="material" map={texTile} color={0xeeeeee} />
      </Box>
      <Box args={[ 1, .1, 0.4 ]} position={[ 0, 0, -0.8 ]}>
        <meshBasicMaterial attach="material" color={color} />
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