import {PLATFORM} from '@env'
import {add_connection_dialog} from "./dialog";
const view = Window.this
export default class Database extends Element {
    mode = 1;

    ["on click at #add-btn"](evt, ele) {
        // Window.this.xcall("fetch", "/create", "{}", (res) => {
        //     console.log(res)
        // })
        add_connection_dialog()
            .then(res => {
                console.log(JSON.stringify(res))
            })
    }

    connectedUI() {

    }
    componentDidMount() {
        view.xcall("fetch", "/connection/list","a",res => {
            console.log(res)
        })
    }
    databaseListUI() {

        let top = PLATFORM === "OSX" ? "20dip" : "12dip"
        return (<div class="nav-bar">
            <div class="add-connection"  style={{"paddingTop": top}}>
                <a styleset="#btn-primary" id="add-btn" class="block text-center">添加</a>
            </div>
            <div class="connection-list">
                <div style={"height:1200dip"}></div>
            </div>
        </div>)
    }

    render() {
        switch (this.mode) {
            case 1:
                return this.databaseListUI()
            case 2:
                return this.connectedUI()
            default:
                this.databaseListUI()
        }
    }
}