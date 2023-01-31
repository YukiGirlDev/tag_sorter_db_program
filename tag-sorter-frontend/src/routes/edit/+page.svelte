<script lang="ts">
	import TagInput from '$lib/tag_input.svelte';

	let id = 0;
	let id_start = 0;
	let id_end = 0;
	let name = '';
	let start_name = '';
	let contains_name = '';
	let date = new Date();
	let date1 = new Date();
	let date2 = new Date();
	let tag: string[] = [];
	let tag_sub: string[] = [];
	let tag_super: string[] = [];

	let action_values: any = { set_tags:[] };

	let show_filter_selector = false;
	enum Action {
		Create,
		Edit,
		Delete,
		SetDate,
		AddTag,
		ReplaceTag,
		RemoveTag,
		SetTags
	}
	let selected_action = Action.Create;
	$: {
		if (
			selected_action == Action.Delete ||
			selected_action == Action.SetDate ||
			selected_action == Action.AddTag ||
			selected_action == Action.ReplaceTag ||
			selected_action == Action.RemoveTag ||
			selected_action == Action.SetTags
		) {
			show_filter_selector = true;
		} else {
			show_filter_selector = false;
		}
	}
	let actions_menu = [
		'Create',
		'Edit',
		'Delete',
		'Set date',
		'Add tag',
		'Replace tag',
		'Remove tag',
		'Set tags'
	];
</script>

<div class="outer-container">
	<div class="main-container">
		<form>
			<table class="search-container search-box">
				<tr>
					<td class="search-container">
						<table>
							<td>
								<label>
									<input type="radio" bind:group={selected_action} value={Action.Create} />
									Create
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.Edit} />
									Edit
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.Delete} />
									Delete
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.SetDate} />
									Set date
								</label>
							</td>
							<td>
								<label>
									<input type="radio" bind:group={selected_action} value={Action.AddTag} />
									Add tag
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.ReplaceTag} />
									Replace tag
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.RemoveTag} />
									Remove tag
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.SetTags} />
									Set tags
								</label>
							</td>
						</table>
					</td>
					<td>
						{#if selected_action == Action.Create}
							todo! item
						{:else if selected_action == Action.Edit}
							edit item
						{:else if selected_action == Action.SetDate}
                            Date: <br><input type="date" bind:value={action_values.set_date}/>
						{:else if selected_action == Action.AddTag}
							Tag: <br><input bind:value={action_values.add_tag} />
						{:else if selected_action == Action.ReplaceTag}
							Old tag: <br><input bind:value={action_values.old_tag} /><br>
							New tag: <br><input bind:value={action_values.new_tag} />
						{:else if selected_action == Action.RemoveTag}
							Remove tag: <br><input bind:value={action_values.remove_tag} />
						{:else if selected_action == Action.SetTags}
                            Tags: <br><TagInput bind:tags={action_values.set_tags} entering='' recommended_search_tags={[]} search_mode={false}/>
						{/if}
					</td>
				</tr>
				{#if show_filter_selector}
					<tr class="search-box">
						<td class="search-container">
							<div>
								Id:
								<input class="number" type="number" bind:value={id} />
							</div>
							<div>
								<p>Id range:</p>
								From:<input class="number" type="number" bind:value={id_start} />
								To: <input class="number" type="number" bind:value={id_end} />
							</div>
							<div>
								Name:
								<input bind:value={name} />
							</div>
							<div>
								Name starts with:
								<input bind:value={start_name} />
							</div>
							<div>
								Name contains:
								<input bind:value={contains_name} />
							</div>

							<div>
								Date:
								<input type="date" bind:value={date} />
							</div>
							<div>
								Date range:
								<input type="date" bind:value={date1} />
								<input type="date" bind:value={date2} />
							</div>
						</td>
						<td class="search-container">
							<div>
								Tags: <TagInput
									bind:tags={tag}
									entering={''}
									search_mode={false}
									recommended_search_tags={[]}
								/>
							</div>
							Tags (subset):<TagInput
								bind:tags={tag_sub}
								entering={''}
								search_mode={false}
								recommended_search_tags={[]}
							/>
							Tags (superset): <TagInput
								bind:tags={tag_super}
								entering={''}
								search_mode={false}
								recommended_search_tags={[]}
							/>
							<button class="submit"
								>Submit Query</button
							>
						</td>
					</tr>
				{/if}
			</table>
		</form>
	</div>
</div>

<style>
	.number {
		width: 4rem;
	}
	.outer-container {
		display: flex;
		justify-content: center;
	}
	.search-box {
		border: 2px solid rgb(245, 194, 231);
		border-collapse: collapse;
	}
	.main-container {
		display: flex;
		justify-content: center;
		padding: 1rem;
		margin: 1.5rem;
		width: 50%;
		line-height: 2rem;
		border: 1px solid rgb(245, 194, 231);
		background-color: rgb(49, 50, 68);
	}
	.search-container {
		padding: 2rem;
	}
	.submit {
		position: relative;
		top: 1rem;
	}
</style>
