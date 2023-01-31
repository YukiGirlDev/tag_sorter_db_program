<script lang="ts">
	import TagInput from '$lib/tag_input.svelte';
	import { file_name, file_name_raw, file_name_without_ext } from './file_operations';
	import type { Item } from './item';

	export let ip: string;
	export let selected_item: Item | null;
	$: {
		selected_item = selected_item;
		editing_outcome = null;
	}
	export let promise: Promise<any>;
	export let is_editing: boolean;
	let editing_outcome: Promise<void> | null = null;
	let editing_buffer: {
		name: string | undefined;
		date: Date;
		tags: string[];
		ext: string | undefined;
	} = {
		name: '',
		date: new Date(),
		tags: [],
		ext: ''
	};
	function start_editing() {
		if (selected_item != null) {
			is_editing = true;
			let fs = file_name_without_ext(selected_item.path);
			editing_buffer = {
				name: fs[0],
				date: new Date(selected_item.year, selected_item.month, selected_item.day),
				tags: selected_item.tags,
				ext: fs[1]
			};
		}
	}
	async function save_editing() {
		if (selected_item != null) {
			is_editing = false;
			let newpath = selected_item.path;
			if (file_name(newpath) != editing_buffer.name + '.' + editing_buffer.ext) {
				let temp: string[] = newpath.split('/');
				temp.pop();
				temp.push(editing_buffer.name + '.' + editing_buffer.ext);
				newpath = temp.join('/');
			}
			let new_date = {
				year: selected_item.year,
				month: selected_item.month,
				day: selected_item.day
			};
			if (!(editing_buffer.date == null || editing_buffer.date == undefined)) {
				let a = editing_buffer.date.toString().split('-');
				if (a.length == 3) {
					new_date.year = parseInt(a[0]);
					new_date.month = parseInt(a[1]);
					new_date.day = parseInt(a[2]);
				}
			}
			const res = await fetch(ip + '/action', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					version: '0.1.0',
					action: {
						Edit: {
							id: selected_item.id,
							path: newpath,
							year: new_date.year,
							month: new_date.month,
							day: new_date.day,
							tags: editing_buffer.tags
						}
					}
				})
			});
			const success = await res.json();
			if (success.is_ok) {
				promise = promise;
				return;
			} else {
				throw new Error(success.error);
			}
		} else {
			alert('A critical error has ocoured try refreshing the page');
		}
	}
	function cancel_editing() {
		is_editing = false;
		if (typeof selected_item?.tags !== 'undefined') {
			editing_buffer.tags = selected_item.tags;
		}
		promise = promise;
	}
</script>

<table class="bottom-table">
	<td class="bottom-table" style="height: 120px; width: min-content;">
		{#if selected_item == null}
			None selected
		{:else if is_editing}
			<form>
				<div>Id: {selected_item.id}</div>
				<div>
					<label for="edit-name">Name:</label>
					<input name="edit-name" bind:value={editing_buffer.name} />.{editing_buffer.ext}
				</div>
				<div>
					<label for="edit-date">Date:</label>
					<input name="edit-date" type="date" bind:value={editing_buffer.date} />
				</div>
				<div>Path: {selected_item.path}</div>
			</form>
		{:else}
			<div>Id: {selected_item.id}</div>
			<div>Name: {file_name(selected_item.path)}</div>
			<div>
				Date: {selected_item.year}/{selected_item.month}/{selected_item.day}
			</div>
			<div>Path: {selected_item.path}</div>
			<div>tags: [{selected_item.tags.join(', ')}]</div>
		{/if}
	</td>
	{#if selected_item != null && is_editing}
		<td>
			<TagInput bind:tags={editing_buffer.tags} search_mode={false} recommended_search_tags={[]} />
		</td>
	{/if}
	<td class="bottom-table edit-button">
		{#if editing_outcome != null}
			{#await editing_outcome}
				Loading...
			{:then _}
				<p>Edit complete.</p>
			{:catch error}
				<p style="color: red;">{error}</p>
			{/await}
		{/if}

		{#if is_editing}
			<button on:click={cancel_editing}>Cancel</button>
			<button
				on:click={(_) => {
					editing_outcome = save_editing();
				}}>Save</button
			>
		{:else}
			<button disabled={is_editing} on:click={start_editing}>Edit</button>
		{/if}
	</td>
</table>

<style>
	table.bottom-table {
		width: 100%;
	}
	.bottom-table {
		padding: 0px;
		margin: 0px;
		border: none;
	}
	.edit-button {
		vertical-align: bottom;
		text-align: right;
	}
</style>
