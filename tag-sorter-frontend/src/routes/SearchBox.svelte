<script lang="ts">
	import { search_tag_recomendations } from '$lib/get_db';
	import TagInput from '$lib/tag_input.svelte';
import { split_with_blank } from './item';

	export let is_editing: boolean;
	export let search_name: string;
	export let entering_tag: string;
	export let search_tags: string[];
	export let search_date1: Date | undefined;
	export let search_date2: Date | undefined;
	export let promise: any;
	let local_search_tags: string[] = [];
	search_tag_recomendations.subscribe(value => {
		local_search_tags = value
	})
</script>

<form>
	<div>
		<label for="name">Name:</label>
		<input type="text" name="name" disabled={is_editing} bind:value={search_name} />
	</div>
	<div>
		{#if !is_editing}
			<TagInput bind:tags={search_tags} search_mode={true} bind:recommended_search_tags={local_search_tags} bind:entering={entering_tag}/>
		{:else}
			<div style="width: min-content;">Tags:  {search_tags.join(',')}{#if search_tags.length!=0}
				,
			{/if}  <input disabled={true} aria-hidden="true"/></div>
		{/if}
		
	</div>
	<div>
		<label for="date-1">Start date:</label>
		<input type="date" name="date-1" disabled={is_editing} bind:value={search_date1} />
	</div>
	<div>
		<label for="date-2">End date:</label>
		<input type="date" name="date-2" disabled={is_editing} bind:value={search_date2} />
	</div>
	<button
		on:click={(_) => {
			promise = promise;
		}}>Refresh</button
	>
</form>