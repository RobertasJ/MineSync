<script lang="ts">
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";
  import { notifications } from "./notification.js";

  export let themes = {
    danger: "#E26D69",
    success: "#84C991",
    warning: "#f0ad4e",
    info: "#5bc0de",
    default: "#aaaaaa", 
  };
</script>

<div class="notifications">
  {#each $notifications as notification (notification.id)}
    <div
      animate:flip
      class="toast"
      style="background: {themes[notification.type]};"
      transition:fly={{ x: 30 }}>
      {notification.message}
    </div>
  {/each}
</div>

<style>
  .notifications {
    @apply fixed z-50 flex right-0 bottom-0 flex-col-reverse items-start mr-4;
  }

  .toast {
    @apply flex items-center justify-start w-48 h-12 rounded-md text-sm pl-4 mb-4;
  }
</style>
