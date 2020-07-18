import React from "react"
// import { useFrame } from "react-three-fiber"

export class Box extends React.Component {
  // This reference will give us direct access to the mesh
  mesh = React.createRef()

  componentDidMount() {
    requestAnimationFrame( this.update )
  }

  update = () => {
    /** @type {import("three").Mesh} */
    const cube = this.mesh.current

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
    const { position, color } = this.props

    return <mesh
      scale={[ .9, .9, .9 ]}
      ref={this.mesh}
      position={position}
      onClick={this.onclick}
    >
      <boxBufferGeometry attach="geometry" args={[1, .5, 1]} />
      <meshStandardMaterial
        attach="material"
        color={color || this.getRandomColor()}
      />
    </mesh>
  }
}