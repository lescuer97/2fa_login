<script lang="ts">
	import { goto } from '$app/navigation';

	const logout = async () => {
		const rawRes = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/logout`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			}
		});
		// const res = await rawRes.json();
		if (rawRes.ok) {
			goto('/login');
		}
	};
</script>

<div class="mainPage">
	<h5>Here a video should go</h5>
	<button on:click={logout}>Logout</button>
</div>

<style lang="scss">
	.mainPage {
		padding: 6rem;
		background-color: #fff;
		border-radius: 5px;
		text-align: center;
		box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
	}
	button {
		background: rgb(144, 239, 110);
		padding: 12px;
		border-radius: 3px;
		font-size: 14px;
		border: 1px solid #8a8a8a;
		cursor: pointer;
	}
</style>
