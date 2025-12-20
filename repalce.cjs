const  {readFileSync,writeFileSync} = require(  "fs")
const {resolve} = require( "path")

const file = resolve(__dirname, "./src-tauri/tauri.conf.release.json")
const content = readFileSync(file)
const pub = readFileSync(resolve(__dirname, "./key.key.pub"))

const  newContent = content.toString().replace("$PUBKEY", pub.toString())

writeFileSync(file, newContent)
