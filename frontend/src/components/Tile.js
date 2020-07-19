import React from "react"
import { Html, Text, Box } from "drei"

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

  render() {
    const { position, sizes, isCorner } = this.props
    const size = sizes || (isCorner ? [ 2, .5, 2 ] : [1, .5, 2])
    const { rotate, color } = this.state

    return <mesh
      // scale={[ scale, scale, scale ]}
      rotation={[ 0, isCorner ? 0 : Math.PI / 180 * rotate, 0 ]}
      ref={this.mesh}
      position={position}
      onClick={this.onclick}
    >
      <Text
        scale={[ 1, -1, 1 ]}
        rotation={[ Math.PI / 180 * 90, 0, isCorner ? Math.PI / 180 * (rotate * 90 + 45) : 0 ]}
        fontSize={0.5}
        position={[ 0, 0.3, 0 ]}
        color="black"
      >
        Text
      </Text>
      <boxBufferGeometry attach="geometry" args={size} />
      <meshStandardMaterial
        attach="material"
        color={color || this.getRandomColor()}
      />
    </mesh>
  }
}

export class TileToBuy extends Tile {
  render() {
    const { position, isCorner } = this.props
    const { rotate, color } = this.state
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
        <meshStandardMaterial attach="material" color={0xeeeeee} />
      </Box>
      <Box args={[ 1, .1, 0.3 ]} position={[ 0, 0, -0.8 ]}>
        <meshStandardMaterial attach="material" color={color} />
      </Box>
    </group>
  }
}

export class CornerTile extends Tile {
  render() {
    const { position, isCorner } = this.props
    const { rotate } = this.state
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
      <Text {...textAttrs} position={[ 0, 0.06, 0 ]}>Text</Text>
      <Box args={[ 2, .1, 2 ]}>
        <meshStandardMaterial attach="material" color={0x333333} />
      </Box>
    </group>
  }
}