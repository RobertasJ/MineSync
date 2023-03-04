<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { buttonContent, configChanged, saveFolder } from "../../stores/Shared";
  import { notifications } from "../notifications/notification";

  async function handleDelete() {
    await invoke("delete_folder", { name: $buttonContent });
    $configChanged += 1;
    $buttonContent = null;

    notifications.success("Deleted", 1000);
  }

  async function handleSave() {
    $saveFolder += 1;

    notifications.success("Saved", 1000);
  }

  async function handleSync() {
    notifications.info("Syncing...", 1000);

    await invoke("sync_folder", { name: $buttonContent });

    notifications.success("Synced", 1000);
  }

</script>
{#if $buttonContent != null}
  <div class="bg-bg-tertiary fixed w-full py-2 flex">
      <button class="btn" on:click={handleDelete}>Delete</button>
      <button class="btn" on:click={handleSave}>Save</button>
      <button class="btn" on:click={handleSync}>Sync</button>
  </div>
{/if}

<style>
</style>