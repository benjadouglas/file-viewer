<script lang="ts">
	import { onMount } from 'svelte';
	// import * as HoverCard from '$lib/components/ui/hover-card';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import ContextMenuItem from '$lib/components/ui/context-menu/context-menu-item.svelte';
	import DiTerminalBadge from 'svelte-icons/di/DiTerminalBadge.svelte';
	// import Table from '$lib/components/ui/table/table.svelte';

	let data = {};
	let path = '';
	let filenames = [];

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:8080/');
			if (response.ok) {
				data = await response.json();
				path = data.path;
				filenames = data.files;
				// console.log(data);
			} else {
				console.error('Server responded with an error:', response.status);
			}
		} catch (error) {
			console.error('There was a problem fetching data:', error);
		}
	});

	async function handleClick(e) {
		try {
			// console.log(`fetching at 8080 with data.path:${data.path}; e.value:${e.target.value}`)
			const response = await fetch(`http://localhost:8080/?file=${data.path}/${e.target.value}`);
			if (response.ok) {
				data = await response.json();
				path = data.path;
				filenames = data.files;
			} else {
				console.error('Server responded with an error:', response.status);
			}
		} catch (error) {
			console.error('There was a problem fetching data:', error);
		}
	}

	async function handleClickBack(e) {
		try {
			const response = await fetch(`http://localhost:8080/?file=${removelastsegment(data.path)}`);
			if (response.ok) {
				data = await response.json();
				path = data.path;
				filenames = data.files;
			} else {
				console.error('Server responded with an error:', response.status);
			}
		} catch (error) {
			console.error('There was a problem fetching data:', error);
		}
	}

	async function handleOpenTerminal(e) {
		try {
			await fetch(`http://localhost:8080/open/terminal?file=${data.path}/${e.target.value}`);
		} catch (error) {
			console.error('There was a problem opening file:', error);
		}
	}

	function removelastsegment(path) {
		const lastslashindex = path.lastIndexOf('/');
		if (lastslashindex === -1) return path; // no '/' found in the string
		return path.substring(0, lastslashindex);
	}

	// function getScrollProgress() {
	// 	const scrollableDiv = document.getElementById('scrollableDiv');
	//
	// 	const scrollTop = scrollableDiv.scrollTop; // Distance from the top of the div to the top of the scrollable area
	// 	const scrollHeight = scrollableDiv.scrollHeight; // Total scrollable height of the div
	// 	const clientHeight = scrollableDiv.clientHeight; // Visible height of the div
	//
	// 	const scrollProgress = scrollTop / (scrollHeight - clientHeight); // Ratio of scrolled distance to total scrollable distance
	//
	// 	return scrollProgress; // This will be a number between 0 (top) and 1 (bottom)
	// }
	//
	// document.getElementById('scrollableDiv').addEventListener('scroll', () => {
	// 	const progress = getScrollProgress();
	// 	console.log('Scroll progress:', progress);
	// });
</script>

<main>
	<div>
		<table>
			<thead>
				<tr>
					<th class="t-header"> type</th>
					<th class="t-header"> name </th>
				</tr>
			</thead>
			<tbody>
				{#each filenames as item}
					<tr>
						<td>
							{#if item.ftype === 'Directory'}
								<i id="folder-icon" class="glyphicon glyphicon-folder-open"></i>
							{:else if item.ftype === 'File'}
								<i id="file-icon" class="glyphicon glyphicon-file"></i>
							{:else}
								?
							{/if}
						</td>
						<td>
							<button on:click={handleClick} value={item.name}>
								{item.name}
							</button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</main>

<style>
	button:hover{
		color: blue;
		text-decoration: underline;
	}

	#folder-icon {
		color: #f6d265;
	}

	#file-icon {
		color: #3792d7;
	}

	th,
	td {
		text-align: left;
		padding-right: 30px;
	}

	div {
		/* Colors */
		background-color: #f2f0f0;
		color: black;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.5);

		/* Layout */
		border-radius: 20px;
		padding: 20px;
		width: 50vw;
		max-height: 80vh;
		margin: 20px;

		/* Scrollbar */
		overflow-y: scroll;
		scroll-behavior: auto;
		scrollbar-width: none;

		/* Structure */
		position: relative;
	}
</style>
