<script lang="ts">
	import Button from '@smui/button';

	interface Props {
		onclick?: () => void;
		type?: 'button' | 'submit' | 'reset';
		disabled?: boolean;
		ariaLabel?: string;
		fullWidth?: boolean;
		children: any;
	}

	let { onclick, type = 'button', disabled = false, ariaLabel, fullWidth = false, children }: Props = $props();
</script>

<Button
	variant="raised"
	class="gradient-button"
	on:click={onclick}
	{type}
	{disabled}
	aria-label={ariaLabel}
	style={fullWidth ? 'width: 100%; min-height: 44px;' : 'min-height: 44px;'}
>
	{@render children()}
</Button>

<style>
	:global(.gradient-button),
	:global(.gradient-button::before),
	:global(.gradient-button .mdc-button__ripple) {
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%) !important;
		color: white !important;
	}

	:global(.gradient-button) {
		border-radius: 4px;
		overflow: hidden;
	}

	:global(.gradient-button:focus) {
		outline: 2px solid #1f2937;
		outline-offset: 2px;
	}
</style>
