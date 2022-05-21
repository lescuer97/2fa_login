<script lang="ts">
	export let type: string = 'text';
	export let placeholder: string = '';

	export let value: string;
	export let error: string | null = null;

	const setType = (node) => {
		node.type = type;
	};
</script>

<div id={placeholder}>
	<input class:error {placeholder} name={value} bind:value use:setType />
	{#if error}
		<small class:error>{error}</small>
	{/if}
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: column;
		row-gap: 5px;
		max-width: 300px;
		input {
			-webkit-appearance: none;
			-ms-appearance: none;
			-moz-appearance: none;
			appearance: none;
			padding: 12px;
			border-radius: 3px;
			width: 250px;
			font-size: 14px;
			border: 1px solid #8a8a8a;
			&:focus {
				background: #f5f4f4;
			}
		}
		.error {
			border: 1px solid red;
		}
		small.error {
			border: none;
			max-width: inherit;
			color: red;
			text-align: left;
			font-size: 0.7rem;
		}
	}
</style>
