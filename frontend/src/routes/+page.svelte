<script lang="ts">
	import { onMount } from 'svelte';
	// import * as HoverCard from '$lib/components/ui/hover-card';
	import * as Table from "$lib/components/ui/table";
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
			} else {
				console.error('Server responded with an error:', response.status);
			}
		} catch (error) {
			console.error('There was a problem fetching data:', error);
		}
	});

	async function handleClick(e) {
		try {
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

	function removelastsegment(path) {
		const lastslashindex = path.lastIndexOf('/');
		if (lastslashindex === -1) return path; // no '/' found in the string
		return path.substring(0, lastslashindex);
	}
</script>

<main>
	<div>
	<h4 
		class="scroll-m-20 text-xl font-semibold tracking-tight"
		>
		<button
		on:click={handleClickBack}
		>
			{path}
		</button>
	</h4>
	<Table.Root>
		<Table.Caption>
			File Explorer
		</Table.Caption>
		<Table.Header>
			<Table.Row>
				<Table.Head class="w-[100px]">name</Table.Head>
			</Table.Row>
		</Table.Header>	
		<Table.Body>
			{#each filenames as filename}
				<Table.Row>
					<Table.Cell >
						<button
						on:click={handleClick}
						value="{filename.name}"
						>
							{filename.name}
						</button>
					</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>

	</div>
</main>

<style>
	/* main { */
	/* 	font-family: 'Roboto', sans-serif; */
	/* } */
	div {
		/* background-color: #1a1a1a; */
		border-radius: 20px;
		padding: 20px;
		color: black;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.5);
		width: 95vw;
		margin: 20px;
	}
</style>
