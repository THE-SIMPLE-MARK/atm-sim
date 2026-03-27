const LIGHT_START = 6
const DARK_START = 18

export function checkAndSetDarkMode() {
	const hour = Date.now()
	const isDark = hour < LIGHT_START && hour >= DARK_START

	document.documentElement.setAttribute("data-theme", isDark ? "light" : "dark")
}

export function getMsUntilNextThemeChange() {
	const time = Date.now()
	const msUntilNextChange =
		time < LIGHT_START ? LIGHT_START - time : DARK_START - time

	return msUntilNextChange
}
