const view = Window.this
view.isResizable = false

class App extends Element {
    nameChangeFn(evt, ele) {
        this.formData.name = ele.value
    }

    ipChangeFn(evt, ele) {
        this.formData.ip = ele.value
    }
    portChangeFn(evt,ele) {
        this.formData.port = ele.value
    }
    usernameChangeFn(evt,ele){
        this.formData.username = ele.value
    }
    passwordChangeFn(evt,ele){
        console.log(ele.value)
        this.formData.password = ele.value
    }

    formData = {
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
                        onChange={(...arg) => this.nameChangeFn(...arg)}
                        type="text"/></div>
            </div>
            <div class="form-item">
                <div class="label">连接IP</div>
                <div class="input">
                    <input type="text"
                           placeholder="连接IP"
                           onChange={(...arg) => this.ipChangeFn(...arg)}/></div>
            </div>
            <div class="form-item">
                <div class="label">连接端口</div>
                <div class="input"><input
                    placeholder="连接端口"
                    onChange={(...arg) => this.portChangeFn(...arg)}
                    type="text"/></div>
            </div>
            <div class="form-item">
                <div class="label">用户名</div>
                <div class="input">
                    <input
                        placeholder="用户名"
                        onChange={(...arg) => this.usernameChangeFn(...arg)}
                        type="text"/>
                </div>
            </div>
            <div class="form-item">
                <div class="label">密码</div>
                <div class="input">
                    <input
                        placeholder="密码"
                        onChange={(...arg) => this.passwordChangeFn(...arg)}
                        type="text"/>
                </div>
            </div>
            <div class="form-item">
                <div class="label"></div>
                <div class="input">
                    <a id="create" styleset="#btn-primary">创建</a>
                    <a style="margin-left:8dip" styleset="#btn-primary">测试</a>
                </div>
            </div>
        </form>)
    }

    ["on click at #create"](evt,ele) {
        view.parent.xcall("fetch", "/connection/create",JSON.stringify(this.formData),res => {
            console.log(res)
        })
    }
}

document.$("#body").patch(<App/>)