import Database from "./database.js"
class App extends Element{
    tab = 0;
    ["on click at .database"](evt, button) {
        //let file = Window.this.selectFile()
        //console.log(file)
    }
    renderContent() {
        return (<div class="content">
            <div class="database">
                <Database />
            </div>
            <div class="context">

            </div>

        </div>)
    }
    renderSetting() {
        return (<div class="content">

        </div>)
    }
    renderTabs(){
        return <div class="sidebar">
            <img class="icon" src="images/icon.png" width="34dip" height="34dip"/>
            <img id="home" class={this.tab === 0 ? "active tab" : "tab"} src="images/home.svg" width="32dip" height="32dip"/>
            <img id="set" class= {this.tab === 1 ? "active tab" : "tab"} src="images/settings.svg" width="30dip" height="30dip"/>
        </div>
    }
    ["on click at #home"](evt,ele) {
        this.componentUpdate({tab:0})
        this.render()
    }
    ["on click at #set"](evt,ele) {
        this.componentUpdate({tab:1})
        this.render()
    }
    render() {
        let content = this.renderContent()
        if (this.tab === 1) {
            content = this.renderSetting()
        }
        return (<div class="app">
            {this.renderTabs()}
            {content}
        </div>)

    }
}

document.body.patch(<App />)
