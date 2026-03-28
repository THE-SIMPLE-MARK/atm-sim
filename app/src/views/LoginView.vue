<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { ref } from "vue"

const shouldShowLogin = ref<boolean>(false)

const accountId = ref<string>("")
const pin = ref<string>("")

async function onSubmit() {
	await invoke("verify_pin", {
		accountId: accountId.value,
		pin: pin.value,
	}).catch(e => console.error("Error caught", e))
}
</script>

<template>
	<main
		@click="shouldShowLogin=true"
		class="w-full h-screen flex justify-center items-center"
	>
		<div v-if="!shouldShowLogin"><p>Tap anywhere to begin...</p></div>
		<form
			@submit.prevent="onSubmit"
			v-else
			class="flex flex-col gap-2 items-center"
		>
			<h1 class="text-2xl">Account ID</h1>
			<input
				type="text"
				class="bg-secondary w-30 h-fit p-2 text-4xl text-center appearance-none"
				maxlength="4"
				pattern="\d*"
				v-model="accountId"
			>

			<h1 class="text-2xl">PIN</h1>
			<input
				type="password"
				class="bg-secondary w-30 h-fit p-2 text-4xl text-center"
				maxlength="4"
				v-model="pin"
			>

			<button type="submit">Submit</button>
		</form>
	</main>
</template>
