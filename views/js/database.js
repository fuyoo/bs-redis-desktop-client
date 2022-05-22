import * as env from '@env'
console.log(env.PLATFORM)
export default class Database extends Element{
    ["on click at #add-btn"](evt,ele) {
        Window.this.xcall("fetch","/create","{}",(res)=>{
          console.log(res)
        })
    }
    render(){
        let top = "12dip"
        if (env.PLATFORM === "OSX") top = "20dip"
        return (
           <div class="nav-bar">
               <div class="add-connection" id="add-btn" style={{"paddingTop": top}}>
                   <a styleset="#btn-primary" class="block text-center">添加</a>
               </div>

           </div>
       )
    }
}