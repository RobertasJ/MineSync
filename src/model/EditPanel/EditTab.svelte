<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open }  from "@tauri-apps/api/dialog";
  import { sep } from "@tauri-apps/api/path";
  import Tooltip from "./tooltip.svelte";
  import type { Folder } from "src/bindings/Folder";
  import { buttonContent, configChanged, saveFolder } from "../../stores/Shared";
  let folder: Folder | undefined;
  let folderName: string = "";
  $: folderName = $buttonContent;
  (async () => {
    if ($buttonContent != null) {
      folder = await invoke("get_folder", { name: $buttonContent });
    }
  });

  $:  {
    $saveFolder;
    updateFolders();
  }
  
  $: {
    $buttonContent;
    getFolders();
  }

  async function getFolders() {
    if ($buttonContent != null) {
      folder = await invoke("get_folder", { name: $buttonContent });
    }
  }

  async function updateFolders() {
    if (folder != undefined) {
      await invoke("update_folder", { folder: folder, currentFolderName: $buttonContent, newFolderName: folderName });
      $buttonContent = folderName;
      $configChanged += 1;
    }
  }
  async function getDir() {
    const dir = await open({
      multiple: false,
      directory: true,
    });
    if (dir != null) {
      folder.path = dir.toString();
      // get last 2 parts of the path
      // make sure works on linux
      folder.path_shortname = dir.toString().split(sep).slice(-2).join(sep);
      $configChanged += 1;
    }
  }

</script>

<div class="w-fit h-fit mt-20 text-white text-xl ml-4 grid grid-cols-2 gap-8">
  {#if folder != undefined && $buttonContent != null} 
    <div>
      <p>Name of the Folder</p>
      <input class="edit-input" type="text" bind:value={folderName}>
      <p>Repository to Sync From</p>
      <input class="edit-input" type="text" bind:value={folder.repo}>
      <p>Branch of the Repository</p>
      <input class="edit-input" type="text" bind:value={folder.branch}>
      <p>Path to the Folder</p>
      <Tooltip tooltip={folder?.path || ""}>
        <button class="edit-input" on:click={getDir}>
          {folder?.path_shortname || "Not Set"}
        </button>
      </Tooltip>
    </div>
    <div>
      <p class="mb-8"><input class="edit-input" type="checkbox" bind:checked={folder.run_instancesync}>Sync Mods</p>
      <p class="mb-8"><input class="edit-input" type="checkbox" bind:checked={folder.server}>Is a Server</p>
    </div>
  {/if}
</div>

<style>
</style>