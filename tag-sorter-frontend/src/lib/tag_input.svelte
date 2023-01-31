<script lang="ts">
	import { error, json } from '@sveltejs/kit';
	export let tags: string[];
	export let entering: string = '';
	let suggestions: string[];
	let editing: boolean = false;
	let selected_item: number = 0;
	export let search_mode: boolean;
	export let recommended_search_tags: string[];
	let tag_recommendations: string[] = [];
	$: tag_recommendations = recommended_search_tags;
	let show_tag_recom: boolean = false;
	let selected_recom: number = -1;
	$: promise = get_recomended(tags, entering);
	function select_for_editing(i: number) {
		selected_item = i;
		entering = tags[i];
		editing = true;
	}
	async function get_recomended(tags: string[], entering: string) {
		if (search_mode) {
			promise = promise;
		} else {
			const res = await fetch('http://localhost:8000/tag_suggestions', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					current_tags: tags,
					entering: entering
				})
			});
			const val = await res.json();
			if (res.ok) {
				selected_recom = -1;
				if (val.is_ok) {
					tag_recommendations = val.tags;
					
				}else {
					throw new Error(val.error)
				}
			} else {
				throw new Error('Failed to fetch tags');
			}
		}return tag_recommendations
	}
</script>

<div class="tag-input">
	{#each tags as e, i}
		{#if editing && i == selected_item}
			<div>
				<input
					bind:value={entering}
					on:keydown={(e) => {
						switch (e.key) {
							case 'Enter': {
								if (selected_recom == -1) {
									tags[selected_item] = entering.trim();
								} else if (entering.trim() != '') {
									entering = tag_recommendations[selected_recom];
									tags[selected_item] = entering.trim();
								} else {
									break;
								}
								tags = tags;
								entering = '';
								editing = false;
								selected_recom = -1;

								break;
							}
							case 'Down':
							case 'ArrowDown': {
								selected_recom++;
								if (selected_recom >= tag_recommendations.length) {
									selected_recom = tag_recommendations.length - 1;
								}
								break;
							}
							case 'Esc':
							case 'Escape': {
								selected_recom = -1;
								break;
							}
							case 'Up':
							case 'ArrowUp': {
								if (selected_recom == 0) {
									selected_recom = 0;
								} else {
									selected_recom--;
								}
								break;
							}
						}
					}}
				/>
				<button
					on:click={(_) => {
						tags.splice(selected_item, 1);
						tags = tags;
						editing = false;
						promise = promise;
						entering = '';
					}}>Remove</button
				>
			</div>
			{#await promise then _}
				{#if tag_recommendations.length != 0}
					<div class="tag-recommend">
						<ul>
							{#each tag_recommendations as tag, i}
								{#if i == selected_recom}
									<li class="text-cyan-500">
										{tag}
									</li>
								{:else}
									<li>
										{tag}
									</li>
								{/if}
							{/each}
						</ul>
					</div>
				{/if}
			{/await}
		{:else}
			<span
				on:click={(_) => {
					if (search_mode) {
						tags.splice(i, 1);
						tags = tags;
						editing = false;
						promise = promise;
						entering = '';
					} else {
						select_for_editing(i);
					}
				}}
				on:keydown={(e) => {
					if (search_mode) {
						tags.splice(selected_item, 1);
						tags = tags;
						editing = false;
						promise = promise;
						entering = '';
					} else {
						select_for_editing(i);
					}
				}}>{e}, {' '}</span
			>
		{/if}
	{/each}
	<div>
		{#if !editing}
			<input
				type="text"
				bind:value={entering}
				on:blur={(_) => {
					show_tag_recom = false;
				}}
				on:focus={(_) => {
					show_tag_recom = true;
					selected_recom = -1;
				}}
				on:keydown={(e) => {
					switch (e.key) {
						case 'Enter': {
							if (selected_recom == -1 && entering.trim()!="") {
								tags.push(entering.trim());
							} else if (selected_recom!=-1) {
								entering = tag_recommendations[selected_recom];
								tags.push(entering.trim());
							}else {
								break;
							}
							tags = tags;
							entering = '';
							selected_recom = -1;
							break;
						}
						case 'Down':
						case 'ArrowDown': {
							selected_recom++;
							if (selected_recom >= tag_recommendations.length) {
								selected_recom = tag_recommendations.length - 1;
							}
							break;
						}
						case 'Esc':
						case 'Escape': {
							selected_recom = -1;
							break;
						}
						case 'Up':
						case 'ArrowUp': {
							if (selected_recom == 0) {
								selected_recom = 0;
							} else {
								selected_recom--;
							}
							break;
						}
					}
				}}
			/>
		{/if}

		{#await promise then _}
			{#if show_tag_recom && tag_recommendations.length != 0}
				<div class="tag-recommend">
					<ul>
						{#each tag_recommendations as tag, i}
							{#if i == selected_recom}
								<li class="text-cyan-500">
									{tag}
								</li>
							{:else}
								<li>
									{tag}
								</li>
							{/if}
						{/each}
					</ul>
				</div>
			{/if}
		{:catch error}
				<div class="tag-recommend">
					<p style="color: red;"> {error}</p>
				</div>
		{/await}
	</div>
</div>

<style>
	.tag-recommend {
		display: block;
		line-height: 90%;
		background-color: rgb(49, 50, 68);
		color: rgb(205, 214, 244);
		border: 1px solid rgb(108, 112, 134);
		padding: 5px;
		width: max-content;
		position: absolute;
	}
	.tag-input {
		width: min-content;
	}
</style>
