export default class RoomedWebSocked extends WebSocket {
  #listeners = new Map()

  emit( event, data ) {
    const msg = data ? { [event]:data } : event
    const send = () => this.send( JSON.stringify( msg ) )

    if (this.readyState !== 1) {
      this.addEventListener( `open`, send )
    } else {
      send()
    }
  }

  on( event, listener ) {
    this.#listeners.set( event, listener )
  }

  onmessage = data => console.log( data )
  // onmessage = ({ data:jsonData }) => {
  //   const { event, data } = JSON.parse( jsonData )

  //   if (this.#listeners.has( event )) this.#listeners.get( event )( data )
  //   else console.warn( `Unhandled event: ${event}` )
  // }
}