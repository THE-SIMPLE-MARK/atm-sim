import { createApp } from "vue"
import App from "./App.vue"
import "./globals.css"
import { checkAndSetDarkMode, getMsUntilNextThemeChange } from "./utils/theme"

checkAndSetDarkMode()
setInterval(() => {
	checkAndSetDarkMode()
}, getMsUntilNextThemeChange())

createApp(App).mount("#app")
