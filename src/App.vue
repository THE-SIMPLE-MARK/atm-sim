<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { ref } from "vue"

const greetMsg = ref<string | null>(null)
const name = ref<string | null>(null)

async function greet() {
	// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
	greetMsg.value = await invoke("greet", { name: name.value })
}
</script>

<template>
  <main class="w-full h-screen flex flex-col items-center justify-center gap-2">
    <form class="flex flex-col gap-2" @submit.prevent="greet">
      <input class="bg-gray-100 w-24" v-model="name" placeholder="Enter a name..." />
      <button class="bg-blue-200 h-12" type="submit">Greet</button>
    </form>

    <p>{{ greetMsg }}</p>
  </main>
</template>