<script lang="ts">
	import { onMount } from 'svelte';
	// import * as HoverCard from '$lib/components/ui/hover-card';
	import * as Table from '$lib/components/ui/table';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import ContextMenuItem from '$lib/components/ui/context-menu/context-menu-item.svelte';
	import DiTerminalBadge from 'svelte-icons/di/DiTerminalBadge.svelte';

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
	
	async function handleOpenTerminal(e){
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
</script>

<main>
	<div>
		<h4 class="scroll-m-20 text-xl font-semibold tracking-tight">
			<button on:click={handleClickBack}>
				~{path}
			</button>
		</h4>
		<Table.Root>
			<Table.Caption>File Explorer</Table.Caption>
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-[100px]">type</Table.Head>
					<Table.Head class="w-[100px]">name</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each filenames as item}
					<Table.Row>
						<Table.Cell>
							<p>
								{item.ftype}
							</p>
						</Table.Cell>
						<Table.Cell>
							<ContextMenu.Root>
								<ContextMenu.Trigger>
									<button on:click={handleClick} value={item.name}>
										{item.name}
									</button>
								</ContextMenu.Trigger>
								<ContextMenu.Content>
									<ContextMenu.Item>
									<button on:click={handleOpenTerminal} value={item.name}> Terminal </button>
										<ContextMenu.Shortcut></ContextMenu.Shortcut>
									</ContextMenu.Item>
								</ContextMenu.Content>
							</ContextMenu.Root>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</main>

<style>
	.icon {
		color: red;
		width: 20px;
		height: 20px;
	}
	div {
		/* background-color: #1a1a1a; */
		border-radius: 20px;
		padding: 20px;
		color: black;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.5);
		width: 50vw;
		margin: 20px;
	}
</style>
