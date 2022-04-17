<script lang="ts">
	import axios from 'axios';
	let email: string;
	let password: string;
	let password_repeat: string;
	const loginUser = () => {
		axios.post(`${import.meta.env.VITE_AUTH_SERVER}/auth/register`, {
			email,
			password,
			password_repeat
		});
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
		max-width: 25%;
	}
</style>
