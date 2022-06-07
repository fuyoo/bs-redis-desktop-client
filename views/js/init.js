import {devicePixels} from '@sciter'
import {PLATFORM} from '@env'

top_border()
move_to_center()


// fit macos
function top_border() {
    if (PLATFORM == 'OSX') {
        document.style.borderTop = "none"
    }
}

// move window to center
function move_to_center() {
    let _this = Window.this
    let [x, y] = _this.screenBox("workarea", "dimension")
    _this.move(devicePixels(x) / 2 - devicePixels(512), devicePixels(y) / 2 - devicePixels(300))
}