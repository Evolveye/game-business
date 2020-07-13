import React from "react"

import WS from "../WS.js"

import "./App.css"

export default class App extends React.Component {
  ref = React.createRef()
  ws = new WS( `ws://localhost:8080` )

  constructor( props ) {
    super( props )

    this.ws.emit( `ping` )
    this.ws.on( `pong`, data => {
      console.log( data )
      this.ws.emit( `searchGame`, { Square:3 } )
    } )
    this.ws.on( `founded game`, console.log )
  }

  render = () => <canvas ref={this.ref} />
}