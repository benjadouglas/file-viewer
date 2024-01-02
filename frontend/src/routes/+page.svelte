<script>
import { onMount } from 'svelte';

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
</script>

<main>
	<link href="https://fonts.googleapis.com/css?family=Roboto:400,700&display=swap" rel="stylesheet">
	<h3>{path}</h3>
	<br />
	{#each filenames as item}
		<p><a target="_self">{item.name}</a></p>
	{/each}
</main>

<style>
	main{
		font-family: 'Roboto', sans-serif;
	}
	a:hover {
	  color: red;
	  background-color: transparent;
	  text-decoration: underline;
	  cursor: pointer;
	}
	a:link {
	  color: blue;
	  background-color: transparent;
	  text-decoration: none;
	}
</style>
