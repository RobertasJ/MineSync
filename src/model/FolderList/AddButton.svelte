<!-- spaghetti code, i know -->
<script lang="ts">
  import { addButtonValue } from "../../stores/Shared";
  import { fly } from "svelte/transition";

  export let closure: (value: string) => void;
  let isEditing: boolean = false;

  function handleInput(event) {
    $addButtonValue = event.target.value;
  }

  function handleBlur() {
    isEditing = false;
    $addButtonValue = '';
  }

  function handleClick() {
    isEditing = true;

    setTimeout(() => {
      const input = document.getElementById('input');
      input.focus();
    }, 0);
  }
</script>

{#if !isEditing}
<!-- make text on button not selectable -->
  <button in:fly={{x: -100, delay: 200, duration: 250}} class="btn" on:click={handleClick}>+</button>
{:else}
  <input class="add-btn"
  type="text"
  id="input"
  bind:value={$addButtonValue}
  on:abort={handleBlur}
  on:input={handleInput}
  on:blur={handleBlur}
  on:keyup={
    (event) => {
      if (event.key === 'Enter') {
        if ($addButtonValue === '') {
          isEditing = false;
          return;
        }

        closure($addButtonValue);
        isEditing = false;
      }
    }
  }>
{/if}