<template>
  <div class="container">Hello World</div>
  <PotatoPage />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import PotatoPage from "./view/potato-clock/index.vue";

export default defineComponent({
  name: "App",
  components: {
    PotatoPage,
  },
});
</script>

<style>
#app {
}
</style>
