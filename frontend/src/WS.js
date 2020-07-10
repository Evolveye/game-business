export default class RoomedWebSocked extends WebSocket {
  emit( event, data ) {
    const send = () => this.send( JSON.stringify( { event, data } ) )
    if (this.readyState !== 1) {
      this.addEventListener( `open`, send )
    } else {
      send()
    }
  }

  on( event, listener ) {
    this.addEventListener( `message`, ({ data }) => listener( JSON.parse( data ) ) )
  }
}