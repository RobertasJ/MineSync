<script lang="ts">
  export let tooltip: string = "";
  let isHovered = false;

  function mouseOver(event) {
		isHovered = true;
	}
	function mouseLeave() {
		isHovered = false;
	}
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div
	on:mouseover={mouseOver}
  on:mouseleave={mouseLeave} class=" w-fit h-fit">
  <slot />
</div>

{#if isHovered}
	<div class="relative w-0">{tooltip}</div>
{/if}
