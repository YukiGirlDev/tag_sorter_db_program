<script lang="ts">
	import DataView from './DataView.svelte';
	import Editing from './Editing.svelte';
	import type { Item } from './item';
	import ItemPreview from './ItemPreview.svelte';
	import SearchBox from './SearchBox.svelte';
	import '../app.css';
	import { browser } from '$app/environment';
	import { IP, items_per_page, items_per_page_default } from '$lib/settings';
	import {
		get_db,
		items,
		page_count,
		type SearchRequest
	} from '$lib/get_db';
	let is_editing = false;
	let has_order_changed = false;
	let selected: number | null = null;
	let items_per_page_local = items_per_page_default;
	let search: SearchRequest = {
		name: '',
		tags: [],
		entering: '',
		page: 0,
		items_per_page: items_per_page_local,
		sort: false
	};
	let local_page_count = 0;
	page_count.subscribe((value) => {
		local_page_count = value;
	});
	if (browser) {
		items_per_page.subscribe((value) => {
			search.items_per_page = value
		});
	}
	let selected_item: Item | null = null;
	$: {
		if (selected == null || selected == undefined) {
			selected_item = null;
		} else {
			selected_item = items[selected];
		}
	}
	let img_url = '';
	$: {
		if (selected_item == null || selected_item == undefined) {
			img_url = '';
		} else {
			img_url = selected_item.path;
		}
	}

	$: {
		if (search.page < 1) {
			search.page = 1;
		}
		if (search.page > local_page_count) {
			search.page = 1;
		}
	}
	$: promise = get_db(search, indirect);
	function indirect(new_items: Item[]) {
		if (selected != null) {
			if (selected < new_items.length && selected_item?.id != new_items[selected].id) {
				selected = null;
				selected_item = null;
				img_url = '';
			} else {
				selected_item = null;
				selected_item = new_items[selected]
			}
		}
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>
<table style="width: 100%;">
	<td style="width: 30%; position: relative;">
		<DataView
			bind:is_editing
			bind:promise
			bind:selected
			bind:date_order_reverse={search.sort}
			bind:has_order_changed
		/>
		<div style="position: absolute; bottom: 5px; right: 5px">
			<button
				on:click={(_) => {
					if (search.page > 1) {
						search.page--;
					}
				}}>Last</button
			>
			Page: {search.page}/{local_page_count}
			<button
				on:click={(_) => {
					if (search.page < local_page_count) {
						search.page++;
					}
				}}>Next</button
			>
		</div>
	</td>
	<td style="height: 780px;">
		<ItemPreview bind:img_url />
	</td>
	<tr>
		<td>
			<SearchBox
				bind:is_editing
				bind:search_name={search.name}
				bind:search_date1={search.start_date}
				bind:search_date2={search.end_data}
				bind:search_tags={search.tags}
				bind:promise
				bind:entering_tag={search.entering}
			/>
		</td>
		<td style="padding: 5px;">
			<Editing bind:selected_item bind:promise bind:is_editing />
		</td>
	</tr>
</table>

<style>
	td,
	tr {
		width: 100%;
		border: 1px solid rgb(108, 112, 134);
		border-collapse: collapse;
		vertical-align: top;
	}
	td {
		width: min-content;
		padding-left: 5px;
		padding-right: 5px;
	}
</style>
