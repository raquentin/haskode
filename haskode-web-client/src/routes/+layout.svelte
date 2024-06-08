<script lang="ts">
	import '../app.scss';
	import Header from '$lib/components/Header.svelte';
</script>

<div id="layout">
	<Header></Header>
	<slot />
</div>

<style>
	#layout {
		height: 100vh;
		width: 100vw;
		display: flex;
		flex-direction: column;
		gap: 22px;
		padding: 26px 32px;
	}
</style>
