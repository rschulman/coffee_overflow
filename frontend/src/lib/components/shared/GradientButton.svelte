<script lang="ts">
	import Button from '@smui/button';
	import type { Snippet } from 'svelte';

	interface Props {
		onclick?: () => void;
		type?: 'button' | 'submit' | 'reset';
		disabled?: boolean;
		ariaLabel?: string;
		fullWidth?: boolean;
		children?: Snippet;
		label?: string;
	}

	let { onclick, type = 'button', disabled = false, ariaLabel, fullWidth = false, children, label }: Props = $props();
</script>

<button
	class="gradient-button mdc-button mdc-button--raised"
	{onclick}
	type={type}
	disabled={disabled}
	aria-label={ariaLabel}
	style={fullWidth ? 'width: 100%; min-height: 44px;' : 'min-height: 44px;'}
>
	<span class="mdc-button__label" style="color: white;">
		{#if children}
			{@render children()}
		{:else if label}
			{label}
		{/if}
	</span>
</button>

<style>
	.gradient-button {
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%) !important;
		border: none;
		border-radius: 4px;
		overflow: hidden;
		color: white !important;
		font-family: Inter, system-ui, sans-serif;
		font-size: 1rem;
		padding: 0 1.5rem;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	.gradient-button:hover {
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
		transform: translateY(-1px);
	}

	.gradient-button:active {
		transform: translateY(0);
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	.gradient-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.gradient-button:focus {
		outline: 2px solid #1f2937;
		outline-offset: 2px;
	}

	.gradient-button .mdc-button__label,
	.gradient-button span,
	.gradient-button * {
		color: white !important;
	}
</style>
