<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import { page } from '$app/stores';

	export let icon: typeof SvelteComponent | undefined = undefined;
	export let text: string;
	export let href: string | undefined = undefined;
	export let onClick: (() => void) | undefined = undefined;
	export let height: string;

	const dispatch = createEventDispatcher();

	function handleClick(event: MouseEvent) {
		if (onClick) {
			event.preventDefault();
			onClick();
		}
		dispatch('click');
	}

	$: isSelected = href === $page.url.pathname;
</script>

<a class="icon-button" class:selected={isSelected} {href} on:click={handleClick}>
	{#if icon}
		<svelte:component this={icon} {height} />
	{/if}
	<h5>{text}</h5>
</a>

<style lang="scss">
	.icon-button {
		border-radius: 8px;
		display: inline-flex;
		gap: 8px;
		align-items: center;
		background-color: var(--light-purple);
		color: var(--white);
		padding: 10px 14px;
		transition: background-color 0.3s ease;
		h5 {
			font-size: 20px;
			font-weight: 700;
		}
	}
	.icon-button:hover {
		background-color: var(--dark-purple);
	}
	.icon-button.selected {
		background-color: var(--pink);
	}
</style>
