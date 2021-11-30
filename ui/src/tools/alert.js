import {ElMessage} from 'element-plus';

const _alert = (t, msg) => {
    ElMessage.closeAll()
    new ElMessage({
        message: msg,
        type: t,
        duration: 2000,
        'show-close': true
    })
}

export const alert = {
    success: msg => _alert('success', msg),
    error: msg => _alert('error', msg),
    warning: msg => _alert('warning', msg),
    info: msg => _alert('info', msg)
}
const install = app => {
    app.config.globalProperties.alert = alert
}
export default {
    install
}