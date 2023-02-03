<script lang="ts">
	import { IP as IP } from '$lib/settings';
	import TagInput from '$lib/tag_input.svelte';
	import { raf } from 'svelte/internal';

	let id = 0;
	let id_start = 0;
	let id_end = 0;
	let name = '';
	let start_name = '';
	let contains_name = '';
	let date = "";
	let date1 = "";
	let date2 = ""; 
	let tag: string[] = [];
	let tag_sub: string[] = [];
	let tag_super: string[] = [];

	let id_enabled = false;
	let id_range_enabled = false;
	let name_enabled = false;
	let name_start_enabled = false;
	let name_contains_enabled = false;
	let date_enabled = false;
	let date_range_enabled = false;
	let tags_enabled = false;
	let tags_sub_enabled = false;
	let tags_super_enabled = false;

	let action_values: any = { set_tags: [] };

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

	async function submit() {
		let inner_action: any = {};
		let inner_value: any = {};
		
		if (id_enabled) {
			inner_action.id = id;
		}
		if (id_range_enabled) {
			inner_action.id_range = {
				start: id_start,
				end: id_end
			};
		}
		if (name_enabled) {
			inner_action.name = name;
		}
		if (name_start_enabled) {
			inner_action.start_name = start_name;
		}
		if (name_contains_enabled) {
			inner_action.contains_name = contains_name;
		}
		if (date_enabled) {
			let date_part = date.split("-")
			inner_action.date = {
				year: date_part[0],
				month: date_part[1],
				day: date_part[2],
			};
		}
		if (date_range_enabled) {
			let date_part_1 = date1.split("-")
			let date_part_2 = date2.split("-")
			inner_action.date_range = {
				start: {
					year: date_part_1[0],
					month: date_part_1[1],
					day: date_part_1[2],
				},
				end: {
					year: date_part_2[0],
					month: date2[1],
					day: date2[2]
				}
			};
		}
		if (tags_enabled) {
			inner_action.tag = tag;
		}
		if (tags_sub_enabled) {
			inner_action.tag_sub= tag_sub
		}
		if (tags_super_enabled) {
			inner_action.tag_super = tag_super;
		}
		switch (selected_action) {
			case Action.Create: {
				break;
			}
			case Action.Edit: {
				break;
			}
			case Action.Delete: {
				inner_value.Delete = inner_action;
				break;
			}
			case Action.SetDate: {
				let date: Date = action_values.set_date;
				inner_value.SetDate = [inner_action, {
					year: date.getFullYear(),
					month: date.getMonth(),
					day: date.getDate(),
				}];
				break;
			}
			case Action.AddTag: {
				inner_value.AddTag = [inner_action, action_values.add_tag];
				break;
			}
			case Action.ReplaceTag: {
				inner_value.ReplaceTag = [inner_action, action_values.old_tag, action_values.new_tag];
				break;
			}
			case Action.RemoveTag: {
				inner_value.RemoveTag = [inner_action, action_values.remove_tag];
				break;
			}
			case Action.SetTags: {
				inner_value.SetTags = [inner_action, action_values.set_tags];
				break;
			}
		}
		let action = {
			version: '0.1.0',
			action: inner_value,
		}
		console.log(JSON.stringify(action));
		const res = await fetch(IP + '/action', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(action)
			});
		const contents = await res.json();
		if (!contents.is_ok) {
			throw new Error(contents.error)
		}
	}
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
									<input type="radio" bind:group={selected_action} value={Action.Delete} />
									Delete
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.SetDate} />
									Set date
								</label><br />
								<label>
									<input type="radio" bind:group={selected_action} value={Action.AddTag} />
									Add tag
								</label>
							</td>
							<td>
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
						{#if selected_action == Action.SetDate}
							Date: <br /><input type="date" bind:value={action_values.set_date} />
						{:else if selected_action == Action.AddTag}
							Tag: <br /><input bind:value={action_values.add_tag} />
						{:else if selected_action == Action.ReplaceTag}
							Old tag: <br /><input bind:value={action_values.old_tag} /><br />
							New tag: <br /><input bind:value={action_values.new_tag} />
						{:else if selected_action == Action.RemoveTag}
							Remove tag: <br /><input bind:value={action_values.remove_tag} />
						{:else if selected_action == Action.SetTags}
							Tags: <br /><TagInput
								bind:tags={action_values.set_tags}
								entering=""
								recommended_search_tags={[]}
								search_mode={false}
							/>
						{/if}
					</td>
				</tr>
				{#if show_filter_selector}
					<tr class="search-box">
						<td class="search-container">
							<div>
								<button on:click={(_) => (id_enabled = !id_enabled)}
									>{#if id_enabled} Hide id {:else} Show id {/if}</button
								>
								{#if id_enabled}
									Id:
									<input class="number" type="number" bind:value={id} />
								{/if}
							</div>
							<div>
								<button on:click={(_) => (id_range_enabled = !id_range_enabled)}
									>{#if id_range_enabled} Hide id range {:else} Show id range {/if}</button
								>
								{#if id_range_enabled}
									<p>Id range:</p>
									From:<input class="number" type="number" bind:value={id_start} />
									To: <input class="number" type="number" bind:value={id_end} />
								{/if}
							</div>
							<div>
								<button on:click={(_) => (name_enabled = !name_enabled)}
									>{#if name_enabled} Hide name {:else} Show name {/if}</button
								>
								{#if name_enabled}
									Name:
									<input bind:value={name} />
								{/if}
							</div>
							<div>
								<button on:click={(_) => (name_start_enabled = !name_start_enabled)}
									>{#if name_start_enabled} Hide start name {:else} Show start name {/if}</button
								>
								{#if name_start_enabled}
									Name starts with:
									<input bind:value={start_name} />
								{/if}
							</div>
							<div>
								<button on:click={(_) => (name_contains_enabled = !name_contains_enabled)}
									>{#if name_contains_enabled}
										Hide contains name
									{:else}
										Show contains name
									{/if}</button
								>
								{#if name_contains_enabled}
									Name contains:
									<input bind:value={contains_name} />
								{/if}
							</div>

							<div>
								<button on:click={(_) => (date_enabled = !date_enabled)}
									>{#if date_enabled}
										Hide date
									{:else}
										Show date
									{/if}</button
								>
								{#if date_enabled}
									Date:
									<input type="date" bind:value={date} />
								{/if}
							</div>
							<div>
								<button on:click={(_) => (date_range_enabled = !date_range_enabled)}
									>{#if date_range_enabled}
										Hide date range
									{:else}
										Show date range
									{/if}</button
								>
								{#if date_range_enabled}
									Date range:
									<input type="date" bind:value={date1} />
									<input type="date" bind:value={date2} />
								{/if}
							</div>
						</td>
						<td class="search-container">
							<div>
								<button on:click={(_) => (tags_enabled = !tags_enabled)}
									>{#if tags_enabled}
										Hide tags
									{:else}
										Show tags
									{/if}</button
								>
								{#if tags_enabled}
									Tags: <TagInput
										bind:tags={tag}
										entering={''}
										search_mode={false}
										recommended_search_tags={[]}
									/>
								{/if}
							</div>
							<div>
								<button on:click={(_) => (tags_sub_enabled = !tags_sub_enabled)}
									>{#if tags_sub_enabled}
										Hide sub tags
									{:else}
										Show sub tags
									{/if}</button
								>
								{#if tags_sub_enabled}
									Tags (subset):<TagInput
										bind:tags={tag_sub}
										entering={''}
										search_mode={false}
										recommended_search_tags={[]}
									/>
								{/if}
							</div>
							<div>
								<button on:click={(_) => (tags_super_enabled = !tags_super_enabled)}
									>{#if tags_super_enabled}
										Hide super tags
									{:else}
										Show super tags
									{/if}</button
								>
								{#if tags_super_enabled}
									Tags (superset): <TagInput
										bind:tags={tag_super}
										entering={''}
										search_mode={false}
										recommended_search_tags={[]}
									/>
								{/if}
							</div>
							<button class="submit" on:click|preventDefault={submit}>Submit Query</button>
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
