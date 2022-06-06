import {PLATFORM} from '@env'
import {add_connection_dialog} from "./dialog";

export default class Database extends Element {
    mode = 1;

    ["on click at #add-btn"](evt, ele) {
        // Window.this.xcall("fetch", "/create", "{}", (res) => {
        //     console.log(res)
        // })
        add_connection_dialog()
            .then(res => {
                console.log(res)
            })
    }

    connectedUI() {

    }

    databaseListUI() {

        let top = PLATFORM === "OSX" ? "20dip" : "12dip"
        return (<div class="nav-bar">
            <div class="add-connection" id="add-btn" style={{"paddingTop": top}}>
                <a styleset="#btn-primary" class="block text-center">添加</a>
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