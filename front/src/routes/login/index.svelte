<script lang="ts">
	import { goto } from '$app/navigation';

	let email: string;
	let password: string;
	let loginError: boolean = false;

	const loginUser = async () => {
		const rawRes = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/login`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email,
				password
			})
		});
		const res = await rawRes.json();

		if (rawRes.ok) {
			goto('/');
		}
	};
</script>

<div>
	<form class="login" on:submit|preventDefault={loginUser}>
		<input placeholder="email" type="email" bind:value={email} />
		<input placeholder="password" type="password" bind:value={password} />
		<button type="submit">Login</button>
	</form>
</div>

<style>
	.login {
		display: flex;
		flex-direction: column;
		/* max-width: 25%; */
	}
</style>
