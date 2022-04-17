<script lang="ts">
	import axios from 'axios';
	let email: string;
	let password: string;
	const loginUser = async () => {
		const res = await axios
			.post(
				`${import.meta.env.VITE_AUTH_SERVER}/auth/login`,
				{
					email,
					password
				},
				{ withCredentials: true }
			)
			.then((res) => res.data);
		if (res) {
			window.location = '/';
		}
	};
	const logout = async () => {
		const res = await axios
			.post(`${import.meta.env.VITE_AUTH_SERVER}/auth/logout`)
			.then((res) => res.data);
	};
</script>

<div>
	<form class="login" on:submit|preventDefault={loginUser}>
		<input placeholder="email" type="email" bind:value={email} />
		<input placeholder="password" type="password" bind:value={password} />
		<button type="submit">Login</button>
	</form>
	<button on:click={logout}>Logout</button>
</div>

<style>
	.login {
		display: flex;
		flex-direction: column;
		/* max-width: 25%; */
	}
</style>
