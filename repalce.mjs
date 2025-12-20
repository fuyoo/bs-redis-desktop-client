import  {readFileSync,writeFileSync} from  "fs"
import {resolve} from "path"
const __dirname = import.meta.dirname

const file = resolve(__dirname, "./src-tauri/tauri.conf.release.json")
const content = readFileSync(file)
const pub = readFileSync(resolve(__dirname, "./key.key.pub"))

const  newContent = content.toString().replace("$PUBKEY", pub.toString())

writeFileSync(file, newContent)
