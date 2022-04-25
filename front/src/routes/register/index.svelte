<script lang="ts">
	import { goto } from '$app/navigation';

	let email: string;
	let password: string;
	let password_repeat: string;
	const loginUser = async () => {
		const req = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/register`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email,
				password,
				password_repeat
			})
		});
		const res = await req.json();
		if (res.ok) {
			goto('/login');
		}
	};
</script>

<div>
	<form class="register" on:submit|preventDefault={loginUser}>
		<input
			placeholder="email"
			type="email"
			bind:value={email}
			pattern="[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+"
		/>
		<input placeholder="password" type="password" bind:value={password} />
		<input placeholder="password_repeat" type="password" bind:value={password_repeat} />
		<button type="submit">Register</button>
	</form>
</div>

<style>
	.register {
		display: flex;
		flex-direction: column;
		/* max-width: 25%; */
	}
</style>
