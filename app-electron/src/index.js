import "./styles.css";
import "./blueprint.css";
import("./pkg/index.js")
    .then( wasm => {
        console.log("WASM loaded")
    }).catch(console.error);