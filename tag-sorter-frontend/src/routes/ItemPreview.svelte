<script lang="ts">
	import { IP } from '$lib/settings';
	import { file_ext } from './file_operations';


	export let img_url: string;
</script>

<div>
	{#if img_url == ''}
		<div>No item has been selected</div>
	{:else if file_ext(img_url) == 'mp4'}
		<!-- svelte-ignore a11y-media-has-caption -->
		<video src={IP + '/db/' + img_url} controls style="height: 700px"/>
	{:else}
		<img src={IP + '/db/' + img_url} alt="Preview" style="height: 700px" />
	{/if}
</div>
