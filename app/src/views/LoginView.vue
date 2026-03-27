<script setup lang="ts">
import { ref } from "vue"

const shouldShowLogin = ref<boolean>(false)
</script>

<template>
	<main
		@click="shouldShowLogin=true"
		class="w-full h-screen flex justify-center items-center"
	>
		<div v-if="!shouldShowLogin"><p>Tap anywhere to begin...</p></div>
		<div v-else class="flex flex-col gap-2 items-center">
			<h1 class="text-2xl">Please enter your PIN:</h1>
			<input
				type="password"
				class="bg-secondary w-30 h-fit p-2 text-4xl text-center"
				value="1234"
				maxlength="4"
			>
			<p>Processing...</p>
			<RouterLink to="/home">Go to home</RouterLink>
			<RouterLink to="/create-account">Don't have an account?</RouterLink>
		</div>
	</main>
</template>
