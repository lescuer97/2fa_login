<script>
	import '../styles/global.css';
</script>

<main class="content">
	<slot />
</main>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
	}
</style>
