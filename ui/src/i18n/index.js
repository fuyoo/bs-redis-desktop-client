import en from "./en"
import zh from "./zh"
export default lang => {
    switch (lang) {
        case "zh": return zh
        default:
           return en
    }
}