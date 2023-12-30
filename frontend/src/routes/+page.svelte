<script>
    import { onMount } from 'svelte';

    let data = '';

    onMount(async () => {
        try {
            const response = await fetch("http://localhost:8080/");
            if (response.ok) {
                data = await response.text();
            } else {
                console.error('Server responded with an error:', response.status);
            }
        } catch (error) {
            console.error('There was a problem fetching data:', error);
        }
    });
</script>

<style>
	h1 {
		font-family: 'Poppins', sans-serif;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	p {
		font-family: 'Poppins', sans-serif;
		/* align items on the left */
		text-align: left;
	}
</style>
<main>
    <h1>Welcome to your home dir</h1>
    <p>{data}</p>
</main>
