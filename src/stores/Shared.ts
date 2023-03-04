import { writable } from 'svelte/store';

export let addButtonValue = writable('');
export let buttonContent = writable(null);
export let configChanged = writable(0);
export let saveFolder = writable(0);