import React from "react"
import ReactDOM from "react-dom"
import App from "./views/App"

import * as serviceWorker from "./components/serviceWorker"

import "./normalize.css"
import "./index.css"

// ReactDOM.render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>,
//   document.getElementById( `app` )
// )
ReactDOM.render(
  <App />,
  document.getElementById( `app` )
)

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister()
