
<script lang="ts">
	import { file_ext, file_name, file_name_raw, file_name_without_ext } from './file_operations';

	export let is_editing: boolean;
	export let promise: Promise<any>;
	export let date_order_reverse = false;
	export let has_order_changed = false;
	export let selected: number | null = null;
	function show_2(num: number): string {
		if (num.toString().length == 1) {
			return '0' + num.toString();
		} else {
			return num.toString();
		}
	}
    function show_date(year: number, month: number, day: number): string {
        if (year==0||month==0||day==0) {
            return "None"
        }else {
            return year + "/" + show_2(month) + "/" + show_2(day)
        }
    }
</script>

<table>
	<tr>
		<th>Name</th>
		<th>Type</th>
		<th
			on:click={(e) => {
				if (!is_editing) {
					date_order_reverse = !date_order_reverse;
					has_order_changed = true;
					promise = promise;
				}
			}}
		>
			{#if date_order_reverse}
				Date↑
			{:else}
				Date↓
			{/if}
		</th>
	</tr>
	{#await promise}
		<tr>
			<td>Loading...</td>
		</tr>
	{:then items}
		{#each items as { id, path, year, month, day }, i}
			{#if i == selected}
				<tr
					class="text-cyan-400"
					on:mouseover={(e) => {
						if (!is_editing) {
							selected = i;
						}
					}}
					on:focus={(e) => {
						if (!is_editing) {
							selected = i;
						}
					}}
				>
					<td>{file_name_without_ext(path)[0]}</td>
					<td>{file_ext(path)}</td>
					<td>{show_date(year, month, day)}</td>
				</tr>
			{:else}
				<tr
					on:mouseover={(e) => {
						if (!is_editing) {
							selected = i;
						}
					}}
					on:focus={(e) => {
						if (!is_editing) {
							selected = i;
						}
					}}
				>
					<td>{file_name_raw(path)}</td>
					<td>{file_ext(path)}</td>
					<td>{show_date(year, month, day)}</td>
				</tr>
			{/if}
		{/each}
	{:catch error}
		<p style="color: red;">{error.message}</p>
	{/await}
</table>

<style>
	table,
	th,
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
