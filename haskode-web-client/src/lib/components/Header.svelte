<script lang="ts">
	import { HaskellIcon, CodeIcon, ProfileIcon, PsetsIcon, HomeIcon } from '$lib/assets';
	import IconButton from './IconButton.svelte';

	const haskellIconHeight = '40px';
	const navIconHeight = '28px';
</script>

<header>
	<div id="title">
		<HaskellIcon height={haskellIconHeight} />
		<a href="/"><h1>haskode</h1></a>
	</div>
	<div id="icon-box">
		<IconButton icon={HomeIcon} text={'Home'} href={'/'} height={navIconHeight} />
		<IconButton icon={CodeIcon} text={'Editor'} href={'/editor'} height={navIconHeight} />
		<IconButton icon={PsetsIcon} text={'Psets'} href={'/psets'} height={navIconHeight} />
		<IconButton icon={ProfileIcon} text={'Sign In'} href={'/sign-in'} height={navIconHeight} />
	</div>
</header>

<style lang="scss">
	header {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	#title {
		display: flex;
		align-items: center;
		gap: 12px;
		h1 {
			font-size: 44px;
			font-weight: 900;
		}
		color: var(--dark-purple);
	}
	#icon-box {
		display: flex;
		align-items: center;
		gap: 12px;
	}
</style>
