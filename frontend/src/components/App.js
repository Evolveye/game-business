import React from "react"

import WS from "../WS.js"

import "./App.css"

export default class App extends React.Component {
  ref = React.createRef()
  ws = new WS( `ws://localhost:8080` )

  constructor( props ) {
    super( props )

    this.ws.emit( `test`, `wysyłam testową wiadomość` )
    this.ws.on( `test`, console.log )
  }

  render = () => <canvas ref={this.ref}/>
}