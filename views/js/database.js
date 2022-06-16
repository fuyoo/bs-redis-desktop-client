import {PLATFORM} from '@env'
import {connection_dialog, view} from "./dialog";
import {request} from "./util";


export default class Database extends Element {
    mode = 1;
    connectionList = [];

    componentDidMount() {
        this.onGlobalEvent("fetch-connections", this.getConnectionList)
        this.getConnectionList()
    }

    ["on click at #add-btn"](evt, ele) {
        connection_dialog()
            .then(res => {
                if (res) {
                    this.getConnectionList()
                }
            })
    }

    ["on click at #add-btn"](evt, ele) {
        connection_dialog()
            .then(res => {
                if (res) {
                    this.getConnectionList()
                }
            })
    }

    ["on click at .connection-name"]() {
        request("/rdb/info", "*")
            .then(res => {
                console.log(JSON.stringify(res.data.split("\r\n").map(item => {
                        let a = item.split(":")
                        let tmp = {}
                        tmp[a[0]] = a[1]
                        return tmp
                    }
                )))
            })
        this.mode = 2
        this.componentUpdate()
    }

    connectionListUI() {
        if (this.connectionList.length === 0) {
            return <div class="empty">无数据</div>
        }
        return <div class="connection-list">
            {
                this.connectionList.map(item => {
                    return <div class="connection-item">
                        <div class="connection-info">
                            <img src="../icons/client.svg" width="14dip" height="14dip"/>
                            <span class="connection-name">{item.name}</span>
                        </div>
                        <div class="connection-action">
                            <img src="../icons/delete.svg" width="18dip" height="18dip"
                                 onClick={() => this.deleteConnection(item.id)}/>
                            <img src="../icons/edit.svg" width="18dip" height="18dip"
                                 onClick={() => this.editConnection(item)}/>
                        </div>
                    </div>
                })
            }
        </div>
    }

    deleteConnection(id) {
        request("/connection/delete", id)
            .then(res => {
                if (res.code == 200) {
                    this.getConnectionList()
                } else {
                    view.modal(<error>{res.msg}</error>)
                }
            })
    }

    getConnectionList() {
        request("/connection/list")
            .then(res => {
                this.connectionList = res.data
                this.componentUpdate()
            })
    }

    editConnection(params) {
        connection_dialog(params)
            .then(res => {
                if (res) {
                    this.getConnectionList()
                }
            })
    }

    databaseListUI() {
        let top = PLATFORM === "OSX" ? "20dip" : "12dip"
        return (<div class="nav-bar">
            <div class="add-connection" style={{"paddingTop": top}}>
                <a styleset="#btn-primary" id="add-btn" class="block text-center">添加</a>
            </div>
            {this.connectionListUI()}
        </div>)
    }

    connectedUI() {
        return (<div>connected!</div>)
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