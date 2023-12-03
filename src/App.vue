<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import adminMode from "./pages/adminMode.vue"
import viwerMode from "./pages/viwerMode.vue"
const mode = ref("");

const modeInitial = async () => {
  await invoke("get_launch_parameters").then((e) => {
    if (e[0] === "--admin") {
      mode.value = "admin";
    } else if (e[0] === "--init") {
      mode.value = "init";
    } else {
      mode.value = "viwer";
    }
  });
};
modeInitial();
</script>

<template>
  <adminMode v-if="mode == 'admin'" ></adminMode>
  <viwerMode v-if="mode == 'viwer'" ></viwerMode>
</template>

<style scoped>
</style>
