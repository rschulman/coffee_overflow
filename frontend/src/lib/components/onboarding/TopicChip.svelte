<script lang="ts">
	import { STRINGS, formatString } from '$lib/constants/strings';

	interface Props {
		topic: string;
		selected: boolean;
		onToggle: (topic: string) => void;
	}

	let { topic, selected, onToggle }: Props = $props();
</script>

<button
	type="button"
	class="topic-chip"
	class:selected
	on:click={() => onToggle(topic)}
	aria-pressed={selected}
	aria-label={formatString(STRINGS.aria.topicChip, { topic })}
>
	{#if selected}
		<span class="material-icons chip-icon">check_circle</span>
	{/if}
	{topic}
</button>

<style>
	.topic-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: white;
		border: 2px solid #e5e7eb;
		border-radius: 24px;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.2s;
		min-height: 44px;
		font-family: inherit;
	}

	.topic-chip:hover {
		border-color: #9ca3af;
		background: #f9fafb;
	}

	.topic-chip:focus {
		outline: 2px solid #3B82F6;
		outline-offset: 2px;
	}

	.topic-chip.selected {
		background: linear-gradient(135deg, #3B82F6 0%, #A855F7 100%);
		border-color: transparent;
		color: white;
	}

	.chip-icon {
		font-size: 1.125rem;
	}
</style>
