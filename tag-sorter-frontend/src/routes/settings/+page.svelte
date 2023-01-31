<script lang="ts">
	import { browser } from '$app/environment';
	import { items_per_page, items_per_page_default } from '../../lib/settings';
	import type { Settings } from './+page';
	let settings: Settings = {
		items_per_page: items_per_page_default
	};
	if (browser) {
		items_per_page.subscribe((value) => {
			settings.items_per_page = value;
		});
	}

	function set_settings() {
		items_per_page.set(settings.items_per_page);
	}
</script>

<div class="outer-container">
	<div class="main-container">
		<h1><u>Settings</u></h1>
		<form>
			<div>
				<label for="page-count">Number of items per page: </label>
				<input type="number" min="1" name="page-count" bind:value={settings.items_per_page} />
			</div>
			<button on:click={set_settings}>Apply</button>
		</form>
	</div>
</div>

<style>
	.outer-container {
		display: flex;
		justify-content: center;
	}
	.main-container {
		text-align: center;
		margin: 1.5rem;
		width: 50%;
		border: 1px solid rgb(245, 194, 231);
		background-color: rgb(49, 50, 68);
	}
</style>
