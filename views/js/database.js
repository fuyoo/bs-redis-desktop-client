import {PLATFORM} from '@env'
import {connection_dialog, custom_dialog, view} from "./dialog";
import {request} from "./util";


export default class Database extends Element {
    mode = 1;
    connectionList = [];
    activeConnectionInfo = {};
    databases = 0;
    activeServerInfo = {}

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


    getActiveConnectionInfo() {
        request("/rdb/info", "*")
            .then(res => {
                console.log(JSON.stringify(res.data.conn_info))
                this.activeConnectionInfo = res.data.conn_info;
                res.data.info.split("\r\n").map(item => {
                        let a = item.split(":")
                        if (a[0]) {
                            this.activeServerInfo[a[0]] = a[1]
                        }
                    }
                )
                this.mode = 2
            })
            .then(() => {
                request("/rdb/cfg", "databases")
                    .then(res => {
                        this.databases = res.data.databases
                        this.componentUpdate()
                    })
            })
    }

    connectServer(data) {
        request("/rdb/connect", data, true)
            .then(async res => {
                if (res.code == 200) {
                    this.getActiveConnectionInfo()
                } else {
                    await custom_dialog(<error>{res.msg}</error>)
                }
            })
    }

    connectionListUI() {
        if (this.connectionList.length === 0) {
            return <div class="empty">无数据</div>
        }
        return <div class="connection-list">
            {
                this.connectionList.map(item => {
                    return <div class="connection-item">
                        <div class="connection-info" onClick={() => this.connectServer(item)}>
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

    async deleteConnection(id) {
        let res = await custom_dialog(<info>are you sure to delete this record?</info>)
        if (res !== "ok") {
            return
        }
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
        return (<div>
            <div class="database-info">
                <div>name: {this.activeConnectionInfo.name}</div>
                <div>ip:port: {this.activeConnectionInfo.ip}:{this.activeConnectionInfo.port}</div>
                <div>server version: {this.activeServerInfo['redis_version']}</div>
            </div>
            <div class="database-action">
                <a styleset="#btn-danger" onClick={() => {
                    this.mode = 1
                    this.componentUpdate()
                }}>断开</a>
            </div>
            <ul class="database-list">
                {(() => {
                    let li = [];
                    for (let i = 0; i < this.databases; i++) {
                        li.push(<li>
                            <img src="../icons/database.svg" width="16dip" height="16dip"/>
                            <span>{i}</span>
                        </li>)
                    }
                    return li
                })()}
            </ul>
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