import React from "react"
import { Text } from "drei"

export class Box extends React.Component {
  // This reference will give us direct access to the mesh
  mesh = React.createRef()

  componentDidMount() {
    requestAnimationFrame( this.update )
  }

  update = () => {
    // /** @type {import("three").Mesh} */
    // const cube = this.mesh.current

    // cube.rotateY( 0.01 )
    // cube.position.z += Math.sin( cube.position.z + 1 ) * 0.01

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
    const { position, color, sizes } = this.props

    return <mesh
      // scale={[ scale, scale, scale ]}
      ref={this.mesh}
      position={position}
      onClick={this.onclick}
    >
      <Text>Test</Text>
      <boxBufferGeometry attach="geometry" args={sizes || [1, .5, 2]} />
      <meshStandardMaterial
        attach="material"
        color={color || this.getRandomColor()}
      />
    </mesh>
  }
}