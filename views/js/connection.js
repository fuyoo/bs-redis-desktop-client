const view = Window.this
view.isResizable = false
document.$("title").innerText = !!view.parameters ? "编辑" : "添加"

class App extends Element {
    componentDidMount() {
        if (view.parameters) {
            this.formData = view.parameters
            this.componentUpdate()
        }
    }

    nameChangeFn(evt, ele) {
        this.formData.name = ele.value
    }

    ipChangeFn(evt, ele) {
        this.formData.ip = ele.value
    }

    portChangeFn(evt, ele) {
        this.formData.port = ele.value
    }

    usernameChangeFn(evt, ele) {
        this.formData.username = ele.value
    }

    passwordChangeFn(evt, ele) {
        this.formData.password = ele.value
    }

    formData = {
        id: Math.random().toString(36),
        name: "",
        ip: "",
        port: "",
        username: "",
        password: ""
    }

    render() {
        return (<form class="form">
            <div class="form-item">
                <div class="label">连接名称</div>
                <div class="input">
                    <input
                        placeholder="连接名称"
                        value={this.formData.name}
                        onChange={(...arg) => this.nameChangeFn(...arg)}
                        type="text"/></div>
            </div>
            <div class="form-item">
                <div class="label">连接IP</div>
                <div class="input">
                    <input type="text"
                           value={this.formData.ip}
                           placeholder="连接IP"
                           onChange={(...arg) => this.ipChangeFn(...arg)}/></div>
            </div>
            <div class="form-item">
                <div class="label">连接端口</div>
                <div class="input"><input
                    placeholder="连接端口"
                    value={this.formData.port}
                    onChange={(...arg) => this.portChangeFn(...arg)}
                    type="text"/></div>
            </div>
            <div class="form-item">
                <div class="label">用户名</div>
                <div class="input">
                    <input
                        value={this.formData.username}
                        placeholder="用户名"
                        onChange={(...arg) => this.usernameChangeFn(...arg)}
                        type="text"/>
                </div>
            </div>
            <div class="form-item">
                <div class="label">密码</div>
                <div class="input">
                    <input
                        value={this.formData.password}
                        placeholder="密码"
                        onChange={(...arg) => this.passwordChangeFn(...arg)}
                        type="text"/>
                </div>
            </div>
            <div class="form-item">
                <div class="label"></div>
                <div class="input">
                    {
                        !!view.parameters
                            ? <a id="edit" styleset="#btn-primary">修改</a>
                            : (<a id="create" styleset="#btn-primary">创建</a>)
                    }
                    <a style="margin-left:8dip" id="test" styleset="#btn-primary">测试</a>
                </div>
            </div>
        </form>)
    }

    ["on click at #create"](evt, ele) {
        view.parent.xcall("fetch", "/connection/create", JSON.stringify(this.formData), res => {
            res = JSON.parse(res)
            if (res.code == 200) {
                view.modal(<info>{res.msg}</info>)
                view.close(true)
            }
        })
    }
    ["on click at #edit"](evt, ele) {
        view.parent.xcall("fetch", "/connection/update", JSON.stringify(this.formData), res => {
            res = JSON.parse(res)
            if (res.code == 200) {
                view.modal(<info>{res.msg}</info>)
                view.close(true)
            }
        })
    }
    ["on click at #test"](evt, ele) {
        view.parent.xcall("fetch", "/connection/test", JSON.stringify(this.formData), res => {

            res = JSON.parse(res)
            if (res.code == 200) {
                view.modal(<info>{res.msg}</info>)
            } else {
                view.modal(<error>{res.msg}</error>)
            }
        })
    }
}

document.$("#body").patch(<App/>)