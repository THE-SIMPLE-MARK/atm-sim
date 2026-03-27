import { createApp } from "vue"
import App from "./App.vue"
import "./globals.css"
import { createMemoryHistory, createRouter } from "vue-router"
import { checkAndSetDarkMode, getMsUntilNextThemeChange } from "./utils/theme"
import CreateAccountView from "./views/CreateAccountView.vue"
import HomeView from "./views/HomeView.vue"
import LoginView from "./views/LoginView.vue"

checkAndSetDarkMode()
setInterval(() => {
	checkAndSetDarkMode()
}, getMsUntilNextThemeChange())

const router = createRouter({
	history: createMemoryHistory(),
	routes: [
		{
			path: "/",
			component: LoginView,
		},
		{
			path: "/create-account",
			component: CreateAccountView,
		},
		{
			path: "/home",
			component: HomeView,
		},
	],
})

createApp(App).use(router).mount("#app")
