<script>
    export let data;

    import { onMount } from 'svelte';

    const url = 'hello?name=onMount';

    let greeting = "";

    async function getHello() {
        const opts = {
            method: "get"
        };
        const response = await fetch(url, opts);
        const hello = await response.text();
        console.log("hello:", hello);

        greeting = hello;
    }

    onMount(() => getHello())    
  </script>

<h1>{greeting}</h1>
  
<h1>{data.hello}</h1>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
