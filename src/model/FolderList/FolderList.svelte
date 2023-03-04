<script lang="ts">
  import AddButton from "./AddButton.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { addButtonValue, buttonContent, configChanged } from "../../stores/Shared";
  import { fly } from "svelte/transition";
  import { notifications } from "../notifications/notification";
  let names: Promise<string[]>;
  $: $configChanged, names = invoke("folder_list");
  let name: string;
  let selectedButton: HTMLButtonElement;

  async function addName(name: string) {
    let folder = await invoke("get_folder", { name: name });
    if (folder != undefined) {
      notifications.warn("Folder already exists", 1000);
      return;
    }
    
    await invoke("add_folder", { name: name });
    $configChanged += 1;
    $buttonContent = name;
  }

  function handleClick(event: MouseEvent) {
    const target = event.target as HTMLButtonElement;
    if (selectedButton) {
      selectedButton.classList.remove('bg-button-hover');
      selectedButton.classList.remove('border-button-hover');
    }
    
    target.classList.add('bg-button-hover');
    target.classList.add('border-button-hover');
    selectedButton = target;
    $buttonContent = target.textContent;
  }
  </script>

<aside class="flex flex-col h-full fixed bg-bg-secondary overflow-y-auto" id="scroll">
  <div class="flex flex-col items-start justify-center mb-10" in:fly={{y: 25, delay: 200, duration: 250}}>
    {#await names then names}
      {#each names as name}
        <button class="btn" on:click={handleClick}>{name}</button>
      {/each}
    {:catch error}
    <button class="btn">{error}</button>
    {/await}

    <AddButton closure={() => {
      addName($addButtonValue);
      // scroll down a buttons length
      const scroll = document.getElementById('scroll');
      scroll.scrollTop += 300;
      
    }} />
  </div>
</aside>

<style>
  ::-webkit-scrollbar {
    @apply w-2;
  }

  ::-webkit-scrollbar-thumb {
    @apply  bg-button-nothover rounded-md;
  }

  ::-webkit-scrollbar-thumb:hover {
    @apply bg-button-hover;
  }

  ::-webkit-scrollbar-thumb:hover {
    @apply bg-button-hover;
  }

  ::-webkit-scrollbar-thumb:active {
    @apply bg-button-clicked;
  }

</style>
