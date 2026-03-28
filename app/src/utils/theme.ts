const LIGHT_START = 6
const DARK_START = 18

export function checkAndSetDarkMode() {
	const hour = new Date().getHours()
	const isDark = hour >= DARK_START || hour < LIGHT_START

	document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light")
}

export function getMsUntilNextThemeChange() {
	const now = new Date()
	const nextChange = new Date(now)
	nextChange.setHours(
		now.getHours() >= DARK_START ? LIGHT_START + 24 : DARK_START,
		0,
		0,
		0,
	)

	return nextChange.getTime() - now.getTime()
}
