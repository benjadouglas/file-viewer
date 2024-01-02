<script>
	import { onMount } from 'svelte';
	import * as HoverCard from '$lib/components/ui/hover-card';
	import * as Table from '$lib/components/ui/table';
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
		<link
			href="https://fonts.googleapis.com/css?family=Roboto:400,700&display=swap"
			rel="stylesheet"
		/>
		<h2>
			<button
				class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0"
				on:click={handleClickBack}
			>
				{path}
			</button>
		</h2>
		<br />
		{#each filenames as item}
			<p>
				<button
					class="rounded-sm underline-offset-4 hover:underline focus-visible:outline-2 focus-visible:outline-offset-8 focus-visible:outline-black"
					on:click={handleClick}
					value={item.name}
				>
					{item.name}
				</button>
			</p>
		{/each}
	</div>
</main>

<style>
	main {
		font-family: 'Roboto', sans-serif;
	}
	div {
		background-color: #1a1a1a;
		border-radius: 20px;
		padding: 20px;
		color: white;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.5);
		width: 95vw;
		margin: 20px;
	}
</style>
