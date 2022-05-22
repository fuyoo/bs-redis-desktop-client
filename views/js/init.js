import {devicePixels} from "@sciter"
move_to_center()
function move_to_center(){
    let _this = Window.this
    let [x,y] = _this.screenBox("workarea","dimension")
    _this.move(devicePixels(x) /2 -devicePixels(512), devicePixels(y) /2 - devicePixels(300))
}