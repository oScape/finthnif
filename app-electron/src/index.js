import("./pkg/index.js")
    .then( wasm => {
        console.log("WASM loaded")
    }).catch(console.error);

import styles from "./styles.css";
