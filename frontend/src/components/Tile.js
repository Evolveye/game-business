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
  }

  componentDidMount() {
    requestAnimationFrame( this.update )
  }

  update = () => {
    // /** @type {import("three").Mesh} */
    // const cube = this.mesh.current

    // cube.rotateY( 0.01 )
    // cube.position.z += Math.sin( cube.position.z + 1 ) * 0.01

    // this.setState( ({ rotate }) => ({ rotate:rotate + 0.5 }) )

    requestAnimationFrame( this.update )
  }

  getRandomColor() {
    return Math.floor( Math.random() * 0xffffff )
  }

  onclick = e => {
    e.stopPropagation()

    this.setState( { color:this.getRandomColor() } )
  }
}

export class TileToBuy extends Tile {
  render() {
    const { position, isCorner } = this.props
    const { rotate, color } = this.state
    const loader = new THREE.TextureLoader()
    const texTile = loader.load( imgTile )
    const textAttrs = {
      scale: [ 1, -1, 1 ],
      rotation: [ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ],
      color: `black`,
      fontSize: 0.2,
    }

    return <group
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      position={position}
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